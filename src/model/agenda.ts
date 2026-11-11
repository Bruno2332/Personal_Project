export type AppointmentType = "Class" | "Assessment"

export type WeekDay = "Sunday" | "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" | "Saturday"

export type AgendaDto = {
    id: number | null
    student_id: number
    appointment_type: AppointmentType
    is_recurring: boolean
    scheduled_date: string | null
    week_day: WeekDay | null
    start_time: string
    end_time: string
    recurrence_start_date: string | null
    recurrence_end_date: string | null
    notes: string
    created_at: string
    updated_at: string
    excluded_dates?: string[]
}
