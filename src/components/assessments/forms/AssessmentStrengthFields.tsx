import { FormInput } from "../../ui/Fields"
import { strengthFields } from "./assessmentFields"


export default function AssessmentStrengthFields() {
	return (
		<div className="grid grid-cols-2 gap-4">
			{strengthFields.map((field) => (
				<FormInput
					key={field.name}
					name={`assessment.${field.name}`}
					label={field.label}
					type="number"
					mask="decimal"
					decimalPlaces={1}
					maxLength={5}
					min={0.1}
					max={200.0}
					step={0.1}
					nullable
				/>
			))}
		</div>
	)
}