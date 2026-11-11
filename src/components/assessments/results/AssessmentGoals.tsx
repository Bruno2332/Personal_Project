import type { PhysicalAssessmentCompleteDto } from "../../../model/physicalAssessment"
import { fatTones } from "../../../labels/classificationTone"
import { fatLabels } from "../../../labels/physicalAssessment"
import Result, {
	badge,
	textColors,
	value,
} from "./AssessmentResultParts"

import {
	abdomenTones,
	waistHipTones,
} from "../../../labels/classificationTone"

import {
	abdomenLabels,
	waistHipLabels,
} from "../../../labels/physicalAssessment"

type Props = {
	result: PhysicalAssessmentCompleteDto
}

export default function AssessmentGoals({ result: r }: Props) {
	const idealMin = r.ideal_weight_fat_percentage_min
	const idealMax = r.ideal_weight_fat_percentage_max

	const lossMin = r.weight_to_lose_min_percentage
	const lossMax = r.weight_to_lose_max_percentage
	const lossHealth = r.weight_to_lose_health
	const lossOverweight = r.weight_to_lose_overweight

	const hasFatData =
		idealMin != null &&
		idealMax != null &&
		lossMin != null &&
		lossMax != null &&
		lossHealth != null &&
		lossOverweight != null

	const nextGoal = !hasFatData
		? null
		: lossMin < 0
			? {
				label: "Alcançar o percentual mínimo de referência",
				amount: -lossMin,
				gain: true,
			}
			: lossOverweight > 0
				? {
					label: "Alcançar o limite para sobrepeso",
					amount: lossOverweight,
					gain: false,
				}
				: lossHealth > 0
					? {
						label: "Alcançar o limite para normalidade",
						amount: lossHealth,
						gain: false,
					}
					: lossMax > 0
						? {
							label: "Entrar na faixa de referência",
							amount: lossMax,
							gain: false,
						}
						: null

	const abdomen = r.physical_assessment_dto.abdomen_circumference_cm
	const abdomenHigh = r.circumference_abdominal_high_risk
	const abdomenVeryHigh = r.circumference_abdominal_very_high_risk

	const hasAbdomenData =
		abdomen != null &&
		abdomenHigh != null &&
		abdomenVeryHigh != null

	const abdomenTarget = hasAbdomenData
		? abdomen >= abdomenVeryHigh
			? abdomenVeryHigh
			: abdomen >= abdomenHigh
				? abdomenHigh
				: null
		: null

	const waistHip = r.waist_hip_calculate
	const waistHipModerate = r.waist_hip_moderate_risk
	const waistHipHigh = r.waist_hip_high_risk
	const waistHipVeryHigh = r.waist_hip_very_high_risk

	const hasWaistHipData =
		waistHip != null &&
		waistHipModerate != null &&
		waistHipHigh != null &&
		waistHipVeryHigh != null

	const waistHipTarget = hasWaistHipData
		? waistHip >= waistHipVeryHigh
			? waistHipVeryHigh
			: waistHip >= waistHipHigh
				? waistHipHigh
				: waistHip >= waistHipModerate
					? waistHipModerate
					: null
		: null

	const ratioValue = (ratio: number) =>
		ratio.toLocaleString("pt-BR", {
			minimumFractionDigits: 2,
			maximumFractionDigits: 3,
		})

	const titleClass =
		"col-span-full border-b border-[#717171] pb-2 text-base font-semibold text-[#717171]"

	return (
		<>

			<h3 className={titleClass}>
				Meta para percentual de gordura
			</h3>

			{hasFatData ? (
				<>
					<Result
						label="Percentual de gordura"
						classification={badge(
							r.fat_percentage_classification_skinfolds,
							fatLabels,
							fatTones,
						)}
					>
						<div
							className={
								r.fat_percentage_classification_skinfolds != null
									? textColors[
									fatTones[r.fat_percentage_classification_skinfolds]
									]
									: undefined
							}
						>
							{r.fat_percentage_skinfolds != null
								? `${r.fat_percentage_skinfolds.toLocaleString("pt-BR", {
									minimumFractionDigits: 2,
									maximumFractionDigits: 2,
								})}%`
								: "—"}
						</div>
					</Result>
					<Result label="Próxima meta">
						<div>
							{nextGoal?.label ?? "Faixa de referência atingida"}
						</div>
					</Result>

					{nextGoal && (
						<Result label="Ajuste de gordura para a próxima meta">
							<div className="text-[#ff8c00]">
								{`${nextGoal.gain ? "Ganhar" : "Reduzir"} ${value(nextGoal.amount, " kg")}`}
							</div>
						</Result>
					)}

					{lossOverweight > 0 && (
						<Result label="Faixa de Sobrepeso">
							<div className="text-yellow-400">
								{`${value(r.ideal_weight_fat_percentage_health)} kg a ${value(r.ideal_weight_fat_percentage_overweight, " kg")}`}
							</div>
							<div className="text-yellow-400">
								{`${value(r.ideal_fat_percentage_health)} % a ${value(r.ideal_fat_percentage_overweight)} % de gordura`}
							</div>
						</Result>
					)}

					{lossHealth > 0 && (
						<Result label="Faixa de Normalidade">
							<div className="text-[#a3e635]">
								{`${value(idealMax)} kg a ${value(r.ideal_weight_fat_percentage_health, " kg")}`}
							</div>
							<div className="text-[#a3e635]">
								{`${value(r.ideal_fat_percentage_max)} % a ${value(r.ideal_fat_percentage_health)} % de gordura`}
							</div>
						</Result>
					)}

					{lossOverweight > 0 && (
						<Result label="Ajuste de gordura para a normalidade">
							<div className="text-[#a3e635]">
								{`Reduzir ${value(lossHealth, " kg")}`}
							</div>
						</Result>
					)}

					<Result label="Faixa de referência">
						<div className="text-[#a3e635]">
							{value(idealMin, " kg")}
							{" a "}
							{value(idealMax, " kg")}
						</div>
						<div className="text-[#3697ff]">
							{`${value(r.ideal_fat_percentage_min)} % a ${value(r.ideal_fat_percentage_max)} % de gordura`}
						</div>
					</Result>

					{lossMax > 0 && (
						<Result label="Ajuste de gordura para a faixa de referência">
							<div className="text-[#3697ff]">
								{`Reduzir ${value(lossMax)} kg a ${value(lossMin)} kg`}
							</div>
						</Result>
					)}
				</>
			) : (
				<p className="col-span-full text-sm text-[#717171] py-4">
					Não há dados suficientes para calcular os objetivos
					de percentual de gordura.
				</p>
			)}

			<h3 className={`${titleClass} mt-4`}>
				Meta para circunferência de abdômen
			</h3>

			{hasAbdomenData ? (
				<>
					<Result
						label="Circunferência Atual"
						classification={
							badge(
								r.circumference_abdominal_classification,
								abdomenLabels,
								abdomenTones,
							)

						}
					>
						<div
							className={
								r.circumference_abdominal_classification != null
									? textColors[
									abdomenTones[r.circumference_abdominal_classification]
									]
									: undefined

							}
						>
							{value(abdomen, " cm")}
						</div>
					</Result>

					<Result label="Próxima meta">
						<div>
							{abdomenTarget != null
								? `Ficar abaixo de ${value(abdomenTarget, " cm")}`
								: "Faixa de baixo risco atingida"}
						</div>
					</Result>

					
					<Result label="Referência para baixo risco">
						<div className="text-[#3697ff]">
							{`Abaixo de ${value(abdomenHigh, " cm")}`}
						</div>
					</Result>

					
				</>
			) : (
				<p className="col-span-full text-sm text-[#717171] py-4">
					Não há dados suficientes para calcular os objetivos
					de circunferência abdominal.
				</p>
			)}

			<h3 className={`${titleClass} mt-4`}>
				Meta para relação cintura-quadril
			</h3>

			{hasWaistHipData ? (
				<>
					<Result
						label="Relação cintura-quadril atual"
						classification={badge(
							r.waist_hip_classification,
							waistHipLabels,
							waistHipTones,
						)}
					>
						<div
							className={
								r.waist_hip_classification != null
									? textColors[waistHipTones[r.waist_hip_classification]]
									: undefined
							}
						>
							{waistHip != null
								? waistHip.toLocaleString("pt-BR", {
									minimumFractionDigits: 2,
									maximumFractionDigits: 2,
								})
								: "—"}
						</div>
					</Result>

					<Result label="Próxima meta">
						<div>
							{waistHipTarget != null
								? `Ficar abaixo de ${ratioValue(waistHipTarget)}`
								: "Faixa de baixo risco atingida"}
						</div>
					</Result>

					

					<Result label="Referência para baixo risco">
						<div className="text-[#3697ff]">
							{`Abaixo de ${ratioValue(waistHipModerate)}`}
						</div>
					</Result>

					
				</>
			) : (
				<p className="col-span-full text-sm text-[#717171] py-4">
					Não há dados suficientes para calcular os objetivos
					de relação cintura-quadril.
				</p>
			)}
		</>
	)
}