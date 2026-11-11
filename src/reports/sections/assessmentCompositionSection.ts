import type { PhysicalAssessmentCompleteDto } from "../../model/physicalAssessment"
import type { PdfSection } from "../assessmentPdfLayout"

import { skinfoldFields } from "../../components/assessments/forms/assessmentFields"
import { classification, measurement } from "../assessmentPdfFormatters"
import * as labels from "../../labels/physicalAssessment"
import * as tones from "../../labels/classificationTone"

export default function assessmentCompositionSection(
    section: PdfSection,
    r: PhysicalAssessmentCompleteDto,
) {
    const a = r.physical_assessment_dto

    section("Composição corporal — pregas cutâneas", [
        ...measurement(
            "Gordura corporal",
            r.fat_percentage_skinfolds,
            " %",
            classification(
                r.fat_percentage_classification_skinfolds,
                labels.fatLabels,
                tones.fatTones,
            ),
        ),
        ...measurement("Massa gorda", r.kilos_fat_skinfolds, " kg"),
        ...measurement(
            "Massa magra",
            r.kilos_lean_body_mass_skinfolds,
            " kg",
        ),
        ...measurement(
            "Percentual de massa magra",
            r.lean_body_mass_percentage_skinfolds,
            " %",
        ),
        ...skinfoldFields.flatMap((field) => {
            const fieldValue = a[field.name]

            return typeof fieldValue === "number"
                ? measurement(field.label, fieldValue, " mm")
                : []
        }),
    ])

    section("Composição corporal — bioimpedância", [
        ...measurement(
            "Gordura corporal",
            a.bioimpedance_fat_percentage,
            " %",
            classification(
                r.fat_percentage_classification_bioimpedance,
                labels.fatLabels,
                tones.fatTones,
            ),
        ),
        ...measurement("Massa gorda", r.kilos_fat_bioimpedance, " kg"),
        ...measurement(
            "Massa magra",
            r.kilos_lean_body_mass_bioimpedance,
            " kg",
        ),
        ...measurement(
            "Percentual de massa magra",
            r.lean_body_mass_percentage_bioimpedance,
            " %",
        ),
    ])
}