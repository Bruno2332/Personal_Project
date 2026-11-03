import {
	ResponsiveContainer,
	LineChart,
	Line,
	XAxis,
	YAxis,
	CartesianGrid,
	Tooltip,
	Legend,
} from "recharts"
import type { AssessmentView } from "../../../utils/assessments"
import AssessmentChartTooltip from "./AssessmentChartTooltip"


type Props = {
	assessments: AssessmentView[]
	pdfOnly?: boolean
}

const series = [
	{ key: "Peso", color: "#a3e635" },
	{ key: "Gordura %", color: "#22d3ee" },
	{ key: "Massa Magra %", color: "#f59e0b" },
]


export default function AssessmentEvolutionChart({
	assessments,
	pdfOnly = false,
}: Props) {
	const data = assessments.map((assessment) => ({
		date: new Date(
			`${assessment.data}T12:00:00`,
		).toLocaleDateString("pt-BR", {
			day: "2-digit",
			month: pdfOnly ? "2-digit" : "short",
			...(pdfOnly ? { year: "numeric" as const } : {}),
		}),
		Peso: assessment.peso,
		"Gordura %": assessment.raw.fat_percentage_skinfolds,
		"Massa Magra %": assessment.raw.lean_body_mass_percentage_skinfolds,
	}))

	if (data.length < 2) {
		return (
			<div className="h-full flex items-center justify-center text-[#717171] text-sm">
				Registre ao menos 2 avaliações para ver a evolução
			</div>
		)
	}

	const yValues = data
		.flatMap((item) => [
			item.Peso,
			item["Gordura %"],
			item["Massa Magra %"],
		])
		.filter(
			(value): value is number =>
				typeof value === "number" && Number.isFinite(value),
		)

	const minTick = Math.floor(Math.min(...yValues) / 10) * 10
	const maxTick = Math.ceil(Math.max(...yValues) / 10) * 10
	const yMax = maxTick === minTick ? minTick + 10 : maxTick
	const yTicks = Array.from(
		{ length: (yMax - minTick) / 10 + 1 },
		(_, index) => minTick + index * 10,
	)

	return (
		<ResponsiveContainer width="100%" height="100%">
			<LineChart data={data}>
				<CartesianGrid
					strokeDasharray="3 3"
					stroke="#252525"
				/>

				<XAxis
					dataKey="date"
					tick={{ fill: "#717171", fontSize: 11 }}
					axisLine={false}
					tickLine={false}
					padding={{ left: 25, right: 25 }}
				/>

				<YAxis
					domain={[minTick, yMax]}
					ticks={yTicks}
					interval={0}
					tick={{ fill: "#717171", fontSize: 11 }}
					axisLine={false}
					tickLine={false}
				/>

				{!pdfOnly && (
					<Tooltip content={<AssessmentChartTooltip />} />
				)}

				<Legend
					wrapperStyle={{
						fontSize: 11,
						color: "#717171",
					}}
				/>

				{series.map(({ key, color }) => (
					<Line
						key={key}
						type="monotone"
						dataKey={key}
						stroke={color}
						strokeWidth={2}
						dot={{ fill: color, r: 4 }}
						activeDot={{ r: 6 }}
						isAnimationActive={!pdfOnly}
					/>
				))}
			</LineChart>
		</ResponsiveContainer>
	)
}