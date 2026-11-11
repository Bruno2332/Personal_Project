use async_trait::async_trait;
use mockall::predicate::eq;

use crate::{
    domain::{
        bmi::bmi::BmiClassification, hypertension_classification::HypertensionClassification,
        vo2_classification::Vo2Classification,
    },
    dtos::{physical_assessment_dto::PhysicalAssessmentDto, vo2_assessment_dto::Vo2AssessmentDto},
    errors::app_errors::AppError,
    models::{
        contract::Contract,
        physical_assessment::PhysicalAssessment,
        students::{Student, StudentProjection},
        vo2_assessment::Vo2Assessment,
    },
    repositories::{
        contract_repository::{ContractRepository, MockContractRepository},
        physical_assessment_repository::{
            MockPhysicalAssessmentConnectionRepository, MockPhysicalAssessmentPoolRepository,
            PhysicalAssessmentConnectionRepository, PhysicalAssessmentPoolRepository,
        },
        student_repository::{MockStudentRepository, StudentRepository},
        vo2_assessment_repository::{
            MockVo2AssessmentConnectionRepository, MockVo2AssessmentPoolRepository,
            Vo2AssessmentConnectionRepository, Vo2AssessmentPoolRepository,
        },
    },
    services::physical_assessment_service,
    tests::factories::physical_assessment_factory::{
        valid_physical_assessment, valid_student, valid_vo2_assessment,
    },
};

struct AssessmentReadMocks {
    students: MockStudentRepository,
    assessments: MockPhysicalAssessmentPoolRepository,
    vo2: MockVo2AssessmentPoolRepository,
}

struct AssessmentUpdateMocks {
    physical: MockPhysicalAssessmentConnectionRepository,
    vo2: MockVo2AssessmentConnectionRepository,
    contracts: MockContractRepository,
}

#[async_trait]
impl PhysicalAssessmentConnectionRepository for AssessmentUpdateMocks {
    async fn insert(
        &mut self,
        entity: PhysicalAssessment,
    ) -> Result<PhysicalAssessment, sqlx::Error> {
        self.physical.insert(entity).await
    }

    async fn update(
        &mut self,
        entity: PhysicalAssessment,
        id: i64,
    ) -> Result<PhysicalAssessment, sqlx::Error> {
        self.physical.update(entity, id).await
    }
}

#[async_trait]
impl Vo2AssessmentConnectionRepository for AssessmentUpdateMocks {
    async fn insert_many(&mut self, entities: Vec<Vo2Assessment>) -> Result<u64, sqlx::Error> {
        self.vo2.insert_many(entities).await
    }

    async fn update_many(
        &mut self,
        physical_assessment_id: i64,
        entities: Vec<Vo2Assessment>,
    ) -> Result<u64, sqlx::Error> {
        self.vo2.update_many(physical_assessment_id, entities).await
    }

    async fn delete_by_ids(
        &mut self,
        ids: Vec<i64>,
        physical_assessment_id: i64,
    ) -> Result<u64, sqlx::Error> {
        self.vo2.delete_by_ids(ids, physical_assessment_id).await
    }
}

#[async_trait]
impl ContractRepository for AssessmentUpdateMocks {
    async fn find_all_by_student_id(
        &mut self,
        student_id: i64,
    ) -> Result<Vec<Contract>, sqlx::Error> {
        self.contracts.find_all_by_student_id(student_id).await
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
        today: chrono::NaiveDate,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<i64>, sqlx::Error> {
        self.contracts
            .complete_expired(today, updated_at)
            .await
    }
}

#[async_trait]
impl StudentRepository for AssessmentReadMocks {
    async fn find_all_paginated(
        &self,
        page: i64,
        size: i64,
        name: Option<String>,
    ) -> Result<Vec<StudentProjection>, sqlx::Error> {
        self.students.find_all_paginated(page, size, name).await
    }

