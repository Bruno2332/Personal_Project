use chrono::NaiveDate;

use crate::{
    models::{
        contract::{BillingType, ContractStatus, ServiceType},
        financial_records::TransactionStatus,
    },
    repositories::{
        contract_repository::ContractRepository,
        financial_records_repository::{
            FinancialRecordsConnectionRepository, FinancialRecordsPoolRepository,
        },
        student_repository::StudentRepository,
    },
    services::{contract_service, financial_records_service},
    tests::{
        factories::{
            contract_factory::valid_create_contract_request, student_factory::valid_student_dto,
        },
        utils::database::create_test_database,
    },
};

#[tokio::test]
async fn contract_service_creates_contract_and_installments_atomically() {
    let pool = create_test_database().await;
    let student = StudentRepository::insert(
        &pool,
        crate::models::students::Student::from(valid_student_dto()),
    )
    .await
    .unwrap();
    let mut request = valid_create_contract_request();
    request.contract_dto.student_id = student.id.unwrap();
    request.contract_dto.amount_cents = 10_001;
    request.contract_dto.installment_count = 3;

    let mut tx = pool.begin().await.unwrap();
    let contract = contract_service::insert(&mut *tx, request).await.unwrap();
    tx.commit().await.unwrap();

    let id = contract.id.unwrap();
    let mut connection = pool.acquire().await.unwrap();
    let records =
        FinancialRecordsConnectionRepository::find_all_by_contract_id(&mut *connection, id)
            .await
            .unwrap();
    assert_eq!(records.len(), 3);
    assert_eq!(
        records.iter().map(|record| record.amount_cent).sum::<i32>(),
        10_001
    );
    assert_eq!(
        records
            .iter()
            .map(|record| record.installment_number)
            .collect::<Vec<_>>(),
        vec![1, 2, 3]
    );

    drop(connection);
    let first_record = records.into_iter().next().unwrap();
    let record_id = first_record.id.unwrap();
    let mut paid_record =
        crate::dtos::financial_records_dto::FinancialRecordsDto::from(first_record);
    paid_record.status_transaction = TransactionStatus::Paid;
    paid_record.payment_date = Some(NaiveDate::from_ymd_opt(2026, 10, 30).unwrap());
    let mut tx = pool.begin().await.unwrap();
    financial_records_service::update(&mut *tx, record_id, paid_record)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    let summary = financial_records_service::summary_months(&pool)
        .await
        .unwrap();
    assert_eq!(summary[0].month, "2026-10");
    assert_eq!(summary[0].income_cents, 3_334);
}

#[tokio::test]
async fn contract_update_resynchronizes_pending_installments_in_sqlite() {
    let pool = create_test_database().await;
    let student = StudentRepository::insert(
        &pool,
        crate::models::students::Student::from(valid_student_dto()),
    )
    .await
    .unwrap();
    let mut create_request = valid_create_contract_request();
    create_request.contract_dto.student_id = student.id.unwrap();
    create_request.contract_dto.amount_cents = 9_000;
    create_request.contract_dto.installment_count = 3;

    let mut tx = pool.begin().await.unwrap();
    let created = contract_service::insert(&mut *tx, create_request)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    let contract_id = created.id.unwrap();

    let mut update_request = valid_create_contract_request();
    update_request.contract_dto.id = Some(contract_id);
    update_request.contract_dto.student_id = student.id.unwrap();
    update_request.contract_dto.amount_cents = 12_001;
    update_request.contract_dto.installment_count = 3;
    let mut tx = pool.begin().await.unwrap();
    let updated = contract_service::update(&mut *tx, contract_id, update_request)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    assert_eq!(updated.amount_cents, 12_001);
    let mut connection = pool.acquire().await.unwrap();
    let records = FinancialRecordsConnectionRepository::find_all_by_contract_id(
        &mut *connection,
        contract_id,
    )
    .await
    .unwrap();
    assert_eq!(records.len(), 3);
    assert_eq!(
        records
            .iter()
            .map(|record| record.amount_cent)
            .collect::<Vec<_>>(),
        vec![4_001, 4_000, 4_000],
    );
}

