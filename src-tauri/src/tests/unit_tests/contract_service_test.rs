use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use mockall::predicate::*;
use std::sync::{Arc, Mutex};

use crate::{
    errors::app_errors::AppError,
    models::{
        contract::{Contract, ContractStatus},
        financial_records::{FinancialRecords, PaymentMethod},
    },
    repositories::{
        contract_repository::{ContractRepository, MockContractRepository},
        financial_records_repository::{
            FinancialRecordsConnectionRepository, MockFinancialRecordsConnectionRepository,
        },
        student_repository::{MockStudentStatusRepository, StudentStatusRepository},
    },
    services::contract_service,
    tests::factories::contract_factory::{valid_contract, valid_create_contract_request},
};

struct ContractServiceRepository {
    contracts: MockContractRepository,
    financial_records: MockFinancialRecordsConnectionRepository,
    student_status: MockStudentStatusRepository,
}

impl ContractServiceRepository {
    fn new() -> Self {
        Self {
            contracts: MockContractRepository::new(),
            financial_records: MockFinancialRecordsConnectionRepository::new(),
            student_status: MockStudentStatusRepository::new(),
        }
    }
}

#[async_trait]
impl ContractRepository for ContractServiceRepository {
    async fn find_all_by_student_id(
        &mut self,
        student_id: i64,
    ) -> Result<Vec<Contract>, sqlx::Error> {
        self.contracts
            .find_all_by_student_id(student_id)
            .await
    }

    async fn insert(
        &mut self,
        contract: Contract,
    ) -> Result<Contract, sqlx::Error> {
        self.contracts.insert(contract).await
    }

    async fn update(
        &mut self,
        contract: Contract,
        id: i64,
    ) -> Result<Contract, sqlx::Error> {
        self.contracts.update(contract, id).await
    }

    async fn belongs_to_student(
        &mut self,
        contract_id: i64,
        student_id: i64,
    ) -> Result<bool, sqlx::Error> {
        self.contracts
            .belongs_to_student(contract_id, student_id)
            .await
    }

    async fn complete_expired(
        &mut self,
        today: NaiveDate,
        updated_at: DateTime<Utc>,
    ) -> Result<Vec<i64>, sqlx::Error> {
        self.contracts
            .complete_expired(today, updated_at)
            .await
    }
}

#[async_trait]
impl FinancialRecordsConnectionRepository for ContractServiceRepository {
    async fn insert(&mut self, record: FinancialRecords) -> Result<FinancialRecords, sqlx::Error> {
        self.financial_records.insert(record).await
    }

    async fn update(
        &mut self,
        id: i64,
        record: FinancialRecords,
    ) -> Result<FinancialRecords, sqlx::Error> {
        self.financial_records.update(id, record).await
    }

    async fn find_all_by_contract_id(
        &mut self,
        contract_id: i64,
    ) -> Result<Vec<FinancialRecords>, sqlx::Error> {
        self.financial_records
            .find_all_by_contract_id(contract_id)
            .await
    }

    async fn delete_pending_by_id(&mut self, id: i64) -> Result<(), sqlx::Error> {
        self.financial_records.delete_pending_by_id(id).await
    }

    async fn generate_monthly_charges(&mut self, today: NaiveDate) -> Result<u64, sqlx::Error> {
        self.financial_records.generate_monthly_charges(today).await
    }
}

#[async_trait]
impl StudentStatusRepository for ContractServiceRepository {
    async fn update_is_active(
        &mut self,
        student_id: i64,
        is_active: bool,
    ) -> Result<(), sqlx::Error> {
        self.student_status
            .update_is_active(student_id, is_active)
            .await
    }
}

fn active_contract_for_student(student_id: i64) -> Contract {
    let mut contract = valid_contract();
    contract.id = Some(77);
    contract.student_id = student_id;
    contract.contract_status = ContractStatus::Active;
    contract
}

fn assert_database_error<T>(result: Result<T, AppError>, expected_message: &str) {
    let Err(AppError::Database { message }) = result else {
        panic!("Esperava erro de banco.");
    };
    assert_eq!(message, expected_message);
}

