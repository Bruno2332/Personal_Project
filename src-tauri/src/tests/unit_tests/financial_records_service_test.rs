use chrono::{NaiveDate, Utc};
use mockall::predicate::*;

use crate::{
    dtos::financial_records_dto::FinancialRecordsDto,
    errors::app_errors::AppError,
    models::financial_records::{FinancialRecords, TransactionStatus, TransactionType},
    repositories::financial_records_repository::{
        MockFinancialRecordsConnectionRepository, MockFinancialRecordsPoolRepository,
    },
    services::financial_records_service,
    tests::factories::financial_records_factory::{
        financial_contract_option, financial_month, financial_record, financial_record_projection,
        valid_financial_record_dto,
    },
};

fn assert_database_error<T>(result: Result<T, AppError>, expected_message: &str) {
    let Err(AppError::Database { message }) = result else {
        panic!("Esperava erro de banco.");
    };
    assert_eq!(message, expected_message);
}

fn assert_validation_error<T>(result: Result<T, AppError>, expected_field: &str) {
    let Err(AppError::Validation { errors }) = result else {
        panic!("Esperava erro de validação.");
    };
    assert!(errors.iter().any(|error| error.field == expected_field));
}

fn desired_record(installment_number: i32, amount_cent: i32) -> FinancialRecordsDto {
    let mut record = valid_financial_record_dto();
    record.contract_id = None;
    record.installment_number = installment_number;
    record.amount_cent = amount_cent;
    record.description = format!("Parcela atualizada {installment_number}");
    record
}

fn current_record(
    id: Option<i64>,
    installment_number: i32,
    status: TransactionStatus,
) -> FinancialRecords {
    financial_record(id, installment_number, status)
}

#[tokio::test]
async fn find_all_returns_page_and_forwards_all_filters() {
    let mut repository = MockFinancialRecordsPoolRepository::new();

    repository
        .expect_find_all_paginated()
        .withf(
            |page, size, name, monthly, kind, status, period, contract_id| {
                *page == 2
                    && *size == 25
                    && name.as_deref() == Some("Bruno")
                    && *monthly == Some(true)
                    && *kind == Some(TransactionType::Income)
                    && status.as_deref() == Some("Paid")
                    && period.as_deref() == Some("3m")
                    && *contract_id == Some(20)
            },
        )
        .times(1)
        .returning(|_, _, _, _, _, _, _, _| {
            Ok(vec![financial_record_projection()])
        });

    repository
        .expect_count_all()
        .withf(|name, monthly, kind, status, period, contract_id| {
            name.as_deref() == Some("Bruno")
                && *monthly == Some(true)
                && *kind == Some(TransactionType::Income)
                && status.as_deref() == Some("Paid")
                && period.as_deref() == Some("3m")
                && *contract_id == Some(20)
        })
        .times(1)
        .returning(|_, _, _, _, _, _| Ok(51));

    let result = financial_records_service::find_all(
        &repository,
        2,
        25,
        Some("Bruno".to_string()),
        Some(true),
        Some(TransactionType::Income),
        Some("Paid".to_string()),
        Some("3m".to_string()),
        Some(20),
    )
        .await
        .unwrap();

    assert_eq!(result.page, 2);
    assert_eq!(result.size, 25);
    assert_eq!(result.total, 51);
    assert_eq!(result.items.len(), 1);
    assert_eq!(
        result.items[0].student_name.as_deref(),
        Some("Aluno de teste"),
    );
    assert_eq!(result.items[0].amount_cent, 5_000);
}

