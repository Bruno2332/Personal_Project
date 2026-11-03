import type { Vo2AssessmentCompleteDto } from "./vo2Assessment"

export type BmiClassification = "SevereThinness" | "Thinness" | "NormalWeight" | "OverWeight" | "Obesity" | "SevereObesity" | "ObesityClassI" | "ObesityClassII" | "ObesityClassIII"

export type HypertensionClassification = "Normal" | "PreHypertension" | "StageOneHypertension" | "StageTwoHypertension" | "StageThreeHypertension"

export type FcRestClassification = "Bradycardia" | "Normal" | "Tachycardia"

export type AbdominalCircumferenceClassification = "LowRisk" | "HighRisk" | "VeryHighRisk"

export type DynamometerStrengthClassification = "Weak" | "Normal" | "Strong"

export type FatPercentageClassification = "Excellent" | "Healthy" | "Overweight" | "Obesity"

export type StrengthAsymmetryClassification = "NormalAsymmetry" | "ModerateAsymmetry" | "SevereAsymmetry"

export type WaistHipRiskClassification = "Low" | "Moderate" | "High" | "VeryHigh"

export type Vo2Classification = "VeryPoor" | "Poor" | "Fair" | "Good" | "Excellent"

export type PhysicalAssessmentDto = {
    id: number | null
    student_id: number
    contract_id: number
    assessment_date: string
    weight_kg: number
    height_m: number
    systolic_pressure: number | null
    diastolic_pressure: number | null
    resting_heart_rate_bpm: number | null
    notes: string | null
    right_handgrip_strength: number | null
    left_handgrip_strength: number | null
    neck_circumference_cm: number | null
    right_arm_circumference_cm: number | null
    left_arm_circumference_cm: number | null
    right_forearm_circumference_cm: number | null
    left_forearm_circumference_cm: number | null
    shoulder_circumference_cm: number | null
    chest_circumference_cm: number | null
    waist_circumference_cm: number | null
    abdomen_circumference_cm: number | null
    hip_circumference_cm: number | null
    right_thigh_circumference_cm: number | null
    left_thigh_circumference_cm: number | null
    right_calf_circumference_cm: number | null
    left_calf_circumference_cm: number | null
    triceps_skinfold_mm: number | null
    chest_skinfold_mm: number | null
    midaxillary_skinfold_mm: number | null
    subscapular_skinfold_mm: number | null
    abdominal_skinfold_mm: number | null
    suprailiac_skinfold_mm: number | null
    thigh_skinfold_mm: number | null
    bioimpedance_fat_percentage: number | null
    right_extensor_strength: number | null
    left_extensor_strength: number | null
    right_flexor_strength: number | null
    left_flexor_strength: number | null
    right_biceps_strength: number | null
    left_biceps_strength: number | null
    right_triceps_strength: number | null
    left_triceps_strength: number | null
    vo2_final_stage_seconds: number | null
    created_at: string
    updated_at: string
}

export type PhysicalAssessmentCompleteDto = {
    physical_assessment_dto: PhysicalAssessmentDto
    age: number
    bmi: number
    bmi_classification: BmiClassification
    min_ideal_weight: number
    medium_ideal_weight: number
    max_ideal_weight: number
    hypertension_classification: HypertensionClassification | null
    circumference_abdominal_classification: AbdominalCircumferenceClassification | null
    fc_heart_rest_classification: FcRestClassification | null
    right_dynamometer_strength_classification: DynamometerStrengthClassification | null
    left_dynamometer_strength_classification: DynamometerStrengthClassification | null
    kilos_lean_body_mass_skinfolds: number | null
    kilos_fat_skinfolds: number | null
    lean_body_mass_percentage_skinfolds: number | null
    fat_percentage_skinfolds: number | null
    ideal_weight_fat_percentage: number | null
    weight_to_lose: number | null
    kilos_lean_body_mass_bioimpedance: number | null
    kilos_fat_bioimpedance: number | null
    lean_body_mass_percentage_bioimpedance: number | null
    fat_percentage_classification_skinfolds: FatPercentageClassification | null
    fat_percentage_classification_bioimpedance: FatPercentageClassification | null
    biceps_asymmetry_percentage: number | null
    triceps_asymmetry_percentage: number | null
    extensor_asymmetry_percentage: number | null
    flexor_asymmetry_percentage: number | null
    biceps_asymmetry_classification: StrengthAsymmetryClassification | null
    triceps_asymmetry_classification: StrengthAsymmetryClassification | null
    extensor_asymmetry_classification: StrengthAsymmetryClassification | null
    flexor_asymmetry_classification: StrengthAsymmetryClassification | null
    waist_hip_calculate: number | null
    waist_hip_classification: WaistHipRiskClassification | null
    maximum_heart_rate: number
    vo2_assessments: Vo2AssessmentCompleteDto[]
    vo2_parameter: number | null
    vo2_classification: Vo2Classification | null
}
