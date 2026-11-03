import type { AssessmentView } from "../../../utils/assessments"
import { dateLabel } from "../../../utils/dates"

type Props = {
	assessments: AssessmentView[]
	studentName: string | null
	loading: boolean
	busy: boolean
	selectedAssessmentId: number | null
	onSelect: (assessment: AssessmentView) => void
}


export default function AssessmentHistory({
	assessments,
	studentName,
	loading,
	busy,
	selectedAssessmentId,
	onSelect,
}: Props) {
	return (
		<div className="bg-[#161616] border border-[#252525] rounded overflow-hidden">
			<div className="px-4 py-3 border-b border-[#252525]">
				<h3 className="text-sm font-medium text-white uppercase tracking-wider font-display">
					Histórico de Avaliações —{" "}
					{studentName ?? "Selecione um aluno"}
				</h3>
			</div>

			{assessments.length === 0 ? (
				<div className="px-4 py-12 text-center text-[#717171] text-sm">
					{loading
						? "Carregando avaliações..."
						: "Nenhuma avaliação registrada para este aluno"}
				</div>
			) : (
				<div className="divide-y divide-[#252525]/60">
					{[...assessments]
						.reverse()
						.map((assessment, index) => {
							const measurements = [
								{
									label: "Peso",
									value: `${assessment.peso}kg`,
								},
								{
									label: "IMC",
									value: assessment.imc.toFixed(1),
								},
								{
									label: "Gordura",
									value:
										assessment.percentualGordura == null
											? "—"
											: `${assessment.percentualGordura.toFixed(1)}%`,
								},
								{
									label: "Magra",
									value:
										assessment.massaMuscular == null
											? "—"
											: `${assessment.massaMuscular.toFixed(1)}kg`,
								},
								{
									label: "PA",
									value: assessment.pressaoArterial,
								},
								{
									label: "FC",
									value:
										assessment.frequenciaCardiacaRepouso == null
											? "—"
											: `${assessment.frequenciaCardiacaRepouso}bpm`,
								},
							]

							const selected =
								selectedAssessmentId === assessment.id

							return (
								<div
									key={assessment.id}
									role="button"
									tabIndex={busy ? -1 : 0}
									aria-disabled={busy}
									onClick={() => {
										if (!busy) onSelect(assessment)
									}}
									onKeyDown={(event) => {
										if (
											!busy &&
											(event.key === "Enter" ||
												event.key === " ")
										) {
											event.preventDefault()
											onSelect(assessment)
										}
									}}
									className={`px-4 py-3 hover:bg-[#1e1e1e] cursor-pointer transition-colors flex items-center gap-6 ${selected ? "bg-[#1e1e1e]" : ""
										} ${busy ? "opacity-60 cursor-not-allowed" : ""}`}
								>
									<div className="min-w-[90px]">
										<div className="text-sm font-mono text-white">
											{dateLabel(assessment.data)}
										</div>

										{index === 0 && (
											<span className="text-[10px] text-[#a3e635] font-medium uppercase tracking-wider">
												Mais recente
											</span>
										)}
									</div>

									<div className="flex gap-8 flex-1 flex-wrap">
										{measurements.map((measurement) => (
											<div key={measurement.label}>
												<div className="text-[10px] text-[#717171] uppercase tracking-wider">
													{measurement.label}
												</div>

												<div className="text-sm font-mono font-medium text-white">
													{measurement.value}
												</div>
											</div>
										))}
									</div>

									<span
										className={`text-[#717171] transition-transform ${selected ? "rotate-90" : ""
											}`}
									>
										›
									</span>
								</div>
							)
						})}
				</div>
			)}
		</div>
	)
}