#[tokio::test]
async fn find_all_rejects_invalid_pagination_before_repository_call() {
    for (page, size) in [(-1, 20), (0, 0), (0, 501), (i64::MAX, 2)] {
        let repository = MockFinancialRecordsPoolRepository::new();

        let result = financial_records_service::find_all(
            &repository,
            page,
            size,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await;

        assert_validation_error(result, "page");
    }
}

#[tokio::test]
async fn find_all_rejects_unknown_status_before_repository_call() {
    let repository = MockFinancialRecordsPoolRepository::new();

    let result = financial_records_service::find_all(
        &repository,
        0,
        20,
        None,
        None,
        None,
        Some("Unknown".to_string()),
        None,
        None,
    )
    .await;

    assert_validation_error(result, "status");
}

#[tokio::test]
async fn find_all_rejects_invalid_period_before_repository_call() {
    for period in ["2026-1", "2026-13", "abc-def"] {
        let repository = MockFinancialRecordsPoolRepository::new();

        let result = financial_records_service::find_all(
            &repository,
            0,
            20,
            None,
            None,
            None,
            None,
            Some(period.to_string()),
            None,
        )
        .await;

        assert_validation_error(result, "period");
    }
}

#[tokio::test]
async fn find_all_maps_list_repository_error() {
    let mut repository = MockFinancialRecordsPoolRepository::new();
    repository
        .expect_find_all_paginated()
        .times(1)
        .returning(|_, _, _, _, _, _, _, _| Err(sqlx::Error::PoolClosed));

    let result =
        financial_records_service::find_all(&repository, 0, 20, None, None, None, None, None, None)
            .await;

    assert_database_error(result, "Erro ao buscar registro financeiros");
}

#[tokio::test]
async fn find_all_maps_count_repository_error() {
    let mut repository = MockFinancialRecordsPoolRepository::new();
    repository
        .expect_find_all_paginated()
        .times(1)
        .returning(|_, _, _, _, _, _, _, _| Ok(Vec::new()));
    repository
        .expect_count_all()
        .times(1)
        .returning(|_, _, _, _, _, _| Err(sqlx::Error::PoolClosed));

    let result =
        financial_records_service::find_all(&repository, 0, 20, None, None, None, None, None, None)
            .await;

    assert_database_error(result, "Erro ao contar registros financeiros");
}

#[tokio::test]
async fn insert_returns_dto_from_created_record() {
    let mut repository = MockFinancialRecordsConnectionRepository::new();
    repository.expect_insert().times(1).returning(|mut record| {
        assert_eq!(record.id, None);
        assert_eq!(record.amount_cent, 12_500);
        record.id = Some(31);
        Ok(record)
    });

    let result = financial_records_service::insert(&mut repository, valid_financial_record_dto())
        .await
        .unwrap();

    assert_eq!(result.id, Some(31));
    assert_eq!(result.description, "Mensalidade de teste");
}

#[tokio::test]
async fn insert_maps_repository_error() {
    let mut repository = MockFinancialRecordsConnectionRepository::new();
    repository
        .expect_insert()
        .times(1)
        .returning(|_| Err(sqlx::Error::PoolClosed));

    let result =
        financial_records_service::insert(&mut repository, valid_financial_record_dto()).await;

    assert_database_error(result, "Erro ao criar registro financeiro");
}

#[tokio::test]
async fn update_returns_dto_from_updated_record_and_forwards_id() {
    let mut repository = MockFinancialRecordsConnectionRepository::new();
    repository
        .expect_update()
        .withf(|id, record| *id == 31 && record.amount_cent == 12_500)
        .times(1)
        .returning(|id, mut record| {
            record.id = Some(id);
            Ok(record)
        });

    let result =
        financial_records_service::update(&mut repository, 31, valid_financial_record_dto())
            .await
            .unwrap();

    assert_eq!(result.id, Some(31));
}

#[tokio::test]
async fn update_returns_not_found_for_missing_record() {
    let mut repository = MockFinancialRecordsConnectionRepository::new();
    repository
        .expect_update()
        .with(eq(404), always())
        .times(1)
        .returning(|_, _| Err(sqlx::Error::RowNotFound));

    let result =
        financial_records_service::update(&mut repository, 404, valid_financial_record_dto()).await;

    assert!(matches!(
        result,
        Err(AppError::NotFound { message })
            if message == "Registro financeiro não encontrado"
    ));
}

#[tokio::test]
async fn update_maps_other_repository_error() {
    let mut repository = MockFinancialRecordsConnectionRepository::new();
    repository
        .expect_update()
        .times(1)
        .returning(|_, _| Err(sqlx::Error::PoolClosed));

    let result =
        financial_records_service::update(&mut repository, 31, valid_financial_record_dto()).await;

    assert_database_error(result, "Erro ao atualizar registro financeiro");
}

#[tokio::test]
async fn summary_months_returns_repository_data() {
    let mut repository = MockFinancialRecordsPoolRepository::new();
    repository
        .expect_summary_months()
        .times(1)
        .returning(|| Ok(vec![financial_month()]));

    let result = financial_records_service::summary_months(&repository)
        .await
        .unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].month, "2026-11");
    assert_eq!(result[0].income_cents, 20_000);
    assert_eq!(result[0].payable_cents, 2_000);
}

