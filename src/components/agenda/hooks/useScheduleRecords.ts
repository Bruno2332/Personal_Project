import {
    useCallback,
    useEffect,
    useMemo,
    useRef,
    useState,
} from "react"
import type { AgendaDto } from "../../../model/agenda"
import { findAgendaByPeriod } from "../../../services/agendaService"
import { findStudentById } from "../../../services/studentService"
import {
    agendaEvents,
    addDays,
    formatISODate,
} from "../../../utils/agenda"
import { errorMessage } from "../../../utils/errors"



export default function useScheduleRecords(weekStart: Date) {
    const [appointments, setAppointments] = useState<AgendaDto[]>([])
    const [studentNames, setStudentNames] = useState<Record<number, string>>({})
    const [loading, setLoading] = useState(false)
    const [error, setError] = useState("")
    const [studentsError, setStudentsError] = useState("")

    const requestId = useRef(0)

    const reload = useCallback(async () => {
        const currentRequest = ++requestId.current

        setLoading(true)
        setError("")

        try {
            const response = await findAgendaByPeriod(
                formatISODate(weekStart),
                formatISODate(addDays(weekStart, 6)),
            )

            if (currentRequest === requestId.current) {
                setAppointments(response)
            }
        } catch (error) {
            if (currentRequest === requestId.current) {
                setError(errorMessage(error))
            }

            throw error
        } finally {
            if (currentRequest === requestId.current) {
                setLoading(false)
            }
        }
    }, [weekStart])

    useEffect(() => {
        setAppointments([])
        void reload().catch(() => { })

        return () => {
            requestId.current++
        }
    }, [reload])

    useEffect(() => {
        let cancelled = false

        const studentIds = [
            ...new Set(
                appointments.map((appointment) => appointment.student_id),
            ),
        ]

        setStudentsError("")

        async function loadStudentNames() {
            const results = await Promise.allSettled(
                studentIds.map(async (id) => {
                    const student = await findStudentById(id)

                    return {
                        id,
                        name: student.name,
                    }
                }),
            )

            if (cancelled) return

            const names: Record<number, string> = {}
            let hasError = false

            for (const result of results) {
                if (result.status === "fulfilled") {
                    names[result.value.id] = result.value.name
                } else {
                    hasError = true
                }
            }

            setStudentNames(names)

            if (hasError) {
                setStudentsError(
                    "Não foi possível carregar o nome de alguns alunos.",
                )
            }
        }

        void loadStudentNames()

        return () => {
            cancelled = true
        }
    }, [appointments])

    const events = useMemo(
        () => agendaEvents(appointments, weekStart),
        [appointments, weekStart],
    )

    function studentName(id: number) {
        return studentNames[id] ?? `Aluno #${id}`
    }

    return {
        events,
        loading,
        error,
        studentsError,
        setError,
        setAppointments,
        studentName,
        reload,
    }
}