fn duration_test_repository(
    updating: bool,
) -> ContractServiceRepository {
    let mut repository = ContractServiceRepository::new();

    if updating {
        repository
            .contracts
            .expect_update()
            .with(always(), eq(77))
            .times(1)
            .returning(|mut contract, id| {
                contract.id = Some(id);
                Ok(contract)
            });

        repository
            .financial_records
            .expect_find_all_by_contract_id()
            .with(eq(77))
            .times(1)
            .returning(|_| Ok(Vec::new()));
    } else {
        repository
            .contracts
            .expect_insert()
            .times(1)
            .returning(|mut contract| {
                contract.id = Some(77);
                Ok(contract)
            });
    }

    repository
        .financial_records
        .expect_insert()
        .times(3)
        .returning(Ok);

    repository
        .contracts
        .expect_find_all_by_student_id()
        .with(eq(7))
        .times(1)
        .returning(|student_id| {
            Ok(vec![active_contract_for_student(student_id)])
        });

    repository
        .student_status
        .expect_update_is_active()
        .with(eq(7), eq(true))
        .times(1)
        .returning(|_, _| Ok(()));

    repository
}

#[tokio::test]
async fn find_all_by_student_id_returns_contract_dtos() {
    let mut repository = ContractServiceRepository::new();
    repository
        .contracts
        .expect_find_all_by_student_id()
        .with(eq(7))
        .times(1)
        .returning(|_| Ok(vec![active_contract_for_student(7)]));

    let result = contract_service::find_all_by_student_id(&mut repository, 7)
        .await
        .unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, Some(77));
    assert_eq!(result[0].student_id, 7);
    assert_eq!(result[0].contract_description, "Pacote de treinamento");
}

#[tokio::test]
async fn find_all_by_student_id_maps_repository_error() {
    let mut repository = ContractServiceRepository::new();
    repository
        .contracts
        .expect_find_all_by_student_id()
        .times(1)
        .returning(|_| Err(sqlx::Error::PoolClosed));

    let result = contract_service::find_all_by_student_id(&mut repository, 7).await;

    assert_database_error(result, "Erro ao buscar contratos");
}

#[tokio::test]
async fn insert_pix_package_creates_installments_and_distributes_remaining_cents() {
    let mut repository = ContractServiceRepository::new();
    let mut request = valid_create_contract_request();
    request.contract_dto.amount_cents = 10_001;
    request.contract_dto.installment_count = 3;

    let inserted_records = Arc::new(Mutex::new(Vec::new()));
    let inserted_records_capture = Arc::clone(&inserted_records);

    repository
        .contracts
        .expect_insert()
        .times(1)
        .returning(|mut contract| {
            contract.id = Some(77);
            Ok(contract)
        });
    repository
        .financial_records
        .expect_insert()
        .times(3)
        .returning(move |record| {
            inserted_records_capture.lock().unwrap().push((
                record.installment_number,
                record.amount_cent,
                record.due_date,
                record.contract_id,
            ));
            Ok(record)
        });
    repository
        .contracts
        .expect_find_all_by_student_id()
        .with(eq(7))
        .times(1)
        .returning(|student_id| Ok(vec![active_contract_for_student(student_id)]));
    repository
        .student_status
        .expect_update_is_active()
        .with(eq(7), eq(true))
        .times(1)
        .returning(|_, _| Ok(()));

    let result = contract_service::insert(&mut repository, request)
        .await
        .unwrap();

    assert_eq!(result.id, Some(77));
    assert_eq!(result.student_id, 7);
    let records = inserted_records.lock().unwrap();
    assert_eq!(records.len(), 3);
    assert_eq!(records[0].0, 1);
    assert_eq!(records[1].0, 2);
    assert_eq!(records[2].0, 3);
    assert_eq!(
        records.iter().map(|record| record.1).collect::<Vec<_>>(),
        [3334, 3334, 3333]
    );
    assert_eq!(records.iter().map(|record| record.1).sum::<i32>(), 10_001);
    assert!(records.iter().all(|record| record.3 == Some(77)));
    assert_eq!(records[0].2, NaiveDate::from_ymd_opt(2026, 11, 10).unwrap());
    assert_eq!(records[1].2, NaiveDate::from_ymd_opt(2026, 12, 10).unwrap());
    assert_eq!(records[2].2, NaiveDate::from_ymd_opt(2027, 1, 10).unwrap());
}

