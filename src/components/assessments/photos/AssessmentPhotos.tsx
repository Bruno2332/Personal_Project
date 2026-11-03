import { FormProvider } from "react-hook-form"

import type { PhotoPosition } from "../../../model/physicalAssessmentPhotos"
import { Button } from "../../ui/Button"
import useAssessmentPhotos from "../hooks/useAssessmentPhotos"
import AssessmentPhotoSlot from "./AssessmentPhotoSlot"

const photoSlots: { position: PhotoPosition; label: string }[] = [
	{ position: "Front", label: "Frente" },
	{ position: "Side", label: "Lado" },
	{ position: "Back", label: "Costas" },
]

export default function AssessmentPhotos({ assessmentId }: { assessmentId: number }) {
	const photos = useAssessmentPhotos(assessmentId)
	const hasPhotoErrors = Object.keys(photos.photoErrors).length > 0

	return (
		<FormProvider {...photos.methods}>
			<div className="space-y-4">
				{photos.error && (
					<p role="alert" className="text-sm text-red-400">
						{photos.error}
					</p>
				)}

				{(photos.error || hasPhotoErrors) && (
					<Button
						type="button"
						disabled={photos.busy || photos.loading}
						onClick={photos.reload}
						className="text-sm text-[#a3e635] disabled:opacity-50"
					>
						Recarregar fotos
					</Button>
				)}

				{photos.loading && (
					<p role="status" className="text-sm text-[#717171]">
						Carregando fotos...
					</p>
				)}

				<div className="grid grid-cols-1 md:grid-cols-3 gap-4">
					{photoSlots.map((slot) => {
						const photo = photos.photos.find(
							(item) => item.position === slot.position,
						)
						const id = photo?.id ?? null

						return (
							<AssessmentPhotoSlot
								key={slot.position}
								position={slot.position}
								label={slot.label}
								photo={photo}
								source={id === null ? undefined : photos.sources[id]}
								error={id === null ? undefined : photos.photoErrors[id]}
								busy={photos.busy}
								loading={photos.loading}
								onAdd={(position) => void photos.addPhoto(position)}
								onRemove={(item) => void photos.removePhoto(item)}
								onImageError={photos.handleImageError}
							/>
						)
					})}
				</div>
			</div>
		</FormProvider>
	)
}
