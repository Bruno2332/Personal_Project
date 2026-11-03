export const WEEK_DAYS = [
    { value: 1, short: "SEG", long: "Segunda" },
    { value: 2, short: "TER", long: "Terça" },
    { value: 3, short: "QUA", long: "Quarta" },
    { value: 4, short: "QUI", long: "Quinta" },
    { value: 5, short: "SEX", long: "Sexta" },
    { value: 6, short: "SÁB", long: "Sábado" },
    { value: 0, short: "DOM", long: "Domingo" },
]

export const START_HOUR = 5
export const END_HOUR = 22
export const HOUR_HEIGHT = 64
export function pad(value: number) {
    return String(value).padStart(2, "0")
}

export function formatISODate(date: Date) {
    return [
        date.getFullYear(),
        pad(date.getMonth() + 1),
        pad(date.getDate()),
    ].join("-")
}

export function parseLocalDate(value: string) {
    return new Date(value + "T12:00:00")
}

export function addDays(date: Date, amount: number) {
    const result = new Date(date)
    result.setDate(result.getDate() + amount)
    return result
}

export function startOfWeek(date: Date) {
    const result = new Date(date)
    result.setHours(12, 0, 0, 0)
    result.setDate(result.getDate() - ((result.getDay() + 6) % 7))
    return result
}

export function firstOccurrence(startDate: string, dayOfWeek: number) {
    const start = parseLocalDate(startDate)
    const offset = (dayOfWeek - start.getDay() + 7) % 7
    return addDays(start, offset)
}

export function timeToMinutes(time: string) {
    const [hours, minutes] = time.split(":").map(Number)
    return hours * 60 + minutes
}

export function minutesToTime(total: number) {
    return pad(Math.floor(total / 60)) + ":" + pad(total % 60)
}

export function overlap(
    startA: string,
    durationA: number,
    startB: string,
    durationB: number,
) {
    const a = timeToMinutes(startA)
    const b = timeToMinutes(startB)
    return a < b + durationB && b < a + durationA
}

export function weekdayLabel(day: number) {
    return WEEK_DAYS.find((item) => item.value === day)?.long ?? ""
}

export function durationLabel(minutes: number) {
    if (minutes < 60) return String(minutes) + " min"
    if (minutes === 60) return "1 hora"
    return String(minutes / 60).replace(".", ",") + " horas"
}

import type { AgendaDto, WeekDay } from "../model/agenda"
export type AgendaEvent = {
    id: string
    sourceId: number
    kind: "aula" | "avaliacao"
    alunoId: number
    date: Date
    horario: string
    duracaoMinutos: number
    observacoes: string
    recurring: boolean
    raw: AgendaDto
}
export const weekDayValues: WeekDay[] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
]
export function agendaEvents(
    rows: AgendaDto[],
    start: Date,
    days = 7,
): AgendaEvent[] {
    return rows
        .flatMap((row) => {
            if (row.id === null) return []
            const dates = Array.from({ length: days }, (_, offset) =>
                addDays(start, offset),
            ).filter((date) => {
                const iso = formatISODate(date)
                return row.is_recurring
                    ? row.week_day === weekDayValues[date.getDay()] &&
                    !!row.recurrence_start_date &&
                    iso >= row.recurrence_start_date &&
                    (!row.recurrence_end_date || iso <= row.recurrence_end_date)
                    : row.scheduled_date === iso
            })
            return dates.map((date) => ({
                id: `${row.id}-${formatISODate(date)}`,
                sourceId: row.id!,
                kind:
                    row.appointment_type === "Class"
                        ? "aula" as const
                        : "avaliacao" as const,
                alunoId: row.student_id,
                date,
                horario: row.start_time.slice(0, 5),
                duracaoMinutos:
                    timeToMinutes(row.end_time) - timeToMinutes(row.start_time),
                observacoes: row.notes,
                recurring: row.is_recurring,
                raw: row,
            }))
        })
        .sort(
            (a, b) =>
                a.date.getTime() - b.date.getTime() ||
                a.horario.localeCompare(b.horario),
        )
}

// Compara as regras de recorrência sem gerar uma lista ilimitada de datas.
export function appointmentsOverlap(a: AgendaDto, b: AgendaDto): boolean {
    if (
        !overlap(
            a.start_time,
            timeToMinutes(a.end_time) - timeToMinutes(a.start_time),
            b.start_time,
            timeToMinutes(b.end_time) - timeToMinutes(b.start_time),
        )
    )
        return false

    const occursOn = (row: AgendaDto, date: string) =>
        row.is_recurring
            ? !!row.recurrence_start_date &&
            date >= row.recurrence_start_date &&
            (!row.recurrence_end_date || date <= row.recurrence_end_date) &&
            row.week_day === weekDayValues[parseLocalDate(date).getDay()]
            : row.scheduled_date === date

    if (!a.is_recurring)
        return !!a.scheduled_date && occursOn(b, a.scheduled_date)
    if (!b.is_recurring)
        return !!b.scheduled_date && occursOn(a, b.scheduled_date)
    if (
        !a.recurrence_start_date ||
        !b.recurrence_start_date ||
        !a.week_day ||
        a.week_day !== b.week_day
    )
        return false
    const start =
        a.recurrence_start_date > b.recurrence_start_date
            ? a.recurrence_start_date
            : b.recurrence_start_date
    const endA = a.recurrence_end_date ?? "9999-12-31"
    const endB = b.recurrence_end_date ?? "9999-12-31"
    const end = endA < endB ? endA : endB
    if (start > end) return false
    return (
        formatISODate(firstOccurrence(start, weekDayValues.indexOf(a.week_day))) <=
        end
    )
}
