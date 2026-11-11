import type { RowInput } from "jspdf-autotable"
import type { PhysicalAssessmentCompleteDto } from "../../model/physicalAssessment"
import type { StudentDto } from "../../model/student"
import type { PdfSection } from "../assessmentPdfLayout"

import {
    classification,
    formatDate,
    measurement,
} from "../assessmentPdfFormatters"

import * as labels from "../../labels/physicalAssessment"
import * as tones from "../../labels/classificationTone"

export default function assessmentGeneralSection(
    section: PdfSection,
    r: PhysicalAssessmentCompleteDto,
    student: StudentDto,
) {
    const a = r.physical_assessment_dto

    section(
        "Identificação",
        [
            ["Aluno", student.name],
            ["Data da avaliação", formatDate(a.assessment_date)],
            ["Idade na avaliação", `${r.age} anos`],
            ["Sexo", student.gender === "Male" ? "Masculino" : "Feminino"],
        ],
        ["Informação", "Dados"],
    )

    const pressureRows: RowInput[] =
        a.systolic_pressure != null && a.diastolic_pressure != null
            ? [[
                "Pressão arterial",
                `${a.systolic_pressure}/${a.diastolic_pressure} mmHg`,
                classification(
                    r.hypertension_classification,
                    labels.pressureLabels,
                    tones.pressureTones,
                ),
            ]]
            : [
                ...measurement(
                    "Pressão sistólica",
                    a.systolic_pressure,
                    " mmHg",
                ),
                ...measurement(
                    "Pressão diastólica",
                    a.diastolic_pressure,
                    " mmHg",
                ),
            ]

    section("Dados gerais", [
        ...measurement("Peso", a.weight_kg, " kg"),
        ...measurement("Altura", a.height_m, " m"),
        ...measurement(
            "IMC",
            r.bmi,
            " kg/m²",
            classification(
                r.bmi_classification,
                labels.bmiLabels,
                tones.bmiTones,
            ),
        ),
        ...measurement("Peso ideal mínimo", r.min_ideal_weight, " kg"),
        ...measurement("Peso ideal médio", r.medium_ideal_weight, " kg"),
        ...measurement("Peso ideal máximo", r.max_ideal_weight, " kg"),
        ...pressureRows,
        ...measurement(
            "Frequência cardíaca em repouso",
            a.resting_heart_rate_bpm,
            " bpm",
            classification(
                r.fc_heart_rest_classification,
                labels.fcLabels,
                tones.fcTones,
            ),
        ),
    ])
}