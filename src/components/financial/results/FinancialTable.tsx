import { financialStatusStyles } from "../../../labels/financialTone"
import { financialRecordDto } from "../../../utils/financialRecord"
import type {
	FinancialRecordsDto,
	FinancialRecordsProjectionDto,
} from "../../../model/financialRecord"
import {
	paymentLabels,
	transactionTypeLabels,
	financialStatusLabels,
} from "../../../labels/financialRecord"
import { Button } from "../../ui/Button"
import { Table } from "../../ui/Table"
import { dateLabel, localDate, money } from "../../../utils/dates"

type Props = {
	items: FinancialRecordsProjectionDto[]
	total: number
	busyId: number | null
	onMarkAsPaid: (id: number) => void
	onCancel: (item: FinancialRecordsProjectionDto) => void
	onEdit: (record: FinancialRecordsDto) => void
}


export default function FinancialTable({
	items,
	total,
	busyId,
	onMarkAsPaid,
	onCancel,
	onEdit,
}: Props) {
	const today = localDate()

	function editRecord(item: FinancialRecordsProjectionDto) {
		onEdit(financialRecordDto(item))
	}

	return (
		<div className="bg-[#161616] border border-[#252525] rounded overflow-x-auto">
			<div className="px-4 py-3 border-b border-[#252525] flex items-center justify-between">
				<div>
					<h2 className="text-sm font-semibold text-white">
						Lançamentos financeiros
					</h2>

					<p className="text-[11px] text-[#5f5f5f] mt-0.5">
						{total} registro{total === 1 ? "" : "s"} encontrado
						{total === 1 ? "" : "s"}
					</p>
				</div>

				<div className="flex items-center gap-3 text-[10px] text-[#717171] uppercase tracking-wider">
					<span>
						<b className="text-[#22d3ee]">↻</b> contrato
					</span>
					<span>
						<b className="text-white">●</b> manual
					</span>
				</div>
			</div>

			<Table className="w-full min-w-[980px]">
				<thead>
					<tr className="border-b border-[#252525]">
						{[
							"Tipo",
							"Descrição",
							"Aluno",
							"Data",
							"Método",
							"Status",
							"Valor",
							"Ações",
						].map((header) => (
							<th
								key={header}
								className="text-left px-4 py-3 text-[10px] font-medium text-[#717171] uppercase tracking-wider"
							>
								{header}
							</th>
						))}
					</tr>
				</thead>

				<tbody>
					{items.map((item, index) => {
						const id = item.id
						const isIncome = item.transaction_type === "Income"
						const isPending = item.status_transaction === "Pending"
						const isCancelled = item.status_transaction === "Cancelled"
						const isOverdue = isPending && item.due_date < today
						const isContractLinked = item.contract_id !== null
						const actionsDisabled = busyId !== null || id === null

						const displayStatus = isOverdue
							? "Overdue"
							: item.status_transaction
						const statusClassName = financialStatusStyles[displayStatus]

						return (
							<tr
								key={id ?? index}
								className="border-b border-[#252525]/60 last:border-b-0 hover:bg-[#1e1e1e] transition-colors"
							>
								<td className="px-4 py-3">
									<span
										title={transactionTypeLabels[item.transaction_type]}
										aria-label={transactionTypeLabels[item.transaction_type]}
										className={`inline-flex items-center justify-center w-7 h-7 rounded-full text-sm ${isIncome
												? "text-[#a3e635] bg-[#a3e635]/10"
												: "text-red-400 bg-red-400/10"
											}`}
									>
										{isIncome ? "↑" : "↓"}
									</span>
								</td>

								<td className="px-4 py-3 text-sm text-white max-w-[240px]">
									<span className="line-clamp-1">
										{isContractLinked && (
											<span className="text-[#22d3ee] mr-1.5">↻</span>
										)}
										{item.description}
									</span>
								</td>

								<td className="px-4 py-3 text-xs text-[#8b8b8b]">
									{item.student_name ?? "—"}
								</td>

								<td className="px-4 py-3 text-xs font-mono text-[#717171]">
									{dateLabel(item.payment_date ?? item.due_date)}
								</td>

								<td className="px-4 py-3 text-xs text-[#717171]">
									{paymentLabels[item.payment_method]}
								</td>

								<td className="px-4 py-3">
									<span
										className={`text-[11px] font-medium px-2 py-1 rounded-full ${statusClassName}`}
									>
										{financialStatusLabels[displayStatus]}
									</span>
								</td>

								<td
									className={`px-4 py-3 text-sm font-mono font-semibold whitespace-nowrap ${isIncome ? "text-[#a3e635]" : "text-red-400"
										}`}
								>
									{isIncome ? "+" : "−"} {money(item.amount_cent / 100)}
								</td>

								<td className="px-4 py-3">
									<div className="flex items-center gap-2">
										<Button
											type="button"
											disabled={actionsDisabled}
											onClick={() => editRecord(item)}
											className="text-[11px] px-2 py-1 rounded border border-[#252525] text-[#717171] hover:text-white"
										>
											Editar
										</Button>

										{isPending && (
											<Button
												type="button"
												disabled={actionsDisabled}
												onClick={() => {
													if (id !== null) {
														onMarkAsPaid(id)
													}
												}}
												title={
													isIncome ? "Marcar como recebido" : "Marcar como pago"
												}
												className="text-[11px] px-2 py-1 rounded border border-[#a3e635]/30 text-[#a3e635] hover:bg-[#a3e635]/10"
											>
												Dar baixa
											</Button>
										)}

										{!isCancelled && (
											<Button
												type="button"
												disabled={actionsDisabled}
												onClick={() => onCancel(item)}
												title="Cancelar lançamento"
												className="text-[#5f5f5f] hover:text-red-400 px-1"
											>
												×
											</Button>
										)}
									</div>
								</td>
							</tr>
						)
					})}

					{items.length === 0 && (
						<tr>
							<td colSpan={8} className="px-4 py-12 text-center text-[#717171]">
								Nenhum lançamento encontrado com esses filtros
							</td>
						</tr>
					)}
				</tbody>
			</Table>
		</div>
	)
}
