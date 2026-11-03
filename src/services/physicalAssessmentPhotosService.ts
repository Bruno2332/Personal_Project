import { invoke } from "@tauri-apps/api/core"
import type { PhysicalAssessmentPhotosDto } from "../model/physicalAssessmentPhotos"

export function findAllPhysicalAssessmentPhotos(assessmentId: number) {
    return invoke<PhysicalAssessmentPhotosDto[]>(
        "find_all_physical_assessment_photos",
        { assessmentId },
    )
}

export function readPhysicalAssessmentPhoto(id: number) {
    return invoke<ArrayBuffer>(
        "read_physical_assessment_photo",
        { id },
    )
}

export function insertPhysicalAssessmentPhoto(
    dto: PhysicalAssessmentPhotosDto,
) {
    return invoke<PhysicalAssessmentPhotosDto>(
        "insert_physical_assessment_photo",
        { dto },
    )
}

export function deletePhysicalAssessmentPhoto(id: number) {
    return invoke<void>("delete_physical_assessment_photo", { id })
}