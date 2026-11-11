import type {
    BmiClassification,
    HypertensionClassification,
    AbdominalCircumferenceClassification,
    DynamometerStrengthClassification,
    FatPercentageClassification,
    StrengthAsymmetryClassification,
    WaistHipRiskClassification,
    Vo2Classification,
    FcRestClassification,
} from "../model/physicalAssessment";
import type { ClassificationTone } from "../components/ui/ClassificationBadge";
import type { ExerciseHeartRateClassification, PerceivedExertion } from "../model/vo2Assessment";


export const bmiTones: Record<BmiClassification, ClassificationTone> = {
    SevereThinness: "risk",
    Thinness: "attention",
    NormalWeight: "normal",
    OverWeight: "attention",
    Obesity: "risk",
    SevereObesity: "risk",
    ObesityClassI: "risk",
    ObesityClassII: "risk",
    ObesityClassIII: "risk",
}
export const pressureTones: Record<HypertensionClassification, ClassificationTone> = {
    Normal: "normal",
    PreHypertension: "attention",
    StageOneHypertension: "risk",
    StageTwoHypertension: "risk",
    StageThreeHypertension: "risk",
}
export const abdomenTones: Record<AbdominalCircumferenceClassification, ClassificationTone> =
{
    LowRisk: "normal",
    HighRisk: "attention",
    VeryHighRisk: "risk",
}
export const dynamometerTones: Record<DynamometerStrengthClassification, ClassificationTone> =
{
    Weak: "risk",
    Normal: "normal",
    Strong: "normal",
}
export const fatTones: Record<FatPercentageClassification, ClassificationTone> =
{
    Excellent: "normal",
    Healthy: "normal",
    Overweight: "attention",
    Obesity: "risk",
}

export const fcTones: Record<FcRestClassification, ClassificationTone> = {
    Bradycardia: "attention",
    Normal: "normal",
    Tachycardia: "risk"

}

export const asymmetryTones: Record<StrengthAsymmetryClassification, ClassificationTone> =
{
    NormalAsymmetry: "normal",
    ModerateAsymmetry: "attention",
    SevereAsymmetry: "risk",
}
export const waistHipTones: Record<WaistHipRiskClassification, ClassificationTone> =
{
    Low: "normal",
    Moderate: "attention",
    High: "risk",
    VeryHigh: "risk",
}
export const vo2Tones: Record<Vo2Classification, ClassificationTone> = {
    VeryPoor: "risk",
    Poor: "risk",
    Fair: "attention",
    Good: "normal",
    Excellent: "normal",
}

export const perceivedExertionTones: Record<PerceivedExertion, ClassificationTone> = {
    Light: "normal",
    Moderate: "attention",
    Intermediate: "attention",
    Intense: "risk",
    Maximal: "risk"
}

export const fcExerciseTones: Record<ExerciseHeartRateClassification, ClassificationTone> = {
    Normal: "normal",
    Alert: "attention",
    Extreme: "risk"
}
