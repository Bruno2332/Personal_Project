import { useFormContext, useWatch } from "react-hook-form"
import type { AgendaDto } from "../../../model/agenda"
import {
	appointmentTypeLabels,
	weekDayLabels,
} from "../../../labels/agenda"
import { FormInput, FormSelect, FormTextArea } from "../../ui/Fields"
import { agendaInputClass } from "../../ui/FormField"
import { Button } from "../../ui/Button"
import { ErrorMessage } from "../../ui/Feedback"



type Props = {
	studentOptions: { value: number; label: string }[]
	loadingStudents: boolean
	studentsError: string
	onRetryStudents: () => void
}


export default function AppointmentFields({
	studentOptions,
	loadingStudents,
	studentsError,
	onRetryStudents,
}: Props) {
	const { control, setValue } = useFormContext<AgendaDto>()

	const appointmentType = useWatch({
		control,
		name: "appointment_type",
	})

	const isRecurring = useWatch({
		control,
		name: "is_recurring",
	})

	return (
		<div className="space-y-5">
			<div className="grid grid-cols-2 gap-2 p-1 bg-[#0d0d0d] border border-[#252525] rounded">
				{(["Class", "Assessment"] as const).map((type) => (
					<Button
						key={type}
						type="button"
						aria-pressed={appointmentType === type}
						onClick={() =>
							setValue("appointment_type", type, {
								shouldDirty: true,
							})
						}
						className={`p-3 rounded text-left transition-colors ${appointmentType === type
							? type === "Class"
								? "bg-[#a3e635]/15 border border-[#a3e635]/30"
								: "bg-[#22d3ee]/15 border border-[#22d3ee]/30"
							: "border border-transparent"
							}`}
					>
						<span
							className={`block text-sm font-semibold ${appointmentType === type
								? type === "Class"
									? "text-[#a3e635]"
									: "text-[#22d3ee]"
								: "text-[#d0d0d0]"
								}`}
						>
							{appointmentTypeLabels[type]}
						</span>
					</Button>
				))}
			</div>

			<FormSelect
				inputStyle={agendaInputClass}
				name="student_id"
				label="Aluno"
				required
				numeric
				options={studentOptions}
			/>

			{loadingStudents && (
				<p role="status" className="text-xs text-[#717171]">
					Carregando alunos...
				</p>
			)}

			{studentsError && (
				<div className="space-y-2">
					<ErrorMessage message={studentsError} />

					<Button
						type="button"
						onClick={onRetryStudents}
						disabled={loadingStudents}
						className="text-xs text-[#a3e635]"
					>
						Tentar novamente
					</Button>
				</div>
			)}
			{appointmentType === "Class" &&
				<>
					<fieldset className="space-y-2">
						<legend className="text-xs text-[#717171] uppercase tracking-wider font-medium">
							Repetição
						</legend>

						<div className="grid grid-cols-2 gap-2">
							{[
								{ value: true, label: "Toda semana" },
								{ value: false, label: "Somente nesta data" },
							].map((option) => (
								<Button
									key={String(option.value)}
									type="button"
									aria-pressed={isRecurring === option.value}
									onClick={() =>
										setValue("is_recurring", option.value, {
											shouldDirty: true,
										})
									}
									className={`px-3 py-2 text-sm rounded border transition-colors ${isRecurring === option.value
										? "border-[#a3e635]/30 bg-[#a3e635]/10 text-[#a3e635]"
										: "border-[#252525] text-[#717171] hover:text-white"
										}`}
								>
									{option.label}
								</Button>
							))}
						</div>
					</fieldset>

					<div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
						{isRecurring ? (
							<>
								<FormInput
									inputStyle={agendaInputClass}
									name="recurrence_start_date"
									label="Começar a partir de"
									type="date"
									required
									nullable
								/>

								<FormSelect
									inputStyle={agendaInputClass}
									name="week_day"
									label="Dia da semana"
									required
									options={Object.entries(weekDayLabels).map(
										([value, label]) => ({ value, label }),
									)}
								/>

								<FormInput
									inputStyle={agendaInputClass}
									name="recurrence_end_date"
									label="Fim da recorrência (opcional)"
									type="date"
									nullable
								/>
							</>
						) : (
							<FormInput
								inputStyle={agendaInputClass}
								name="scheduled_date"
								label="Data"
								type="date"
								required
								nullable
							/>
						)}
					</div>
				</>
			}
			<div className="grid grid-cols-2 gap-4">
				<FormInput
					inputStyle={agendaInputClass}
					name="start_time"
					label="Horário"
					type="time"
					required
				/>

				<FormInput
					inputStyle={agendaInputClass}
					name="end_time"
					label="Término"
					type="time"
					required
				/>
			</div>

			<FormTextArea
				inputStyle={agendaInputClass}
				name="notes"
				label="Observações"
				placeholder="Notas sobre o compromisso..."
			/>
		</div>
	)
}