import { FormProvider } from "react-hook-form"

import type { StudentDto } from "../../../model/student"
import ContractPanel from "../../contracts/forms/ContractPanel"
import { Button } from "../../ui/Button"
import { ErrorMessage } from "../../ui/Feedback"
import { Modal } from "../../ui/Modal"
import useStudentForm from "../hooks/useStudentForm"
import StudentFields from "./StudentFields"
import { useEffect } from "react"

type Props = {
	student: StudentDto | null
	onClose: () => void
}

export default function StudentModal({ student, onClose }: Props) {
	const form = useStudentForm(student)
	const { methods } = form

	useEffect(() => {
		if (form.tab !== "personal") return

		const frame = window.requestAnimationFrame(() => {
			methods.setFocus("name")
		})

		return () => window.cancelAnimationFrame(frame)
	}, [form.tab, methods.setFocus])

	return (
		<Modal
			label="Aluno"
			onClose={onClose}
			busy={methods.formState.isSubmitting}
			backdropClassName="bg-black/70 backdrop-blur-sm"
		>
			<div className="relative bg-[#111] border border-[#252525] rounded-lg w-full max-w-3xl max-h-[90vh] overflow-y-auto shadow-2xl mx-2">
				<div className="sticky top-0 bg-[#111] border-b border-[#252525] px-6 py-4 flex items-center justify-between z-10">
					<h2 className="font-display text-2xl font-700 uppercase tracking-wide text-white">
						{form.saved ? form.name : "Novo Aluno"}
					</h2>

					<div className="flex items-center gap-3">
						{form.saved && (
							<Button
								type="button"
								onClick={() =>
									methods.setValue("is_active", !form.active, {
										shouldDirty: true,
									})
								}
								className={`text-xs px-3 py-1 rounded-full border transition-colors ${form.active
										? "border-[#a3e635] text-[#a3e635]"
										: "border-[#717171] text-[#717171]"
									}`}
							>
								{form.active ? "Ativo" : "Inativo"}
							</Button>
						)}

						<Button
							type="button"
							disabled={methods.formState.isSubmitting}
							onClick={onClose}
							aria-label="Fechar"
							className="text-[#717171] hover:text-white text-xl leading-none transition-colors"
						>
							×
						</Button>
					</div>
				</div>

				<div className="flex border-b border-[#252525] px-6">
					{[
						{ id: "personal", label: "Dados Pessoais" },
						{ id: "contract", label: "Contrato & Parcelas" },
					].map((item) => (
						<Button
							key={item.id}
							type="button"
							disabled={methods.formState.isSubmitting}
							onClick={() => form.setTab(item.id as typeof form.tab)}
							className={`py-3 px-4 text-sm font-medium border-b-2 -mb-px transition-colors ${form.tab === item.id
									? "border-[#a3e635] text-[#a3e635]"
									: "border-transparent text-[#717171] hover:text-white"
								}`}
						>
							{item.label}
						</Button>
					))}
				</div>

				{form.tab === "personal" ? (
					<FormProvider {...methods}>
						<form noValidate onSubmit={methods.handleSubmit(form.save)}>
							<div className="p-6">
								<StudentFields />
							</div>

							{form.error && (
								<div className="mx-6 mb-4">
									<ErrorMessage message={form.error} />
								</div>
							)}

							<div className="sticky bottom-0 bg-[#111] border-t border-[#252525] px-6 py-4 flex justify-end gap-3">
								<Button
									type="button"
									onClick={onClose}
									disabled={methods.formState.isSubmitting}
									className="px-4 py-2 text-sm text-[#717171] hover:text-white border border-[#252525] rounded transition-colors"
								>
									Cancelar
								</Button>
								<Button
									type="submit"
									disabled={methods.formState.isSubmitting}
									className="px-5 py-2 text-sm bg-[#a3e635] text-black font-semibold rounded hover:bg-[#bef264] transition-colors"
								>
									{methods.formState.isSubmitting
										? "Salvando..."
										: form.saved
											? "Salvar Alterações"
											: "Cadastrar Aluno"}
								</Button>
							</div>
						</form>
					</FormProvider>
				) : form.saved?.id != null ? (
					<ContractPanel studentId={form.saved.id} onClose={onClose} />
				) : (
					<div className="p-6 text-sm text-[#717171]">
						Salve os dados pessoais para cadastrar o contrato.
					</div>
				)}
			</div>
		</Modal>
	)
}
