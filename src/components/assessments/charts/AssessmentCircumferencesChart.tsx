import {
	ResponsiveContainer,
	BarChart,
	Bar,
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
}

const series = [
	{ key: "Cintura", color: "#a3e635" },
	{ key: "Quadril", color: "#22d3ee" },
	{ key: "Abdômen", color: "#f59e0b" },
	{ key: "Peitoral", color: "#a3e635" },
]

export default function AssessmentCircumferencesChart({
	assessments,
}: Props) {
	const data = assessments.slice(-3).map((assessment) => ({
		date: new Date(
			`${assessment.data}T12:00:00`,
		).toLocaleDateString("pt-BR", {
			day: "2-digit",
			month: "short",
		}),
		Cintura: assessment.circunfencias.cintura,
		Quadril: assessment.circunfencias.quadril,
		Abdômen: assessment.circunfencias.abdomen,
		Peitoral: assessment.circunfencias.peitoral,
	}))

	return (
		<ResponsiveContainer width="100%" height="100%">
			<BarChart data={data} barCategoryGap="30%">
				<CartesianGrid
					strokeDasharray="3 3"
					stroke="#252525"
				/>

				<XAxis
					dataKey="date"
					tick={{ fill: "#717171", fontSize: 11 }}
					axisLine={false}
					tickLine={false}
				/>

				<YAxis
					tick={{ fill: "#717171", fontSize: 11 }}
					axisLine={false}
					tickLine={false}
				/>

				<Tooltip content={<AssessmentChartTooltip />} />

				<Legend
					wrapperStyle={{
						fontSize: 11,
						color: "#717171",
					}}
				/>

				{series.map(({ key, color }) => (
					<Bar
						key={key}
						dataKey={key}
						fill={color}
						radius={[3, 3, 0, 0]}
						opacity={0.85}
					/>
				))}
			</BarChart>
		</ResponsiveContainer>
	)
}