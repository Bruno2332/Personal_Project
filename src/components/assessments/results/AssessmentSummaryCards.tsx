import type { AssessmentView } from "../../../utils/assessments"

type Props = {
	current: AssessmentView
	previous?: AssessmentView
}


export default function AssessmentSummaryCards({
	current,
	previous,
}: Props) {
	const difference = (
		value?: number | null,
		previousValue?: number | null,
	) => {
		if (value == null || previousValue == null) return null
		return value - previousValue
	}

	const cards = [
		{
			label: "Peso Atual",
			value: `${current.peso} kg`,
			diff: difference(current.peso, previous?.peso),
			unit: "kg",
			inverse: true,
		},
		{
			label: "IMC",
			value: current.imc.toFixed(1),
			diff: difference(current.imc, previous?.imc),
			unit: "",
			inverse: true,
		},
		{
			label: "% Gordura",
			value:
				current.percentualGordura == null
					? "—"
					: `${current.percentualGordura.toFixed(1)}%`,
			diff: difference(
				current.percentualGordura,
				previous?.percentualGordura,
			),
			unit: "%",
			inverse: true,
		},
		{
			label: "Massa Magra",
			value:
				current.massaMuscular == null
					? "—"
					: `${current.massaMuscular.toFixed(1)} kg`,
			diff: difference(
				current.massaMuscular,
				previous?.massaMuscular,
			),
			unit: "kg",
			inverse: false,
		},
	]

	return (
		<div className="grid grid-cols-2 lg:grid-cols-4 gap-3">
			{cards.map((card) => (
				<div
					key={card.label}
					className="bg-[#161616] border border-[#252525] rounded p-4"
				>
					<p className="text-xs text-[#717171] uppercase tracking-wider font-medium mb-2">
						{card.label}
					</p>

					<p className="text-2xl font-display font-700 text-white">
						{card.value}
					</p>

					{card.diff !== null && (
						<p
							className={`text-xs mt-1 font-mono ${(card.inverse ? card.diff < 0 : card.diff > 0)
									? "text-[#a3e635]"
									: card.diff === 0
										? "text-[#717171]"
										: "text-red-400"
								}`}
						>
							{card.diff > 0 ? "+" : ""}
							{card.diff.toFixed(1)}
							{card.unit} vs anterior
						</p>
					)}
				</div>
			))}
		</div>
	)
}