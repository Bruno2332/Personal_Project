import type { PhysicalAssessmentDto } from "../../../model/physicalAssessment"
import type { Vo2AssessmentDto } from "../../../model/vo2Assessment"


export type AssessmentFormValues = {
    assessment: PhysicalAssessmentDto
    vo2_assessments: Vo2AssessmentDto[]
}