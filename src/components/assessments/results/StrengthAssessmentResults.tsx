import type { PhysicalAssessmentCompleteDto } from "../../../model/physicalAssessment"

import {
	asymmetryTones,
	dynamometerTones,
} from "../../../labels/classificationTone"

import {
	asymmetryLabels,
	dynamometerLabels,
} from "../../../labels/physicalAssessment"

import Result, {
	badge,
	textColors,
	value,
} from "./AssessmentResultParts"

type Props = {
	result: PhysicalAssessmentCompleteDto
}

export default function StrengthAssessmentResults({ result: r }: Props) {
	const a = r.physical_assessment_dto

	return (
		<>
			<Result label="Bíceps direito">
				{value(a.right_biceps_strength, " kg")}
			</Result>

			<Result label="Bíceps esquerdo">
				{value(a.left_biceps_strength, " kg")}
			</Result>

			<Result label="Tríceps direito">
				{value(a.right_triceps_strength, " kg")}
			</Result>

			<Result label="Tríceps esquerdo">
				{value(a.left_triceps_strength, " kg")}
			</Result>

			<Result label="Extensora direita">
				{value(a.right_extensor_strength, " kg")}
			</Result>

			<Result label="Extensora esquerda">
				{value(a.left_extensor_strength, " kg")}
			</Result>

			<Result label="Flexora direita">
				{value(a.right_flexor_strength, " kg")}
			</Result>

			<Result label="Flexora esquerda">
				{value(a.left_flexor_strength, " kg")}
			</Result>

			<Result
				label="Assimetria de bíceps"
				classification={badge(
					r.biceps_asymmetry_classification,
					asymmetryLabels,
					asymmetryTones,
				)}
			>
				<div
					className={
						r.biceps_asymmetry_classification != null
							? textColors[
							asymmetryTones[
							r.biceps_asymmetry_classification
							]
							]
							: undefined
					}
				>
					{value(r.biceps_asymmetry_percentage, " %")}
				</div>
			</Result>

			<Result
				label="Assimetria de tríceps"
				classification={badge(
					r.triceps_asymmetry_classification,
					asymmetryLabels,
					asymmetryTones,
				)}
			>
				<div
					className={
						r.triceps_asymmetry_classification != null
							? textColors[
							asymmetryTones[
							r.triceps_asymmetry_classification
							]
							]
							: undefined
					}
				>
					{value(r.triceps_asymmetry_percentage, " %")}
				</div>
			</Result>

			<Result
				label="Assimetria de extensor"
				classification={badge(
					r.extensor_asymmetry_classification,
					asymmetryLabels,
					asymmetryTones,
				)}
			>
				<div
					className={
						r.extensor_asymmetry_classification != null
							? textColors[
							asymmetryTones[
							r.extensor_asymmetry_classification
							]
							]
							: undefined
					}
				>
					{value(r.extensor_asymmetry_percentage, " %")}
				</div>
			</Result>

			<Result
				label="Assimetria de flexor"
				classification={badge(
					r.flexor_asymmetry_classification,
					asymmetryLabels,
					asymmetryTones,
				)}
			>
				<div
					className={
						r.flexor_asymmetry_classification != null
							? textColors[
							asymmetryTones[
							r.flexor_asymmetry_classification
							]
							]
							: undefined
					}
				>
					{value(r.flexor_asymmetry_percentage, " %")}
				</div>
			</Result>

			<Result
				label="Dinamômetro direito"
				classification={badge(
					r.right_dynamometer_strength_classification,
					dynamometerLabels,
					dynamometerTones,
				)}
			>
				<div
					className={
						r.flexor_asymmetry_classification != null
							? textColors[
							asymmetryTones[
							r.flexor_asymmetry_classification
							]
							]
							: undefined
					}
				>
					{value(a.right_handgrip_strength, " kgf")}
				</div>
			</Result>

			<Result
				label="Dinamômetro esquerdo"
				classification={badge(
					r.left_dynamometer_strength_classification,
					dynamometerLabels,
					dynamometerTones,
				)}
			>
				<div
					className={
						r.flexor_asymmetry_classification != null
							? textColors[
							asymmetryTones[
							r.flexor_asymmetry_classification
							]
							]
							: undefined
					}
				>
					{value(a.left_handgrip_strength, " kgf")}
				</div>
			</Result>
		</>
	)
}