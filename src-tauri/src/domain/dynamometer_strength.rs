use crate::models::students::Gender;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum DynamometerStrengthClassification {
    Weak,
    Normal,
    Strong,
}

pub fn dynamometer_classification(
    gender: Gender,
    age: i32,
    strength: Option<f64>,
) -> Option<DynamometerStrengthClassification> {
    let strength = strength?;

    if age < 18 {
        return None;
    }

    let (normal, strong) = match gender {
        Gender::Male => match age {
            18..=29 => (37.0, 57.0),
            30..=39 => (36.0, 55.5),
            40..=49 => (35.5, 55.0),
            50..=59 => (32.5, 50.5),
            _ => (28.0, 44.0),
        },
        Gender::Female => match age {
            18..=29 => (25.0, 41.0),
            30..=39 => (24.5, 40.0),
            40..=49 => (23.5, 38.5),
            50..=59 => (21.0, 35.5),
            _ => (18.0, 31.0),
        },
    };
    if strength < normal {
        return Some(DynamometerStrengthClassification::Weak);
    }
    if strength < strong {
        return Some(DynamometerStrengthClassification::Normal);
    }
    Some(DynamometerStrengthClassification::Strong)
}