#[tokio::test]
async fn monthly_charge_generation_is_idempotent_and_only_targets_active_monthly_training() {
    let pool = create_test_database().await;
    let student = StudentRepository::insert(
        &pool,
        crate::models::students::Student::from(valid_student_dto()),
    )
    .await
    .unwrap();
    let student_id = student.id.unwrap();

    let mut monthly = crate::tests::factories::contract_factory::valid_contract();
    monthly.id = None;
    monthly.student_id = student_id;
    monthly.billing_type = BillingType::Monthly;
    monthly.service_type = ServiceType::Training;
    monthly.contract_status = ContractStatus::Active;
    monthly.start_date = NaiveDate::from_ymd_opt(2026, 10, 1).unwrap();
    monthly.first_due_date = Some(NaiveDate::from_ymd_opt(2026, 10, 10).unwrap());
    monthly.due_day = Some(10);

    let mut tx = pool.begin().await.unwrap();
    let saved = ContractRepository::insert(&mut *tx, monthly).await.unwrap();
    let mut package = crate::tests::factories::contract_factory::valid_contract();
    package.id = None;
    package.student_id = student_id;
    package.billing_type = BillingType::Package;
    ContractRepository::insert(&mut *tx, package).await.unwrap();
    let mut cancelled = crate::tests::factories::contract_factory::valid_contract();
    cancelled.id = None;
    cancelled.student_id = student_id;
    cancelled.billing_type = BillingType::Monthly;
    cancelled.contract_status = ContractStatus::Cancelled;
    ContractRepository::insert(&mut *tx, cancelled)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    let contract_id = saved.id.unwrap();

    let today = NaiveDate::from_ymd_opt(2026, 11, 1).unwrap();
    let mut tx = pool.begin().await.unwrap();
    let first_count =
        FinancialRecordsConnectionRepository::generate_monthly_charges(&mut *tx, today)
            .await
            .unwrap();
    tx.commit().await.unwrap();
    assert_eq!(first_count, 1);

    let mut tx = pool.begin().await.unwrap();
    let second_count =
        FinancialRecordsConnectionRepository::generate_monthly_charges(&mut *tx, today)
            .await
            .unwrap();
    tx.commit().await.unwrap();
    assert_eq!(second_count, 0);

    let mut connection = pool.acquire().await.unwrap();
    let rows = FinancialRecordsConnectionRepository::find_all_by_contract_id(
        &mut *connection,
        contract_id,
    )
    .await
    .unwrap();
    drop(connection);
    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].due_date,
        NaiveDate::from_ymd_opt(2026, 11, 10).unwrap()
    );
    let competence: String =
        sqlx::query_scalar("SELECT competence_month FROM financial_records WHERE contract_id = $1")
            .bind(contract_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(competence, "2026-11");

    let summary = financial_records_service::summary_months(&pool)
        .await
        .unwrap();
    assert_eq!(summary.len(), 1);
    assert_eq!(summary[0].month, "2026-11");
    assert_eq!(summary[0].receivable_cents, rows[0].amount_cent as i64);
    assert_eq!(rows[0].status_transaction, TransactionStatus::Pending);

    let options = FinancialRecordsPoolRepository::contract_options(&pool)
        .await
        .unwrap();
    assert_eq!(options.len(), 3);

    let page = financial_records_service::find_all(
        &pool,
        0,
        20,
        None,
        Some(true),
        Some(crate::models::financial_records::TransactionType::Income),
        Some("Pending".into()),
        None,
        Some(contract_id),
    )
    .await
    .unwrap();
    assert_eq!(page.total, 1);
    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].student_id, Some(student_id));
}

