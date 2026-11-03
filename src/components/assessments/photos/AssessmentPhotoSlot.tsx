import type {
	PhotoPosition,
	PhysicalAssessmentPhotosDto,
} from "../../../model/physicalAssessmentPhotos"
import { Button } from "../../ui/Button"
import { FormInput, FormSelect } from "../../ui/Fields"

type Props = {
	position: PhotoPosition
	label: string
	photo: PhysicalAssessmentPhotosDto | undefined
	source: string | undefined
	error: string | undefined
	busy: boolean
	loading: boolean
	onAdd: (position: PhotoPosition) => void
	onRemove: (photo: PhysicalAssessmentPhotosDto) => void
	onImageError: (id: number | null) => void
}

export default function AssessmentPhotoSlot({
	position,
	label,
	photo,
	source,
	error,
	busy,
	loading,
	onAdd,
	onRemove,
	onImageError,
}: Props) {
	const id = photo?.id ?? null
	const photoError = photo
		? id !== null
			? error
			: "A foto não possui um ID válido."
		: undefined

	return (
		<div className="min-w-0 bg-[#161616] border border-[#252525] rounded overflow-hidden">
			<div className="p-4 space-y-3">
				<fieldset
					disabled={busy || loading || !!photo}
					className="min-w-0 space-y-3"
				>
					<FormSelect
						name={`${position}.position`}
						label="Posição da foto"
						required
						options={[{ value: position, label }]}
					/>
					<FormInput
						name={`${position}.description`}
						label="Descrição"
						placeholder="Observação sobre a foto..."
						nullable
					/>
				</fieldset>

				<Button
					type="button"
					onClick={() => onAdd(position)}
					disabled={busy || loading || !!photo}
					className="w-full px-4 py-2 bg-[#a3e635] text-black text-sm font-semibold rounded hover:bg-[#bef264] disabled:opacity-50"
				>
					{photo ? "Foto adicionada" : "+ Adicionar foto"}
				</Button>
			</div>

			<div className="h-72 bg-[#0d0d0d] flex items-center justify-center">
				{photoError ? (
					<p className="p-4 text-sm text-red-400 text-center">{photoError}</p>
				) : source ? (
					<img
						src={source}
						alt={`Foto de ${label.toLowerCase()}`}
						className="w-full h-full object-contain"
						onError={() => onImageError(id)}
					/>
				) : (
					<p className="p-4 text-sm text-[#717171] text-center">
						{loading || photo
							? "Carregando imagem..."
							: `Adicione a foto de ${label.toLowerCase()}.`}
					</p>
				)}
			</div>

			{photo && (
				<div className="p-3 flex justify-end">
					<Button
						type="button"
						disabled={busy || loading || id === null}
						onClick={() => onRemove(photo)}
						className="px-3 py-1.5 text-xs text-red-400 hover:bg-red-400/10 rounded disabled:opacity-50"
					>
						Excluir foto
					</Button>
				</div>
			)}
		</div>
	)
}
