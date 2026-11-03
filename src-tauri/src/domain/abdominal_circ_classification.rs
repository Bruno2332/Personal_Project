use crate::models::students::Gender;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum AbdominalCircumferenceClassification {
    LowRisk,
    HighRisk,
    VeryHighRisk,
}

pub fn abdominal_circumference_calculate(
    gender: Gender,
    circumference: Option<f64>,
) -> Option<AbdominalCircumferenceClassification> {
    let circumference = circumference?;

    let (high_risk, very_high_risk) = match gender {
        Gender::Male => (94.0, 102.0),
        Gender::Female => (80.0, 88.0),
    };

    if circumference < high_risk {
        return Some(AbdominalCircumferenceClassification::LowRisk);
    }
    if circumference < very_high_risk {
        return Some(AbdominalCircumferenceClassification::HighRisk);
    }
    Some(AbdominalCircumferenceClassification::VeryHighRisk)
}
