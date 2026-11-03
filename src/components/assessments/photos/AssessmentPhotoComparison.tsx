import { useEffect, useState } from "react"
import { FormProvider, useForm, useWatch } from "react-hook-form"
import type { AssessmentView } from "../../../utils/assessments"
import type { PhotoPosition } from "../../../model/physicalAssessmentPhotos"
import {
	findAllPhysicalAssessmentPhotos,
	readPhysicalAssessmentPhoto,
} from "../../../services/physicalAssessmentPhotosService"
import { errorMessage } from "../../../utils/errors"
import { FormSelect } from "../../ui/Fields"
import { Button } from "../../ui/Button"


type Props = {
	previous?: AssessmentView
	current?: AssessmentView
}

type PhotoPreview = {
	source: string | null
	description: string | null
	error: string | null
}


export default function AssessmentPhotoComparison({
	previous,
	current,
}: Props) {
	const methods = useForm<{ position: PhotoPosition }>({
		defaultValues: {
			position: "Front",
		},
	})

	const position = useWatch({
		control: methods.control,
		name: "position",
	})

	const [previews, setPreviews] = useState<Record<number, PhotoPreview>>({})
	const [loading, setLoading] = useState(true)
	const [revision, setRevision] = useState(0)

	const previousId = previous?.id
	const currentId = current?.id

	useEffect(() => {
		let cancelled = false
		const urls: string[] = []

		async function load() {
			setLoading(true)
			setPreviews({})

			const ids = [...new Set(
				[previousId, currentId].filter(
					(id): id is number => id != null,
				),
			)]

			try {
				await Promise.all(
					ids.map(async (assessmentId) => {
						try {
							const photos =
								await findAllPhysicalAssessmentPhotos(assessmentId)

							if (cancelled) return

							const photo = photos.find(
								(item) => item.position === position,
							)

							if (!photo) {
								setPreviews((value) => ({
									...value,
									[assessmentId]: {
										source: null,
										description: null,
										error: null,
									},
								}))
								return
							}

							if (photo.id === null) {
								throw new Error("A foto não possui um ID válido.")
							}

							const buffer = await readPhysicalAssessmentPhoto(photo.id)

							if (cancelled) return

							const bytes = new Uint8Array(buffer)
							let mimeType: string

							if (
								bytes[0] === 0xff &&
								bytes[1] === 0xd8 &&
								bytes[2] === 0xff
							) {
								mimeType = "image/jpeg"
							} else if (
								bytes[0] === 0x89 &&
								bytes[1] === 0x50 &&
								bytes[2] === 0x4e &&
								bytes[3] === 0x47
							) {
								mimeType = "image/png"
							} else if (
								bytes.length >= 12 &&
								bytes[0] === 0x52 &&
								bytes[1] === 0x49 &&
								bytes[2] === 0x46 &&
								bytes[3] === 0x46 &&
								bytes[8] === 0x57 &&
								bytes[9] === 0x45 &&
								bytes[10] === 0x42 &&
								bytes[11] === 0x50
							) {
								mimeType = "image/webp"
							} else {
								throw new Error("Formato de imagem não reconhecido.")
							}

							const source = URL.createObjectURL(
								new Blob([buffer], { type: mimeType }),
							)

							urls.push(source)

							setPreviews((value) => ({
								...value,
								[assessmentId]: {
									source,
									description: photo.description,
									error: null,
								},
							}))
						} catch (error) {
							if (cancelled) return

							setPreviews((value) => ({
								...value,
								[assessmentId]: {
									source: null,
									description: null,
									error: errorMessage(error),
								},
							}))
						}
					}),
				)
			} finally {
				if (!cancelled) setLoading(false)
			}
		}

		void load()

		return () => {
			cancelled = true
			urls.forEach((url) => URL.revokeObjectURL(url))
		}
	}, [previousId, currentId, position, revision])

	const hasError = Object.values(previews).some((preview) => preview.error)

	return (
		<FormProvider {...methods}>
			<div className="space-y-4">
				<div className="flex items-end gap-3">
					<FormSelect
						name="position"
						label="Posição da foto"
						className="w-full max-w-xs"
						required
						options={[
							{ value: "Front", label: "Frente" },
							{ value: "Side", label: "Lado" },
							{ value: "Back", label: "Costas" },
						]}
					/>

					{hasError && (
						<Button
							type="button"
							disabled={loading}
							onClick={() => setRevision((value) => value + 1)}
							className="px-3 py-2 text-sm text-[#a3e635] disabled:opacity-50"
						>
							Recarregar fotos
						</Button>
					)}
				</div>

				<div className="grid grid-cols-2 gap-3">
					{[
						{ label: "Anterior", assessment: previous },
						{ label: "Atual", assessment: current },
					].map(({ label, assessment }) => {
						const id = assessment?.id
						const preview = id != null ? previews[id] : undefined

						return (
							<div
								key={label}
								className="min-w-0 border border-[#252525] rounded overflow-hidden"
							>
								<div className="px-3 py-2 border-b border-[#252525]">
									<p className="text-sm font-medium text-white">
										{label}
									</p>

									<p className="text-xs text-[#717171]">
										{assessment
											? new Date(
												`${assessment.data}T12:00:00`,
											).toLocaleDateString("pt-BR")
											: "Sem avaliação"}
									</p>
								</div>

								<div className="h-64 sm:h-80 bg-[#0d0d0d] flex items-center justify-center">
									{!assessment ? (
										<p className="p-4 text-sm text-[#717171] text-center">
											Não há avaliação anterior.
										</p>
									) : loading ? (
										<p className="text-sm text-[#717171]">
											Carregando...
										</p>
									) : preview?.error ? (
										<p className="p-4 text-sm text-red-400 text-center">
											{preview.error}
										</p>
									) : preview?.source ? (
										<img
											src={preview.source}
											alt={`Foto da avaliação ${label.toLowerCase()}`}
											className="w-full h-full object-contain"
											onError={() => {
												if (id == null) return

												setPreviews((value) => ({
													...value,
													[id]: {
														source: null,
														description: preview.description,
														error: "Não foi possível exibir esta imagem.",
													},
												}))
											}}
										/>
									) : (
										<p className="p-4 text-sm text-[#717171] text-center">
											Sem foto nesta posição.
										</p>
									)}
								</div>

								{!loading && preview?.description && (
									<p className="p-3 text-sm text-[#d0d0d0] break-words">
										{preview.description}
									</p>
								)}
							</div>
						)
					})}
				</div>
			</div>
		</FormProvider>
	)
}