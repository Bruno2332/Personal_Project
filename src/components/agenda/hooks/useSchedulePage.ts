import {
    useEffect,
    useRef,
    useState,
    type MouseEvent,
} from "react"
import type { AgendaDto } from "../../../model/agenda"
import {
    deleteAgenda,
    endAgendaRecurrence,
} from "../../../services/agendaService"
import {
    type AgendaEvent,
    startOfWeek,
    addDays,
    formatISODate,
    minutesToTime,
    weekDayValues,
    START_HOUR,
    END_HOUR,
} from "../../../utils/agenda"
import { errorMessage } from "../../../utils/errors"
import useScheduleRecords from "./useScheduleRecords"
import {
    excludeAgendaOccurrence,
} from "../../../services/agendaExclusionService"



export default function useSchedulePage() {
    const [now, setNow] = useState(() => new Date())
    const [weekStart, setWeekStart] = useState(() =>
        startOfWeek(new Date()),
    )

    const [notice, setNotice] = useState("")
    const [busy, setBusy] = useState(false)
    const [initial, setInitial] = useState<AgendaDto | null>(null)
    const [selectedEvent, setSelectedEvent] =
        useState<AgendaEvent | null>(null)

    const removing = useRef(false)
    const records = useScheduleRecords(weekStart)

    const interactionBlocked =
        busy || initial !== null || selectedEvent !== null

    useEffect(() => {
        const timer = window.setInterval(() => {
            setNow(new Date())
        }, 60_000)

        return () => {
            window.clearInterval(timer)
        }
    }, [])

    function previousWeek() {
        if (interactionBlocked) return

        setNotice("")
        setWeekStart((current) => addDays(current, -7))
    }

    function nextWeek() {
        if (interactionBlocked) return

        setNotice("")
        setWeekStart((current) => addDays(current, 7))
    }

    function goToToday() {
        if (interactionBlocked) return

        const currentDate = new Date()
        const currentWeek = startOfWeek(currentDate)

        setNow(currentDate)
        setNotice("")

        setWeekStart((previous) =>
            formatISODate(previous) === formatISODate(currentWeek)
                ? previous
                : currentWeek,
        )
    }

    function openNew(date = new Date(), startTime = "08:00") {
        if (interactionBlocked) return

        const timestamp = new Date().toISOString()
        const dateString = formatISODate(date)
        const [hours, minutes] = startTime.split(":").map(Number)

        setNotice("")
        records.setError("")

        setInitial({
            id: null,
            student_id: 0,
            appointment_type: "Class",
            is_recurring: true,
            scheduled_date: dateString,
            week_day: weekDayValues[date.getDay()],
            start_time: startTime,
            end_time: minutesToTime(
                Math.min(hours * 60 + minutes + 60, 23 * 60 + 59),
            ),
            recurrence_start_date: dateString,
            recurrence_end_date: null,
            notes: "",
            created_at: timestamp,
            updated_at: timestamp,
        })
    }

    function handleCalendarClick(
        event: MouseEvent<HTMLDivElement>,
        date: Date,
    ) {
        if (interactionBlocked) return

        const rect = event.currentTarget.getBoundingClientRect()

        if (rect.height <= 0) return

        const ratio = Math.max(
            0,
            Math.min(1, (event.clientY - rect.top) / rect.height),
        )

        const minutes =
            START_HOUR * 60 +
            Math.round(
                (ratio * (END_HOUR - START_HOUR) * 60) / 30,
            ) * 30

        openNew(
            date,
            minutesToTime(Math.min(minutes, END_HOUR * 60 - 30)),
        )
    }

    function selectEvent(event: AgendaEvent) {
        if (interactionBlocked || records.loading) return
        setSelectedEvent(event)
    }

    function closeDetails() {
        if (busy) return
        setSelectedEvent(null)
    }

    function closeForm() {
        setInitial(null)
    }

    function editSelected() {
        if (!selectedEvent || busy) return

        setInitial({ ...selectedEvent.raw })
        setSelectedEvent(null)
    }

    function dismissNotice() {
        setNotice("")
    }

    function retry() {
        if (interactionBlocked) return
        void records.reload().catch(() => { })
    }

    async function removeSelected() {
        if (!selectedEvent || removing.current) return

        const appointment = selectedEvent.raw

        if (appointment.id === null) {
            records.setError("O agendamento não possui um ID válido.")
            return
        }

        const recurring = appointment.is_recurring

        const confirmed = window.confirm(
            recurring
                ? "Encerrar esta recorrência a partir de hoje? Os agendamentos anteriores serão preservados."
                : "Excluir este compromisso?",
        )

        if (!confirmed) return

        removing.current = true
        setBusy(true)
        records.setError("")
        setNotice("")

        try {
            if (recurring) {
                const updated = await endAgendaRecurrence(appointment.id)

                records.setAppointments((current) =>
                    current.map((item) =>
                        item.id === updated.id
                            ? {
                                ...updated,
                                excluded_dates: item.excluded_dates,
                            }
                            : item,
                    ),
                )
            } else {
                await deleteAgenda(appointment.id)

                records.setAppointments((current) =>
                    current.filter((item) => item.id !== appointment.id),
                )
            }

            setSelectedEvent(null)

            try {
                await records.reload()

                setNotice(
                    recurring
                        ? "Recorrência encerrada. O histórico foi preservado."
                        : "Compromisso removido.",
                )
            } catch {
                records.setError(
                    recurring
                        ? "Recorrência encerrada, mas não foi possível recarregar a agenda."
                        : "Compromisso removido, mas não foi possível recarregar a agenda.",
                )
            }
        } catch (error) {
            records.setError(errorMessage(error))
        } finally {
            removing.current = false
            setBusy(false)
        }
    }

    async function removeSelectedOccurrence() {
        if (!selectedEvent || removing.current) return

        const appointment = selectedEvent.raw

        if (!appointment.is_recurring) return

        if (appointment.id === null) {
            records.setError("O agendamento não possui um ID válido.")
            return
        }

        const appointmentId = appointment.id
        const occurrenceDate = formatISODate(selectedEvent.date)

        const confirmed = window.confirm(
            `Excluir somente a aula de ${selectedEvent.date.toLocaleDateString("pt-BR")}? As outras ocorrências serão mantidas.`,
        )

        if (!confirmed) return

        removing.current = true
        setBusy(true)
        records.setError("")
        setNotice("")

        try {
            await excludeAgendaOccurrence({
                agenda_id: appointmentId,
                occurrence_date: occurrenceDate,
            })

            records.setAppointments((current) =>
                current.map((item) =>
                    item.id === appointmentId
                        ? {
                            ...item,
                            excluded_dates: [
                                ...new Set([
                                    ...(item.excluded_dates ?? []),
                                    occurrenceDate,
                                ]),
                            ],
                        }
                        : item,
                ),
            )

            setSelectedEvent(null)

            try {
                await records.reload()
                setNotice("Aula excluída somente na data selecionada.")
            } catch {
                records.setError(
                    "Aula excluída, mas não foi possível recarregar a agenda.",
                )
            }
        } catch (error) {
            records.setError(errorMessage(error))
        } finally {
            removing.current = false
            setBusy(false)
        }
    }

    return {
        now,
        weekStart,
        notice,
        busy,
        initial,
        selectedEvent,
        interactionBlocked,

        events: records.events,
        loading: records.loading,
        error: records.error,
        studentsError: records.studentsError,
        studentName: records.studentName,
        reload: records.reload,

        previousWeek,
        nextWeek,
        goToToday,
        openNew,
        handleCalendarClick,
        selectEvent,
        closeDetails,
        closeForm,
        editSelected,
        dismissNotice,
        retry,
        removeSelected,
        removeSelectedOccurrence,
    }
}