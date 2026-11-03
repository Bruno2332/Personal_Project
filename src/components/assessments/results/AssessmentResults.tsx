import type { PhysicalAssessmentCompleteDto } from "../../../model/physicalAssessment"
import type { AssessmentTab } from "../forms/assessmentFields"

import GeneralAssessmentResults from "./GeneralAssessmentResults"
import CircumferenceAssessmentResults from "./CircumferenceAssessmentResults"
import CompositionAssessmentResults from "./CompositionAssessmentResults"
import StrengthAssessmentResults from "./StrengthAssessmentResults"
import Vo2AssessmentResults from "./Vo2AssessmentResults"
import PhotosAssessmentResults from "./PhotosAssessmentResults"

type Props = {
	result: PhysicalAssessmentCompleteDto
	tab: AssessmentTab
}

export default function AssessmentResults({ result, tab }: Props) {
	return (
		<div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-4">
			{tab === "general" && (
				<GeneralAssessmentResults result={result} />
			)}

			{tab === "circumferences" && (
				<CircumferenceAssessmentResults result={result} />
			)}

			{tab === "composition" && (
				<CompositionAssessmentResults result={result} />
			)}

			{tab === "strength" && (
				<StrengthAssessmentResults result={result} />
			)}

			{tab === "vo2" && (
				<Vo2AssessmentResults result={result} />
			)}

			{tab === "photos" && (
				<PhotosAssessmentResults result={result} />
			)}
		</div>
	)
}