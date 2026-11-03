import type { MouseEvent } from "react"
import type { AgendaEvent } from "../../../utils/agenda"
import {
	WEEK_DAYS,
	addDays,
	formatISODate,
} from "../../../utils/agenda"
import WeekCalendarDesktop from "./WeekCalendarDesktop"
import WeekCalendarMobile from "./WeekCalendarMobile"

export type CalendarDay = {
	short: string
	long: string
	date: Date
	dateString: string
	isToday: boolean
	events: AgendaEvent[]
}

type Props = {
	weekStart: Date
	today: Date
	events: AgendaEvent[]
	studentName: (id: number) => string
	onSelect: (event: AgendaEvent) => void
	onNew: (date: Date) => void
	onCalendarClick: (
		event: MouseEvent<HTMLDivElement>,
		date: Date,
	) => void
}

export default function WeekCalendar({
	weekStart,
	today,
	events,
	studentName,
	onSelect,
	onNew,
	onCalendarClick,
}: Props) {
	const todayDate = formatISODate(today)

	const weekDays: CalendarDay[] = WEEK_DAYS.map((day, index) => {
		const date = addDays(weekStart, index)
		const dateString = formatISODate(date)

		const dayEvents = events
			.filter((event) => formatISODate(event.date) === dateString)
			.sort((a, b) =>
				a.raw.start_time.localeCompare(b.raw.start_time),
			)

		return {
			short: day.short,
			long: day.long,
			date,
			dateString,
			isToday: dateString === todayDate,
			events: dayEvents,
		}
	})

	return (
		<>
			<WeekCalendarDesktop
				days={weekDays}
				studentName={studentName}
				onSelect={onSelect}
				onCalendarClick={onCalendarClick}
			/>

			<WeekCalendarMobile
				days={weekDays}
				studentName={studentName}
				onSelect={onSelect}
				onNew={onNew}
			/>
		</>
	)
}