#[tokio::test]
async fn insert_credit_card_creates_one_record_for_full_amount() {
    let mut repository = ContractServiceRepository::new();
    let mut request = valid_create_contract_request();
    request.contract_dto.payment_method = PaymentMethod::CreditCard;
    request.contract_dto.installment_count = 6;
    request.financial_record.payment_method = PaymentMethod::CreditCard;

    let inserted_count = Arc::new(Mutex::new(0));
    let inserted_count_capture = Arc::clone(&inserted_count);

    repository
        .contracts
        .expect_insert()
        .times(1)
        .returning(|mut contract| {
            contract.id = Some(77);
            Ok(contract)
        });
    repository
        .financial_records
        .expect_insert()
        .times(1)
        .returning(move |record| {
            assert_eq!(record.amount_cent, 10_000);
            assert_eq!(record.installment_number, 1);
            assert_eq!(record.payment_method, PaymentMethod::CreditCard);
            *inserted_count_capture.lock().unwrap() += 1;
            Ok(record)
        });
    repository
        .contracts
        .expect_find_all_by_student_id()
        .times(1)
        .returning(|student_id| Ok(vec![active_contract_for_student(student_id)]));
    repository
        .student_status
        .expect_update_is_active()
        .with(eq(7), eq(true))
        .times(1)
        .returning(|_, _| Ok(()));

    contract_service::insert(&mut repository, request)
        .await
        .unwrap();

    assert_eq!(*inserted_count.lock().unwrap(), 1);
}

#[tokio::test]
async fn insert_maps_contract_repository_error() {
    let mut repository = ContractServiceRepository::new();
    repository
        .contracts
        .expect_insert()
        .times(1)
        .returning(|_| Err(sqlx::Error::PoolClosed));

    let result = contract_service::insert(&mut repository, valid_create_contract_request()).await;

    assert_database_error(result, "Erro ao criar contrato");
}

#[tokio::test]
async fn insert_rejects_contract_without_id_before_creating_financial_records() {
    let mut repository = ContractServiceRepository::new();
    repository.contracts.expect_insert().times(1).returning(Ok);

    let result = contract_service::insert(&mut repository, valid_create_contract_request()).await;

    assert!(matches!(
        result,
        Err(AppError::Validation { errors })
            if errors.iter().any(|error| error.field == "contract_id")
    ));
}

#[tokio::test]
async fn insert_rejects_zero_installments() {
    let mut repository = ContractServiceRepository::new();
    let mut request = valid_create_contract_request();
    request.contract_dto.installment_count = 0;
    repository
        .contracts
        .expect_insert()
        .times(1)
        .returning(|mut contract| {
            contract.id = Some(77);
            Ok(contract)
        });

    let result = contract_service::insert(&mut repository, request).await;

    assert!(matches!(
        result,
        Err(AppError::Validation { errors })
            if errors.iter().any(|error| error.field == "installment_count")
    ));
}

#[tokio::test]
async fn insert_rejects_unrepresentable_installment_due_date() {
    let mut repository = ContractServiceRepository::new();
    let mut request = valid_create_contract_request();
    request.contract_dto.installment_count = 2;
    request.contract_dto.due_day = Some(31);
    request.financial_record.due_date = NaiveDate::from_ymd_opt(2026, 1, 31).unwrap();
    repository
        .contracts
        .expect_insert()
        .times(1)
        .returning(|mut contract| {
            contract.id = Some(77);
            Ok(contract)
        });

    let result = contract_service::insert(&mut repository, request).await;

    assert!(matches!(
        result,
        Err(AppError::Validation { errors })
            if errors.iter().any(|error| error.field == "due_date")
    ));
}

#[tokio::test]
async fn update_returns_not_found_when_contract_does_not_exist() {
    let mut repository = ContractServiceRepository::new();
    repository
        .contracts
        .expect_update()
        .with(always(), eq(404))
        .times(1)
        .returning(|_, _| Err(sqlx::Error::RowNotFound));

    let result =
        contract_service::update(&mut repository, 404, valid_create_contract_request()).await;

    assert!(matches!(
        result,
        Err(AppError::NotFound { message }) if message == "Contrato não encontrado"
    ));
}

#[tokio::test]
async fn update_maps_other_contract_repository_errors() {
    let mut repository = ContractServiceRepository::new();
    repository
        .contracts
        .expect_update()
        .times(1)
        .returning(|_, _| Err(sqlx::Error::PoolClosed));

    let result =
        contract_service::update(&mut repository, 77, valid_create_contract_request()).await;

    assert_database_error(result, "Erro ao atualizar contrato");
}

