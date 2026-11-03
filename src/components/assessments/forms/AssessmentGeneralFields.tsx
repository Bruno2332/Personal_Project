import { FormInput, FormTextArea } from "../../ui/Fields"



export default function AssessmentGeneralFields() {
	return (
		<div className="space-y-4">
			<div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
				<FormInput
					name="assessment.weight_kg"
					label="Peso (kg)"
					type="number"
					mask="decimal"
					decimalPlaces={1}
					maxLength={5}
					min={0.1}
					max={300.0}
					step="0.1"
					required
				/>

				<FormInput
					name="assessment.height_m"
					label="Altura (m)"
					type="number"
					mask="decimal"
					decimalPlaces={2}
					maxLength={4}
					min={0.5}
					max={2.50}
					step="0.01"
					required
				/>

				<FormInput
					name="assessment.systolic_pressure"
					label="Pressão sistólica"
					type="number"
					maxLength={3}
					min={1}
					max={200}
					step={1}
					nullable
				/>

				<FormInput
					name="assessment.diastolic_pressure"
					label="Pressão diastólica"
					type="number"
					maxLength={3}
					min={1}
					max={200}
					step={1}
					nullable
				/>

				<FormInput
					name="assessment.resting_heart_rate_bpm"
					label="FC Repouso (bpm)"
					type="number"
					maxLength={3}
					min={1}
					max={250}
					step={1}
					nullable
				/>
			</div>

			<FormTextArea
				name="assessment.notes"
				label="Anamnese"
				nullable
				placeholder="Ex: possui lesão, etc..."
			/>
		</div>
	)
}