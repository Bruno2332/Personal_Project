import { FormInput } from "../../ui/Fields"
import { circumferenceFields } from "./assessmentFields"



export default function AssessmentCircumferenceFields() {
	return (
		<div>
			<h4 className="text-base text-[#717171] uppercase tracking-wider font-medium mb-3">
				Circunferências (cm)
			</h4>

			<div className="grid grid-cols-2 sm:grid-cols-3 gap-4">
				{circumferenceFields.map((field) => (
					<FormInput
						key={field.name}
						name={`assessment.${field.name}`}
						label={field.label}
						type="number"
						mask="decimal"
						decimalPlaces={1}
						maxLength={5}
						min={0.1}
						max={300.0}
						step="0.1"
						nullable
					/>
				))}
			</div>
		</div>
	)
}
