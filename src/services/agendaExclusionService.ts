import { invoke } from "@tauri-apps/api/core"

import type {
	AgendaExclusionDto,
	CreateAgendaExclusionDto,
} from "../model/agendaExclusion"


export function findAgendaExclusionsByPeriod(
	startDate: string,
	endDate: string,
) {
	return invoke<AgendaExclusionDto[]>(
		"find_agenda_exclusions_by_period",
		{ startDate, endDate },
	)
}

export function excludeAgendaOccurrence(
	dto: CreateAgendaExclusionDto,
) {
	return invoke<AgendaExclusionDto>(
		"exclude_agenda_occurrence",
		{ dto },
	)
}