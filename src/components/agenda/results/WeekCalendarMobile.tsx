import type { AgendaEvent } from "../../../utils/agenda"
import {
	timeToMinutes,
	durationLabel,
} from "../../../utils/agenda"
import { appointmentTypeLabels } from "../../../labels/agenda"
import { Button } from "../../ui/Button"
import type { CalendarDay } from "./WeekCalendar"

type Props = {
	days: CalendarDay[]
	studentName: (id: number) => string
	onSelect: (event: AgendaEvent) => void
	onNew: (date: Date) => void
}


export default function WeekCalendarMobile({
	days,
	studentName,
	onSelect,
	onNew,
}: Props) {
	return (
		<div className="md:hidden space-y-3">
			{days.map((day) => (
				<div
					key={day.dateString}
					className="bg-[#161616] border border-[#252525] rounded overflow-hidden"
				>
					<div className="flex items-center justify-between px-4 py-3 border-b border-[#252525]">
						<div>
							<span
								className={`text-xs font-medium uppercase tracking-wider ${day.isToday
										? "text-[#a3e635]"
										: "text-[#717171]"
									}`}
							>
								{day.long}
							</span>

							<span className="ml-2 text-sm text-white">
								{day.date.toLocaleDateString("pt-BR", {
									day: "2-digit",
									month: "short",
								})}
							</span>
						</div>

						<Button
							type="button"
							onClick={() => onNew(day.date)}
							aria-label="Novo agendamento"
							className="text-[#a3e635] text-xl leading-none"
						>
							+
						</Button>
					</div>

					{day.events.length > 0 ? (
						<div className="divide-y divide-[#252525]/60">
							{day.events.map((event) => {
								const appointment = event.raw

								const isClass =
									appointment.appointment_type === "Class"

								const durationMinutes =
									timeToMinutes(appointment.end_time) -
									timeToMinutes(appointment.start_time)

								const label =
									isClass && appointment.is_recurring
										? "Aula semanal"
										: appointmentTypeLabels[
										appointment.appointment_type
										]

								return (
									<Button
										key={event.id}
										type="button"
										onClick={() => onSelect(event)}
										className="w-full flex items-center gap-3 px-4 py-3 text-left hover:bg-[#1e1e1e]"
									>
										<span
											className={`w-1 h-10 rounded-full ${isClass
													? "bg-[#a3e635]"
													: "bg-[#22d3ee]"
												}`}
										/>

										<span className="font-mono text-xs text-[#d0d0d0]">
											{appointment.start_time.slice(0, 5)}
										</span>

										<span className="min-w-0 flex-1">
											<span className="block text-sm text-white font-medium truncate">
												{studentName(appointment.student_id)}
											</span>

											<span className="block text-xs text-[#717171]">
												{label}
											</span>
										</span>

										<span className="text-xs text-[#717171]">
											{durationLabel(durationMinutes)}
										</span>
									</Button>
								)
							})}
						</div>
					) : (
						<div className="px-4 py-4 text-xs text-[#5f5f5f]">
							Nenhum compromisso
						</div>
					)}
				</div>
			))}
		</div>
	)
}