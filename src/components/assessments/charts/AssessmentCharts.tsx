import { useState } from "react"
import type { AssessmentView } from "../../../utils/assessments"
import { Button } from "../../ui/Button"
import AssessmentSummaryCards from "../results/AssessmentSummaryCards"
import AssessmentEvolutionChart from "./AssessmentEvolutionChart"
import AssessmentCircumferencesChart from "./AssessmentCircumferencesChart"
import AssessmentRadarChart from "./AssessmentRadarChart"
import AssessmentVo2Chart from "./AssessmentVo2Chart"
import AssessmentPhotoComparison from "../photos/AssessmentPhotoComparison"


type Props = {
	assessments: AssessmentView[]
	pdfOnly?: boolean
}

type ChartTab =
	| "evolution"
	| "circumferences"
	| "radar"
	| "vo2"
	| "photo_comparison"

const chartTabs: { id: ChartTab; label: string }[] = [
	{ id: "evolution", label: "Evolução Geral" },
	{ id: "circumferences", label: "Circunferências" },
	{ id: "radar", label: "Radar Comparativo" },
	{ id: "vo2", label: "VO₂" },
	{ id: "photo_comparison", label: "Comparativo de Fotos" },
]

export default function AssessmentCharts({
	assessments,
	pdfOnly = false,
}: Props) {
	const [activeTab, setActiveTab] = useState<ChartTab>("evolution")

	const displayedTab = pdfOnly ? "evolution" : activeTab

	const orderedAssessments = [...assessments].sort(
		(a, b) =>
			a.data.localeCompare(b.data) ||
			(a.id ?? 0) - (b.id ?? 0),
	)

	const latest = orderedAssessments[orderedAssessments.length - 1]
	const previous = orderedAssessments[orderedAssessments.length - 2]

	if (!latest || (pdfOnly && assessments.length < 2)) {
		return null
	}

	return (
		<>
			{!pdfOnly && (
				<AssessmentSummaryCards
					current={latest}
					previous={previous}
				/>
			)}

			<div className="bg-[#161616] border border-[#252525] rounded overflow-hidden">
				{!pdfOnly && (
					<div className="flex overflow-x-auto border-b border-[#252525]">
						{chartTabs.map((tab) => (
							<Button
								key={tab.id}
								type="button"
								onClick={() => setActiveTab(tab.id)}
								className={`px-5 py-3 text-sm font-medium border-b-2 -mb-px transition-colors ${activeTab === tab.id
										? "border-[#a3e635] text-[#a3e635]"
										: "border-transparent text-[#717171] hover:text-white"
									}`}
							>
								{tab.label}
							</Button>
						))}
					</div>
				)}

				<div
					className="p-4"
					style={{
						height:
							displayedTab === "photo_comparison"
								? "auto"
								: 280,
					}}
				>
					{displayedTab === "evolution" && (
						<AssessmentEvolutionChart
							assessments={assessments}
							pdfOnly={pdfOnly}
						/>
					)}

					{displayedTab === "circumferences" && (
						<AssessmentCircumferencesChart
							assessments={assessments}
						/>
					)}

					{displayedTab === "radar" && (
						<AssessmentRadarChart
							current={latest}
							previous={previous}
						/>
					)}

					{displayedTab === "vo2" && (
						<AssessmentVo2Chart
							assessments={orderedAssessments}
						/>
					)}

					{displayedTab === "photo_comparison" && (
						<AssessmentPhotoComparison
							previous={previous}
							current={latest}
						/>
					)}
				</div>
			</div>
		</>
	)
}