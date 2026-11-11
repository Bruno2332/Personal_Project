import type { CellDef, RowInput } from "jspdf-autotable"
import type { ClassificationTone } from "../components/ui/ClassificationBadge"

const colors: Record<ClassificationTone, [number, number, number]> = {
    normal: [35, 112, 51],
    attention: [139, 91, 0],
    risk: [177, 35, 35],
}

export function classification<T extends string>(
    value: T | null | undefined,
    names: Record<T, string>,
    severity: Record<T, ClassificationTone>,
): CellDef {
    if (value == null) return { content: "" }

    return {
        content: names[value],
        styles: {
            textColor: colors[severity[value]],
            fontStyle: "bold",
        },
    }
}

export function formatNumber(
    value: number | null | undefined,
    unit = "",
    decimalPlaces?: number,
): string {
    if (value == null || !Number.isFinite(value)) return "—"

    return `${value.toLocaleString("pt-BR", {
        minimumFractionDigits: decimalPlaces ?? 0,
        maximumFractionDigits: decimalPlaces ?? 2,
    })}${unit}`
}

export function formatDate(value: string): string {
    const [year, month, day] = value.slice(0, 10).split("-")
    return `${day}/${month}/${year}`
}

export function measurement(
    label: string,
    value: number | null | undefined,
    unit = "",
    result: CellDef = { content: "" },
): RowInput[] {
    if (value == null) return []

    return [[label, formatNumber(value, unit), result]]
}