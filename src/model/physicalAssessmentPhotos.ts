
export type PhotoPosition = "Front" | "Side" | "Back"


export type PhysicalAssessmentPhotosDto = {
	id: number | null
	physical_assessment_id: number
	file_path: string
    position: PhotoPosition
	description: string | null
	created_at: string
	updated_at: string
}