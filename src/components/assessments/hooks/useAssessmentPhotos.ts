import { useEffect, useRef, useState } from "react"
import { open } from "@tauri-apps/plugin-dialog"
import { useForm } from "react-hook-form"

import type {
	PhotoPosition,
	PhysicalAssessmentPhotosDto,
} from "../../../model/physicalAssessmentPhotos"
import {
	deletePhysicalAssessmentPhoto,
	findAllPhysicalAssessmentPhotos,
	insertPhysicalAssessmentPhoto,
	readPhysicalAssessmentPhoto,
} from "../../../services/physicalAssessmentPhotosService"
import { errorMessage } from "../../../utils/errors"

export type PhotoFormValues = Record<
	PhotoPosition,
	Pick<PhysicalAssessmentPhotosDto, "position" | "description">
>

const photoPositions: PhotoPosition[] = ["Front", "Side", "Back"]

function imageMimeType(bytes: Uint8Array): string {
	if (bytes[0] === 0xff && bytes[1] === 0xd8 && bytes[2] === 0xff) {
		return "image/jpeg"
	}

	if (
		bytes[0] === 0x89 &&
		bytes[1] === 0x50 &&
		bytes[2] === 0x4e &&
		bytes[3] === 0x47
	) {
		return "image/png"
	}

	if (
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
		return "image/webp"
	}

	throw new Error("Formato de imagem não reconhecido.")
}

export default function useAssessmentPhotos(assessmentId: number) {
	const [photos, setPhotos] = useState<PhysicalAssessmentPhotosDto[]>([])
	const [sources, setSources] = useState<Record<number, string>>({})
	const [photoErrors, setPhotoErrors] = useState<Record<number, string>>({})
	const [loading, setLoading] = useState(true)
	const [busy, setBusy] = useState(false)
	const [error, setError] = useState("")
	const [revision, setRevision] = useState(0)
	const operationRunning = useRef(false)
	const objectUrls = useRef(new Map<number, string>())
	const methods = useForm<PhotoFormValues>({
		defaultValues: {
			Front: { position: "Front", description: "" },
			Side: { position: "Side", description: "" },
			Back: { position: "Back", description: "" },
		},
	})

	const { setValue } = methods

	useEffect(() => {
		let cancelled = false
		const createdUrls = new Map<number, string>()
		objectUrls.current = createdUrls

		async function load() {
			setLoading(true)
			setError("")
			setPhotos([])
			setSources({})
			setPhotoErrors({})

			try {
				const records = await findAllPhysicalAssessmentPhotos(assessmentId)
				if (cancelled) return

				setPhotos(records)
				for (const photo of records) {
					if (photoPositions.includes(photo.position)) {
						setValue(`${photo.position}.description`, photo.description ?? "")
					}
				}

				for (const photo of records) {
					if (cancelled) return
					if (photo.id === null) continue

					try {
						const buffer = await readPhysicalAssessmentPhoto(photo.id)
						if (cancelled) return

						const mimeType = imageMimeType(new Uint8Array(buffer))
						const url = URL.createObjectURL(new Blob([buffer], { type: mimeType }))
						createdUrls.set(photo.id, url)
						setSources((current) => ({ ...current, [photo.id!]: url }))
					} catch (err) {
						if (cancelled) return
						setPhotoErrors((current) => ({
							...current,
							[photo.id!]: errorMessage(err),
						}))
					}
				}
			} catch (err) {
				if (!cancelled) setError(errorMessage(err))
			} finally {
				if (!cancelled) setLoading(false)
			}
		}

		void load()
		return () => {
			cancelled = true
			for (const url of createdUrls.values()) URL.revokeObjectURL(url)
			createdUrls.clear()
		}
	}, [assessmentId, revision, setValue])

	async function addPhoto(position: PhotoPosition) {
		if (operationRunning.current || loading) return
		if (photos.some((photo) => photo.position === position)) {
			setError("Já existe uma foto nessa posição. Exclua a atual para adicionar outra.")
			return
		}

		operationRunning.current = true
		setBusy(true)
		setError("")
		try {
			const valid = await methods.trigger([
				`${position}.position`,
				`${position}.description`,
			])
			if (!valid) return

			const { description } = methods.getValues(position)
			const selected = await open({
				title: "Selecionar foto da avaliação",
				multiple: false,
				directory: false,
				filters: [{ name: "Imagens", extensions: ["jpg", "jpeg", "png", "webp"] }],
			})
			if (typeof selected !== "string") return

			const now = new Date().toISOString()
			await insertPhysicalAssessmentPhoto({
				id: null,
				physical_assessment_id: assessmentId,
				file_path: selected,
				position,
				description: description?.trim() || null,
				created_at: now,
				updated_at: now,
			})
			setRevision((value) => value + 1)
		} catch (err) {
			setError(errorMessage(err))
		} finally {
			operationRunning.current = false
			setBusy(false)
		}
	}

	async function removePhoto(photo: PhysicalAssessmentPhotosDto) {
		if (photo.id === null || operationRunning.current) return
		const id = photo.id
		operationRunning.current = true
		setBusy(true)
		setError("")
		try {
			await deletePhysicalAssessmentPhoto(id)
			const url = objectUrls.current.get(id)
			if (url) {
				URL.revokeObjectURL(url)
				objectUrls.current.delete(id)
			}
			setPhotos((current) => current.filter((item) => item.id !== id))
			setSources((current) => {
				const next = { ...current }
				delete next[id]
				return next
			})
			setPhotoErrors((current) => {
				const next = { ...current }
				delete next[id]
				return next
			})
			setValue(`${photo.position}.description`, "")
		} catch (err) {
			setError(errorMessage(err))
		} finally {
			operationRunning.current = false
			setBusy(false)
		}
	}

	function handleImageError(id: number | null) {
		if (id === null) return
		setPhotoErrors((current) => ({ ...current, [id]: "Não foi possível exibir esta imagem." }))
	}

	return {
		methods,
		photos,
		sources,
		photoErrors,
		loading,
		busy,
		error,
		addPhoto,
		removePhoto,
		handleImageError,
		reload: () => setRevision((value) => value + 1),
	}
}
