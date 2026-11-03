use crate::models::students::Gender;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum FatPercentageClassification {
    Excellent,
    Healthy,
    Overweight,
    Obesity,
}

pub fn fat_percentage(
    gender: Gender,
    age: i32,
    triceps_skinfold: Option<f64>,
    chest_skinfold: Option<f64>,
    midaxillary_skinfold: Option<f64>,
    subscapular_skinfold: Option<f64>,
    abdominal_skinfold: Option<f64>,
    suprailiac_skinfold: Option<f64>,
    thigh_skinfold: Option<f64>,
) -> Option<f64> {
    let triceps_skinfold = triceps_skinfold?;
    let chest_skinfold = chest_skinfold?;
    let midaxillary_skinfold = midaxillary_skinfold?;
    let subscapular_skinfold = subscapular_skinfold?;
    let abdominal_skinfold = abdominal_skinfold?;
    let suprailiac_skinfold = suprailiac_skinfold?;
    let thigh_skinfold = thigh_skinfold?;

    let sum = triceps_skinfold
        + chest_skinfold
        + midaxillary_skinfold
        + subscapular_skinfold
        + abdominal_skinfold
        + suprailiac_skinfold
        + thigh_skinfold;

    let result = match gender {
        Gender::Male => {
            1.112 - (0.00043499 * sum) + (0.00000055 * (sum * sum)) - (0.00028826 * age as f64)
        }
        Gender::Female => {
            1.097 - (0.00046971 * sum) + (0.00000056 * (sum * sum)) - (0.00012828 * age as f64)
        }
    };

    Some((495.0 / result) - 450.0)
}

pub fn calculate_kilos_fat(kilos: f64, fat_percentage: Option<f64>) -> Option<f64> {
    let fat_percentage = fat_percentage?;

    Some(kilos * fat_percentage / 100.0)
}

fn values_fat_percentage(gender: Gender, age: i32) -> (f64, f64, f64) {
    let values = match gender {
        Gender::Male => match age {
            0..=29 => (11.0, 20.0, 25.0),
            30..=39 => (12.0, 21.0, 25.0),
            40..=49 => (14.0, 23.0, 27.0),
            50..=59 => (15.0, 24.0, 28.0),
            _ => (16.0, 25.0, 30.0),
        },
        Gender::Female => match age {
            0..=29 => (16.0, 28.0, 32.0),
            30..=39 => (17.0, 29.0, 33.0),
            40..=49 => (20.0, 32.0, 35.0),
            50..=59 => (22.0, 33.0, 38.0),
            _ => (23.0, 35.0, 40.0),
        },
    };
    values
}
pub fn classify_fat_percentage(
    gender: Gender,
    age: i32,
    fat_percentage: Option<f64>,
) -> Option<FatPercentageClassification> {
    let fat_percentage = fat_percentage?;

    let (health, overweight, obesity) = values_fat_percentage(gender, age);

    if fat_percentage < health {
        return Some(FatPercentageClassification::Excellent);
    }
    if fat_percentage <= overweight {
        return Some(FatPercentageClassification::Healthy);
    }
    if fat_percentage < obesity {
        return Some(FatPercentageClassification::Overweight);
    }
    Some(FatPercentageClassification::Obesity)
}

pub fn ideal_weight_by_fat_percentage(
    gender: Gender,
    age: i32,
    fat_percentage: Option<f64>,
    weight: f64,
) -> Option<f64> {
    let fat_percentage = fat_percentage?;

    let (_, overweight, obesity) = values_fat_percentage(gender, age);

    if fat_percentage < overweight {
        return None;
    }
    let lean_mass = weight * (1.0 - fat_percentage / 100.0);
    if fat_percentage < obesity {
        return Some(lean_mass / (1.0 - overweight / 100.0));
    }
    Some(lean_mass / (1.0 - obesity / 100.0))
}
