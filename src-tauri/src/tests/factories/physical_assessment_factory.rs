use chrono::{NaiveDate, Utc};

use crate::{
    models::{
        physical_assessment::PhysicalAssessment,
        physical_assessment_photos::{PhotoPosition, PhysicalAssessmentPhotos},
        students::{Gender, Student},
        vo2_assessment::{PerceivedExertion, Vo2Assessment},
    },
    tests::factories::student_factory::valid_student_dto,
};

pub fn valid_student() -> Student {
    let mut dto = valid_student_dto();
    dto.id = Some(7);
    dto.gender = Gender::Male;
    dto.birth_date = NaiveDate::from_ymd_opt(1990, 5, 15).unwrap();
    Student::from(dto)
}

pub fn valid_physical_assessment() -> PhysicalAssessment {
    let now = Utc::now();
    PhysicalAssessment {
        id: Some(21),
        student_id: 7,
        contract_id: 31,
        assessment_date: NaiveDate::from_ymd_opt(2026, 1, 15).unwrap(),
        weight_kg: 80.0,
        height_m: 2.0,
        systolic_pressure: Some(150),
        diastolic_pressure: Some(95),
        resting_heart_rate_bpm: Some(70),
        notes: Some("Anamnese de teste".to_string()),
        right_handgrip_strength: Some(40.0),
        left_handgrip_strength: Some(38.0),
        neck_circumference_cm: Some(38.0),
        right_arm_circumference_cm: Some(35.0),
        left_arm_circumference_cm: Some(34.0),
        right_forearm_circumference_cm: Some(29.0),
        left_forearm_circumference_cm: Some(28.0),
        shoulder_circumference_cm: Some(110.0),
        chest_circumference_cm: Some(98.0),
        waist_circumference_cm: Some(90.0),
        abdomen_circumference_cm: Some(92.0),
        hip_circumference_cm: Some(100.0),
        right_thigh_circumference_cm: Some(58.0),
        left_thigh_circumference_cm: Some(57.0),
        right_calf_circumference_cm: Some(38.0),
        left_calf_circumference_cm: Some(37.0),
        triceps_skinfold_mm: Some(10.0),
        chest_skinfold_mm: Some(10.0),
        midaxillary_skinfold_mm: Some(10.0),
        subscapular_skinfold_mm: Some(10.0),
        abdominal_skinfold_mm: Some(10.0),
        suprailiac_skinfold_mm: Some(10.0),
        thigh_skinfold_mm: Some(10.0),
        bioimpedance_fat_percentage: Some(20.0),
        right_extensor_strength: Some(40.0),
        left_extensor_strength: Some(38.0),
        right_flexor_strength: Some(35.0),
        left_flexor_strength: Some(33.0),
        right_biceps_strength: Some(30.0),
        left_biceps_strength: Some(28.0),
        right_triceps_strength: Some(25.0),
        left_triceps_strength: Some(23.0),
        vo2_final_stage_seconds: Some(30),
        created_at: now,
        updated_at: now,
    }
}

pub fn valid_vo2_assessment() -> Vo2Assessment {
    let now = Utc::now();
    Vo2Assessment {
        id: Some(51),
        physical_assessment_id: 21,
        treadmill_speed_kmh: Some(10),
        heart_rate_bpm: Some(150),
        perceived_exertion: Some(PerceivedExertion::Intense),
        created_at: now,
        updated_at: now,
    }
}

pub fn valid_photo() -> PhysicalAssessmentPhotos {
    let now = Utc::now();
    PhysicalAssessmentPhotos {
        id: Some(61),
        physical_assessment_id: 21,
        file_path: "assessment-photos/21/front.enc".to_string(),
        position: PhotoPosition::Front,
        description: Some("Frente".to_string()),
        created_at: now,
        updated_at: now,
    }
}
