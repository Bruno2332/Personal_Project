import { FormProvider } from "react-hook-form"
import type { AgendaDto } from "../../../model/agenda"
import { Modal } from "../../ui/Modal"
import { Button } from "../../ui/Button"
import { ErrorMessage } from "../../ui/Feedback"
import AppointmentFields from "./AppointmentFields"
import useAppointmentForm from "../hooks/useAppointmentForm"

type Props = {
	initial: AgendaDto
	onClose: () => void
	onSaved: () => Promise<void>
}

export default function AppointmentForm({
	initial,
	onClose,
	onSaved,
}: Props) {
	const form = useAppointmentForm({
		initial,
		onClose,
		onSaved,
	})

	return (
		<Modal
			label="Horário"
			onClose={onClose}
			busy={form.isSubmitting}
			backdropClassName="bg-black/75 backdrop-blur-sm"
		>
			<div className="relative bg-[#111] border border-[#252525] rounded-lg w-full max-w-xl max-h-[92vh] overflow-y-auto shadow-2xl">
				<div className="sticky top-0 z-10 flex items-center justify-between px-6 py-4 bg-[#111] border-b border-[#252525]">
					<div>
						<h2 className="font-display text-2xl font-700 uppercase tracking-wide text-white">
							{form.savedId === null
								? "Novo Horário"
								: "Editar Horário"}
						</h2>

						<p className="text-xs text-[#717171]">
							Defina se o compromisso se repete ou acontece uma única vez
						</p>
					</div>

					<Button
						type="button"
						disabled={form.isSubmitting}
						onClick={onClose}
						aria-label="Fechar"
						className="text-[#717171] hover:text-white text-xl"
					>
						×
					</Button>
				</div>

				<FormProvider {...form.methods}>
					<form
						noValidate
						onSubmit={form.methods.handleSubmit(form.save)}
					>
						<fieldset
							disabled={form.isSubmitting}
							className="min-w-0 p-6 space-y-5"
						>
							<AppointmentFields
								studentOptions={form.studentOptions}
								loadingStudents={form.loadingStudents}
								studentsError={form.studentsError}
								onRetryStudents={form.retryStudents}
							/>

							<ErrorMessage message={form.error} />
						</fieldset>

						<div className="sticky bottom-0 flex justify-end gap-3 px-6 py-4 bg-[#111] border-t border-[#252525]">
							<Button
								type="button"
								onClick={onClose}
								disabled={form.isSubmitting}
								className="px-4 py-2 text-sm text-[#717171] hover:text-white border border-[#252525] rounded"
							>
								Cancelar
							</Button>

							<Button
								type="submit"
								disabled={
									form.isSubmitting ||
									form.loadingStudents ||
									!!form.studentsError
								}
								className={`px-5 py-2 text-sm text-black font-semibold rounded transition-colors ${form.appointmentType === "Class"
										? "bg-[#a3e635] hover:bg-[#bef264]"
										: "bg-[#22d3ee] hover:bg-[#67e8f9]"
									}`}
							>
								{form.isSubmitting
									? "Salvando..."
									: "Salvar horário"}
							</Button>
						</div>
					</form>
				</FormProvider>
			</div>
		</Modal>
	)
}