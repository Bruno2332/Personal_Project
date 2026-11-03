import type { AgendaEvent } from "../../../utils/agenda"
import { durationLabel, timeToMinutes } from "../../../utils/agenda"
import { Button } from "../../ui/Button"
import { Modal } from "../../ui/Modal"


type Props = {
	selectedEvent: AgendaEvent
	studentName: (id: number) => string
	onClose: () => void
	onEdit: () => void
	removeSelected: () => void
	busy: boolean
}


export default function AppointmentDetails({
	selectedEvent,
	studentName: studentName,
	onClose,
	onEdit,
	removeSelected,
	busy,
}: Props) {
	const appointment = selectedEvent.raw
	const isClass = appointment.appointment_type === "Class"

	const appointmentLabel = isClass
		? appointment.is_recurring
			? "Aula semanal"
			: "Aula"
		: "Avaliação física"

	const durationMinutes =
		timeToMinutes(appointment.end_time) -
		timeToMinutes(appointment.start_time)

	return (
		<Modal
			label="Detalhes do agendamento"
			onClose={onClose}
			busy={busy}
			backdropClassName="bg-black/75 backdrop-blur-sm"
		>
			<div className="relative w-full max-w-md bg-[#111] border border-[#252525] rounded-lg shadow-2xl overflow-hidden">
				<div
					className={`h-1 ${isClass ? "bg-[#a3e635]" : "bg-[#22d3ee]"
						}`}
				/>

				<div className="p-6">
					<div className="flex items-start justify-between gap-4">
						<div>
							<p
								className={`text-xs uppercase tracking-wider font-medium ${isClass
										? "text-[#a3e635]"
										: "text-[#22d3ee]"
									}`}
							>
								{appointmentLabel}
							</p>

							<h2 className="font-display text-3xl font-700 text-white mt-1">
								{studentName(appointment.student_id)}
							</h2>
						</div>

						<Button
							disabled={busy}
							onClick={onClose}
							aria-label="Fechar detalhes"
							className="text-[#717171] hover:text-white text-xl"
						>
							×
						</Button>
					</div>

					<div className="mt-5 grid grid-cols-2 gap-3">
						<Detail
							label="Data"
							value={selectedEvent.date.toLocaleDateString("pt-BR", {
								weekday: "long",
								day: "2-digit",
								month: "long",
							})}
						/>

						<Detail
							label="Horário"
							value={
								appointment.start_time.slice(0, 5) +
								" · " +
								durationLabel(durationMinutes)
							}
						/>
					</div>

					{appointment.notes && (
						<div className="mt-3 p-3 bg-[#161616] border border-[#252525] rounded">
							<p className="text-[10px] text-[#717171] uppercase tracking-wider">
								Observações
							</p>
							<p className="text-sm text-[#d0d0d0] mt-1">
								{appointment.notes}
							</p>
						</div>
					)}

					{appointment.is_recurring && (
						<p className="mt-3 text-xs text-[#717171]">
							Este é um horário fixo. Ao remover, todas as
							ocorrências desta recorrência são excluídas.
						</p>
					)}
				</div>

				<div className="flex justify-between gap-3 px-6 py-4 border-t border-[#252525] bg-[#0f0f0f]">
					<Button
						disabled={busy}
						onClick={removeSelected}
						className="px-3 py-2 text-xs text-red-400 hover:bg-red-400/10 rounded"
					>
						Remover{" "}
						{appointment.is_recurring
							? "recorrência"
							: isClass
								? "aula"
								: "agendamento"}
					</Button>

					<Button
						disabled={busy}
						onClick={onEdit}
						className="px-4 py-2 text-sm text-[#a3e635] border border-[#252525] rounded"
					>
						Editar
					</Button>

					<Button
						disabled={busy}
						onClick={onClose}
						className="px-4 py-2 text-sm text-white bg-[#252525] rounded hover:bg-[#303030]"
					>
						Fechar
					</Button>
				</div>
			</div>
		</Modal>
	)
}

function Detail({ label, value }: { label: string; value: string }) {
	return (
		<div className="p-3 bg-[#161616] border border-[#252525] rounded">
			<p className="text-[10px] text-[#717171] uppercase tracking-wider">
				{label}
			</p>
			<p className="text-sm text-white mt-1 capitalize">
				{value}
			</p>
		</div>
	)
}