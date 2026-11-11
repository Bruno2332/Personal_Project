import type { PhysicalAssessmentCompleteDto } from "../../../model/physicalAssessment"

import {
	bmiTones,
	pressureTones,
	fcTones,
} from "../../../labels/classificationTone"

import {
	bmiLabels,
	pressureLabels,
	fcLabels,
} from "../../../labels/physicalAssessment"

import { dateLabel } from "../../../utils/dates"

import Result, {
	badge,
	textColors,
	value,
} from "./AssessmentResultParts"

type Props = {
	result: PhysicalAssessmentCompleteDto
}

export default function GeneralAssessmentResults({ result: r }: Props) {
	const a = r.physical_assessment_dto

	return (
		<>
			<Result label="Data">
				{dateLabel(a.assessment_date)}
			</Result>

			<Result label="Idade">
				{r.age} anos
			</Result>

			<Result label="Peso">
				{value(a.weight_kg, " kg")}
			</Result>

			<Result label="Altura">
				{value(a.height_m, " m")}
			</Result>

			<Result
				label="IMC"
				classification={badge(
					r.bmi_classification,
					bmiLabels,
					bmiTones,
				)}
			>
				<div
					className={
						r.bmi_classification != null
							? textColors[bmiTones[r.bmi_classification]]
							: undefined
					}
				>
					{value(r.bmi)}
				</div>
			</Result>

			<Result
				label="Pressão arterial"
				classification={badge(
					r.hypertension_classification,
					pressureLabels,
					pressureTones,
				)}
			>
				<div
					className={
						r.hypertension_classification != null
							? textColors[
							pressureTones[r.hypertension_classification]
							]
							: undefined
					}
				>
					{a.systolic_pressure !== null &&
						a.diastolic_pressure !== null
						? `${a.systolic_pressure}/${a.diastolic_pressure}`
						: "—"}
				</div>
			</Result>

			<Result
				label="FC repouso"
				classification={badge(
					r.fc_heart_rest_classification,
					fcLabels,
					fcTones,
				)}
			>
				<div
					className={
						r.fc_heart_rest_classification != null
							? textColors[
							fcTones[r.fc_heart_rest_classification]
							]
							: undefined
					}
				>
					{value(a.resting_heart_rate_bpm, " bpm")}
				</div>
			</Result>
			
			<Result label="Objetivo final">
				<div>
					{value(r.ideal_weight_fat_percentage_min, " Kg a ")}
					{value(r.ideal_weight_fat_percentage_max, " Kg")}
				</div>

			</Result>


			<Result label="Objetivo atual">
				<div className="text-[#ff8c00]">
					{
						r.weight_to_lose_overweight != null
							? <div className="text-[#ff8c00]">
								{`- ${value(r.weight_to_lose_overweight)} Kg`}
							</div>
							: <p className="text-[#3697ff]">Sem gordura a perder</p>
					}
				</div>
			</Result>
			<Result label="Peso Alvo para sobrepeso">
				<div>
					{
						r.ideal_weight_fat_percentage_min != null
							? <div className="text-[#ff8c00]">
								{value(r.ideal_weight_fat_percentage_overweight, " Kg")}
							</div>
							: <p className="text-[#3697ff]">Peso Atual</p>
					}
				</div>
			</Result>

			<Result label="Peso Alvo para normalidade">
				<div>
					{value(r.ideal_weight_fat_percentage_health)}
				</div>
			</Result>
			
		</>
	)
}