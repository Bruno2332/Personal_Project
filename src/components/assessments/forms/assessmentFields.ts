import type { PhysicalAssessmentDto } from "../../../model/physicalAssessment"


export type MeasurementField = {
    name: keyof PhysicalAssessmentDto
    label: string
}


export const circumferenceFields: MeasurementField[] = [
    { name: "neck_circumference_cm", label: "Pescoço" },
    { name: "right_arm_circumference_cm", label: "Braço direito" },
    { name: "left_arm_circumference_cm", label: "Braço esquerdo" },
    { name: "right_forearm_circumference_cm", label: "Antebraço direito" },
    { name: "left_forearm_circumference_cm", label: "Antebraço esquerdo" },
    { name: "shoulder_circumference_cm", label: "Ombro" },
    { name: "chest_circumference_cm", label: "Peitoral" },
    { name: "right_thigh_circumference_cm", label: "Coxa direita" },
    { name: "left_thigh_circumference_cm", label: "Coxa esquerda" },
    { name: "right_calf_circumference_cm", label: "Panturrilha direita" },
    { name: "left_calf_circumference_cm", label: "Panturrilha esquerda" },
    { name: "waist_circumference_cm", label: "Cintura" },
    { name: "hip_circumference_cm", label: "Quadril" },
    { name: "abdomen_circumference_cm", label: "Abdômen" },
    
]

export const skinfoldFields: MeasurementField[] = [
    { name: "triceps_skinfold_mm", label: "Tríceps" },
    { name: "chest_skinfold_mm", label: "Peitoral" },
    { name: "midaxillary_skinfold_mm", label: "Axilar média" },
    { name: "subscapular_skinfold_mm", label: "Subescapular" },
    { name: "abdominal_skinfold_mm", label: "Abdominal" },
    { name: "suprailiac_skinfold_mm", label: "Supra-ilíaca" },
    { name: "thigh_skinfold_mm", label: "Coxa" },
]

export const strengthFields: MeasurementField[] = [
    { name: "right_handgrip_strength", label: "Dinamômetro direito" },
    { name: "left_handgrip_strength", label: "Dinamômetro esquerdo" },
    { name: "right_extensor_strength", label: "Extensora direita" },
    { name: "left_extensor_strength", label: "Extensora esquerda" },
    { name: "right_flexor_strength", label: "Flexora direita" },
    { name: "left_flexor_strength", label: "Flexora esquerda" },
    { name: "right_biceps_strength", label: "Bíceps direito" },
    { name: "left_biceps_strength", label: "Bíceps esquerdo" },
    { name: "right_triceps_strength", label: "Tríceps direito" },
    { name: "left_triceps_strength", label: "Tríceps esquerdo" },
]

export const assessmentTabs = [
    { id: "general", label: "Dados gerais" },
    { id: "goals", label: "Objetivos" },
    { id: "circumferences", label: "Circunferências" },
    { id: "composition", label: "Composição corporal" },
    { id: "strength", label: "Força" },
    { id: "vo2", label: "VO₂" },
    { id: "photos", label: "Fotos" }
] as const

export type AssessmentTab = typeof assessmentTabs[number]["id"]

export const assessmentFormTabs = assessmentTabs.filter(
    (tab) => tab.id !== "photos"
)