    async fn count_all(&self, name: Option<String>) -> Result<i64, sqlx::Error> {
        self.students.count_all(name).await
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Student>, sqlx::Error> {
        self.students.find_by_id(id).await
    }

    async fn insert(&self, student: Student) -> Result<Student, sqlx::Error> {
        self.students.insert(student).await
    }

    async fn update(&self, id: i64, student: Student) -> Result<Student, sqlx::Error> {
        self.students.update(id, student).await
    }
}

#[async_trait]
impl PhysicalAssessmentPoolRepository for AssessmentReadMocks {
    async fn find_all_by_student_id(
        &self,
        student_id: i64,
    ) -> Result<Vec<PhysicalAssessment>, sqlx::Error> {
        self.assessments.find_all_by_student_id(student_id).await
    }

    async fn delete_by_id(&self, id: i64) -> Result<u64, sqlx::Error> {
        self.assessments.delete_by_id(id).await
    }
}

#[async_trait]
impl Vo2AssessmentPoolRepository for AssessmentReadMocks {
    async fn find_all_by_student_id(
        &self,
        student_id: i64,
    ) -> Result<Vec<Vo2Assessment>, sqlx::Error> {
        self.vo2.find_all_by_student_id(student_id).await
    }
}

fn assert_database_error<T>(result: Result<T, AppError>, expected: &str) {
    assert!(matches!(result, Err(AppError::Database { message }) if message == expected));
}

#[tokio::test]
async fn find_all_returns_assessment_with_calculated_domain_results_and_vo2_stages() {
    let mut students = MockStudentRepository::new();
    students
        .expect_find_by_id()
        .with(eq(7))
        .times(1)
        .returning(|_| Ok(Some(valid_student())));

    let mut assessments = MockPhysicalAssessmentPoolRepository::new();
    assessments
        .expect_find_all_by_student_id()
        .with(eq(7))
        .times(1)
        .returning(|_| Ok(vec![valid_physical_assessment()]));

    let mut vo2 = MockVo2AssessmentPoolRepository::new();
    vo2.expect_find_all_by_student_id()
        .with(eq(7))
        .times(1)
        .returning(|_| Ok(vec![valid_vo2_assessment()]));

    let repository = AssessmentReadMocks {
        students,
        assessments,
        vo2,
    };
    let results = physical_assessment_service::find_all_by_student_id(&repository, 7)
        .await
        .unwrap();

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.physical_assessment_dto.id, Some(21));
    assert_eq!(result.age, 35);
    assert_eq!(result.bmi, 20.0);
    assert!(matches!(
        result.bmi_classification,
        BmiClassification::NormalWeight
    ));
    assert_eq!(
        result.hypertension_classification,
        Some(HypertensionClassification::StageOneHypertension)
    );
    assert_eq!(result.vo2_parameter, Some(38.34));
    assert_eq!(result.vo2_classification, Some(Vo2Classification::Fair));
    assert_eq!(result.vo2_assessments.len(), 1);
    assert_eq!(result.vo2_assessments[0].vo2_assessment_dto.id, Some(51));
    assert!(result.vo2_assessments[0].fc_heart_percentage.is_some());
}

#[tokio::test]
async fn find_all_returns_empty_when_student_has_no_assessments() {
    let mut students = MockStudentRepository::new();
    students
        .expect_find_by_id()
        .returning(|_| Ok(Some(valid_student())));
    let mut assessments = MockPhysicalAssessmentPoolRepository::new();
    assessments
        .expect_find_all_by_student_id()
        .returning(|_| Ok(Vec::new()));
    let mut vo2 = MockVo2AssessmentPoolRepository::new();
    vo2.expect_find_all_by_student_id()
        .returning(|_| Ok(Vec::new()));

    let repository = AssessmentReadMocks {
        students,
        assessments,
        vo2,
    };
    let results = physical_assessment_service::find_all_by_student_id(&repository, 7)
        .await
        .unwrap();

    assert!(results.is_empty());
}

#[tokio::test]
async fn find_all_returns_not_found_when_student_does_not_exist() {
    let mut students = MockStudentRepository::new();
    students.expect_find_by_id().returning(|_| Ok(None));
    let repository = AssessmentReadMocks {
        students,
        assessments: MockPhysicalAssessmentPoolRepository::new(),
        vo2: MockVo2AssessmentPoolRepository::new(),
    };

    let result = physical_assessment_service::find_all_by_student_id(&repository, 404).await;

    assert!(matches!(
        result,
        Err(AppError::NotFound { message }) if message == "Aluno não encontrado"
    ));
}

#[tokio::test]
async fn find_all_maps_student_repository_error_to_database_error() {
    let mut students = MockStudentRepository::new();
    students
        .expect_find_by_id()
        .returning(|_| Err(sqlx::Error::PoolClosed));
    let repository = AssessmentReadMocks {
        students,
        assessments: MockPhysicalAssessmentPoolRepository::new(),
        vo2: MockVo2AssessmentPoolRepository::new(),
    };

    let result = physical_assessment_service::find_all_by_student_id(&repository, 7).await;

    assert_database_error(result, "Erro ao buscar aluno");
}

#[tokio::test]
async fn find_all_maps_assessment_repository_error_to_database_error() {
    let mut students = MockStudentRepository::new();
    students
        .expect_find_by_id()
        .returning(|_| Ok(Some(valid_student())));
    let mut assessments = MockPhysicalAssessmentPoolRepository::new();
    assessments
        .expect_find_all_by_student_id()
        .returning(|_| Err(sqlx::Error::PoolClosed));
    let repository = AssessmentReadMocks {
        students,
        assessments,
        vo2: MockVo2AssessmentPoolRepository::new(),
    };

    let result = physical_assessment_service::find_all_by_student_id(&repository, 7).await;

    assert_database_error(result, "Erro ao buscar avaliações físicas");
}

#[tokio::test]
async fn find_all_maps_vo2_repository_error_to_database_error() {
    let mut students = MockStudentRepository::new();
    students
        .expect_find_by_id()
        .returning(|_| Ok(Some(valid_student())));
    let mut assessments = MockPhysicalAssessmentPoolRepository::new();
    assessments
        .expect_find_all_by_student_id()
        .returning(|_| Ok(Vec::new()));
    let mut vo2 = MockVo2AssessmentPoolRepository::new();
    vo2.expect_find_all_by_student_id()
        .returning(|_| Err(sqlx::Error::PoolClosed));
    let repository = AssessmentReadMocks {
        students,
        assessments,
        vo2,
    };

    let result = physical_assessment_service::find_all_by_student_id(&repository, 7).await;

    assert_database_error(result, "Erro ao buscar etapas de VO₂");
}

#[tokio::test]
async fn delete_succeeds_when_repository_deletes_assessment() {
    let mut repository = MockPhysicalAssessmentPoolRepository::new();
    repository
        .expect_delete_by_id()
        .with(eq(21))
        .times(1)
        .returning(|_| Ok(1));

    physical_assessment_service::delete(&repository, 21)
        .await
        .unwrap();
}

#[tokio::test]
async fn delete_returns_not_found_when_repository_deletes_no_rows() {
    let mut repository = MockPhysicalAssessmentPoolRepository::new();
    repository
        .expect_delete_by_id()
        .with(eq(404))
        .times(1)
        .returning(|_| Ok(0));

    let result = physical_assessment_service::delete(&repository, 404).await;

    assert!(matches!(
        result,
        Err(AppError::NotFound { message }) if message == "Avaliação física não encontrada"
    ));
}

#[tokio::test]
async fn delete_maps_repository_error_to_database_error() {
    let mut repository = MockPhysicalAssessmentPoolRepository::new();
    repository
        .expect_delete_by_id()
        .returning(|_| Err(sqlx::Error::PoolClosed));

    let result = physical_assessment_service::delete(&repository, 21).await;

    assert_database_error(result, "Erro ao deletar avaliação física");
}

#[tokio::test]
async fn update_validates_contract_and_batches_vo2_deletes_updates_and_inserts() {
    let mut physical = MockPhysicalAssessmentConnectionRepository::new();
    physical
        .expect_update()
        .withf(|entity, id| entity.student_id == 7 && entity.contract_id == 31 && *id == 21)
        .times(1)
        .returning(|mut entity, id| {
            entity.id = Some(id);
            Ok(entity)
        });

    let mut contracts = MockContractRepository::new();
    contracts
        .expect_belongs_to_student()
        .with(eq(31), eq(7))
        .times(1)
        .returning(|_, _| Ok(true));

    let mut vo2 = MockVo2AssessmentConnectionRepository::new();
    vo2.expect_delete_by_ids()
        .withf(|ids, assessment_id| ids == &[77] && *assessment_id == 21)
        .times(1)
        .returning(|_, _| Ok(1));
    vo2.expect_update_many()
        .withf(|assessment_id, entities| {
            *assessment_id == 21
                && entities.len() == 1
                && entities[0].id == Some(51)
                && entities[0].treadmill_speed_kmh == Some(11)
        })
        .times(1)
        .returning(|_, entities| Ok(entities.len() as u64));
    vo2.expect_insert_many()
        .withf(|entities| {
            entities.len() == 1
                && entities[0].id.is_none()
                && entities[0].physical_assessment_id == 21
                && entities[0].treadmill_speed_kmh == Some(12)
        })
        .times(1)
        .returning(|entities| Ok(entities.len() as u64));

    let mut existing = Vo2AssessmentDto::from(valid_vo2_assessment());
    existing.treadmill_speed_kmh = Some(11);
    let mut new_stage = Vo2AssessmentDto::from(valid_vo2_assessment());
    new_stage.id = None;
    new_stage.treadmill_speed_kmh = Some(12);

    let result = physical_assessment_service::update(
        &mut AssessmentUpdateMocks {
            physical,
            vo2,
            contracts,
        },
        21,
        PhysicalAssessmentDto::from(valid_physical_assessment()),
        vec![existing, new_stage],
        vec![77],
    )
    .await
    .unwrap();

    assert_eq!(result.id, Some(21));
}

async fn load_assessment_goals_for_test(
    assessment: PhysicalAssessment,
) -> crate::dtos::physical_assessment_dto::PhysicalAssessmentCompleteDto {
    let mut students = MockStudentRepository::new();

    students
        .expect_find_by_id()
        .with(eq(7))
        .times(1)
        .returning(|_| Ok(Some(valid_student())));

    let mut assessments = MockPhysicalAssessmentPoolRepository::new();

    assessments
        .expect_find_all_by_student_id()
        .with(eq(7))
        .times(1)
        .return_once(move |_| Ok(vec![assessment]));

    let mut vo2 = MockVo2AssessmentPoolRepository::new();

    vo2.expect_find_all_by_student_id()
        .with(eq(7))
        .times(1)
        .returning(|_| Ok(Vec::new()));

    let repository = AssessmentReadMocks {
        students,
        assessments,
        vo2,
    };

    let mut results =
        physical_assessment_service::find_all_by_student_id(
            &repository,
            7,
        )
            .await
            .unwrap();

    assert_eq!(results.len(), 1);

    results.remove(0)
}

fn assert_goal_value(actual: Option<f64>, expected: f64) {
    let actual = actual.expect("O objetivo deveria estar preenchido.");

    assert!(
        (actual - expected).abs() < 0.000001,
        "Esperado {expected}, recebido {actual}",
    );
}

#[tokio::test]
async fn assessment_maps_weight_goals_and_losses_to_complete_dto() {
    let mut assessment = valid_physical_assessment();

    assessment.triceps_skinfold_mm = Some(30.0);
    assessment.chest_skinfold_mm = Some(30.0);
    assessment.midaxillary_skinfold_mm = Some(30.0);
    assessment.subscapular_skinfold_mm = Some(30.0);
    assessment.abdominal_skinfold_mm = Some(30.0);
    assessment.suprailiac_skinfold_mm = Some(30.0);
    assessment.thigh_skinfold_mm = Some(30.0);

    let result = load_assessment_goals_for_test(assessment).await;

    // Factory: homem de 35 anos na avaliação, 80 kg.
    assert_eq!(result.age, 35);

    assert_goal_value(
        result.fat_percentage_skinfolds,
        28.34498433540972,
    );

    assert_goal_value(
        result.ideal_weight_fat_percentage_min,
        66.65582852520026,
    );
    assert_goal_value(
        result.ideal_weight_fat_percentage_max,
        69.90733235569783,
    );
    assert_goal_value(
        result.ideal_weight_fat_percentage_health,
        72.56204117933193,
    );
    assert_goal_value(
        result.ideal_weight_fat_percentage_overweight,
        76.4320167088963,
    );

    assert_goal_value(
        result.weight_to_lose_min_percentage,
        13.34417147479974,
    );
    assert_goal_value(
        result.weight_to_lose_max_percentage,
        10.09266764430217,
    );
    assert_goal_value(
        result.weight_to_lose_health,
        7.43795882066807,
    );
    assert_goal_value(
        result.weight_to_lose_overweight,
        3.5679832911037,
    );

    assert_eq!(result.ideal_fat_percentage_min, Some(14.0));
    assert_eq!(result.ideal_fat_percentage_max, Some(18.0));
    assert_eq!(result.ideal_fat_percentage_health, Some(21.0));
    assert_eq!(result.ideal_fat_percentage_overweight, Some(25.0));

    assert_eq!(
        result.circumference_abdominal_high_risk,
        Some(94.0),
    );
    assert_eq!(
        result.circumference_abdominal_very_high_risk,
        Some(102.0),
    );

    assert_eq!(result.waist_hip_moderate_risk, Some(0.84));
    assert_eq!(result.waist_hip_high_risk, Some(0.91));
    assert_eq!(result.waist_hip_very_high_risk, Some(0.96));
}

#[tokio::test]
async fn below_reference_only_minimum_weight_adjustment_can_be_negative() {
    // Factory: sete pregas de 10 mm, homem de 35 anos e 80 kg.
    let result =
        load_assessment_goals_for_test(valid_physical_assessment())
            .await;

    assert_goal_value(
        result.ideal_weight_fat_percentage_min,
        82.95194270945785,
    );

    assert_goal_value(
        result.weight_to_lose_min_percentage,
        -2.95194270945785,
    );

    assert_eq!(result.weight_to_lose_max_percentage, Some(0.0));
    assert_eq!(result.weight_to_lose_health, Some(0.0));
    assert_eq!(result.weight_to_lose_overweight, Some(0.0));
}

#[tokio::test]
async fn missing_skinfold_removes_fat_goals_but_preserves_other_references() {
    let mut assessment = valid_physical_assessment();
    assessment.triceps_skinfold_mm = None;

    let result = load_assessment_goals_for_test(assessment).await;

    assert_eq!(result.fat_percentage_skinfolds, None);

    assert_eq!(result.ideal_weight_fat_percentage_min, None);
    assert_eq!(result.ideal_weight_fat_percentage_max, None);
    assert_eq!(result.ideal_weight_fat_percentage_health, None);
    assert_eq!(result.ideal_weight_fat_percentage_overweight, None);

    assert_eq!(result.ideal_fat_percentage_min, None);
    assert_eq!(result.ideal_fat_percentage_max, None);
    assert_eq!(result.ideal_fat_percentage_health, None);
    assert_eq!(result.ideal_fat_percentage_overweight, None);

    assert_eq!(result.weight_to_lose_min_percentage, None);
    assert_eq!(result.weight_to_lose_max_percentage, None);
    assert_eq!(result.weight_to_lose_health, None);
    assert_eq!(result.weight_to_lose_overweight, None);

    assert_eq!(
        result.circumference_abdominal_high_risk,
        Some(94.0),
    );
    assert_eq!(
        result.circumference_abdominal_very_high_risk,
        Some(102.0),
    );

    assert_eq!(result.waist_hip_moderate_risk, Some(0.84));
    assert_eq!(result.waist_hip_high_risk, Some(0.91));
    assert_eq!(result.waist_hip_very_high_risk, Some(0.96));
}
