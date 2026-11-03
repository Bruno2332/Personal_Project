import type { PhysicalAssessmentCompleteDto } from "../model/physicalAssessment"
export function assessmentView(result: PhysicalAssessmentCompleteDto) {
  const a = result.physical_assessment_dto
  return {
    id: a.id,
    data: a.assessment_date,
    peso: a.weight_kg,
    imc: result.bmi,
    percentualGordura: result.fat_percentage_skinfolds,
    massaMuscular: result.kilos_lean_body_mass_skinfolds,
    pressaoArterial:
      a.systolic_pressure !== null && a.diastolic_pressure !== null
        ? `${a.systolic_pressure}/${a.diastolic_pressure}`
        : "—",
    frequenciaCardiacaRepouso: a.resting_heart_rate_bpm,
    circunfencias: {
      cintura: a.waist_circumference_cm,
      quadril: a.hip_circumference_cm,
      braco: a.right_arm_circumference_cm,
      coxa: a.right_thigh_circumference_cm,
      panturrilha: a.right_calf_circumference_cm,
      peitoral: a.chest_circumference_cm,
      abdomen: a.abdomen_circumference_cm,
    },
    raw: result,
  }
}
export type AssessmentView = ReturnType<typeof assessmentView>
