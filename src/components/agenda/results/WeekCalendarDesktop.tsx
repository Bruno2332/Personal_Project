import type { MouseEvent } from "react"
import type { AgendaEvent } from "../../../utils/agenda"
import {
	timeToMinutes,
	START_HOUR,
	END_HOUR,
	HOUR_HEIGHT,
	pad,
} from "../../../utils/agenda"
import { appointmentTypeLabels } from "../../../labels/agenda"
import { Button } from "../../ui/Button"
import type { CalendarDay } from "./WeekCalendar"

type Props = {
	days: CalendarDay[]
	studentName: (id: number) => string
	onSelect: (event: AgendaEvent) => void
	onCalendarClick: (
		event: MouseEvent<HTMLDivElement>,
		date: Date,
	) => void
}

export default function WeekCalendarDesktop({
	days,
	studentName,
	onSelect,
	onCalendarClick,
}: Props) {
	const calendarHeight = (END_HOUR - START_HOUR) * HOUR_HEIGHT

	const hours = Array.from(
		{ length: END_HOUR - START_HOUR + 1 },
		(_, index) => ({
			value: START_HOUR + index,
			top: index * HOUR_HEIGHT,
		}),
	)

	return (
		<div className="hidden md:block bg-[#111] border border-[#252525] rounded-lg overflow-x-auto">
			<div className="min-w-[980px]">
				<div className="grid grid-cols-[64px_repeat(7,minmax(128px,1fr))] border-b border-[#252525] bg-[#141414]">
					<div className="border-r border-[#252525]" />

					{days.map((day) => (
						<div
							key={day.dateString}
							className={`px-2 py-3 text-center border-r border-[#252525] last:border-r-0 ${day.isToday ? "bg-[#a3e635]/5" : ""
								}`}
						>
							<div
								className={`text-[10px] font-medium tracking-widest ${day.isToday
										? "text-[#a3e635]"
										: "text-[#717171]"
									}`}
							>
								{day.short}
							</div>

							<div
								className={`font-display text-xl font-700 leading-tight ${day.isToday
										? "text-[#a3e635]"
										: "text-white"
									}`}
							>
								{pad(day.date.getDate())}
							</div>
						</div>
					))}
				</div>

				<div className="grid grid-cols-[64px_repeat(7,minmax(128px,1fr))]">
					<div
						className="relative border-r border-[#252525]"
						style={{ height: calendarHeight }}
					>
						{hours.map((hour) => (
							<span
								key={hour.value}
								className="absolute right-3 -translate-y-1/2 text-[10px] font-mono text-[#5f5f5f]"
								style={{ top: hour.top }}
							>
								{pad(hour.value)}:00
							</span>
						))}
					</div>

					{days.map((day) => (
						<div
							key={day.dateString}
							className={`relative border-r border-[#252525] last:border-r-0 cursor-crosshair ${day.isToday ? "bg-[#a3e635]/[0.025]" : ""
								}`}
							style={{ height: calendarHeight }}
							onClick={(event) =>
								onCalendarClick(event, day.date)
							}
						>
							{hours.map((hour) => (
								<div
									key={hour.value}
									className="absolute left-0 right-0 border-t border-[#202020]"
									style={{ top: hour.top }}
								/>
							))}

							{day.events.map((event) => {
								const appointment = event.raw

								const startMinutes = timeToMinutes(
									appointment.start_time,
								)
								const endMinutes = timeToMinutes(
									appointment.end_time,
								)

								const visibleStart = Math.max(
									startMinutes,
									START_HOUR * 60,
								)
								const visibleEnd = Math.min(
									endMinutes,
									END_HOUR * 60,
								)

								if (
									!Number.isFinite(visibleStart) ||
									!Number.isFinite(visibleEnd) ||
									visibleEnd <= visibleStart
								) {
									return null
								}

								const top =
									((visibleStart - START_HOUR * 60) / 60) *
									HOUR_HEIGHT

								const height = Math.max(
									42,
									((visibleEnd - visibleStart) / 60) *
									HOUR_HEIGHT,
								)

								const isClass =
									appointment.appointment_type === "Class"

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
										onClick={(click) => {
											click.stopPropagation()
											onSelect(event)
										}}
										className={`absolute left-1 right-1 z-10 overflow-hidden rounded border px-2 py-1.5 text-left transition-all hover:brightness-110 hover:z-20 ${isClass
												? "bg-[#a3e635]/15 border-[#a3e635]/35 text-[#d8ff98]"
												: "bg-[#22d3ee]/15 border-[#22d3ee]/35 text-[#8cecff]"
											}`}
										style={{
											top: top + 2,
											height: Math.max(
												0,
												Math.min(
													height - 4,
													calendarHeight - top - 2,
												),
											),
										}}
									>
										<div className="text-[10px] font-mono opacity-75">
											{appointment.start_time.slice(0, 5)}
										</div>

										<div className="text-xs font-semibold truncate">
											{studentName(appointment.student_id)}
										</div>

										<div className="text-[10px] opacity-65 truncate">
											{label}
										</div>
									</Button>
								)
							})}
						</div>
					))}
				</div>
			</div>
		</div>
	)
}