import type { RowInput } from "jspdf-autotable"
import type { PhysicalAssessmentCompleteDto } from "../../model/physicalAssessment"
import type { PdfSection } from "../assessmentPdfLayout"

import {
    classification,
    formatNumber,
    measurement,
} from "../assessmentPdfFormatters"

import * as labels from "../../labels/physicalAssessment"
import * as tones from "../../labels/classificationTone"

export default function assessmentVo2Section(
    section: PdfSection,
    r: PhysicalAssessmentCompleteDto,
) {
    const a = r.physical_assessment_dto

    if (
        r.vo2_assessments.length === 0 &&
        r.vo2_parameter == null
    ) {
        return
    }

    const stages: RowInput[] = r.vo2_assessments.map((stage, index) => {
        const data = stage.vo2_assessment_dto

        return [
            String(index + 1),
            formatNumber(data.treadmill_speed_kmh),
            formatNumber(data.heart_rate_bpm),
            formatNumber(stage.fc_heart_percentage, " %"),
            classification(
                stage.fc_heart_classification,
                labels.fcExerciseLabels,
                tones.fcExerciseTones,
            ),
            classification(
                data.perceived_exertion,
                labels.perceivedExertionLabels,
                tones.perceivedExertionTones,
            ),
        ]
    })

    section(
        "VO2 — etapas da avaliação",
        stages,
        [
            "Etapa",
            "km/h",
            "FC (bpm)",
            "% FC máx.",
            "Intensidade FC",
            "Esforço",
        ],
    )

    section("VO2 — resultado", [
        ...measurement(
            "Segundos do último estágio",
            a.vo2_final_stage_seconds,
            " s",
        ),
        ...measurement(
            "Frequência cardíaca máxima",
            r.maximum_heart_rate,
            " bpm",
        ),
        ...measurement(
            "VO2 máximo",
            r.vo2_parameter,
            " mL/kg/min",
            classification(
                r.vo2_classification,
                labels.vo2Labels,
                tones.vo2Tones,
            ),
        ),
    ])
}