#[tokio::test]
async fn update_syncs_new_installments_and_student_status() {
    let mut repository = ContractServiceRepository::new();
    let mut updated_contract = valid_contract();
    updated_contract.id = Some(77);
    updated_contract.amount_cents = 10_000;
    let inserted_count = Arc::new(Mutex::new(0));
    let inserted_count_capture = Arc::clone(&inserted_count);

    repository
        .contracts
        .expect_update()
        .withf(|contract, id| contract.student_id == 7 && *id == 77)
        .times(1)
        .returning(move |_, _| {
            let mut contract = valid_contract();
            contract.id = Some(77);
            Ok(contract)
        });
    repository
        .financial_records
        .expect_find_all_by_contract_id()
        .with(eq(77))
        .times(1)
        .returning(|_| Ok(Vec::new()));
    repository
        .financial_records
        .expect_insert()
        .times(3)
        .returning(move |record| {
            *inserted_count_capture.lock().unwrap() += 1;
            Ok(record)
        });
    repository
        .contracts
        .expect_find_all_by_student_id()
        .with(eq(7))
        .times(1)
        .returning(|student_id| Ok(vec![active_contract_for_student(student_id)]));
    repository
        .student_status
        .expect_update_is_active()
        .with(eq(7), eq(true))
        .times(1)
        .returning(|_, _| Ok(()));

    let result = contract_service::update(&mut repository, 77, valid_create_contract_request())
        .await
        .unwrap();

    assert_eq!(result.id, Some(77));
    assert_eq!(*inserted_count.lock().unwrap(), 3);
}

#[tokio::test]
async fn update_maps_financial_repository_error() {
    let mut repository = ContractServiceRepository::new();
    repository
        .contracts
        .expect_update()
        .times(1)
        .returning(|_, _| {
            let mut contract = valid_contract();
            contract.id = Some(77);
            Ok(contract)
        });
    repository
        .financial_records
        .expect_find_all_by_contract_id()
        .times(1)
        .returning(|_| Err(sqlx::Error::PoolClosed));

    let result =
        contract_service::update(&mut repository, 77, valid_create_contract_request()).await;

    assert_database_error(result, "Erro ao buscar registros financeiros do contrato.");
}

#[tokio::test]
async fn complete_expired_contracts_returns_zero_when_none_expired() {
    let mut repository = ContractServiceRepository::new();

    repository
        .contracts
        .expect_complete_expired()
        .times(1)
        .returning(|_, _| Ok(Vec::new()));

    repository
        .contracts
        .expect_find_all_by_student_id()
        .times(0);

    repository
        .student_status
        .expect_update_is_active()
        .times(0);

    let completed =
        contract_service::complete_expired_contracts(&mut repository)
            .await
            .unwrap();

    assert_eq!(completed, 0);
}

#[tokio::test]
async fn complete_expired_contracts_deactivates_student_without_active_contract() {
    let mut repository = ContractServiceRepository::new();

    repository
        .contracts
        .expect_complete_expired()
        .times(1)
        .returning(|_, _| Ok(vec![7]));

    repository
        .contracts
        .expect_find_all_by_student_id()
        .with(eq(7))
        .times(1)
        .returning(|_| {
            let mut completed = valid_contract();
            completed.contract_status = ContractStatus::Completed;

            let mut cancelled = valid_contract();
            cancelled.contract_status = ContractStatus::Cancelled;

            Ok(vec![completed, cancelled])
        });

    repository
        .student_status
        .expect_update_is_active()
        .with(eq(7), eq(false))
        .times(1)
        .returning(|_, _| Ok(()));

    let completed =
        contract_service::complete_expired_contracts(&mut repository)
            .await
            .unwrap();

    assert_eq!(completed, 1);
}

#[tokio::test]
async fn complete_expired_contracts_keeps_student_with_another_active_contract() {
    let mut repository = ContractServiceRepository::new();

    repository
        .contracts
        .expect_complete_expired()
        .times(1)
        .returning(|_, _| Ok(vec![7]));

    repository
        .contracts
        .expect_find_all_by_student_id()
        .with(eq(7))
        .times(1)
        .returning(|student_id| {
            let mut completed = valid_contract();
            completed.contract_status = ContractStatus::Completed;

            Ok(vec![
                completed,
                active_contract_for_student(student_id),
            ])
        });

    repository
        .student_status
        .expect_update_is_active()
        .with(eq(7), eq(true))
        .times(1)
        .returning(|_, _| Ok(()));

    let completed =
        contract_service::complete_expired_contracts(&mut repository)
            .await
            .unwrap();

    assert_eq!(completed, 1);
}

