import type { FinancialRecordsProjectionDto } from "../../../model/financialRecord"
import { financialStatusLabels } from "../../../labels/financialRecord"
import { dateLabel, localDate, money } from "../../../utils/dates"
import { Button } from "../../ui/Button"
import { EmptyTableRow, Table, TableHeader } from "../../ui/Table"

type Props = {
	charges: FinancialRecordsProjectionDto[]
	loading: boolean
	error: string
	onReload: () => void
}

export default function ContractCharges({
	charges,
	loading,
	error,
	onReload,
}: Props) {
	const today = localDate()

	return (
		<section>
			<div className="flex items-center justify-between mb-2">
				<p className="text-xs text-[#717171] uppercase tracking-wider font-medium">
					Cronograma de Parcelas
				</p>
			</div>

			{error && (
				<div className="mb-3 space-y-2">
					<p role="alert" className="text-sm text-red-400">
						{error}
					</p>
					<Button
						type="button"
						disabled={loading}
						onClick={onReload}
						className="text-sm text-[#a3e635]"
					>
						Recarregar cobranças
					</Button>
				</div>
			)}

			<div className="bg-[#161616] border border-[#252525] rounded overflow-hidden">
				<Table className="w-full text-sm">
					<TableHeader
						columns={["Parcela", "Vencimento", "Valor", "Status"]}
						className="text-left px-4 py-2 text-xs text-[#717171] uppercase tracking-wider"
					/>
					<tbody>
						{charges.map((charge) => {
							const isOverdue =
								charge.status_transaction === "Pending" &&
								charge.due_date < today
							const status = isOverdue
								? "Overdue"
								: charge.status_transaction

							return (
								<tr
									key={charge.id}
									className="border-b border-[#252525]/50 last:border-b-0"
								>
									<td className="px-4 py-2 font-mono text-[#717171]">
										{String(charge.installment_number).padStart(2, "0")}
									</td>
									<td className="px-4 py-2 font-mono text-sm">
										{dateLabel(charge.due_date)}
									</td>
									<td className="px-4 py-2 font-mono font-medium text-white">
										{money(charge.amount_cent / 100)}
									</td>
									<td className="px-4 py-2">
										<span
											className={`text-xs px-2 py-0.5 rounded-full font-medium ${charge.status_transaction === "Paid"
													? "text-[#a3e635] bg-[#a3e635]/10"
													: isOverdue
														? "text-red-400 bg-red-400/10"
														: "text-yellow-400 bg-yellow-400/10"
												}`}
										>
											{financialStatusLabels[status]}
										</span>
									</td>
								</tr>
							)
						})}

						{charges.length === 0 && (
							<EmptyTableRow columns={4}>
								{loading
									? "Carregando cobranças..."
									: error
										? "Não foi possível carregar as cobranças."
										: "Nenhuma cobrança registrada"}
							</EmptyTableRow>
						)}
					</tbody>
				</Table>
			</div>
		</section>
	)
}