import type { AgendaEvent } from "../../../utils/agenda"
import { appointmentTypeLabels } from "../../../labels/agenda"
import { Button } from "../../ui/Button"
import { Modal } from "../../ui/Modal"

type Props = {
	events: AgendaEvent[]
	studentName: (id: number) => string
	onSelect: (event: AgendaEvent) => void
	onClose: () => void
}

export default function AppointmentGroupModal({
	events,
	studentName,
	onSelect,
	onClose,
}: Props) {
	const first = events[0]

	if (!first) return null

	return (
		<Modal
			label="Alunos neste horário"
			onClose={onClose}
			backdropClassName="bg-black/75 backdrop-blur-sm"
		>
			<div className="relative z-10 w-full max-w-md bg-[#111] border border-[#252525] rounded-lg shadow-2xl overflow-hidden">
				<div className="px-5 py-4 border-b border-[#252525] flex items-start justify-between gap-3">
					<div>
						<h2 className="text-lg font-semibold text-white">
							{events.length} alunos às{" "}
							{first.raw.start_time.slice(0, 5)}
						</h2>

						<p className="mt-1 text-xs text-[#717171] capitalize">
							{first.date.toLocaleDateString("pt-BR", {
								weekday: "long",
								day: "2-digit",
								month: "long",
							})}
						</p>
					</div>

					<Button
						type="button"
						onClick={onClose}
						aria-label="Fechar lista"
						className="text-xl leading-none text-[#717171] hover:text-white"
					>
						×
					</Button>
				</div>

				<div className="max-h-[60vh] overflow-y-auto divide-y divide-[#252525]">
					{events.map((event) => {
						const appointment = event.raw
						const isClass =
							appointment.appointment_type === "Class"

						return (
							<Button
								key={event.id}
								type="button"
								onClick={() => onSelect(event)}
								className="w-full flex items-center gap-3 px-5 py-4 text-left hover:bg-[#1b1b1b] transition-colors"
							>
								<span
									className={`w-1 h-10 shrink-0 rounded-full ${isClass
											? "bg-[#a3e635]"
											: "bg-[#22d3ee]"
										}`}
								/>

								<span className="min-w-0 flex-1">
									<span className="block text-sm font-medium text-white break-words">
										{studentName(appointment.student_id)}
									</span>

									<span className="block mt-1 text-xs text-[#717171]">
										{appointmentTypeLabels[
											appointment.appointment_type
										]}
										{appointment.is_recurring
											? " · Semanal"
											: ""}
									</span>
								</span>

								<span className="shrink-0 text-xs font-mono text-[#b0b0b0]">
									{appointment.start_time.slice(0, 5)}
									{"–"}
									{appointment.end_time.slice(0, 5)}
								</span>
							</Button>
						)
					})}
				</div>

				<div className="px-5 py-3 border-t border-[#252525] flex justify-end">
					<Button
						type="button"
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