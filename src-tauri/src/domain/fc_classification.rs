use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum FcRestClassification {
    Bradycardia,
    Normal,
    Tachycardia,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum ExerciseHeartRateClassification {
    Normal,
    Alert,
    Extreme,
}

pub fn fc_rest_classification(heart_fc: Option<i32>) -> Option<FcRestClassification> {
    let heart_fc = heart_fc?;

    if heart_fc < 60 {
        return Some(FcRestClassification::Bradycardia);
    }
    if heart_fc < 100 {
        return Some(FcRestClassification::Normal);
    }
    Some(FcRestClassification::Tachycardia)
}

pub fn fc_percentage(age: i32, fc: Option<i32>) -> Option<f64> {
    let fc = fc?;
    let maximal_fc = 220 - age;

    Some(fc as f64 * 100.0 / maximal_fc as f64)
}

pub fn exercise_heart_rate_classification(
    fc_percentage: Option<f64>,
) -> Option<ExerciseHeartRateClassification> {
    let fc_percentage = fc_percentage?;

    if fc_percentage < 70.0 {
        return Some(ExerciseHeartRateClassification::Normal);
    }
    if fc_percentage < 90.0 {
        return Some(ExerciseHeartRateClassification::Alert);
    }
    Some(ExerciseHeartRateClassification::Extreme)
}
