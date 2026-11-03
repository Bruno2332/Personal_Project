use crate::models::students::Gender;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum WaistHipRiskClassification {
    Low,
    Moderate,
    High,
    VeryHigh,
}

pub fn waist_hip_ratio_calculate(waist_cm: Option<f64>, hip_cm: Option<f64>) -> Option<f64> {
    let waist_cm = waist_cm?;
    let hip_cm = hip_cm?;

    Some(waist_cm / hip_cm)
}

pub fn waist_hip_ratio_classification(
    ratio_hundredths: Option<f64>,
    age: i32,
    gender: Gender,
) -> Option<WaistHipRiskClassification> {
    let ratio_hundredths = ratio_hundredths?;

    let (moderate_start, moderate_end, high_end) = match gender {
        Gender::Male => match age {
            0..=29 => (0.83, 0.88, 0.94),
            30..=39 => (0.84, 0.91, 0.96),
            40..=49 => (0.88, 0.95, 1.00),
            50..=59 => (0.90, 0.96, 1.02),
            _ => (0.91, 0.98, 1.03),
        },

        Gender::Female => match age {
            0..=29 => (0.71, 0.77, 0.82),
            30..=39 => (0.72, 0.78, 0.84),
            40..=49 => (0.73, 0.79, 0.87),
            50..=59 => (0.74, 0.81, 0.88),
            _ => (0.76, 0.83, 0.90),
        },
    };

    if ratio_hundredths < moderate_start {
        return Some(WaistHipRiskClassification::Low);
    }
    if ratio_hundredths <= moderate_end {
        return Some(WaistHipRiskClassification::Moderate);
    }
    if ratio_hundredths <= high_end {
        return Some(WaistHipRiskClassification::High);
    }
    Some(WaistHipRiskClassification::VeryHigh)
}
