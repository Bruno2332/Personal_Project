import type { TransactionType } from "../../../model/financialRecord"
import { financialStatusLabels, transactionTypeLabels } from "../../../labels/financialRecord"
import { Button } from "../../ui/Button"
import { FormSelect } from "../../ui/Fields"
import { SearchInput } from "../../ui/SearchInput"


type Props = {
	typeFilter: "all" | TransactionType
	search: string
	onTypeFilter: (value: "all" | TransactionType) => void
	onSearch: (value: string) => void
}


export default function FinancialFilters({
	typeFilter,
	search,
	onTypeFilter,
	onSearch,
}: Props) {
	return (
		<div className="flex flex-col gap-3">

			<div className="flex flex-col sm:flex-row sm:items-end justify-between gap-3">
				<div className="flex flex-col sm:flex-row sm:items-end gap-2">
					<FormSelect
						name="status"
						label="Status"
						inputStyle="px-3 py-1.5 rounded text-xs bg-[#161616] border border-[#252525] text-[#b0b0b0] focus:outline-none focus:border-[#a3e635]"
						options={[
							{ value: "all", label: "Todos os status" },
							...Object.entries(financialStatusLabels).map(
								([value, label]) => ({ value, label }),
							),
						]}
					/>

					<FormSelect
						name="period"
						label="Período"
						inputStyle="px-3 py-1.5 rounded text-xs bg-[#161616] border border-[#252525] text-[#b0b0b0] focus:outline-none focus:border-[#a3e635] capitalize"
						options={[
							{ value: "all", label: "Todo o período" },
							{ value: "1m", label: "Último mês" },
							{ value: "3m", label: "Últimos 3 meses" },
							{ value: "6m", label: "Últimos 6 meses" },
							{ value: "12m", label: "Último ano" },
						]}
					/>
				</div>

				<div className="flex flex-wrap items-center gap-2">
					{(["all", "Income", "Expense"] as const).map((filter) => (
						<Button
							key={filter}
							type="button"
							onClick={() => onTypeFilter(filter)}
							className={`px-3 py-1.5 rounded text-xs font-medium transition-colors ${typeFilter === filter
								? "bg-[#a3e635] text-black"
								: "bg-[#161616] border border-[#252525] text-[#717171] hover:text-white"
								}`}
						>
							{filter === "all"
								? "Todos"
								: transactionTypeLabels[filter]}
						</Button>
					))}

				</div>

			</div>
			<SearchInput
				value={search}
				onChange={(event) => onSearch(event.target.value)}
				placeholder="Buscar descrição ou aluno..."
				containerClassName="w-full"
				className="w-full px-3 py-2.5 rounded text-xs bg-[#161616] border border-[#252525] text-white placeholder:text-[#5f5f5f] focus:outline-none focus:border-[#a3e635]"
				showIcon={false}
			/>

		</div>
	)
}
