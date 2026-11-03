import {
	ResponsiveContainer,
	RadarChart,
	Radar,
	PolarGrid,
	PolarAngleAxis,
	Tooltip,
	Legend,
} from "recharts"
import type { AssessmentView } from "../../../utils/assessments"
import AssessmentChartTooltip from "./AssessmentChartTooltip"


type Props = {
	current: AssessmentView
	previous?: AssessmentView
}

export default function AssessmentRadarChart({
	current,
	previous,
}: Props) {
	const data = [
		{
			measurement: "Cintura",
			current: current.circunfencias.cintura,
			previous: previous?.circunfencias.cintura,
		},
		{
			measurement: "Quadril",
			current: current.circunfencias.quadril,
			previous: previous?.circunfencias.quadril,
		},
		{
			measurement: "Braço",
			current: current.circunfencias.braco,
			previous: previous?.circunfencias.braco,
		},
		{
			measurement: "Coxa",
			current: current.circunfencias.coxa,
			previous: previous?.circunfencias.coxa,
		},
		{
			measurement: "Panturrilha",
			current: current.circunfencias.panturrilha,
			previous: previous?.circunfencias.panturrilha,
		},
		{
			measurement: "Peitoral",
			current: current.circunfencias.peitoral,
			previous: previous?.circunfencias.peitoral,
		},
	]

	return (
		<ResponsiveContainer width="100%" height="100%">
			<RadarChart data={data}>
				<PolarGrid stroke="#252525" />

				<PolarAngleAxis
					dataKey="measurement"
					tick={{ fill: "#717171", fontSize: 11 }}
				/>

				<Radar
					name="Atual"
					dataKey="current"
					stroke="#a3e635"
					fill="#a3e635"
					fillOpacity={0.2}
					strokeWidth={2}
				/>

				{previous && (
					<Radar
						name="Anterior"
						dataKey="previous"
						stroke="#22d3ee"
						fill="#22d3ee"
						fillOpacity={0.15}
						strokeWidth={2}
					/>
				)}

				<Legend
					wrapperStyle={{
						fontSize: 11,
						color: "#717171",
					}}
				/>

				<Tooltip content={<AssessmentChartTooltip />} />
			</RadarChart>
		</ResponsiveContainer>
	)
}