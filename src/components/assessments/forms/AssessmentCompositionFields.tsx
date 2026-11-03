import { FormInput } from "../../ui/Fields"
import { skinfoldFields } from "./assessmentFields"

export default function AssessmentCompositionFields() {
	return (
		<div className="space-y-4">
			<h4 className="text-base text-[#717171] uppercase tracking-wider font-medium mb-3">
				Pregas cutâneas (mm)
			</h4>

			<div className="grid grid-cols-2 gap-4">
				{skinfoldFields.map((field) => (
					<FormInput
						key={field.name}
						name={`assessment.${field.name}`}
						label={field.label}
						type="number"
						mask="decimal"
						decimalPlaces={1}
						maxLength={3}
						min={0.1}
						max={99.9}
						step="0.1"
						nullable
					/>
				))}

				<FormInput
					name="assessment.bioimpedance_fat_percentage"
					label="(%) Gordura por bioimpedância"
					type="number"
					mask="decimal"
					decimalPlaces={1}
					maxLength={4}
					min={0.1}
					max={100.0}
					step="0.1"
					nullable
				/>
			</div>
		</div>
	)
}
