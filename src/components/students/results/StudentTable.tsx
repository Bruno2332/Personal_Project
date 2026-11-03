import type { StudentProjectionDto } from "../../../model/student"
import { dateLabel, money } from "../../../utils/dates"
import { billingLabels } from "../../../labels/contract"
import { objectiveLabels } from "../../../labels/student"
import { Table } from "../../ui/Table"

type Props = {
	students: StudentProjectionDto[]
	onSelect: (student: StudentProjectionDto) => void
}

export default function StudentTable({ students, onSelect }: Props) {
	return (
		<div className="flex-1 bg-[#161616] border border-[#252525] rounded overflow-x-auto">
			<Table className="w-full min-w-[700px]">
				<thead>
					<tr className="border-b border-[#252525]">
						{[
							"Aluno",
							"Contato",
							"Objetivo",
							"Primeiro vencimento",
							"Contrato",
							"Status",
							"",
						].map((heading) => (
							<th
								key={heading}
								className="text-left px-4 py-3 text-xs font-medium text-[#717171] uppercase tracking-wider"
							>
								{heading}
							</th>
						))}
					</tr>
				</thead>

				<tbody>
					{students.map((student, index) => (
						<tr
							key={student.id}
							onClick={() => onSelect(student)}
							className={`border-b border-[#252525]/60 hover:bg-[#1e1e1e] cursor-pointer transition-colors ${index === students.length - 1
									? "border-b-0"
									: ""
								}`}
						>
							<td className="px-4 py-3">
								<div className="flex items-center gap-3">
									<div className="w-8 h-8 rounded-full bg-[#a3e635]/20 flex items-center justify-center text-[#a3e635] text-xs font-bold font-display uppercase">
										{student.name
											.trim()
											.split(/\s+/)
											.map((part) => part[0])
											.slice(0, 2)
											.join("")}
									</div>

									<div className="text-white font-medium text-sm">
										{student.name}
									</div>
								</div>
							</td>

							<td className="px-4 py-3">
								<div className="text-sm text-[#d0d0d0]">
									{student.email}
								</div>
								<div className="text-xs text-[#717171]">
									{student.phone || "—"}
								</div>
							</td>

							<td className="px-4 py-3 text-sm text-[#d0d0d0] max-w-[180px]">
								<span className="line-clamp-1">
									{objectiveLabels[student.objective] ?? "—"}
								</span>
							</td>

							<td className="px-4 py-3 text-sm text-[#717171] font-mono">
								{student.first_due_date
									? dateLabel(student.first_due_date)
									: "—"}
							</td>

							<td className="px-4 py-3">
								<div className="text-sm text-white font-mono font-medium">
									{money(student.amount_cents / 100)}
									{student.billing_type === "Monthly"
										? "/mês"
										: ""}
								</div>
								<div className="text-xs text-[#717171]">
									{billingLabels[student.billing_type]}
								</div>
							</td>

							<td className="px-4 py-3">
								<span
									className={`text-xs font-medium px-2 py-0.5 rounded-full ${student.is_active
											? "text-[#a3e635] bg-[#a3e635]/10"
											: "text-[#717171] bg-[#717171]/10"
										}`}
								>
									{student.is_active ? "Ativo" : "Inativo"}
								</span>
							</td>

							<td className="px-4 py-3">
								<span className="text-[#717171] hover:text-white transition-colors">
									→
								</span>
							</td>
						</tr>
					))}

					{students.length === 0 && (
						<tr>
							<td
								colSpan={7}
								className="px-4 py-12 text-center text-[#717171]"
							>
								Nenhum aluno encontrado
							</td>
						</tr>
					)}
				</tbody>
			</Table>
		</div>
	)
}
