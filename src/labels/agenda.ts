import type { AppointmentType, WeekDay } from "../model/agenda"

export const appointmentTypeLabels: Record<AppointmentType, string> = {
	Class: "Aula",
	Assessment: "Avaliação física",
}

export const weekDayLabels: Record<WeekDay, string> = {
	Sunday: "Domingo",
	Monday: "Segunda",
	Tuesday: "Terça",
	Wednesday: "Quarta",
	Thursday: "Quinta",
	Friday: "Sexta",
	Saturday: "Sábado",
}