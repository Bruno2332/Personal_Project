use crate::{
    dtos::{physical_assessment_dto::PhysicalAssessmentDto, vo2_assessment_dto::Vo2AssessmentDto},
    models::students::Student,
    repositories::{
        contract_repository::ContractRepository,
        financial_records_repository::FinancialRecordsConnectionRepository,
        physical_assessment_repository::PhysicalAssessmentPoolRepository,
        student_repository::StudentRepository,
        vo2_assessment_repository::Vo2AssessmentPoolRepository,
    },
    services::physical_assessment_service,
    tests::{
        factories::{
            contract_factory::valid_create_contract_request,
            physical_assessment_factory::{valid_physical_assessment, valid_vo2_assessment},
            student_factory::valid_student_dto,
        },
        utils::database::create_test_database,
    },
};

#[tokio::test]
async fn create_assessment_service_saves_contract_assessment_and_vo2_in_one_transaction() {
    let pool = create_test_database().await;
    let student = StudentRepository::insert(&pool, Student::from(valid_student_dto()))
        .await
        .unwrap();
    let student_id = student.id.unwrap();

    let mut contract_request = valid_create_contract_request();
    contract_request.contract_dto.student_id = student_id;
    let assessment = PhysicalAssessmentDto::from(valid_physical_assessment());
    let mut stage = Vo2AssessmentDto::from(valid_vo2_assessment());
    stage.id = None;
    let mut tx = pool.begin().await.unwrap();
    let created =
        physical_assessment_service::insert(&mut *tx, contract_request, assessment, vec![stage])
            .await
            .unwrap();
    tx.commit().await.unwrap();

    let assessment_id = created.id.unwrap();
    assert_eq!(created.student_id, student_id);
    let mut connection = pool.acquire().await.unwrap();
    let contracts = ContractRepository::find_all_by_student_id(&mut *connection, student_id)
        .await
        .unwrap();
    assert_eq!(contracts.len(), 1);
    let contract_id = contracts[0].id.unwrap();
    drop(connection);
    assert_eq!(
        PhysicalAssessmentPoolRepository::find_all_by_student_id(&pool, student_id)
            .await
            .unwrap()
            .len(),
        1
    );
    let stages = Vo2AssessmentPoolRepository::find_all_by_student_id(&pool, student_id)
        .await
        .unwrap();
    assert_eq!(stages.len(), 1);
    assert_eq!(stages[0].physical_assessment_id, assessment_id);
    let mut connection = pool.acquire().await.unwrap();
    let records = FinancialRecordsConnectionRepository::find_all_by_contract_id(
        &mut *connection,
        contract_id,
    )
    .await
    .unwrap();
    assert_eq!(records.len(), 3);
}

#[tokio::test]
async fn failed_assessment_service_transaction_leaves_no_contract_or_financial_rows() {
    let pool = create_test_database().await;
    let student = StudentRepository::insert(&pool, Student::from(valid_student_dto()))
        .await
        .unwrap();
    let student_id = student.id.unwrap();
    let mut request = valid_create_contract_request();
    request.contract_dto.student_id = student_id;
    request.contract_dto.installment_count = 0;

    let mut tx = pool.begin().await.unwrap();
    let result = physical_assessment_service::insert(
        &mut *tx,
        request,
        PhysicalAssessmentDto::from(valid_physical_assessment()),
        vec![],
    )
    .await;
    assert!(result.is_err());
    drop(tx); // The command's transaction rolls back when the service returns an error.

    let mut connection = pool.acquire().await.unwrap();
    let contracts = ContractRepository::find_all_by_student_id(&mut *connection, student_id)
        .await
        .unwrap();
    assert!(contracts.is_empty());
    drop(connection);
    let financial_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM financial_records")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(financial_count, 0);
}
