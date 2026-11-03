use crate::{
    models::students::Student,
    repositories::{
        contract_repository::ContractRepository,
        physical_assessment_photos_repository::PhysicalAssessmentPhotosRepository,
        physical_assessment_repository::{
            PhysicalAssessmentConnectionRepository, PhysicalAssessmentPoolRepository,
        },
        student_repository::StudentRepository,
        vo2_assessment_repository::{
            Vo2AssessmentConnectionRepository, Vo2AssessmentPoolRepository,
        },
    },
    tests::{
        factories::{
            contract_factory::valid_contract,
            physical_assessment_factory::{
                valid_photo, valid_physical_assessment, valid_vo2_assessment,
            },
            student_factory::valid_student_dto,
        },
        utils::database::create_test_database,
    },
};

#[tokio::test]
async fn assessment_vo2_and_photo_repositories_work_together_with_foreign_keys() {
    let pool = create_test_database().await;
    let student = StudentRepository::insert(&pool, Student::from(valid_student_dto()))
        .await
        .unwrap();
    let student_id = student.id.unwrap();

    let mut contract = valid_contract();
    contract.id = None;
    contract.student_id = student_id;
    let mut tx = pool.begin().await.unwrap();
    let contract = ContractRepository::insert(&mut *tx, contract)
        .await
        .unwrap();

    let mut assessment = valid_physical_assessment();
    assessment.id = None;
    assessment.student_id = student_id;
    assessment.contract_id = contract.id.unwrap();
    let assessment = PhysicalAssessmentConnectionRepository::insert(&mut *tx, assessment)
        .await
        .unwrap();
    let assessment_id = assessment.id.unwrap();

    let mut first_stage = valid_vo2_assessment();
    first_stage.id = None;
    first_stage.physical_assessment_id = assessment_id;
    let mut second_stage = valid_vo2_assessment();
    second_stage.id = None;
    second_stage.physical_assessment_id = assessment_id;
    second_stage.treadmill_speed_kmh = Some(11);
    let inserted =
        Vo2AssessmentConnectionRepository::insert_many(&mut *tx, vec![first_stage, second_stage])
            .await
            .unwrap();
    assert_eq!(inserted, 2);

    tx.commit().await.unwrap();

    let mut photo = valid_photo();
    photo.id = None;
    photo.physical_assessment_id = assessment_id;
    let photo = PhysicalAssessmentPhotosRepository::insert(&pool, photo)
        .await
        .unwrap();

    let assessments = PhysicalAssessmentPoolRepository::find_all_by_student_id(&pool, student_id)
        .await
        .unwrap();
    assert_eq!(assessments.len(), 1);
    assert_eq!(assessments[0].notes.as_deref(), Some("Anamnese de teste"));

    let stages = Vo2AssessmentPoolRepository::find_all_by_student_id(&pool, student_id)
        .await
        .unwrap();
    assert_eq!(stages.len(), 2);

    let photos =
        PhysicalAssessmentPhotosRepository::find_all_by_assessment_id(&pool, assessment_id)
            .await
            .unwrap();
    assert_eq!(photos.len(), 1);
    assert_eq!(photos[0].id, photo.id);
    let photo_id = photo.id.unwrap();
    let changed_photo = PhysicalAssessmentPhotosRepository::update(
        &pool,
        photo_id,
        crate::models::physical_assessment_photos::PhotoPosition::Side,
        Some("Perfil atualizado".into()),
        chrono::Utc::now(),
    )
    .await
    .unwrap();
    assert_eq!(
        changed_photo.description.as_deref(),
        Some("Perfil atualizado")
    );
    assert_eq!(
        changed_photo.position,
        crate::models::physical_assessment_photos::PhotoPosition::Side
    );

    let mut changed_assessment = valid_physical_assessment();
    changed_assessment.id = Some(assessment_id);
    changed_assessment.student_id = student_id;
    changed_assessment.contract_id = contract.id.unwrap();
    changed_assessment.weight_kg = 78.5;
    changed_assessment.notes = Some("Anamnese atualizada".to_string());
    let mut tx = pool.begin().await.unwrap();
    let updated =
        PhysicalAssessmentConnectionRepository::update(&mut *tx, changed_assessment, assessment_id)
            .await
            .unwrap();
    tx.commit().await.unwrap();
    assert_eq!(updated.weight_kg, 78.5);
    assert_eq!(updated.notes.as_deref(), Some("Anamnese atualizada"));

    let removed = PhysicalAssessmentPoolRepository::delete_by_id(&pool, assessment_id)
        .await
        .unwrap();
    assert_eq!(removed, 1);
    assert!(
        Vo2AssessmentPoolRepository::find_all_by_student_id(&pool, student_id)
            .await
            .unwrap()
            .is_empty()
    );
    assert!(
        PhysicalAssessmentPhotosRepository::find_all_by_assessment_id(&pool, assessment_id)
            .await
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        PhysicalAssessmentPhotosRepository::delete_by_id(&pool, photo_id)
            .await
            .unwrap(),
        0,
    );
}

#[tokio::test]
async fn vo2_batch_update_and_delete_are_scoped_to_the_assessment() {
    let pool = create_test_database().await;
    let student = StudentRepository::insert(&pool, Student::from(valid_student_dto()))
        .await
        .unwrap();
    let mut contract = valid_contract();
    contract.id = None;
    contract.student_id = student.id.unwrap();
    let mut tx = pool.begin().await.unwrap();
    let contract = ContractRepository::insert(&mut *tx, contract)
        .await
        .unwrap();
    let mut assessment = valid_physical_assessment();
    assessment.id = None;
    assessment.student_id = student.id.unwrap();
    assessment.contract_id = contract.id.unwrap();
    let assessment = PhysicalAssessmentConnectionRepository::insert(&mut *tx, assessment)
        .await
        .unwrap();
    let assessment_id = assessment.id.unwrap();

    let mut stage = valid_vo2_assessment();
    stage.id = None;
    stage.physical_assessment_id = assessment_id;
    Vo2AssessmentConnectionRepository::insert_many(&mut *tx, vec![stage])
        .await
        .unwrap();
    tx.commit().await.unwrap();
    let mut saved_stages =
        Vo2AssessmentPoolRepository::find_all_by_student_id(&pool, student.id.unwrap())
            .await
            .unwrap();
    saved_stages[0].treadmill_speed_kmh = Some(12);
    let stage_id = saved_stages[0].id.unwrap();
    let mut tx = pool.begin().await.unwrap();
    let changed =
        Vo2AssessmentConnectionRepository::update_many(&mut *tx, assessment_id, saved_stages)
            .await
            .unwrap();
    assert_eq!(changed, 1);
    let deleted =
        Vo2AssessmentConnectionRepository::delete_by_ids(&mut *tx, vec![stage_id], assessment_id)
            .await
            .unwrap();
    assert_eq!(deleted, 1);
    tx.commit().await.unwrap();
    assert!(
        Vo2AssessmentPoolRepository::find_all_by_student_id(&pool, student.id.unwrap())
            .await
            .unwrap()
            .is_empty()
    );
}
