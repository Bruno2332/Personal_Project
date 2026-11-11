import { invoke } from "@tauri-apps/api/core"

import type { AgendaDto } from "../model/agenda"
import { findAgendaExclusionsByPeriod } from "./agendaExclusionService"

export async function findAgendaByPeriod(
	startDate: string,
	endDate: string,
): Promise<AgendaDto[]> {
	const [appointments, exclusions] = await Promise.all([
		invoke<AgendaDto[]>("find_agenda_by_period", {
			startDate,
			endDate,
		}),
		findAgendaExclusionsByPeriod(startDate, endDate),
	])

	const datesByAppointment = new Map<number, string[]>()

	for (const exclusion of exclusions) {
		const dates = datesByAppointment.get(exclusion.agenda_id) ?? []
		dates.push(exclusion.occurrence_date)
		datesByAppointment.set(exclusion.agenda_id, dates)
	}

	return appointments.map((appointment) => ({
		...appointment,
		excluded_dates:
			appointment.id === null
				? []
				: datesByAppointment.get(appointment.id) ?? [],
	}))
}

export function endAgendaRecurrence(id: number) {
	return invoke<AgendaDto>("end_agenda_recurrence", { id })
}

export function insertAgenda(dto: AgendaDto) {
	const { excluded_dates: _excludedDates, ...payload } = dto

	return invoke<AgendaDto>("insert_agenda", { dto: payload })
}

export function updateAgenda(id: number, dto: AgendaDto) {
	const { excluded_dates: _excludedDates, ...payload } = dto

	return invoke<AgendaDto>("update_agenda", {
		id,
		dto: payload,
	})
}

export function deleteAgenda(id: number) {
	return invoke<void>("delete_agenda", { id })
}