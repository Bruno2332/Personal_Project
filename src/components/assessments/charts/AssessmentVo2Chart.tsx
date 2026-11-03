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
}


export default function AssessmentVo2Chart({ assessments }: Props) {
	const data = assessments.map((assessment) => ({
		date: new Date(
			`${assessment.data}T12:00:00`,
		).toLocaleDateString("pt-BR"),
		vo2: assessment.raw.vo2_parameter,
	}))

	const hasData = data.some((item) => item.vo2 != null)

	if (!hasData) {
		return (
			<div className="h-full flex items-center justify-center text-[#717171] text-sm">
				Nenhuma avaliação com resultado de VO₂.
			</div>
		)
	}

	const axisMax = Math.max(
		20,
		Math.ceil(
			Math.max(...data.map((item) => item.vo2 ?? 0)) / 5,
		) * 5,
	)

	const ticks = Array.from(
		{ length: (axisMax - 15) / 5 + 1 },
		(_, index) => 15 + index * 5,
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
					domain={[15, axisMax]}
					ticks={ticks}
					interval={0}
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

				<Line
					type="monotone"
					dataKey="vo2"
					name="VO₂ (ml/kg/min)"
					stroke="#22d3ee"
					strokeWidth={2}
					dot={{ fill: "#22d3ee", r: 4 }}
					activeDot={{ r: 6 }}
					connectNulls={false}
				/>
			</LineChart>
		</ResponsiveContainer>
	)
}