#[tokio::test]
async fn monthly_adjustment_preserves_existing_charges_and_changes_next_month() {
    use crate::{
        dtos::contract_dto::ContractDto,
        models::students::Student,
        tests::factories::contract_factory::valid_contract,
    };

    let pool = create_test_database().await;

    let student = StudentRepository::insert(
        &pool,
        Student::from(valid_student_dto()),
    )
        .await
        .unwrap();

    let student_id = student.id.unwrap();

    let mut monthly = valid_contract();
    monthly.id = None;
    monthly.student_id = student_id;
    monthly.billing_type = BillingType::Monthly;
    monthly.service_type = ServiceType::Training;
    monthly.contract_status = ContractStatus::Active;
    monthly.amount_cents = 10_000;
    monthly.installment_count = 1;
    monthly.duration_months = None;
    monthly.end_date = None;
    monthly.start_date =
        NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
    monthly.first_due_date =
        Some(NaiveDate::from_ymd_opt(2026, 1, 10).unwrap());
    monthly.due_day = Some(10);

    let mut tx = pool.begin().await.unwrap();

    let saved = ContractRepository::insert(
        &mut *tx,
        monthly,
    )
        .await
        .unwrap();

    let contract_id = saved.id.unwrap();

    for month in 1..=3 {
        let inserted =
            FinancialRecordsConnectionRepository::generate_monthly_charges(
                &mut *tx,
                NaiveDate::from_ymd_opt(2026, month, 1).unwrap(),
            )
                .await
                .unwrap();

        assert_eq!(inserted, 1);
    }

    let mut records =
        FinancialRecordsConnectionRepository::find_all_by_contract_id(
            &mut *tx,
            contract_id,
        )
            .await
            .unwrap();

    records.sort_by_key(|record| record.due_date);
    assert_eq!(records.len(), 3);

    for (index, mut record) in records.into_iter().enumerate() {
        record.status_transaction = match index {
            0 => TransactionStatus::Paid,
            1 => TransactionStatus::Cancelled,
            _ => TransactionStatus::Pending,
        };

        record.payment_date = if index == 0 {
            Some(NaiveDate::from_ymd_opt(2026, 1, 10).unwrap())
        } else {
            None
        };

        FinancialRecordsConnectionRepository::update(
            &mut *tx,
            record.id.unwrap(),
            record,
        )
            .await
            .unwrap();
    }

    let mut before =
        FinancialRecordsConnectionRepository::find_all_by_contract_id(
            &mut *tx,
            contract_id,
        )
            .await
            .unwrap();

    before.sort_by_key(|record| record.id);

    // Compara todos os campos dos lançamentos existentes.
    let before_snapshot = serde_json::to_value(&before).unwrap();

    tx.commit().await.unwrap();

    let mut request = valid_create_contract_request();
    request.contract_dto = ContractDto::from(saved);
    request.contract_dto.amount_cents = 15_000;

    let mut tx = pool.begin().await.unwrap();

    let updated = contract_service::update(
        &mut *tx,
        contract_id,
        request,
    )
        .await
        .unwrap();

    assert_eq!(updated.amount_cents, 15_000);

    let mut after =
        FinancialRecordsConnectionRepository::find_all_by_contract_id(
            &mut *tx,
            contract_id,
        )
            .await
            .unwrap();

    after.sort_by_key(|record| record.id);

    assert_eq!(
        serde_json::to_value(&after).unwrap(),
        before_snapshot,
        "O reajuste não deve modificar cobranças existentes."
    );

    let april = NaiveDate::from_ymd_opt(2026, 4, 1).unwrap();

    let inserted =
        FinancialRecordsConnectionRepository::generate_monthly_charges(
            &mut *tx,
            april,
        )
            .await
            .unwrap();

    assert_eq!(inserted, 1);

    let inserted_again =
        FinancialRecordsConnectionRepository::generate_monthly_charges(
            &mut *tx,
            april,
        )
            .await
            .unwrap();

    assert_eq!(inserted_again, 0);

    let records =
        FinancialRecordsConnectionRepository::find_all_by_contract_id(
            &mut *tx,
            contract_id,
        )
            .await
            .unwrap();

    assert_eq!(records.len(), 4);

    let new_charge = records
        .iter()
        .find(|record| {
            record.due_date
                == NaiveDate::from_ymd_opt(2026, 4, 10).unwrap()
        })
        .unwrap();

    assert_eq!(new_charge.amount_cent, 15_000);
    assert_eq!(
        new_charge.status_transaction,
        TransactionStatus::Pending,
    );

    let new_charge_id = new_charge.id;

    let mut old_charges: Vec<_> = records
        .into_iter()
        .filter(|record| record.id != new_charge_id)
        .collect();

    old_charges.sort_by_key(|record| record.id);

    assert_eq!(
        serde_json::to_value(&old_charges).unwrap(),
        before_snapshot,
    );

    tx.commit().await.unwrap();
}