#[tokio::test]
async fn summary_months_maps_repository_error() {
    let mut repository = MockFinancialRecordsPoolRepository::new();
    repository
        .expect_summary_months()
        .times(1)
        .returning(|| Err(sqlx::Error::PoolClosed));

    let result = financial_records_service::summary_months(&repository).await;

    assert_database_error(result, "Erro ao buscar resumo financeiro");
}

#[tokio::test]
async fn contract_options_returns_repository_data() {
    let mut repository = MockFinancialRecordsPoolRepository::new();
    repository
        .expect_contract_options()
        .times(1)
        .returning(|| Ok(vec![financial_contract_option()]));

    let result = financial_records_service::contract_options(&repository)
        .await
        .unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].value, 20);
    assert_eq!(result[0].label, "Aluno de teste — Plano mensal");
}

#[tokio::test]
async fn contract_options_maps_repository_error() {
    let mut repository = MockFinancialRecordsPoolRepository::new();
    repository
        .expect_contract_options()
        .times(1)
        .returning(|| Err(sqlx::Error::PoolClosed));

    let result = financial_records_service::contract_options(&repository).await;

    assert_database_error(result, "Erro ao buscar opções de contrato");
}

#[tokio::test]
async fn sync_contract_records_inserts_missing_installment() {
    let mut repository = MockFinancialRecordsConnectionRepository::new();
    repository
        .expect_find_all_by_contract_id()
        .with(eq(20))
        .times(1)
        .returning(|_| Ok(Vec::new()));
    repository.expect_insert().times(1).returning(|record| {
        assert_eq!(record.id, None);
        assert_eq!(record.contract_id, Some(20));
        assert_eq!(record.installment_number, 1);
        Ok(record)
    });

    financial_records_service::sync_contract_records(
        &mut repository,
        20,
        vec![desired_record(1, 5_000)],
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn sync_contract_records_updates_pending_installment_and_preserves_identity_fields() {
    let mut repository = MockFinancialRecordsConnectionRepository::new();
    let original_created_at = Utc::now() - chrono::Duration::days(15);
    let original_payment_date = Some(NaiveDate::from_ymd_opt(2026, 11, 8).unwrap());

    repository
        .expect_find_all_by_contract_id()
        .times(1)
        .returning(move |_| {
            let mut current = current_record(Some(44), 1, TransactionStatus::Pending);
            current.created_at = original_created_at.clone();
            current.payment_date = original_payment_date;
            Ok(vec![current])
        });
    repository
        .expect_update()
        .withf(move |id, record| {
            *id == 44
                && record.id == Some(44)
                && record.contract_id == Some(20)
                && record.created_at == original_created_at
                && record.payment_date == original_payment_date
                && record.status_transaction == TransactionStatus::Pending
                && record.description == "Parcela atualizada 1"
        })
        .times(1)
        .returning(|_, record| Ok(record));

    financial_records_service::sync_contract_records(
        &mut repository,
        20,
        vec![desired_record(1, 5_000)],
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn sync_contract_records_keeps_unchanged_paid_installment() {
    let mut repository = MockFinancialRecordsConnectionRepository::new();
    repository
        .expect_find_all_by_contract_id()
        .times(1)
        .returning(|_| Ok(vec![current_record(Some(44), 1, TransactionStatus::Paid)]));

    financial_records_service::sync_contract_records(
        &mut repository,
        20,
        vec![desired_record(1, 5_000)],
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn sync_contract_records_rejects_changes_to_paid_installment() {
    let mut repository = MockFinancialRecordsConnectionRepository::new();
    repository
        .expect_find_all_by_contract_id()
        .times(1)
        .returning(|_| Ok(vec![current_record(Some(44), 1, TransactionStatus::Paid)]));

    let result = financial_records_service::sync_contract_records(
        &mut repository,
        20,
        vec![desired_record(1, 6_000)],
    )
    .await;

    assert_validation_error(result, "financial_records");
}

#[tokio::test]
async fn sync_contract_records_rejects_removing_cancelled_installment() {
    let mut repository = MockFinancialRecordsConnectionRepository::new();
    repository
        .expect_find_all_by_contract_id()
        .times(1)
        .returning(|_| {
            Ok(vec![current_record(
                Some(44),
                2,
                TransactionStatus::Cancelled,
            )])
        });

    let result =
        financial_records_service::sync_contract_records(&mut repository, 20, Vec::new()).await;

    assert_validation_error(result, "financial_records");
}

#[tokio::test]
async fn sync_contract_records_deletes_removed_pending_installment() {
    let mut repository = MockFinancialRecordsConnectionRepository::new();
    repository
        .expect_find_all_by_contract_id()
        .times(1)
        .returning(|_| {
            Ok(vec![current_record(
                Some(44),
                2,
                TransactionStatus::Pending,
            )])
        });
    repository
        .expect_delete_pending_by_id()
        .with(eq(44))
        .times(1)
        .returning(|_| Ok(()));

    financial_records_service::sync_contract_records(&mut repository, 20, Vec::new())
        .await
        .unwrap();
}

#[tokio::test]
async fn sync_contract_records_maps_find_error() {
    let mut repository = MockFinancialRecordsConnectionRepository::new();
    repository
        .expect_find_all_by_contract_id()
        .times(1)
        .returning(|_| Err(sqlx::Error::PoolClosed));

    let result =
        financial_records_service::sync_contract_records(&mut repository, 20, Vec::new()).await;

    assert_database_error(result, "Erro ao buscar registros financeiros do contrato.");
}

#[tokio::test]
async fn sync_contract_records_maps_insert_error() {
    let mut repository = MockFinancialRecordsConnectionRepository::new();
    repository
        .expect_find_all_by_contract_id()
        .times(1)
        .returning(|_| Ok(Vec::new()));
    repository
        .expect_insert()
        .times(1)
        .returning(|_| Err(sqlx::Error::PoolClosed));

    let result = financial_records_service::sync_contract_records(
        &mut repository,
        20,
        vec![desired_record(1, 5_000)],
    )
    .await;

    assert_database_error(result, "Erro ao criar registro financeiro");
}

#[tokio::test]
async fn sync_contract_records_maps_update_error() {
    let mut repository = MockFinancialRecordsConnectionRepository::new();
    repository
        .expect_find_all_by_contract_id()
        .times(1)
        .returning(|_| {
            Ok(vec![current_record(
                Some(44),
                1,
                TransactionStatus::Pending,
            )])
        });
    repository
        .expect_update()
        .times(1)
        .returning(|_, _| Err(sqlx::Error::PoolClosed));

    let result = financial_records_service::sync_contract_records(
        &mut repository,
        20,
        vec![desired_record(1, 5_000)],
    )
    .await;

    assert_database_error(result, "Erro ao atualizar registro financeiro");
}

#[tokio::test]
async fn sync_contract_records_maps_delete_error() {
    let mut repository = MockFinancialRecordsConnectionRepository::new();
    repository
        .expect_find_all_by_contract_id()
        .times(1)
        .returning(|_| {
            Ok(vec![current_record(
                Some(44),
                2,
                TransactionStatus::Pending,
            )])
        });
    repository
        .expect_delete_pending_by_id()
        .times(1)
        .returning(|_| Err(sqlx::Error::PoolClosed));

    let result =
        financial_records_service::sync_contract_records(&mut repository, 20, Vec::new()).await;

    assert_database_error(result, "Erro ao remover parcela pendente.");
}

#[tokio::test]
async fn sync_contract_records_rejects_existing_record_without_id() {
    let mut repository = MockFinancialRecordsConnectionRepository::new();
    repository
        .expect_find_all_by_contract_id()
        .times(1)
        .returning(|_| Ok(vec![current_record(None, 1, TransactionStatus::Pending)]));

    let result = financial_records_service::sync_contract_records(
        &mut repository,
        20,
        vec![desired_record(1, 5_000)],
    )
    .await;

    assert_database_error(result, "Registro financeiro encontrado sem ID.");
}

#[tokio::test]
async fn generate_monthly_charges_forwards_today_and_returns_count() {
    let mut repository = MockFinancialRecordsConnectionRepository::new();
    let today = chrono::Local::now().date_naive();

    repository
        .expect_generate_monthly_charges()
        .with(eq(today))
        .times(1)
        .returning(|_| Ok(4));

    let result = financial_records_service::generate_monthly_charges(&mut repository)
        .await
        .unwrap();

    assert_eq!(result, 4);
}

#[tokio::test]
async fn generate_monthly_charges_maps_repository_error() {
    let mut repository = MockFinancialRecordsConnectionRepository::new();
    repository
        .expect_generate_monthly_charges()
        .times(1)
        .returning(|_| Err(sqlx::Error::PoolClosed));

    let result = financial_records_service::generate_monthly_charges(&mut repository).await;

    assert_database_error(result, "Erro ao gerar mensalidades");
}
