import type { PhysicalAssessmentCompleteDto } from "../../../model/physicalAssessment"

import {
	fcExerciseTones,
	vo2Tones,
	perceivedExertionTones,
} from "../../../labels/classificationTone"

import {
	vo2Labels,
	perceivedExertionLabels,
} from "../../../labels/physicalAssessment"

import { Table, TableHeader } from "../../ui/Table"

import Result, {
	badge,
	textColors,
	value,
} from "./AssessmentResultParts"

type Props = {
	result: PhysicalAssessmentCompleteDto
}

export default function Vo2AssessmentResults({ result: r }: Props) {
	const a = r.physical_assessment_dto

	return (
		<>
			<div className="col-span-full overflow-x-auto">
				<Table className="w-full text-sm">
					<TableHeader
						columns={[
							"Estágio",
							"Velocidade (km/h)",
							"FC (bpm) / (%) FC Máxima",
							"Percepção de Esforço",
						]}
					/>

					<tbody>
						{r.vo2_assessments.map((v, i) => (
							<tr
								key={v.vo2_assessment_dto.id ?? i}
								className="border-b border-[#252525]/50"
							>
								<td className="px-4 py-2 font-mono">
									{i + 1}
								</td>

								<td className="px-4 py-2 font-mono">
									{value(
										v.vo2_assessment_dto.treadmill_speed_kmh,
									)}
								</td>

								<td className="px-4 py-2 font-mono">
									{value(
										v.vo2_assessment_dto.heart_rate_bpm,
										" bpm",
									)}
									{" / "}
									<span
										className={
											v.fc_heart_classification != null
												? textColors[
												fcExerciseTones[
												v.fc_heart_classification
												]
												]
												: undefined
										}
									>
										{value(v.fc_heart_percentage, " %")}
									</span>
								</td>

								<td className="px-4 py-2 font-mono">
									{badge(
										v.vo2_assessment_dto.perceived_exertion,
										perceivedExertionLabels,
										perceivedExertionTones,
									)}
								</td>
							</tr>
						))}
					</tbody>
				</Table>

				{!r.vo2_assessments.length && (
					<p className="text-sm text-[#717171] py-4">
						VO₂ não realizado nesta avaliação.
					</p>
				)}
			</div>

			<Result label="Segundos do último estágio">
				<div className="text-[#3697ff]">
					{value(a.vo2_final_stage_seconds)}
				</div>
			</Result>

			<Result
				label="VO₂ máximo (mL/kg/min)"
				classification={badge(
					r.vo2_classification,
					vo2Labels,
					vo2Tones,
				)}
			>
				<div
					className={
						r.vo2_classification != null
							? textColors[vo2Tones[r.vo2_classification]]
							: undefined
					}
				>
					{value(r.vo2_parameter)}
				</div>
			</Result>
			<Result
				label="Frequência cardíaca máxima"
			>
				<div className="text-[#3697ff]">
					{value(r.maximum_heart_rate, " bpm")}
				</div>
			</Result>
		</>
	)
}