import { invoke } from "@tauri-apps/api/core"
import type {
    PhysicalAssessmentDto,
    PhysicalAssessmentCompleteDto,
} from "../model/physicalAssessment"
import type { Vo2AssessmentDto } from "../model/vo2Assessment"
import type { CreateContractRequest } from "../model/contract"


export function findAssessmentsByStudentId(id: number) {
    return invoke<PhysicalAssessmentCompleteDto[]>(
        "find_all_physical_assessments_by_student_id",
        { id },
    )
}
export function insertPhysicalAssessment(
    contractRequest: CreateContractRequest,
    dto: PhysicalAssessmentDto,
    vo2Assessments: Vo2AssessmentDto[],
) {
    return invoke<PhysicalAssessmentDto>("insert_physical_assessment", {
        contractRequest,
        dto,
        vo2Assessments,
    })
}

export function updatePhysicalAssessment(
    id: number,
    dto: PhysicalAssessmentDto,
    vo2Assessments: Vo2AssessmentDto[],
    removedVo2Ids: number[]
) {
    return invoke<PhysicalAssessmentDto>("update_physical_assessment", {
        id,
        dto,
        vo2Assessments,
        removedVo2Ids
    })
}


export function deletePhysicalAssessment(id: number) {
    return invoke<void>("delete_physical_assessment", { id })
}