#[tokio::test]
async fn package_adjustment_still_rejects_changes_to_paid_or_cancelled_installments() {
    use crate::{
        errors::app_errors::AppError,
        models::students::Student,
    };

    for status in [
        TransactionStatus::Paid,
        TransactionStatus::Cancelled,
    ] {
        let pool = create_test_database().await;

        let student = StudentRepository::insert(
            &pool,
            Student::from(valid_student_dto()),
        )
            .await
            .unwrap();

        let student_id = student.id.unwrap();

        let mut request = valid_create_contract_request();
        request.contract_dto.student_id = student_id;
        request.contract_dto.amount_cents = 9_000;
        request.contract_dto.installment_count = 3;

        let mut tx = pool.begin().await.unwrap();

        let created = contract_service::insert(
            &mut *tx,
            request,
        )
            .await
            .unwrap();

        let contract_id = created.id.unwrap();

        let mut records =
            FinancialRecordsConnectionRepository::find_all_by_contract_id(
                &mut *tx,
                contract_id,
            )
                .await
                .unwrap();

        let mut record = records.remove(0);
        record.status_transaction = status;
        record.payment_date = if status == TransactionStatus::Paid {
            Some(record.due_date)
        } else {
            None
        };

        FinancialRecordsConnectionRepository::update(
            &mut *tx,
            record.id.unwrap(),
            record,
        )
            .await
            .unwrap();

        let mut before =
            FinancialRecordsConnectionRepository::find_all_by_contract_id(
                &mut *tx,
                contract_id,
            )
                .await
                .unwrap();

        before.sort_by_key(|record| record.id);
        let snapshot = serde_json::to_value(&before).unwrap();

        tx.commit().await.unwrap();

        let mut request = valid_create_contract_request();
        request.contract_dto = created;
        request.contract_dto.amount_cents = 12_000;

        let mut tx = pool.begin().await.unwrap();

        let result = contract_service::update(
            &mut *tx,
            contract_id,
            request,
        )
            .await;

        assert!(
            matches!(result, Err(AppError::Validation { .. })),
            "Deveria impedir alteração de parcela {status:?}."
        );

        tx.rollback().await.unwrap();

        let mut connection = pool.acquire().await.unwrap();

        let contracts = ContractRepository::find_all_by_student_id(
            &mut *connection,
            student_id,
        )
            .await
            .unwrap();

        let contract = contracts
            .iter()
            .find(|contract| contract.id == Some(contract_id))
            .unwrap();

        assert_eq!(contract.amount_cents, 9_000);

        let mut after =
            FinancialRecordsConnectionRepository::find_all_by_contract_id(
                &mut *connection,
                contract_id,
            )
                .await
                .unwrap();

        after.sort_by_key(|record| record.id);

        assert_eq!(
            serde_json::to_value(&after).unwrap(),
            snapshot,
        );
    }
}
