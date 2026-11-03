use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum HypertensionClassification {
    Normal,
    PreHypertension,
    StageOneHypertension,
    StageTwoHypertension,
    StageThreeHypertension,
}

pub fn hypertension_classification(
    systolic: Option<i32>,
    diastolic: Option<i32>,
) -> Option<HypertensionClassification> {
    let systolic = systolic?;
    let diastolic = diastolic?;

    if systolic <= 120 && diastolic <= 80 {
        return Some(HypertensionClassification::Normal);
    }
    if systolic <= 139 && diastolic <= 89 {
        return Some(HypertensionClassification::PreHypertension);
    }
    if systolic <= 159 && diastolic <= 99 {
        return Some(HypertensionClassification::StageOneHypertension);
    }
    if systolic <= 179 && diastolic <= 109 {
        return Some(HypertensionClassification::StageTwoHypertension);
    }
    Some(HypertensionClassification::StageThreeHypertension)
}
