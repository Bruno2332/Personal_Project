import {
	type AgendaEvent,
	formatISODate,
	minutesToTime,
} from "../../../utils/agenda"

type Props = {
	events: AgendaEvent[]
	now: Date
	studentName: (id: number) => string
}

export default function ScheduleSummaryCards({
	events,
	now,
	studentName,
}: Props) {
	const classesThisWeek = events.filter(
		(event) => event.raw.appointment_type === "Class",
	).length

	const assessmentsThisWeek = events.filter(
		(event) => event.raw.appointment_type === "Assessment",
	).length

	const todayDate = formatISODate(now)

	const todayEvents = events.filter(
		(event) => formatISODate(event.date) === todayDate,
	)

	const studentsToday = new Set(
		todayEvents.map((event) => event.raw.student_id),
	).size

	const nextAppointment =
		events
			.map((event) => ({
				studentId: event.raw.student_id,
				color:
					event.raw.appointment_type === "Assessment"
						? "#22d3ee"
						: "#a3e635",
				date: new Date(
					`${formatISODate(event.date)}T${event.raw.start_time.slice(0, 8)}`,
				),
			}))
			.filter((appointment) => appointment.date > now)
			.sort((a, b) => a.date.getTime() - b.date.getTime())[0] ?? null

	const nextAppointmentTime = nextAppointment
		? minutesToTime(
			nextAppointment.date.getHours() * 60 +
			nextAppointment.date.getMinutes(),
		)
		: ""

	const nextAppointmentValue = nextAppointment
		? formatISODate(nextAppointment.date) === todayDate
			? nextAppointmentTime
			: `${nextAppointment.date
				.toLocaleDateString("pt-BR", { weekday: "short" })
				.replace(".", "")} ${nextAppointmentTime}`
		: "Livre"

	const cards = [
		{
			label: "Alunos hoje",
			value: studentsToday,
			sub: null,
			color: "#a3e635",
		},
		{
			label: "Próximo atendimento",
			value: nextAppointmentValue,
			sub: nextAppointment
				? studentName(nextAppointment.studentId).trim().split(/\s+/)[0]
				: "Nenhum nesta semana",
			color: nextAppointment?.color ?? "#a3e635",
		},
		{
			label: "Avaliações na semana",
			value: assessmentsThisWeek,
			sub: null,
			color: "#22d3ee",
		},
		{
			label: "Aulas na semana",
			value: classesThisWeek,
			sub: null,
			color: "#a3e635",
		},
	]

	return (
		<div className="grid grid-cols-2 lg:grid-cols-4 gap-3">
			{cards.map((card) => (
				<div
					key={card.label}
					className="bg-[#161616] border border-[#252525] rounded p-4"
				>
					<p className="text-[10px] sm:text-xs text-[#717171] uppercase tracking-wider font-medium">
						{card.label}
					</p>

					<p
						className="mt-1 font-display text-2xl sm:text-3xl font-800"
						style={{ color: card.color }}
					>
						{card.value}
					</p>

					{card.sub && (
						<p className="text-[10px] sm:text-xs text-[#717171] mt-1 truncate">
							{card.sub}
						</p>
					)}
				</div>
			))}
		</div>
	)
}