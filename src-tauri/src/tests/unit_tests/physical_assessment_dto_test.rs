use crate::{
    dtos::{
        physical_assessment_dto::PhysicalAssessmentDto,
        physical_assessment_photos_dto::PhysicalAssessmentPhotosDto,
        vo2_assessment_dto::Vo2AssessmentDto,
    },
    models::{
        physical_assessment::PhysicalAssessment,
        physical_assessment_photos::PhysicalAssessmentPhotos, vo2_assessment::Vo2Assessment,
    },
    tests::factories::physical_assessment_factory::{
        valid_photo, valid_physical_assessment, valid_vo2_assessment,
    },
};

#[test]
fn physical_assessment_model_and_dto_preserve_all_fields() {
    let entity = valid_physical_assessment();
    let entity_json = serde_json::to_value(&entity).unwrap();
    let dto = PhysicalAssessmentDto::from(entity);
    let dto_json = serde_json::to_value(&dto).unwrap();
    assert_eq!(dto_json, entity_json);

    let restored = PhysicalAssessment::from(dto);
    assert_eq!(serde_json::to_value(restored).unwrap(), entity_json);
}

#[test]
fn vo2_assessment_model_and_dto_preserve_all_fields() {
    let entity = valid_vo2_assessment();
    let entity_json = serde_json::to_value(&entity).unwrap();
    let dto = Vo2AssessmentDto::from(entity);
    assert_eq!(serde_json::to_value(&dto).unwrap(), entity_json);

    let restored = Vo2Assessment::from(dto);
    assert_eq!(serde_json::to_value(restored).unwrap(), entity_json);
}

#[test]
fn physical_assessment_photo_conversions_preserve_fields() {
    let entity = valid_photo();
    let dto = PhysicalAssessmentPhotosDto::from(entity);

    assert_eq!(dto.id, Some(61));
    assert_eq!(dto.physical_assessment_id, 21);
    assert_eq!(dto.file_path, "assessment-photos/21/front.enc");
    assert_eq!(
        dto.position,
        crate::models::physical_assessment_photos::PhotoPosition::Front
    );
    assert_eq!(dto.description.as_deref(), Some("Frente"));

    let restored = PhysicalAssessmentPhotos::from(dto);
    assert_eq!(restored.id, Some(61));
    assert_eq!(restored.physical_assessment_id, 21);
    assert_eq!(restored.file_path, "assessment-photos/21/front.enc");
    assert_eq!(
        restored.position,
        crate::models::physical_assessment_photos::PhotoPosition::Front
    );
    assert_eq!(restored.description.as_deref(), Some("Frente"));
}
