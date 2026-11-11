import { useState } from "react"
import { useForm, useWatch } from "react-hook-form"
import type { AgendaDto } from "../../../model/agenda"
import {
    findAgendaByPeriod,
    insertAgenda,
    updateAgenda,
} from "../../../services/agendaService"
import {
    appointmentsOverlap,
    timeToMinutes,
} from "../../../utils/agenda"
import { errorMessage } from "../../../utils/errors"
import useAssessmentStudentOptions from "../../assessments/hooks/useAssessmentStudentOptions"



type Options = {
    initial: AgendaDto
    onClose: () => void
    onSaved: () => Promise<void>
}


export default function useAppointmentForm({
    initial,
    onClose,
    onSaved,
}: Options) {
    const [error, setError] = useState("")
    const [savedId, setSavedId] = useState(initial.id)

    const methods = useForm<AgendaDto>({
        defaultValues: {
            ...initial,
            start_time: initial.start_time.slice(0, 5),
            end_time: initial.end_time.slice(0, 5),
        },
    })

    const appointmentType = useWatch({
        control: methods.control,
        name: "appointment_type",
    })

    const {
        studentOptions,
        loading: loadingStudents,
        error: studentsError,
        handleRetry: retryStudents,
    } = useAssessmentStudentOptions(
        initial.student_id > 0 ? initial.student_id : null,
    )

    async function save(values: AgendaDto) {
        setError("")

        if (loadingStudents || studentsError) {
            setError("Carregue a lista de alunos antes de salvar.")
            return
        }

        if (!values.student_id || values.student_id <= 0) {
            methods.setError("student_id", {
                message: "Selecione um aluno.",
            })
            return
        }

        const startMinutes = timeToMinutes(values.start_time)
        const endMinutes = timeToMinutes(values.end_time)

        if (
            !Number.isFinite(startMinutes) ||
            !Number.isFinite(endMinutes) ||
            endMinutes <= startMinutes
        ) {
            methods.setError("end_time", {
                message: "O término deve ser posterior ao início.",
            })
            return
        }

        const dto: AgendaDto = {
            ...values,
            id: savedId,
            start_time: `${values.start_time.slice(0, 5)}:00`,
            end_time: `${values.end_time.slice(0, 5)}:00`,
            notes: values.notes ?? "",
            updated_at: new Date().toISOString(),
        }

        if (dto.is_recurring) {
            dto.scheduled_date = null

            if (!dto.recurrence_start_date) {
                methods.setError("recurrence_start_date", {
                    message: "Informe o início da recorrência.",
                })
                return
            }

            if (!dto.week_day) {
                methods.setError("week_day", {
                    message: "Selecione o dia da semana.",
                })
                return
            }

            if (
                dto.recurrence_end_date &&
                dto.recurrence_end_date < dto.recurrence_start_date
            ) {
                methods.setError("recurrence_end_date", {
                    message: "O término não pode ser anterior ao início.",
                })
                return
            }
        } else {
            dto.week_day = null
            dto.recurrence_start_date = null
            dto.recurrence_end_date = null

            if (!dto.scheduled_date) {
                methods.setError("scheduled_date", {
                    message: "Informe a data do compromisso.",
                })
                return
            }
        }

        const startDate = dto.is_recurring
            ? dto.recurrence_start_date
            : dto.scheduled_date

        if (!startDate) return

        const endDate = dto.is_recurring
            ? dto.recurrence_end_date ?? "9999-12-31"
            : startDate

        try {
            const existingAppointments = await findAgendaByPeriod(
                startDate,
                endDate,
            )

            const persistedAppointment = existingAppointments.find(
                (appointment) => appointment.id === savedId,
            )

            const candidate: AgendaDto = {
                ...dto,
                excluded_dates: persistedAppointment?.excluded_dates ?? [],
            }

            const hasConflict = existingAppointments.some(
                (appointment) =>
                    appointment.id !== savedId &&
                    appointmentsOverlap(candidate, appointment),
            )

            if (hasConflict) {
                setError("Este horário já está ocupado. Escolha outro horário.")
                return
            }

            const saved =
                savedId !== null
                    ? await updateAgenda(savedId, dto)
                    : await insertAgenda(dto)

            setSavedId(saved.id)

            methods.reset({
                ...saved,
                start_time: saved.start_time.slice(0, 5),
                end_time: saved.end_time.slice(0, 5),
            })

            try {
                await onSaved()
            } catch {
                setError("Horário salvo. Não foi possível recarregar a agenda.")
                return
            }

            onClose()
        } catch (error) {
            setError(errorMessage(error))
        }
    }

    return {
        methods,
        save,
        savedId,
        error,
        appointmentType,
        isSubmitting: methods.formState.isSubmitting,
        studentOptions,
        loadingStudents,
        studentsError,
        retryStudents,
    }
}