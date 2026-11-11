use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum StrengthAsymmetryClassification {
    NormalAsymmetry,
    ModerateAsymmetry,
    SevereAsymmetry,
}

pub fn strength_asymmetry(left_side: Option<f64>, right_side: Option<f64>) -> Option<f64> {
    let left_side = left_side? as f64;
    let right_side = right_side? as f64;

    let maximum = right_side.max(left_side);
    let minimum = right_side.min(left_side);

    if maximum == 0.0 {
        return None;
    }

    Some((maximum - minimum) / maximum * 100.0)
}

pub fn asymmetry_classification(
    asymmetry_percentage: Option<f64>,
) -> Option<StrengthAsymmetryClassification> {
    let asymmetry_percentage = asymmetry_percentage?;

    if asymmetry_percentage < 10.0 {
        return Some(StrengthAsymmetryClassification::NormalAsymmetry);
    }
    if asymmetry_percentage < 15.0 {
        return Some(StrengthAsymmetryClassification::ModerateAsymmetry);
    }
    Some(StrengthAsymmetryClassification::SevereAsymmetry)
}
