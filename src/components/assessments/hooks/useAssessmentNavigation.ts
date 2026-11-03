import { useState } from "react"
import type { FieldErrors } from "react-hook-form"
import {
    circumferenceFields,
    skinfoldFields,
    strengthFields,
    type AssessmentTab,
} from "../forms/assessmentFields"
import type { AssessmentFormValues } from "../forms/types"


export default function useAssessmentNavigation() {
    const [tab, setTab] = useState<AssessmentTab>("general")

    function handleValidationErrors(
        errors: FieldErrors<AssessmentFormValues>,
    ) {
        const names = Object.keys(errors.assessment ?? {})

        const inGroup = (fields: { name: string }[]) =>
            fields.some((field) => names.includes(field.name))

        if (
            errors.vo2_assessments ||
            names.includes("vo2_final_stage_seconds")
        ) {
            setTab("vo2")
        } else if (inGroup(circumferenceFields)) {
            setTab("circumferences")
        } else if (
            inGroup(skinfoldFields) ||
            names.includes("bioimpedance_fat_percentage")
        ) {
            setTab("composition")
        } else if (inGroup(strengthFields)) {
            setTab("strength")
        } else {
            setTab("general")
        }
    }

    return {
        tab,
        setTab,
        handleValidationErrors,
    }
}