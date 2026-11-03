use crate::domain::bmi::bmi::BmiClassification;

const THINNESS: f64 = 18.5;
const NORMAL_WEIGHT: f64 = 24.9;
const OVERWEIGHT: f64 = 29.9;
const OBESITY_CLASS_I: f64 = 34.9;
const OBESITY_CLASS_II: f64 = 39.9;

pub fn adult_bmi_classification(bmi: f64) -> BmiClassification {
    if bmi < THINNESS {
        BmiClassification::Thinness
    } else if bmi <= NORMAL_WEIGHT {
        BmiClassification::NormalWeight
    } else if bmi <= OVERWEIGHT {
        BmiClassification::OverWeight
    } else if bmi <= OBESITY_CLASS_I {
        BmiClassification::ObesityClassI
    } else if bmi <= OBESITY_CLASS_II {
        BmiClassification::ObesityClassII
    } else {
        BmiClassification::ObesityClassIII
    }
}

pub fn calculate_ideal_weight_adult(height: f64) -> [f64; 3] {
    let height_squared = height * height;

    [
        height_squared * 18.5,
        height_squared * 22.2,
        height_squared * 24.9,
    ]
}