#[tokio::test]
async fn complete_expired_contracts_syncs_same_student_only_once() {
    let mut repository = ContractServiceRepository::new();

    repository
        .contracts
        .expect_complete_expired()
        .times(1)
        .returning(|_, _| Ok(vec![7, 7]));

    repository
        .contracts
        .expect_find_all_by_student_id()
        .with(eq(7))
        .times(1)
        .returning(|_| {
            let mut contract = valid_contract();
            contract.contract_status = ContractStatus::Completed;

            Ok(vec![contract])
        });

    repository
        .student_status
        .expect_update_is_active()
        .with(eq(7), eq(false))
        .times(1)
        .returning(|_, _| Ok(()));

    let completed =
        contract_service::complete_expired_contracts(&mut repository)
            .await
            .unwrap();

    // Dois contratos encerrados, um único aluno sincronizado.
    assert_eq!(completed, 2);
}

#[tokio::test]
async fn complete_expired_contracts_maps_repository_error() {
    let mut repository = ContractServiceRepository::new();

    repository
        .contracts
        .expect_complete_expired()
        .times(1)
        .returning(|_, _| Err(sqlx::Error::PoolClosed));

    repository
        .contracts
        .expect_find_all_by_student_id()
        .times(0);

    repository
        .student_status
        .expect_update_is_active()
        .times(0);

    let result =
        contract_service::complete_expired_contracts(&mut repository)
            .await;

    assert_database_error(
        result,
        "Erro ao encerrar contratos vencidos.",
    );
}

#[tokio::test]
async fn complete_expired_contracts_maps_student_contract_lookup_error() {
    let mut repository = ContractServiceRepository::new();

    repository
        .contracts
        .expect_complete_expired()
        .times(1)
        .returning(|_, _| Ok(vec![7]));

    repository
        .contracts
        .expect_find_all_by_student_id()
        .with(eq(7))
        .times(1)
        .returning(|_| Err(sqlx::Error::PoolClosed));

    repository
        .student_status
        .expect_update_is_active()
        .times(0);

    let result =
        contract_service::complete_expired_contracts(&mut repository)
            .await;

    assert_database_error(
        result,
        "Erro ao buscar contratos do aluno",
    );
}

#[tokio::test]
async fn complete_expired_contracts_maps_student_status_update_error() {
    let mut repository = ContractServiceRepository::new();

    repository
        .contracts
        .expect_complete_expired()
        .times(1)
        .returning(|_, _| Ok(vec![7]));

    repository
        .contracts
        .expect_find_all_by_student_id()
        .with(eq(7))
        .times(1)
        .returning(|_| {
            let mut contract = valid_contract();
            contract.contract_status = ContractStatus::Completed;

            Ok(vec![contract])
        });

    repository
        .student_status
        .expect_update_is_active()
        .with(eq(7), eq(false))
        .times(1)
        .returning(|_, _| Err(sqlx::Error::PoolClosed));

    let result =
        contract_service::complete_expired_contracts(&mut repository)
            .await;

    assert_database_error(
        result,
        "Erro ao atualizar status do aluno",
    );
}

#[tokio::test]
async fn insert_calculates_package_end_date() {
    let cases = [
        // Seis meses, encerrando no último dia de junho.
        ("2026-01-01", 6, "2026-06-30"),
        // Virada de ano.
        ("2026-11-01", 3, "2027-01-31"),
        // Fevereiro comum.
        ("2027-02-01", 1, "2027-02-28"),
        // Fevereiro bissexto.
        ("2028-02-01", 1, "2028-02-29"),
        // Início no meio do mês.
        ("2026-05-15", 2, "2026-07-14"),
    ];

    for (start, months, expected_end) in cases {
        let mut repository = duration_test_repository(false);
        let mut request = valid_create_contract_request();

        request.contract_dto.start_date =
            NaiveDate::parse_from_str(start, "%Y-%m-%d").unwrap();

        request.contract_dto.duration_months = Some(months);

        // Confirma que o service substitui a data recebida.
        request.contract_dto.end_date = None;

        let result =
            contract_service::insert(&mut repository, request)
                .await
                .unwrap();

        assert_eq!(
            result.end_date,
            Some(
                NaiveDate::parse_from_str(expected_end, "%Y-%m-%d")
                    .unwrap()
            ),
            "Falha para início {start} e duração {months}",
        );
    }
}

