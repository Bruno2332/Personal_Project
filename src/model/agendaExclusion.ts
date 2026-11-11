export type CreateAgendaExclusionDto = {
	agenda_id: number
	occurrence_date: string
}

export type AgendaExclusionDto = {
	id: number | null
	agenda_id: number
	occurrence_date: string
	created_at: string
}