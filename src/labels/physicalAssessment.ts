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
} from "../model/physicalAssessment"
import { ExerciseHeartRateClassification, PerceivedExertion } from "../model/vo2Assessment"
import { PhotoPosition } from "../model/physicalAssessmentPhotos"


export const bmiLabels: Record<BmiClassification, string> = {
    SevereThinness: "Magreza acentuada",
    Thinness: "Magreza",
    NormalWeight: "Normal",
    OverWeight: "Sobrepeso",
    Obesity: "Obesidade",
    SevereObesity: "Obesidade grave",
    ObesityClassI: "Obesidade grau I",
    ObesityClassII: "Obesidade grau II",
    ObesityClassIII: "Obesidade grau III",
}

export const pressureLabels: Record<HypertensionClassification, string> = {
    Normal: "Normal",
    PreHypertension: "Pré-hipertensão",
    StageOneHypertension: "Hipertensão estágio 1",
    StageTwoHypertension: "Hipertensão estágio 2",
    StageThreeHypertension: "Hipertensão estágio 3",
}

export const abdomenLabels: Record<AbdominalCircumferenceClassification, string> =
{
    LowRisk: "Baixo risco",
    HighRisk: "Risco aumentado",
    VeryHighRisk: "Risco alto",
}

export const fcLabels: Record<FcRestClassification, string> = {
    Bradycardia: "Bradicardia",
    Normal: "Normal",
    Tachycardia: "Taquicardia"
}

export const dynamometerLabels: Record<DynamometerStrengthClassification, string> ={
     Weak: "Fraco", Normal: "Normal", Strong: "Forte" 
    }

export const fatLabels: Record<FatPercentageClassification, string> = {
    Excellent: "Excelente",
    Healthy: "Saudável",
    Overweight: "Sobrepeso",
    Obesity: "Obesidade",
}

export const asymmetryLabels: Record<StrengthAsymmetryClassification, string> = {
    NormalAsymmetry: "Assimetria normal",
    ModerateAsymmetry: "Assimetria moderada",
    SevereAsymmetry: "Assimetria acentuada",
}

export const waistHipLabels: Record<WaistHipRiskClassification, string> = {
    Low: "Risco baixo",
    Moderate: "Risco moderado",
    High: "Risco alto",
    VeryHigh: "Risco muito alto",
}

export const vo2Labels: Record<Vo2Classification, string> = {
    VeryPoor: "Muito baixo",
    Poor: "Baixo",
    Fair: "Regular",
    Good: "Bom",
    Excellent: "Excelente",
}

export const fcExerciseLabels: Record<ExerciseHeartRateClassification, string> = {
    Normal: "Leve",
    Alert: "Moderado",
    Extreme: "Extremo"
}

export const perceivedExertionLabels: Record<PerceivedExertion, string> = {
    Light: "Leve",
    Moderate: "Moderado",
    Intermediate: "Intermediário",
    Intense: "Intenso",
    Maximal: "Máximo"
}

export const photoPositionLabels: Record<PhotoPosition, string> = {
    Front: "Frente",
    Side: "Lado",
    Back: "Costas",
}
