use chrono::{Local, NaiveDate, Utc};

use crate::{
    errors::app_errors::AppError,
    handlers::contract_handler,
    models::{
        contract::{BillingType, ContractStatus, ServiceType},
        financial_records::FinancialRecords,
        students::Student,
    },
    repositories::{
        contract_repository::ContractRepository,
        financial_records_repository::FinancialRecordsConnectionRepository,
        student_repository::StudentRepository,
    },
    tests::{
        factories::{
            contract_factory::{
                valid_contract,
                valid_financial_record_dto,
            },
            student_factory::valid_student_dto,
        },
        utils::database::create_test_database,
    },
};

#[tokio::test]
async fn expiration_query_only_completes_expired_active_training_packages() {
    let pool = create_test_database().await;

    let student = StudentRepository::insert(
        &pool,
        Student::from(valid_student_dto()),
    )
        .await
        .unwrap();

    let student_id = student.id.unwrap();
    let today = NaiveDate::from_ymd_opt(2027, 7, 1).unwrap();
    let yesterday = today.pred_opt().unwrap();
    let tomorrow = today.succ_opt().unwrap();

    let cases = [
        (
            "Pacote vencido",
            ContractStatus::Active,
            ServiceType::Training,
            BillingType::Package,
            Some(yesterday),
            ContractStatus::Completed,
        ),
        (
            "Termina hoje",
            ContractStatus::Active,
            ServiceType::Training,
            BillingType::Package,
            Some(today),
            ContractStatus::Active,
        ),
        (
            "Termina amanhã",
            ContractStatus::Active,
            ServiceType::Training,
            BillingType::Package,
            Some(tomorrow),
            ContractStatus::Active,
        ),
        (
            "Cancelado",
            ContractStatus::Cancelled,
            ServiceType::Training,
            BillingType::Package,
            Some(yesterday),
            ContractStatus::Cancelled,
        ),
        (
            "Já concluído",
            ContractStatus::Completed,
            ServiceType::Training,
            BillingType::Package,
            Some(yesterday),
            ContractStatus::Completed,
        ),
        (
            "Mensalidade",
            ContractStatus::Active,
            ServiceType::Training,
            BillingType::Monthly,
            Some(yesterday),
            ContractStatus::Active,
        ),
        (
            "Avaliação física",
            ContractStatus::Active,
            ServiceType::PhysicalAssessment,
            BillingType::Package,
            Some(yesterday),
            ContractStatus::Active,
        ),
        (
            "Sem término",
            ContractStatus::Active,
            ServiceType::Training,
            BillingType::Package,
            None,
            ContractStatus::Active,
        ),
    ];

    let mut tx = pool.begin().await.unwrap();
    let mut expected = Vec::new();

    for (
        description,
        status,
        service_type,
        billing_type,
        end_date,
        expected_status,
    ) in cases
    {
        let mut contract = valid_contract();

        contract.id = None;
        contract.student_id = student_id;
        contract.contract_description = description.to_string();
        contract.contract_status = status;
        contract.service_type = service_type;
        contract.billing_type = billing_type;
        contract.end_date = end_date;

        let saved = ContractRepository::insert(
            &mut *tx,
            contract,
        )
            .await
            .unwrap();

        expected.push((saved.id.unwrap(), expected_status));
    }

    let affected = ContractRepository::complete_expired(
        &mut *tx,
        today,
        Utc::now(),
    )
        .await
        .unwrap();

    assert_eq!(affected, vec![student_id]);

    tx.commit().await.unwrap();

    let mut connection = pool.acquire().await.unwrap();

    let contracts = ContractRepository::find_all_by_student_id(
        &mut *connection,
        student_id,
    )
        .await
        .unwrap();

    assert_eq!(contracts.len(), expected.len());

    for (id, expected_status) in expected {
        let contract = contracts
            .iter()
            .find(|contract| contract.id == Some(id))
            .unwrap();

        assert_eq!(
            contract.contract_status,
            expected_status,
            "Status incorreto: {}",
            contract.contract_description,
        );
    }

    // Repetir a operação não encerra novamente o mesmo contrato.
    let affected_again = ContractRepository::complete_expired(
        &mut *connection,
        today,
        Utc::now(),
    )
        .await
        .unwrap();

    assert!(affected_again.is_empty());
}

