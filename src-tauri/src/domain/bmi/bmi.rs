use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy)]
#[repr(i32)]
pub enum BmiClassification {
    SevereThinness = 0,
    Thinness = 1,
    NormalWeight = 2,
    OverWeight = 3,
    Obesity = 4,
    SevereObesity = 5,
    ObesityClassI = 6,
    ObesityClassII = 7,
    ObesityClassIII = 8,
}

pub fn bmi_calculate(weight: f64, height: f64) -> f64 {
    weight / (height * height)
}
