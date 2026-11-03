import type { Gender, Objective } from "../model/student"

export const genderLabels: Record<Gender, string> = {
    Male: "Masculino",
    Female: "Feminino",
}
export const objectiveLabels: Record<Objective, string> = {
    HypertrophyAndDefinition: "Hipertrofia e definição muscular",
    WeightLoss: "Emagrecimento e condicionamento físico",
    RehabilitationStrengthening: "Reabilitação e fortalecimento",
    SportsPerformance: "Performance esportiva",
    GeneralConditioningHealth: "Condicionamento geral e saúde",
    FlexibilityMobility: "Flexibilidade e mobilidade",
}
