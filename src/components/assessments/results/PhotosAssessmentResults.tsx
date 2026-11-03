import type { PhysicalAssessmentCompleteDto } from "../../../model/physicalAssessment"
import AssessmentPhotos from "../photos/AssessmentPhotos"

type Props = {
	result: PhysicalAssessmentCompleteDto
}

export default function PhotosAssessmentResults({ result }: Props) {
	const a = result.physical_assessment_dto

	return (
		<div className="col-span-full">
			{a.id !== null ? (
				<AssessmentPhotos
					key={a.id}
					assessmentId={a.id}
				/>
			) : (
				<p className="text-sm text-[#717171]">
					Salve a avaliação antes de adicionar fotos.
				</p>
			)}
		</div>
	)
}
