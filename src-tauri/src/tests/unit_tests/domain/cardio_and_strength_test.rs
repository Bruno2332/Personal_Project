use crate::{
    domain::{
        abdominal_circ_classification::{
            abdominal_circumference_calculate, AbdominalCircumferenceClassification,
        },
        dynamometer_strength::{dynamometer_classification, DynamometerStrengthClassification},
        fc_classification::{
            exercise_heart_rate_classification, fc_percentage, fc_rest_classification,
            ExerciseHeartRateClassification, FcRestClassification,
        },
        hypertension_classification::{hypertension_classification, HypertensionClassification},
    },
    models::students::Gender,
};

#[test]
fn hypertension_requires_both_readings_and_selects_highest_category() {
    assert_eq!(hypertension_classification(None, Some(80)), None);
    assert_eq!(
        hypertension_classification(Some(120), Some(80)),
        Some(HypertensionClassification::Normal)
    );
    assert_eq!(
        hypertension_classification(Some(130), Some(80)),
        Some(HypertensionClassification::PreHypertension)
    );
    assert_eq!(
        hypertension_classification(Some(150), Some(95)),
        Some(HypertensionClassification::StageOneHypertension)
    );
    assert_eq!(
        hypertension_classification(Some(170), Some(105)),
        Some(HypertensionClassification::StageTwoHypertension)
    );
    assert_eq!(
        hypertension_classification(Some(190), Some(120)),
        Some(HypertensionClassification::StageThreeHypertension)
    );
}

#[test]
fn abdominal_circumference_classifies_male_and_female_thresholds() {
    assert_eq!(abdominal_circumference_calculate(Gender::Male, None), None);
    assert_eq!(
        abdominal_circumference_calculate(Gender::Male, Some(93.9)),
        Some(AbdominalCircumferenceClassification::LowRisk)
    );
    assert_eq!(
        abdominal_circumference_calculate(Gender::Male, Some(94.0)),
        Some(AbdominalCircumferenceClassification::HighRisk)
    );
    assert_eq!(
        abdominal_circumference_calculate(Gender::Female, Some(88.0)),
        Some(AbdominalCircumferenceClassification::VeryHighRisk)
    );
}

#[test]
fn resting_heart_rate_classifies_boundaries_and_missing_values() {
    assert_eq!(fc_rest_classification(None), None);
    assert_eq!(
        fc_rest_classification(Some(59)),
        Some(FcRestClassification::Bradycardia)
    );
    assert_eq!(
        fc_rest_classification(Some(60)),
        Some(FcRestClassification::Normal)
    );
    assert_eq!(
        fc_rest_classification(Some(99)),
        Some(FcRestClassification::Normal)
    );
    assert_eq!(
        fc_rest_classification(Some(100)),
        Some(FcRestClassification::Tachycardia)
    );
}

#[test]
fn exercise_heart_rate_percentage_uses_age_predicted_maximum() {
    assert_eq!(fc_percentage(40, None), None);
    assert_eq!(fc_percentage(40, Some(180)), Some(100.0));
    assert_eq!(
        exercise_heart_rate_classification(Some(69.9)),
        Some(ExerciseHeartRateClassification::Normal)
    );
    assert_eq!(
        exercise_heart_rate_classification(Some(70.0)),
        Some(ExerciseHeartRateClassification::Alert)
    );
    assert_eq!(
        exercise_heart_rate_classification(Some(90.0)),
        Some(ExerciseHeartRateClassification::Extreme)
    );
    assert_eq!(exercise_heart_rate_classification(None), None);
}

#[test]
fn dynamometer_uses_age_gender_thresholds_and_omits_children() {
    assert_eq!(
        dynamometer_classification(Gender::Male, 17, Some(80.0)),
        None
    );
    assert_eq!(dynamometer_classification(Gender::Female, 25, None), None);
    assert_eq!(
        dynamometer_classification(Gender::Male, 25, Some(36.9)),
        Some(DynamometerStrengthClassification::Weak)
    );
    assert_eq!(
        dynamometer_classification(Gender::Male, 25, Some(37.0)),
        Some(DynamometerStrengthClassification::Normal)
    );
    assert_eq!(
        dynamometer_classification(Gender::Female, 25, Some(41.0)),
        Some(DynamometerStrengthClassification::Strong)
    );
}
