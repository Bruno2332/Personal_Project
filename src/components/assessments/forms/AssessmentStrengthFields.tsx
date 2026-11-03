import { FormInput } from "../../ui/Fields"
import { strengthFields } from "./assessmentFields"


export default function AssessmentStrengthFields() {
	return (
		<div className="grid grid-cols-2 gap-4">
			{strengthFields.map((field) => {
				const isHandgrip =
					field.name === "right_handgrip_strength" ||
					field.name === "left_handgrip_strength"

				return (
					<FormInput
						key={field.name}
						name={`assessment.${field.name}`}
						label={field.label}
						type="number"
						mask={isHandgrip ? "decimal" : "integer"}
						decimalPlaces={1}
						maxLength={isHandgrip ? 5 : 3}
						min={isHandgrip ? 0.1 : 1}
						max={isHandgrip ? 200.0 : 200}
						step={isHandgrip ? 0.1 : 1}
						nullable
					/>
				)
			})}
		</div>
	)
}