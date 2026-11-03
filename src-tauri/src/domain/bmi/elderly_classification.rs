use crate::domain::bmi::bmi::BmiClassification;

const THINNESS: f64 = 22.0;
const NORMAL_WEIGHT: f64 = 26.9;

pub fn elderly_bmi_classification(bmi: f64) -> BmiClassification {
    if bmi < THINNESS {
        BmiClassification::Thinness
    } else if bmi <= NORMAL_WEIGHT {
        BmiClassification::NormalWeight
    } else {
        BmiClassification::OverWeight
    }
}

pub fn calculate_ideal_weight_elderly(height: f64) -> [f64; 3] {
    let height_squared = height * height;

    [
        height_squared * 22.0,
        height_squared * 24.5,
        height_squared * 27.0,
    ]
}
