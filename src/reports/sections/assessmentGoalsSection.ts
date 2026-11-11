import type { RowInput } from "jspdf-autotable"
import type { PhysicalAssessmentCompleteDto } from "../../model/physicalAssessment"
import type { PdfSection } from "../assessmentPdfLayout"

import {
    classification,
    formatNumber,
} from "../assessmentPdfFormatters"

import * as labels from "../../labels/physicalAssessment"
import * as tones from "../../labels/classificationTone"

export default function assessmentGoalsSection(
    section: PdfSection,
    r: PhysicalAssessmentCompleteDto,
) {
    const idealMin = r.ideal_weight_fat_percentage_min
    const idealMax = r.ideal_weight_fat_percentage_max

    const lossMin = r.weight_to_lose_min_percentage
    const lossMax = r.weight_to_lose_max_percentage
    const lossHealth = r.weight_to_lose_health
    const lossOverweight = r.weight_to_lose_overweight

    const hasFatData =
        idealMin != null &&
        idealMax != null &&
        lossMin != null &&
        lossMax != null &&
        lossHealth != null &&
        lossOverweight != null

    if (hasFatData) {
        const nextGoal =
            lossMin < 0
                ? {
                    label: "Alcançar o percentual mínimo de referência",
                    amount: -lossMin,
                    gain: true,
                }
                : lossOverweight > 0
                    ? {
                        label: "Alcançar o limite para sobrepeso",
                        amount: lossOverweight,
                        gain: false,
                    }
                    : lossHealth > 0
                        ? {
                            label: "Alcançar o limite para normalidade",
                            amount: lossHealth,
                            gain: false,
                        }
                        : lossMax > 0
                            ? {
                                label: "Entrar na faixa de referência",
                                amount: lossMax,
                                gain: false,
                            }
                            : null

        const rows: RowInput[] = [
            [
                "Percentual de gordura atual",
                formatNumber(r.fat_percentage_skinfolds, " %", 2),
                classification(
                    r.fat_percentage_classification_skinfolds,
                    labels.fatLabels,
                    tones.fatTones,
                ),
            ],
            [
                "Próxima meta",
                nextGoal?.label ?? "Faixa de referência atingida",
                "",
            ],
        ]

        if (nextGoal) {
            rows.push([
                "Ajuste de gordura para a próxima meta",
                `${nextGoal.gain ? "Ganhar" : "Reduzir"} ${formatNumber(nextGoal.amount, " kg")}`,
                "",
            ])
        }

        if (lossOverweight > 0) {
            rows.push([
                "Faixa de sobrepeso",
                `${formatNumber(r.ideal_weight_fat_percentage_health, " kg")} a ${formatNumber(r.ideal_weight_fat_percentage_overweight, " kg")}`,
                `${formatNumber(r.ideal_fat_percentage_health, " %")} a ${formatNumber(r.ideal_fat_percentage_overweight, " %")} de gordura`,
            ])
        }

        if (lossHealth > 0) {
            rows.push([
                "Faixa de normalidade",
                `${formatNumber(idealMax, " kg")} a ${formatNumber(r.ideal_weight_fat_percentage_health, " kg")}`,
                `${formatNumber(r.ideal_fat_percentage_max, " %")} a ${formatNumber(r.ideal_fat_percentage_health, " %")} de gordura`,
            ])
        }

        if (lossOverweight > 0) {
            rows.push([
                "Ajuste de gordura para a normalidade",
                `Reduzir ${formatNumber(lossHealth, " kg")}`,
                "",
            ])
        }

        rows.push([
            "Faixa de referência",
            `${formatNumber(idealMin, " kg")} a ${formatNumber(idealMax, " kg")}`,
            `${formatNumber(r.ideal_fat_percentage_min, " %")} a ${formatNumber(r.ideal_fat_percentage_max, " %")} de gordura`,
        ])

        if (lossMax > 0) {
            rows.push([
                "Ajuste de gordura para a faixa de referência",
                `Reduzir ${formatNumber(lossMax, " kg")} a ${formatNumber(lossMin, " kg")}`,
                "",
            ])
        }

        rows.push([
            "Referência do cálculo",
            "Estimativas considerando a manutenção da massa magra atual.",
            "As reduções partem do peso atual e não devem ser somadas.",
        ])

        section(
            "Objetivos — percentual de gordura",
            rows,
            ["Objetivo / medida", "Resultado / meta", "Referência"],
        )
    }

    const abdomen = r.physical_assessment_dto.abdomen_circumference_cm
    const abdomenHigh = r.circumference_abdominal_high_risk
    const abdomenVeryHigh = r.circumference_abdominal_very_high_risk

    if (
        abdomen != null &&
        abdomenHigh != null &&
        abdomenVeryHigh != null
    ) {
        const target =
            abdomen >= abdomenVeryHigh
                ? abdomenVeryHigh
                : abdomen >= abdomenHigh
                    ? abdomenHigh
                    : null

        section(
            "Objetivos — circunferência de abdômen",
            [
                [
                    "Circunferência atual",
                    formatNumber(abdomen, " cm"),
                    classification(
                        r.circumference_abdominal_classification,
                        labels.abdomenLabels,
                        tones.abdomenTones,
                    ),
                ],
                [
                    "Próxima meta",
                    target != null
                        ? `Ficar abaixo de ${formatNumber(target, " cm")}`
                        : "Faixa de baixo risco atingida",
                    "",
                ],
                [
                    "Referência para baixo risco",
                    `Abaixo de ${formatNumber(abdomenHigh, " cm")}`,
                    "",
                ],
            ],
            ["Objetivo / medida", "Resultado / meta", "Classificação"],
        )
    }

    const waistHip = r.waist_hip_calculate
    const moderate = r.waist_hip_moderate_risk
    const high = r.waist_hip_high_risk
    const veryHigh = r.waist_hip_very_high_risk

    if (
        waistHip != null &&
        moderate != null &&
        high != null &&
        veryHigh != null
    ) {
        const target =
            waistHip >= veryHigh
                ? veryHigh
                : waistHip >= high
                    ? high
                    : waistHip >= moderate
                        ? moderate
                        : null

        section(
            "Objetivos — relação cintura-quadril",
            [
                [
                    "Relação cintura-quadril atual",
                    formatNumber(waistHip, "", 2),
                    classification(
                        r.waist_hip_classification,
                        labels.waistHipLabels,
                        tones.waistHipTones,
                    ),
                ],
                [
                    "Próxima meta",
                    target != null
                        ? `Ficar abaixo de ${formatNumber(target)}`
                        : "Faixa de baixo risco atingida",
                    "",
                ],
                [
                    "Referência para baixo risco",
                    `Abaixo de ${formatNumber(moderate)}`,
                    "",
                ],
            ],
            ["Objetivo / medida", "Resultado / meta", "Classificação"],
        )
    }
}