import { invoke } from "@tauri-apps/api/core"
import type { AgendaDto } from "../model/agenda"

export function findAgendaByPeriod(startDate: string, endDate: string) {
  return invoke<AgendaDto[]>("find_agenda_by_period", { startDate, endDate })
}
export function insertAgenda(dto: AgendaDto) {
  return invoke<AgendaDto>("insert_agenda", { dto })
}
export function updateAgenda(id: number, dto: AgendaDto) {
  return invoke<AgendaDto>("update_agenda", { id, dto })
}
export function deleteAgenda(id: number) {
  return invoke<void>("delete_agenda", { id })
}
