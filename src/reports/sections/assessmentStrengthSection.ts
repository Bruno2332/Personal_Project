import type { PhysicalAssessmentCompleteDto } from "../../model/physicalAssessment"
import type { PdfSection } from "../assessmentPdfLayout"

import { classification, measurement } from "../assessmentPdfFormatters"
import * as labels from "../../labels/physicalAssessment"
import * as tones from "../../labels/classificationTone"

export default function assessmentStrengthSection(
    section: PdfSection,
    r: PhysicalAssessmentCompleteDto,
) {
    const a = r.physical_assessment_dto

    section("Força — dinamometria", [
        ...measurement(
            "Dinamômetro direito",
            a.right_handgrip_strength,
            " kgf",
            classification(
                r.right_dynamometer_strength_classification,
                labels.dynamometerLabels,
                tones.dynamometerTones,
            ),
        ),
        ...measurement(
            "Dinamômetro esquerdo",
            a.left_handgrip_strength,
            " kgf",
            classification(
                r.left_dynamometer_strength_classification,
                labels.dynamometerLabels,
                tones.dynamometerTones,
            ),
        ),
    ])

    section("Força muscular e assimetrias", [
        ...measurement("Bíceps direito", a.right_biceps_strength, " kg"),
        ...measurement("Bíceps esquerdo", a.left_biceps_strength, " kg"),
        ...measurement("Tríceps direito", a.right_triceps_strength, " kg"),
        ...measurement("Tríceps esquerdo", a.left_triceps_strength, " kg"),
        ...measurement("Extensora direita", a.right_extensor_strength, " kg"),
        ...measurement("Extensora esquerda", a.left_extensor_strength, " kg"),
        ...measurement("Flexora direita", a.right_flexor_strength, " kg"),
        ...measurement("Flexora esquerda", a.left_flexor_strength, " kg"),
        ...measurement(
            "Assimetria de bíceps",
            r.biceps_asymmetry_percentage,
            " %",
            classification(
                r.biceps_asymmetry_classification,
                labels.asymmetryLabels,
                tones.asymmetryTones,
            ),
        ),
        ...measurement(
            "Assimetria de tríceps",
            r.triceps_asymmetry_percentage,
            " %",
            classification(
                r.triceps_asymmetry_classification,
                labels.asymmetryLabels,
                tones.asymmetryTones,
            ),
        ),
        ...measurement(
            "Assimetria de extensores",
            r.extensor_asymmetry_percentage,
            " %",
            classification(
                r.extensor_asymmetry_classification,
                labels.asymmetryLabels,
                tones.asymmetryTones,
            ),
        ),
        ...measurement(
            "Assimetria de flexores",
            r.flexor_asymmetry_percentage,
            " %",
            classification(
                r.flexor_asymmetry_classification,
                labels.asymmetryLabels,
                tones.asymmetryTones,
            ),
        ),
    ])
}