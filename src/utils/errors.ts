import type { AppError } from "../model/appError"

export function isAppError(value: unknown): value is AppError {
  if (!value || typeof value !== "object" || !("type" in value)) return false
  if (value.type === "validation") {
    return (
      "errors" in value &&
      Array.isArray(value.errors) &&
      value.errors.every(
        (item) =>
          item &&
          typeof item.field === "string" &&
          typeof item.message === "string",
      )
    )
  }
  return (
    ["not_found", "database", "internal"].includes(String(value.type)) &&
    "message" in value &&
    typeof value.message === "string"
  )
}

export function errorMessage(error: unknown): string {
  if (isAppError(error)) {
    return error.type === "validation"
      ? error.errors.map((item) => item.message).join(" ")
      : error.message
  }
  if (error instanceof Error) return error.message
  if (typeof error === "string") return error
  return "Não foi possível concluir a operação. Tente novamente."
}
