use crate::models::students::Gender;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum AbdominalCircumferenceClassification {
    LowRisk,
    HighRisk,
    VeryHighRisk,
}

pub fn values_abdominal_circumference(
    gender: Gender
) -> (f64, f64) {
    let values = match gender {
        Gender::Male => (94.0, 102.0),
        Gender::Female => (80.0, 88.0)
    };
    values
}

pub fn abdominal_circumference_calculate(
    gender: Gender,
    circumference: Option<f64>,
) -> Option<AbdominalCircumferenceClassification> {
    let circumference = circumference?;

    let (high_risk, very_high_risk) = values_abdominal_circumference(gender);

    if circumference < high_risk {
        return Some(AbdominalCircumferenceClassification::LowRisk);
    }
    if circumference < very_high_risk {
        return Some(AbdominalCircumferenceClassification::HighRisk);
    }
    Some(AbdominalCircumferenceClassification::VeryHighRisk)
}

