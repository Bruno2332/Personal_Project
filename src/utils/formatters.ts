export function formatCPF(value: string) {
    return value
        .replace(/\D/g, "")
        .slice(0, 11)
        .replace(/(\d{3})(\d)/, "$1.$2")
        .replace(/(\d{3})(\d)/, "$1.$2")
        .replace(/(\d{3})(\d{1,2})$/, "$1-$2")
}

export function formatCEP(value: string) {
    return value
        .replace(/\D/g, "")
        .slice(0, 8)
        .replace(/(\d{5})(\d)/, "$1-$2")
}


export function formatCurrency(value: number | null): string {
    if (value === null) return ""

    return value.toLocaleString("pt-BR", {
        minimumFractionDigits: 2,
        maximumFractionDigits: 2,
    })
}

export function parseCurrency(value: string): number | null {
    const digits = value.replace(/\D/g, "")
    return digits ? Number(digits) / 100 : null
}

export function formatPhone(value: string) {
    const digits = value.replace(/\D/g, "").slice(0, 11)

    if (!digits) return ""
    if (digits.length <= 2) return `(${digits}`

    const ddd = digits.slice(0, 2)
    const number = digits.slice(2)
    const prefixLength = digits.length === 11 ? 5 : 4

    if (number.length <= prefixLength) {
        return `(${ddd}) ${number}`
    }

    return `(${ddd}) ${number.slice(0, prefixLength)}-${number.slice(prefixLength)}`
}

export function formatDate(value: string) {
    if (/^\d{4}-\d{2}-\d{2}$/.test(value)) {
        const [year, month, day] = value.split("-")
        return `${day}/${month}/${year}`
    }

    return value
        .replace(/\D/g, "")
        .slice(0, 8)
        .replace(/(\d{2})(\d)/, "$1/$2")
        .replace(/(\d{2})\/(\d{2})(\d)/, "$1/$2/$3")
}

export function isValidDate(value: string) {
    if (!/^\d{4}-\d{2}-\d{2}$/.test(value)) return false

    const [year, month, day] = value.split("-").map(Number)
    const date = new Date(0)
    date.setUTCFullYear(year, month - 1, day)

    return (
        year > 0 &&
        date.getUTCFullYear() === year &&
        date.getUTCMonth() === month - 1 &&
        date.getUTCDate() === day
    )
}

export function formatTime(value: string) {
	return value
		.replace(/\D/g, "")
		.slice(0, 4)
		.replace(/(\d{2})(\d)/, "$1:$2")
}

