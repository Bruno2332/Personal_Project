use crate::{
    domain::{
        strength_asymmetry::{
            asymmetry_classification, strength_asymmetry, StrengthAsymmetryClassification,
        },
        vo2_classification::{calculate_vo2_max, vo2_classification, Vo2Classification},
        waist_to_hip_ratio::{
            waist_hip_ratio_calculate, waist_hip_ratio_classification, WaistHipRiskClassification,
        },
    },
    models::students::Gender,
};

fn approximately(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 0.01, "{actual} != {expected}");
}

#[test]
fn vo2_calculation_uses_completed_minute_from_final_stage() {
    assert_eq!(calculate_vo2_max(None, Some(30)), None);
    assert_eq!(calculate_vo2_max(Some(10), None), None);
    approximately(calculate_vo2_max(Some(10), Some(59)).unwrap(), 38.34);
    approximately(calculate_vo2_max(Some(10), Some(60)).unwrap(), 41.82);
}

#[test]
fn vo2_classification_uses_age_and_gender_reference_ranges() {
    assert_eq!(
        vo2_classification(Gender::Male, 25, Some(24.9)),
        Some(Vo2Classification::VeryPoor)
    );
    assert_eq!(
        vo2_classification(Gender::Male, 25, Some(25.0)),
        Some(Vo2Classification::Poor)
    );
    assert_eq!(
        vo2_classification(Gender::Male, 25, Some(43.0)),
        Some(Vo2Classification::Good)
    );
    assert_eq!(
        vo2_classification(Gender::Male, 25, Some(53.0)),
        Some(Vo2Classification::Excellent)
    );
    assert_eq!(
        vo2_classification(Gender::Female, 35, Some(34.0)),
        Some(Vo2Classification::Good)
    );
    assert_eq!(vo2_classification(Gender::Male, 19, Some(40.0)), None);
    assert_eq!(vo2_classification(Gender::Male, 25, None), None);
}

#[test]
fn waist_hip_ratio_requires_both_measurements_and_classifies_risk() {
    assert_eq!(waist_hip_ratio_calculate(Some(80.0), None), None);
    approximately(
        waist_hip_ratio_calculate(Some(90.0), Some(100.0)).unwrap(),
        0.9,
    );
    assert_eq!(
        waist_hip_ratio_classification(Some(0.82), 25, Gender::Male),
        Some(WaistHipRiskClassification::Low)
    );
    assert_eq!(
        waist_hip_ratio_classification(Some(0.88), 25, Gender::Male),
        Some(WaistHipRiskClassification::Moderate)
    );
    assert_eq!(
        waist_hip_ratio_classification(Some(0.94), 25, Gender::Male),
        Some(WaistHipRiskClassification::High)
    );
    assert_eq!(
        waist_hip_ratio_classification(Some(0.95), 25, Gender::Male),
        Some(WaistHipRiskClassification::VeryHigh)
    );
    assert_eq!(waist_hip_ratio_classification(None, 25, Gender::Male), None);
}

#[test]
fn strength_asymmetry_handles_both_sides_and_zero_maximum() {
    assert_eq!(strength_asymmetry(Some(10.0), None), None);
    assert_eq!(strength_asymmetry(Some(0.0), Some(0.0)), None);
    approximately(strength_asymmetry(Some(80.0), Some(100.0)).unwrap(), 20.0);
    assert_eq!(
        asymmetry_classification(Some(9.99)),
        Some(StrengthAsymmetryClassification::NormalAsymmetry)
    );
    assert_eq!(
        asymmetry_classification(Some(10.0)),
        Some(StrengthAsymmetryClassification::ModerateAsymmetry)
    );
    assert_eq!(
        asymmetry_classification(Some(15.0)),
        Some(StrengthAsymmetryClassification::SevereAsymmetry)
    );
    assert_eq!(asymmetry_classification(None), None);
}
