import type { PhysicalAssessmentCompleteDto } from "../../../model/physicalAssessment"

import { fatTones } from "../../../labels/classificationTone"
import { fatLabels } from "../../../labels/physicalAssessment"
import { skinfoldFields } from "../forms/assessmentFields"

import Result, {
	badge,
	textColors,
	value,
} from "./AssessmentResultParts"

type Props = {
	result: PhysicalAssessmentCompleteDto
}

export default function CompositionAssessmentResults({ result: r }: Props) {
	const a = r.physical_assessment_dto

	return (
		<>
			<div className="col-span-full text-xs text-[#717171] uppercase tracking-wider">
				Pregas cutâneas
			</div>

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
							fatTones[
							r.fat_percentage_classification_skinfolds
							]
							]
							: undefined
					}
				>
					{value(r.fat_percentage_skinfolds, "%")}
				</div>
			</Result>

			<Result label="Massa gorda">
				<div className="text-[#3697ff]">
					{value(r.kilos_fat_skinfolds, " kg")}
				</div>
			</Result>

			<Result label="Massa magra">
				<div className="text-[#3697ff]">
					{value(r.kilos_lean_body_mass_skinfolds, " kg")}
				</div>
			</Result>

			<Result label="Percentual de massa magra">
				<div className="text-[#3697ff]">
					{value(r.lean_body_mass_percentage_skinfolds, "%")}
				</div>
			</Result>

			{skinfoldFields.map((f) => (
				<Result key={f.name} label={f.label}>
					{value(a[f.name] as number | null, " mm")}
				</Result>
			))}

			<div className="col-span-full text-xs text-[#717171] uppercase tracking-wider">
				Bioimpedância
			</div>

				<Result
					label="Percentual de gordura"
					classification={badge(
						r.fat_percentage_classification_bioimpedance,
						fatLabels,
						fatTones,
					)}
				>
					<div
						className={
							r.fat_percentage_classification_bioimpedance != null
								? textColors[
								fatTones[
								r.fat_percentage_classification_bioimpedance
								]
								]
								: undefined
						}
					>
						{value(a.bioimpedance_fat_percentage, "%")}
					</div>
				</Result>

			<Result label="Massa gorda">
				<div className="text-[#3697ff]">
					{value(r.kilos_fat_bioimpedance, " kg")}
				</div>
			</Result>

			<Result label="Massa magra">
				<div className="text-[#3697ff]">
					{value(r.kilos_lean_body_mass_bioimpedance, " kg")}
				</div>
			</Result>

			<Result label="Percentual de massa magra">
				<div className="text-[#3697ff]">
					{value(r.lean_body_mass_percentage_bioimpedance, "%")}
				</div>
			</Result>
		</>
	)
}