import type { PhysicalAssessmentCompleteDto } from "../../../model/physicalAssessment"

import {
	abdomenTones,
	waistHipTones,
} from "../../../labels/classificationTone"

import {
	abdomenLabels,
	waistHipLabels,
} from "../../../labels/physicalAssessment"

import { circumferenceFields } from "../forms/assessmentFields"

import Result, {
	badge,
	textColors,
	value,
} from "./AssessmentResultParts"

type Props = {
	result: PhysicalAssessmentCompleteDto
}

export default function CircumferenceAssessmentResults({ result: r }: Props) {
	const a = r.physical_assessment_dto

	return (
		<>
			{circumferenceFields.map((f) => (
				<Result
					key={f.name}
					label={f.label}
					classification={
						f.name === "abdomen_circumference_cm"
							? badge(
								r.circumference_abdominal_classification,
								abdomenLabels,
								abdomenTones,
							)
							: undefined
					}
				>
					<div
						className={
							f.name === "abdomen_circumference_cm" &&
								r.circumference_abdominal_classification != null
								? textColors[
								abdomenTones[
								r.circumference_abdominal_classification
								]
								]
								: undefined
						}
					>
						{value(a[f.name] as number | null, " cm")}
					</div>
				</Result>
			))}

			<Result
				label="Relação cintura/quadril"
				classification={badge(
					r.waist_hip_classification,
					waistHipLabels,
					waistHipTones,
				)}
			>
				<div
					className={
						r.waist_hip_classification != null
							? textColors[
							waistHipTones[r.waist_hip_classification]
							]
							: undefined
					}
				>
					{value(r.waist_hip_calculate)}
				</div>
			</Result>
		</>
	)
}