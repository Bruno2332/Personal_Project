import useSchedulePage from "../components/agenda/hooks/useSchedulePage"
import AppointmentForm from "../components/agenda/forms/AppointmentForm"
import AppointmentDetails from "../components/agenda/results/AppointmentDetails"
import WeekCalendar from "../components/agenda/results/WeekCalendar"
import ScheduleHeader from "../components/agenda/results/ScheduleHeader"
import ScheduleSummaryCards from "../components/agenda/results/ScheduleSummaryCards"
import { appointmentTypeLabels } from "../labels/agenda"
import { Button } from "../components/ui/Button"
import {
	ErrorMessage,
	LoadingMessage,
} from "../components/ui/Feedback"



export default function Schedule() {
	const page = useSchedulePage()

	return (
		<div className="space-y-6">
			<ErrorMessage message={page.error} />
			<ErrorMessage message={page.studentsError} />

			{(page.error || page.studentsError) && (
				<Button
					type="button"
					disabled={page.loading || page.interactionBlocked}
					onClick={page.retry}
					className="text-sm text-[#a3e635]"
				>
					Recarregar agenda
				</Button>
			)}

			{page.loading && <LoadingMessage />}

			<ScheduleHeader
				weekStart={page.weekStart}
				disabled={page.interactionBlocked}
				onToday={page.goToToday}
				onPreviousWeek={page.previousWeek}
				onNextWeek={page.nextWeek}
				onNew={() => page.openNew()}
			/>

			{page.notice && (
				<div className="flex items-center justify-between gap-4 px-4 py-3 bg-[#a3e635]/10 border border-[#a3e635]/20 rounded text-sm text-[#c8f47c]">
					<span>{page.notice}</span>

					<Button
						type="button"
						onClick={page.dismissNotice}
						aria-label="Fechar aviso"
						className="text-[#717171] hover:text-white"
					>
						×
					</Button>
				</div>
			)}

			<ScheduleSummaryCards
				events={page.events}
				now={page.now}
				studentName={page.studentName}
			/>

			<div className="flex flex-wrap items-center gap-x-5 gap-y-2 text-xs text-[#717171]">
				<span className="flex items-center gap-2">
					<span className="w-2.5 h-2.5 rounded-sm bg-[#a3e635]" />
					{appointmentTypeLabels.Class}
				</span>

				<span className="flex items-center gap-2">
					<span className="w-2.5 h-2.5 rounded-sm bg-[#22d3ee]" />
					{appointmentTypeLabels.Assessment}
				</span>

				<span className="hidden md:inline ml-auto">
					Clique em um espaço vazio para agendar
				</span>
			</div>

			<WeekCalendar
				weekStart={page.weekStart}
				today={page.now}
				events={page.events}
				studentName={page.studentName}
				onSelect={page.selectEvent}
				onNew={page.openNew}
				onCalendarClick={page.handleCalendarClick}
			/>

			{page.initial && (
				<AppointmentForm
					key={page.initial.id ?? "new"}
					initial={page.initial}
					onClose={page.closeForm}
					onSaved={page.reload}
				/>
			)}

			{page.selectedEvent && (
				<AppointmentDetails
					selectedEvent={page.selectedEvent}
					studentName={page.studentName}
					onClose={page.closeDetails}
					onEdit={page.editSelected}
					removeSelected={() => void page.removeSelected()}
					removeSelectedOccurrence={() =>
						void page.removeSelectedOccurrence()
					}
					busy={page.busy}
				/>
			)}
		</div>
	)
}