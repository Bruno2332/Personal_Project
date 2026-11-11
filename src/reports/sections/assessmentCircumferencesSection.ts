import type { PhysicalAssessmentCompleteDto } from "../../model/physicalAssessment"
import type { PdfSection } from "../assessmentPdfLayout"

import { circumferenceFields } from "../../components/assessments/forms/assessmentFields"
import { classification, measurement } from "../assessmentPdfFormatters"
import * as labels from "../../labels/physicalAssessment"
import * as tones from "../../labels/classificationTone"

export default function assessmentCircumferencesSection(
    section: PdfSection,
    r: PhysicalAssessmentCompleteDto,
) {
    const a = r.physical_assessment_dto

    section("Circunferências", [
        ...circumferenceFields.flatMap((field) => {
            const fieldValue = a[field.name]

            if (typeof fieldValue !== "number") return []

            return measurement(
                field.label,
                fieldValue,
                " cm",
                field.name === "abdomen_circumference_cm"
                    ? classification(
                        r.circumference_abdominal_classification,
                        labels.abdomenLabels,
                        tones.abdomenTones,
                    )
                    : { content: "" },
            )
        }),
        ...measurement(
            "Relação cintura/quadril",
            r.waist_hip_calculate,
            "",
            classification(
                r.waist_hip_classification,
                labels.waistHipLabels,
                tones.waistHipTones,
            ),
        ),
    ])
}