#[tokio::test]
async fn expiration_handler_syncs_student_and_preserves_financial_records() {
    // Executa os dois cenários em bancos independentes.
    for has_another_active_contract in [false, true] {
        let pool = create_test_database().await;

        let mut student_dto = valid_student_dto();
        student_dto.is_active = true;

        let student = StudentRepository::insert(
            &pool,
            Student::from(student_dto),
        )
            .await
            .unwrap();

        let student_id = student.id.unwrap();
        let mut tx = pool.begin().await.unwrap();

        // Inserção direta pelo repository prepara um contrato que
        // ficou vencido enquanto o aplicativo estava fechado.
        let mut expired = valid_contract();
        expired.id = None;
        expired.student_id = student_id;
        expired.start_date =
            NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
        expired.end_date =
            Some(NaiveDate::from_ymd_opt(2000, 6, 30).unwrap());
        expired.contract_status = ContractStatus::Active;

        let expired = ContractRepository::insert(
            &mut *tx,
            expired,
        )
            .await
            .unwrap();

        let expired_id = expired.id.unwrap();

        let other_id = if has_another_active_contract {
            let mut other = valid_contract();
            other.id = None;
            other.student_id = student_id;
            other.start_date = Local::now().date_naive();
            other.end_date = Some(
                other.start_date
                    .checked_add_months(chrono::Months::new(6))
                    .unwrap(),
            );
            other.contract_status = ContractStatus::Active;

            Some(
                ContractRepository::insert(&mut *tx, other)
                    .await
                    .unwrap()
                    .id
                    .unwrap(),
            )
        } else {
            None
        };

        let mut financial_dto = valid_financial_record_dto();
        financial_dto.contract_id = Some(expired_id);

        FinancialRecordsConnectionRepository::insert(
            &mut *tx,
            FinancialRecords::from(financial_dto),
        )
            .await
            .unwrap();

        // Snapshot de todas as colunas atuais dos registros financeiros.
        let before: Vec<FinancialRecords> =
            FinancialRecordsConnectionRepository::find_all_by_contract_id(
                &mut *tx,
                expired_id,
            )
                .await
                .unwrap();

        let before_json = sqlx::query_scalar::<_, String>(
            r#"
            SELECT json_object(
                'id', id,
                'contract_id', contract_id,
                'transaction_type', transaction_type,
                'description', description,
                'amount_cent', amount_cent,
                'due_date', due_date,
                'payment_date', payment_date,
                'payment_method', payment_method,
                'installment_number', installment_number,
                'status_transaction', status_transaction,
                'competence_month', competence_month,
                'created_at', created_at,
                'updated_at', updated_at
            )
            FROM financial_records
            WHERE contract_id = $1
            ORDER BY id
            "#,
        )
            .bind(expired_id)
            .fetch_all(&mut *tx)
            .await
            .unwrap();

        assert_eq!(before.len(), 1);
        tx.commit().await.unwrap();

        let completed =
            contract_handler::complete_expired_contracts(&pool)
                .await
                .unwrap();

        assert_eq!(completed, 1);

        let student = StudentRepository::find_by_id(
            &pool,
            student_id,
        )
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            student.is_active,
            has_another_active_contract,
        );

        let mut connection = pool.acquire().await.unwrap();

        let contracts = ContractRepository::find_all_by_student_id(
            &mut *connection,
            student_id,
        )
            .await
            .unwrap();

        let expired = contracts
            .iter()
            .find(|contract| contract.id == Some(expired_id))
            .unwrap();

        assert_eq!(
            expired.contract_status,
            ContractStatus::Completed,
        );

        if let Some(other_id) = other_id {
            let other = contracts
                .iter()
                .find(|contract| contract.id == Some(other_id))
                .unwrap();

            assert_eq!(
                other.contract_status,
                ContractStatus::Active,
            );
        }

        let after_json = sqlx::query_scalar::<_, String>(
            r#"
            SELECT json_object(
                'id', id,
                'contract_id', contract_id,
                'transaction_type', transaction_type,
                'description', description,
                'amount_cent', amount_cent,
                'due_date', due_date,
                'payment_date', payment_date,
                'payment_method', payment_method,
                'installment_number', installment_number,
                'status_transaction', status_transaction,
                'competence_month', competence_month,
                'created_at', created_at,
                'updated_at', updated_at
            )
            FROM financial_records
            WHERE contract_id = $1
            ORDER BY id
            "#,
        )
            .bind(expired_id)
            .fetch_all(&mut *connection)
            .await
            .unwrap();

        assert_eq!(after_json, before_json);
        drop(connection);

        let completed_again =
            contract_handler::complete_expired_contracts(&pool)
                .await
                .unwrap();

        assert_eq!(completed_again, 0);
    }
}

#[tokio::test]
async fn expiration_handler_rolls_back_when_student_update_fails() {
    let pool = create_test_database().await;

    let mut student_dto = valid_student_dto();
    student_dto.is_active = true;

    let student = StudentRepository::insert(
        &pool,
        Student::from(student_dto),
    )
        .await
        .unwrap();

    let student_id = student.id.unwrap();
    let mut tx = pool.begin().await.unwrap();

    let mut contract = valid_contract();
    contract.id = None;
    contract.student_id = student_id;
    contract.start_date =
        NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    contract.end_date =
        Some(NaiveDate::from_ymd_opt(2000, 6, 30).unwrap());
    contract.contract_status = ContractStatus::Active;

    let saved = ContractRepository::insert(
        &mut *tx,
        contract,
    )
        .await
        .unwrap();

    tx.commit().await.unwrap();

    // Falha provocada somente no banco isolado deste teste.
    sqlx::query(
        r#"
        CREATE TRIGGER fail_student_status_update
        BEFORE UPDATE OF is_active ON students
        BEGIN
            SELECT RAISE(ABORT, 'Falha simulada no status do aluno');
        END
        "#,
    )
        .execute(&pool)
        .await
        .unwrap();

    let result =
        contract_handler::complete_expired_contracts(&pool).await;

    assert!(
        matches!(
            result,
            Err(AppError::Database { message })
                if message == "Erro ao atualizar status do aluno"
        )
    );

    let mut connection = pool.acquire().await.unwrap();

    let contracts = ContractRepository::find_all_by_student_id(
        &mut *connection,
        student_id,
    )
        .await
        .unwrap();

    let contract = contracts
        .iter()
        .find(|contract| contract.id == saved.id)
        .unwrap();

    // O UPDATE do contrato aconteceu antes da falha, mas foi desfeito.
    assert_eq!(contract.contract_status, ContractStatus::Active);
    assert_eq!(contract.updated_at, saved.updated_at);

    drop(connection);

    let student = StudentRepository::find_by_id(
        &pool,
        student_id,
    )
        .await
        .unwrap()
        .unwrap();

    assert!(student.is_active);
}