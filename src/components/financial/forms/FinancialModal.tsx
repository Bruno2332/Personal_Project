import { FormProvider } from "react-hook-form"
import type {
	FinancialRecordsDto,
	TransactionType,
} from "../../../model/financialRecord"
import { transactionTypeLabels } from "../../../labels/financialRecord"
import { Modal } from "../../ui/Modal"
import { Button } from "../../ui/Button"
import { ErrorMessage } from "../../ui/Feedback"
import FinancialRecordFields from "./FinancialRecordFields"
import useFinancialModal from "../hooks/useFinancialModal"

type Props = {
	record: FinancialRecordsDto | null
	kind: TransactionType
	onClose: () => void
	onSaved: (record: FinancialRecordsDto) => void
}


export default function FinancialModal({
	record,
	kind,
	onClose,
	onSaved,
}: Props) {
	const form = useFinancialModal({
		record,
		kind,
		onClose,
		onSaved,
	})

	const { methods } = form
	const transactionType = methods.watch("transaction_type")

	return (
		<Modal
			label="Lançamento financeiro"
			onClose={onClose}
			busy={methods.formState.isSubmitting}
			backdropClassName="bg-black/75 backdrop-blur-sm"
		>
			<div className="relative bg-[#111] border border-[#252525] rounded-lg w-full max-w-lg shadow-2xl mx-2 max-h-[92vh] overflow-y-auto">
				<div
					className={`h-1 w-full ${form.income ? "bg-[#a3e635]" : "bg-red-500"
						}`}
				/>

				<div className="border-b border-[#252525] px-6 py-4 flex items-center justify-between">
					<div>
						<p
							className={`text-[10px] uppercase tracking-[0.18em] ${form.income
									? "text-[#a3e635]"
									: "text-red-400"
								}`}
						>
							Lançamento financeiro
						</p>

						<h2 className="font-display text-2xl font-700 uppercase tracking-wide text-white">
							{record
								? "Editar lançamento"
								: form.income
									? "Nova entrada"
									: "Nova despesa"}
						</h2>
					</div>

					<Button
						type="button"
						disabled={methods.formState.isSubmitting}
						onClick={onClose}
						className="text-[#717171] hover:text-white text-xl transition-colors"
					>
						×
					</Button>
				</div>

				<FormProvider {...methods}>
					<form
						noValidate
						onSubmit={methods.handleSubmit(form.save)}
					>
						<div className="p-6 space-y-4">
							<FinancialRecordFields
								contractOptions={form.contractOptions}
								optionsLoading={form.optionsLoading}
								optionsError={form.optionsError}
								onRetryOptions={form.retryOptions}
							/>

							<ErrorMessage message={form.error} />
						</div>

						<div className="border-t border-[#252525] px-6 py-4 flex justify-end gap-3">
							<Button
								type="button"
								disabled={methods.formState.isSubmitting}
								onClick={onClose}
								className="px-4 py-2 text-sm text-[#717171] hover:text-white border border-[#252525] rounded transition-colors"
							>
								Cancelar
							</Button>

							<Button
								type="submit"
								disabled={
									methods.formState.isSubmitting ||
									form.optionsLoading ||
									!!form.optionsError
								}
								className={`px-5 py-2 text-sm font-semibold rounded transition-colors disabled:opacity-40 disabled:cursor-not-allowed ${form.income
										? "bg-[#a3e635] text-black hover:bg-[#bef264]"
										: "bg-red-500 text-white hover:bg-red-400"
									}`}
							>
								{methods.formState.isSubmitting
									? "Salvando..."
									: `Salvar ${transactionTypeLabels[
										transactionType
									].toLowerCase()
									}`}
							</Button>
						</div>
					</form>
				</FormProvider>
			</div>
		</Modal>
	)
}