#[tokio::test]
async fn update_recalculates_end_date_when_start_or_duration_changes() {
    let cases = [
        // Mesma data inicial, duração diferente.
        ("2026-11-01", 2, "2026-12-31"),
        // Mesma duração da factory, data inicial diferente.
        ("2027-01-01", 6, "2027-06-30"),
        // Ambos diferentes.
        ("2027-03-15", 3, "2027-06-14"),
    ];

    for (start, months, expected_end) in cases {
        let mut repository = duration_test_repository(true);
        let mut request = valid_create_contract_request();

        request.contract_dto.id = Some(77);
        request.contract_dto.start_date =
            NaiveDate::parse_from_str(start, "%Y-%m-%d").unwrap();
        request.contract_dto.duration_months = Some(months);

        let result =
            contract_service::update(&mut repository, 77, request)
                .await
                .unwrap();

        assert_eq!(
            result.end_date,
            Some(
                NaiveDate::parse_from_str(expected_end, "%Y-%m-%d")
                    .unwrap()
            ),
        );
    }
}

#[tokio::test]
async fn insert_and_update_reject_invalid_package_duration() {
    for updating in [false, true] {
        for duration in [None, Some(0), Some(-1), Some(100)] {
            let mut repository = ContractServiceRepository::new();
            let mut request = valid_create_contract_request();

            request.contract_dto.duration_months = duration;

            repository.contracts.expect_insert().times(0);
            repository.contracts.expect_update().times(0);
            repository.financial_records.expect_insert().times(0);

            let result = if updating {
                contract_service::update(&mut repository, 77, request)
                    .await
            } else {
                contract_service::insert(&mut repository, request)
                    .await
            };

            assert!(
                matches!(
                    result,
                    Err(AppError::Validation { errors })
                        if errors.iter().any(
                            |error| error.field == "duration_months"
                        )
                ),
                "Deveria rejeitar duração {duration:?}; update={updating}",
            );
        }
    }
}

#[tokio::test]
async fn insert_does_not_apply_package_duration_to_other_contract_types() {
    use crate::models::contract::{BillingType, ServiceType};

    let cases = [
        (
            ServiceType::Training,
            BillingType::Monthly,
            None,
        ),
        (
            ServiceType::PhysicalAssessment,
            BillingType::Package,
            Some(NaiveDate::from_ymd_opt(2026, 11, 1).unwrap()),
        ),
    ];

    for (service_type, billing_type, end_date) in cases {
        let mut repository = duration_test_repository(false);
        let mut request = valid_create_contract_request();

        request.contract_dto.service_type = service_type;
        request.contract_dto.billing_type = billing_type;
        request.contract_dto.duration_months = None;
        request.contract_dto.end_date = end_date;

        let result =
            contract_service::insert(&mut repository, request)
                .await
                .unwrap();

        assert_eq!(result.end_date, end_date);
        assert_eq!(result.duration_months, None);
    }
}

#[tokio::test]
async fn saving_expired_package_completes_only_active_contracts() {
    let cases = [
        (ContractStatus::Active, ContractStatus::Completed),
        (ContractStatus::Cancelled, ContractStatus::Cancelled),
        (ContractStatus::Completed, ContractStatus::Completed),
    ];

    for updating in [false, true] {
        for (initial_status, expected_status) in cases {
            let mut repository = duration_test_repository(updating);
            let mut request = valid_create_contract_request();

            request.contract_dto.start_date =
                NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
            request.contract_dto.duration_months = Some(1);
            request.contract_dto.contract_status = initial_status;

            let result = if updating {
                contract_service::update(&mut repository, 77, request)
                    .await
                    .unwrap()
            } else {
                contract_service::insert(&mut repository, request)
                    .await
                    .unwrap()
            };

            assert_eq!(
                result.end_date,
                Some(NaiveDate::from_ymd_opt(2000, 1, 31).unwrap()),
            );
            assert_eq!(result.contract_status, expected_status);
        }
    }
}

#[tokio::test]
async fn saving_unexpired_package_keeps_active_status() {
    for updating in [false, true] {
        let mut repository = duration_test_repository(updating);
        let mut request = valid_create_contract_request();

        request.contract_dto.start_date =
            chrono::Local::now().date_naive();
        request.contract_dto.duration_months = Some(6);
        request.contract_dto.contract_status = ContractStatus::Active;

        let result = if updating {
            contract_service::update(&mut repository, 77, request)
                .await
                .unwrap()
        } else {
            contract_service::insert(&mut repository, request)
                .await
                .unwrap()
        };

        assert_eq!(result.contract_status, ContractStatus::Active);
    }
}