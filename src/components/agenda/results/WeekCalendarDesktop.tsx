import { useMemo, useState } from "react"
import type { MouseEvent } from "react"

import type { AgendaEvent } from "../../../utils/agenda"
import {
	START_HOUR,
	END_HOUR,
	HOUR_HEIGHT,
	pad,
} from "../../../utils/agenda"

import calculateAgendaLayout from "../../../utils/agendaLayout"
import type { AgendaGroupLayout } from "../../../utils/agendaLayout"

import { Button } from "../../ui/Button"
import AppointmentGroupModal from "./AppointmentGroupModal"
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
	const [selectedGroupKey, setSelectedGroupKey] =
		useState<string | null>(null)

	const calendarHeight =
		(END_HOUR - START_HOUR) * HOUR_HEIGHT

	const hours = Array.from(
		{ length: END_HOUR - START_HOUR + 1 },
		(_, index) => ({
			value: START_HOUR + index,
			top: index * HOUR_HEIGHT,
		}),
	)

	const calendarDays = useMemo(
		() =>
			days.map((day) => ({
				...day,
				groups: calculateAgendaLayout(day.events).map(
					(group) => ({
						...group,
						key: `${day.dateString}-${group.key}`,
					}),
				),
			})),
		[days],
	)

	const selectedGroup = calendarDays
		.flatMap((day) => day.groups)
		.find((group) => group.key === selectedGroupKey)

	function selectGroup(group: AgendaGroupLayout) {
		if (group.events.length === 1) {
			setSelectedGroupKey(null)
			onSelect(group.events[0])
			return
		}

		setSelectedGroupKey(group.key)
	}

	function selectAppointment(event: AgendaEvent) {
		setSelectedGroupKey(null)
		onSelect(event)
	}

	return (
		<>
			<div className="hidden md:block bg-[#111] border border-[#252525] rounded-lg overflow-x-auto">
				<div className="min-w-[980px]">
					<div className="grid grid-cols-[64px_repeat(7,minmax(128px,1fr))] border-b border-[#252525] bg-[#141414]">
						<div className="border-r border-[#252525]" />

						{calendarDays.map((day) => (
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

						{calendarDays.map((day) => (
							<div
								key={day.dateString}
								className={`relative border-r border-[#252525] last:border-r-0 cursor-crosshair ${day.isToday
										? "bg-[#a3e635]/[0.025]"
										: ""
									}`}
								style={{ height: calendarHeight }}
								onClick={(event) =>
									onCalendarClick(event, day.date)
								}
							>
								{hours.map((hour) => (
									<div
										key={hour.value}
										className="absolute left-0 right-0 border-t border-[#202020] pointer-events-none"
										style={{ top: hour.top }}
									/>
								))}

								{day.groups.map((group) => {
									const first = group.events[0]
									const count = group.events.length

									const time =
										first.raw.start_time.slice(0, 5)

									const label =
										count > 1
											? `${count} alunos`
											: studentName(first.raw.student_id)

									const onlyClasses = group.events.every(
										(event) =>
											event.raw.appointment_type === "Class",
									)

									const onlyAssessments = group.events.every(
										(event) =>
											event.raw.appointment_type === "Assessment",
									)

									const colors = onlyClasses
										? "bg-[#a3e635]/15 border-[#a3e635]/35 text-[#d8ff98]"
										: onlyAssessments
											? "bg-[#22d3ee]/15 border-[#22d3ee]/35 text-[#8cecff]"
											: "bg-[#a78bfa]/15 border-[#a78bfa]/35 text-[#ddd0ff]"

									const compact = group.height < 38

									return (
										<Button
											key={group.key}
											type="button"
											title={`${time} · ${label}`}
											aria-label={`${time} · ${label}. Abrir detalhes`}
											onClick={(click) => {
												click.stopPropagation()
												selectGroup(group)
											}}
											className={`absolute z-10 box-border min-w-0 min-h-0 overflow-hidden rounded border text-left cursor-pointer hover:brightness-110 focus-visible:outline focus-visible:outline-2 focus-visible:outline-white ${compact
													? "px-1 py-0"
													: "px-1.5 py-1"
												} ${colors}`}
											style={{
												top: group.top,
												height: group.height,
												left: `calc(${(group.column / group.columns) * 100}% + 2px)`,
												width: `calc(${100 / group.columns}% - 4px)`,
											}}
										>
											{compact ? (
												<div className="truncate text-[10px] leading-tight">
													<span className="font-mono opacity-75">
														{time}
													</span>
													{" "}
													<span className="font-semibold">
														{label}
													</span>
												</div>
											) : (
												<>
													<div className="truncate text-[10px] font-mono leading-tight opacity-75">
														{time}
													</div>

													<div className="mt-1 text-xs font-semibold truncate leading-tight">
														{label}
													</div>

													{count > 1 &&
														group.columns === 1 &&
														group.height >= 58 && (
															<div className="mt-1 text-[10px] opacity-65 truncate">
																Clique para ver a lista
															</div>
														)}
												</>
											)}
										</Button>
									)
								})}
							</div>
						))}
					</div>
				</div>
			</div>

			{selectedGroup && (
				<AppointmentGroupModal
					events={selectedGroup.events}
					studentName={studentName}
					onSelect={selectAppointment}
					onClose={() => setSelectedGroupKey(null)}
				/>
			)}
		</>
	)
}