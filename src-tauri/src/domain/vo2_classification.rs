use crate::models::students::Gender;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Vo2Classification {
    VeryPoor,
    Poor,
    Fair,
    Good,
    Excellent,
}

pub fn calculate_vo2_max(
    prev_speed: Option<i32>,
    current_stage_seconds: Option<i32>,
) -> Option<f64> {
    let prev_speed = prev_speed?;
    let current_stage_seconds = current_stage_seconds?;

    let final_speed = prev_speed + (current_stage_seconds / 60);

    let vo2 = (final_speed as f64 * 3.483866666666667) + 3.5;

    Some((vo2 * 100.0).round() / 100.0)
}

pub fn vo2_classification(gender: Gender, age: i32, vo2: Option<f64>) -> Option<Vo2Classification> {
    let vo2 = vo2?;

    let (poor, fair, good, excellent) = match gender {
        Gender::Male => match age {
            20..=29 => (25.0, 34.0, 43.0, 53.0),
            30..=39 => (23.0, 31.0, 39.0, 49.0),
            40..=49 => (20.0, 27.0, 36.0, 45.0),
            50..=59 => (18.0, 25.0, 34.0, 43.0),
            60..=69 => (16.0, 23.0, 31.0, 41.0),
            _ => return None,
        },

        Gender::Female => match age {
            20..=29 => (24.0, 31.0, 38.0, 49.0),
            30..=39 => (20.0, 28.0, 34.0, 45.0),
            40..=49 => (17.0, 24.0, 31.0, 42.0),
            50..=59 => (15.0, 21.0, 28.0, 38.0),
            60..=69 => (13.0, 18.0, 24.0, 35.0),
            _ => return None,
        },
    };

    if vo2 < poor {
        return Some(Vo2Classification::VeryPoor);
    }
    if vo2 < fair {
        return Some(Vo2Classification::Poor);
    }
    if vo2 < good {
        return Some(Vo2Classification::Fair);
    }
    if vo2 < excellent {
        return Some(Vo2Classification::Good);
    }
    Some(Vo2Classification::Excellent)
}
