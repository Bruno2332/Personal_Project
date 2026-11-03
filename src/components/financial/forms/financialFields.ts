export const financialTabs = [
	{ id: "monthly", label: "Mensalidades" },
	{ id: "other", label: "Outros lançamentos" },
] as const

export type FinancialTab = typeof financialTabs[number]["id"]