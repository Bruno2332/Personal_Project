use crate::domain::abdominal_circ_classification::AbdominalCircumferenceClassification;
use crate::domain::bmi::bmi::BmiClassification;
use crate::domain::dynamometer_strength::DynamometerStrengthClassification;
use crate::domain::fc_classification::FcRestClassification;
use crate::domain::hypertension_classification::HypertensionClassification;
use crate::domain::skin_folds::FatPercentageClassification;
use crate::domain::strength_asymmetry::StrengthAsymmetryClassification;
use crate::domain::vo2_classification::Vo2Classification;
use crate::domain::waist_to_hip_ratio::WaistHipRiskClassification;
use crate::dtos::vo2_assessment_dto::Vo2AssessmentCompleteDto;
use crate::models::physical_assessment::PhysicalAssessment;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PhysicalAssessmentDto {
    pub id: Option<i64>,
    pub student_id: i64,
    pub contract_id: i64,
    pub assessment_date: NaiveDate,
    #[validate(range(min = 0.1, max = 300.0, message = "O peso deve estar entre 0,1 e 300 kg."))]
    pub weight_kg: f64,

    #[validate(range(min = 0.5, max = 2.5, message = "A altura deve estar entre 0,50 e 2,50 m."))]
    pub height_m: f64,

    #[validate(range(min = 1, max = 200, message = "A pressão sistólica deve estar entre 1 e 200 mmHg."))]
    pub systolic_pressure: Option<i32>,

    #[validate(range(min = 1, max = 200, message = "A pressão diastólica deve estar entre 1 e 200 mmHg."))]
    pub diastolic_pressure: Option<i32>,

    #[validate(range(min = 1, max = 250, message = "A frequência cardíaca deve estar entre 1 e 250 bpm."))]
    pub resting_heart_rate_bpm: Option<i32>,

    pub notes: Option<String>,

    #[validate(range(min = 0.1, max = 200.0, message = "A força deve estar entre 0,1 e 200."))]
    pub right_handgrip_strength: Option<f64>,

    #[validate(range(min = 0.1, max = 200.0, message = "A força deve estar entre 0,1 e 200."))]
    pub left_handgrip_strength: Option<f64>,

    #[validate(range(min = 0.1, max = 300.0, message = "A circunferência deve estar entre 0,1 e 300 cm."))]
    pub neck_circumference_cm: Option<f64>,

    #[validate(range(min = 0.1, max = 300.0, message = "A circunferência deve estar entre 0,1 e 300 cm."))]
    pub right_arm_circumference_cm: Option<f64>,

    #[validate(range(min = 0.1, max = 300.0, message = "A circunferência deve estar entre 0,1 e 300 cm."))]
    pub left_arm_circumference_cm: Option<f64>,

    #[validate(range(min = 0.1, max = 300.0, message = "A circunferência deve estar entre 0,1 e 300 cm."))]
    pub right_forearm_circumference_cm: Option<f64>,

    #[validate(range(min = 0.1, max = 300.0, message = "A circunferência deve estar entre 0,1 e 300 cm."))]
    pub left_forearm_circumference_cm: Option<f64>,

    #[validate(range(min = 0.1, max = 300.0, message = "A circunferência deve estar entre 0,1 e 300 cm."))]
    pub shoulder_circumference_cm: Option<f64>,

    #[validate(range(min = 0.1, max = 300.0, message = "A circunferência deve estar entre 0,1 e 300 cm."))]
    pub chest_circumference_cm: Option<f64>,

    #[validate(range(min = 0.1, max = 300.0, message = "A circunferência deve estar entre 0,1 e 300 cm."))]
    pub waist_circumference_cm: Option<f64>,

    #[validate(range(min = 0.1, max = 300.0, message = "A circunferência deve estar entre 0,1 e 300 cm."))]
    pub abdomen_circumference_cm: Option<f64>,

    #[validate(range(min = 0.1, max = 300.0, message = "A circunferência deve estar entre 0,1 e 300 cm."))]
    pub hip_circumference_cm: Option<f64>,

    #[validate(range(min = 0.1, max = 300.0, message = "A circunferência deve estar entre 0,1 e 300 cm."))]
    pub right_thigh_circumference_cm: Option<f64>,

    #[validate(range(min = 0.1, max = 300.0, message = "A circunferência deve estar entre 0,1 e 300 cm."))]
    pub left_thigh_circumference_cm: Option<f64>,

    #[validate(range(min = 0.1, max = 300.0, message = "A circunferência deve estar entre 0,1 e 300 cm."))]
    pub right_calf_circumference_cm: Option<f64>,

    #[validate(range(min = 0.1, max = 300.0, message = "A circunferência deve estar entre 0,1 e 300 cm."))]
    pub left_calf_circumference_cm: Option<f64>,

    #[validate(range(min = 0.1, max = 99.0, message = "A prega cutânea deve estar entre 0,1 e 99 mm."))]
    pub triceps_skinfold_mm: Option<f64>,

    #[validate(range(min = 0.1, max = 99.0, message = "A prega cutânea deve estar entre 0,1 e 99 mm."))]
    pub chest_skinfold_mm: Option<f64>,

    #[validate(range(min = 0.1, max = 99.0, message = "A prega cutânea deve estar entre 0,1 e 99 mm."))]
    pub midaxillary_skinfold_mm: Option<f64>,

    #[validate(range(min = 0.1, max = 99.0, message = "A prega cutânea deve estar entre 0,1 e 99 mm."))]
    pub subscapular_skinfold_mm: Option<f64>,

    #[validate(range(min = 0.1, max = 99.0, message = "A prega cutânea deve estar entre 0,1 e 99 mm."))]
    pub abdominal_skinfold_mm: Option<f64>,

    #[validate(range(min = 0.1, max = 99.0, message = "A prega cutânea deve estar entre 0,1 e 99 mm."))]
    pub suprailiac_skinfold_mm: Option<f64>,

    #[validate(range(min = 0.1, max = 99.0, message = "A prega cutânea deve estar entre 0,1 e 99 mm."))]
    pub thigh_skinfold_mm: Option<f64>,

    #[validate(range(min = 0.1, max = 100.0, message = "O percentual de gordura deve estar entre 0,1% e 100%."))]
    pub bioimpedance_fat_percentage: Option<f64>,

    #[validate(range(min = 1, max = 200, message = "A força deve estar entre 1 e 200."))]
    pub right_extensor_strength: Option<i32>,

    #[validate(range(min = 1, max = 200, message = "A força deve estar entre 1 e 200."))]
    pub left_extensor_strength: Option<i32>,

    #[validate(range(min = 1, max = 200, message = "A força deve estar entre 1 e 200."))]
    pub right_flexor_strength: Option<i32>,

    #[validate(range(min = 1, max = 200, message = "A força deve estar entre 1 e 200."))]
    pub left_flexor_strength: Option<i32>,

    #[validate(range(min = 1, max = 200, message = "A força deve estar entre 1 e 200."))]
    pub right_biceps_strength: Option<i32>,

    #[validate(range(min = 1, max = 200, message = "A força deve estar entre 1 e 200."))]
    pub left_biceps_strength: Option<i32>,

    #[validate(range(min = 1, max = 200, message = "A força deve estar entre 1 e 200."))]
    pub right_triceps_strength: Option<i32>,

    #[validate(range(min = 1, max = 200, message = "A força deve estar entre 1 e 200."))]
    pub left_triceps_strength: Option<i32>,
    #[validate(range(min = 0, max = 59, message = "Informe de 0 a 59 segundos."))]
    pub vo2_final_stage_seconds: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<PhysicalAssessment> for PhysicalAssessmentDto {
    fn from(entity: PhysicalAssessment) -> Self {
        Self {
            id: entity.id,
            student_id: entity.student_id,
            contract_id: entity.contract_id,
            assessment_date: entity.assessment_date,
            weight_kg: entity.weight_kg,
            height_m: entity.height_m,
            systolic_pressure: entity.systolic_pressure,
            diastolic_pressure: entity.diastolic_pressure,
            resting_heart_rate_bpm: entity.resting_heart_rate_bpm,
            notes: entity.notes,
            right_handgrip_strength: entity.right_handgrip_strength,
            left_handgrip_strength: entity.left_handgrip_strength,
            neck_circumference_cm: entity.neck_circumference_cm,
            right_arm_circumference_cm: entity.right_arm_circumference_cm,
            left_arm_circumference_cm: entity.left_arm_circumference_cm,
            right_forearm_circumference_cm: entity.right_forearm_circumference_cm,
            left_forearm_circumference_cm: entity.left_forearm_circumference_cm,
            shoulder_circumference_cm: entity.shoulder_circumference_cm,
            chest_circumference_cm: entity.chest_circumference_cm,
            waist_circumference_cm: entity.waist_circumference_cm,
            abdomen_circumference_cm: entity.abdomen_circumference_cm,
            hip_circumference_cm: entity.hip_circumference_cm,
            right_thigh_circumference_cm: entity.right_thigh_circumference_cm,
            left_thigh_circumference_cm: entity.left_thigh_circumference_cm,
            right_calf_circumference_cm: entity.right_calf_circumference_cm,
            left_calf_circumference_cm: entity.left_calf_circumference_cm,
            triceps_skinfold_mm: entity.triceps_skinfold_mm,
            chest_skinfold_mm: entity.chest_skinfold_mm,
            midaxillary_skinfold_mm: entity.midaxillary_skinfold_mm,
            subscapular_skinfold_mm: entity.subscapular_skinfold_mm,
            abdominal_skinfold_mm: entity.abdominal_skinfold_mm,
            suprailiac_skinfold_mm: entity.suprailiac_skinfold_mm,
            thigh_skinfold_mm: entity.thigh_skinfold_mm,
            bioimpedance_fat_percentage: entity.bioimpedance_fat_percentage,
            right_extensor_strength: entity.right_extensor_strength,
            left_extensor_strength: entity.left_extensor_strength,
            right_flexor_strength: entity.right_flexor_strength,
            left_flexor_strength: entity.left_flexor_strength,
            right_biceps_strength: entity.right_biceps_strength,
            left_biceps_strength: entity.left_biceps_strength,
            right_triceps_strength: entity.right_triceps_strength,
            left_triceps_strength: entity.left_triceps_strength,
            vo2_final_stage_seconds: entity.vo2_final_stage_seconds,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

impl From<PhysicalAssessmentDto> for PhysicalAssessment {
    fn from(dto: PhysicalAssessmentDto) -> Self {
        Self {
            id: dto.id,
            student_id: dto.student_id,
            contract_id: dto.contract_id,
            assessment_date: dto.assessment_date,
            weight_kg: dto.weight_kg,
            height_m: dto.height_m,
            systolic_pressure: dto.systolic_pressure,
            diastolic_pressure: dto.diastolic_pressure,
            resting_heart_rate_bpm: dto.resting_heart_rate_bpm,
            notes: dto.notes,
            right_handgrip_strength: dto.right_handgrip_strength,
            left_handgrip_strength: dto.left_handgrip_strength,
            neck_circumference_cm: dto.neck_circumference_cm,
            right_arm_circumference_cm: dto.right_arm_circumference_cm,
            left_arm_circumference_cm: dto.left_arm_circumference_cm,
            right_forearm_circumference_cm: dto.right_forearm_circumference_cm,
            left_forearm_circumference_cm: dto.left_forearm_circumference_cm,
            shoulder_circumference_cm: dto.shoulder_circumference_cm,
            chest_circumference_cm: dto.chest_circumference_cm,
            waist_circumference_cm: dto.waist_circumference_cm,
            abdomen_circumference_cm: dto.abdomen_circumference_cm,
            hip_circumference_cm: dto.hip_circumference_cm,
            right_thigh_circumference_cm: dto.right_thigh_circumference_cm,
            left_thigh_circumference_cm: dto.left_thigh_circumference_cm,
            right_calf_circumference_cm: dto.right_calf_circumference_cm,
            left_calf_circumference_cm: dto.left_calf_circumference_cm,
            triceps_skinfold_mm: dto.triceps_skinfold_mm,
            chest_skinfold_mm: dto.chest_skinfold_mm,
            midaxillary_skinfold_mm: dto.midaxillary_skinfold_mm,
            subscapular_skinfold_mm: dto.subscapular_skinfold_mm,
            abdominal_skinfold_mm: dto.abdominal_skinfold_mm,
            suprailiac_skinfold_mm: dto.suprailiac_skinfold_mm,
            thigh_skinfold_mm: dto.thigh_skinfold_mm,
            bioimpedance_fat_percentage: dto.bioimpedance_fat_percentage,
            right_extensor_strength: dto.right_extensor_strength,
            left_extensor_strength: dto.left_extensor_strength,
            right_flexor_strength: dto.right_flexor_strength,
            left_flexor_strength: dto.left_flexor_strength,
            right_biceps_strength: dto.right_biceps_strength,
            left_biceps_strength: dto.left_biceps_strength,
            right_triceps_strength: dto.right_triceps_strength,
            left_triceps_strength: dto.left_triceps_strength,
            vo2_final_stage_seconds: dto.vo2_final_stage_seconds,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicalAssessmentCompleteDto {
    pub physical_assessment_dto: PhysicalAssessmentDto,
    pub age: i32,
    pub bmi: f64,
    pub bmi_classification: BmiClassification,

    pub min_ideal_weight: f64,
    pub medium_ideal_weight: f64,
    pub max_ideal_weight: f64,

    pub hypertension_classification: Option<HypertensionClassification>,
    pub circumference_abdominal_classification: Option<AbdominalCircumferenceClassification>,
    pub fc_heart_rest_classification: Option<FcRestClassification>,

    pub right_dynamometer_strength_classification: Option<DynamometerStrengthClassification>,
    pub left_dynamometer_strength_classification: Option<DynamometerStrengthClassification>,

    pub kilos_lean_body_mass_skinfolds: Option<f64>,
    pub kilos_fat_skinfolds: Option<f64>,
    pub lean_body_mass_percentage_skinfolds: Option<f64>,
    pub fat_percentage_skinfolds: Option<f64>,

    pub ideal_weight_fat_percentage: Option<f64>,
    pub weight_to_lose: Option<f64>,

    pub kilos_lean_body_mass_bioimpedance: Option<f64>,
    pub kilos_fat_bioimpedance: Option<f64>,
    pub lean_body_mass_percentage_bioimpedance: Option<f64>,

    pub fat_percentage_classification_skinfolds: Option<FatPercentageClassification>,
    pub fat_percentage_classification_bioimpedance: Option<FatPercentageClassification>,

    pub biceps_asymmetry_percentage: Option<f64>,
    pub triceps_asymmetry_percentage: Option<f64>,
    pub extensor_asymmetry_percentage: Option<f64>,
    pub flexor_asymmetry_percentage: Option<f64>,
    pub biceps_asymmetry_classification: Option<StrengthAsymmetryClassification>,
    pub triceps_asymmetry_classification: Option<StrengthAsymmetryClassification>,
    pub extensor_asymmetry_classification: Option<StrengthAsymmetryClassification>,
    pub flexor_asymmetry_classification: Option<StrengthAsymmetryClassification>,

    pub waist_hip_calculate: Option<f64>,
    pub waist_hip_classification: Option<WaistHipRiskClassification>,
    pub maximum_heart_rate: i32,
    pub vo2_assessments: Vec<Vo2AssessmentCompleteDto>,
    pub vo2_parameter: Option<f64>,
    pub vo2_classification: Option<Vo2Classification>,
}
