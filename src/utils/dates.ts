export function localDate(date = new Date()): string {
  return `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, "0")}-${String(date.getDate()).padStart(2, "0")}`
}
export function dateLabel(value: string | null | undefined): string {
  return value
    ? new Date(value.slice(0, 10) + "T12:00:00").toLocaleDateString("pt-BR")
    : "—"
}
export function ageAtToday(birth: string): number {
  const today = new Date()
  const [year, month, day] = birth.split("-").map(Number)
  return (
    today.getFullYear() -
    year -
    (today.getMonth() + 1 < month ||
    (today.getMonth() + 1 === month && today.getDate() < day)
      ? 1
      : 0)
  )
}
export const money = (value: number) =>
  value.toLocaleString("pt-BR", { style: "currency", currency: "BRL" })
export const normalizeSearch = (value: string) =>
  value
    .normalize("NFD")
    .replace(/[\u0300-\u036f]/g, "")
    .toLocaleLowerCase("pt-BR")
