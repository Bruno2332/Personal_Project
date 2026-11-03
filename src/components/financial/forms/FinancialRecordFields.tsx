import { useFormContext } from "react-hook-form"
import type { FinancialContractOptionDto } from "../../../model/financialRecord"
import {
	paymentLabels,
	transactionStatusLabels,
	transactionTypeLabels,
} from "../../../labels/financialRecord"
import { FormInput, FormSelect } from "../../ui/Fields"
import { Button } from "../../ui/Button"
import { ErrorMessage } from "../../ui/Feedback"
import type { FinancialFormValues } from "../hooks/useFinancialModal"

type Props = {
	contractOptions: FinancialContractOptionDto[]
	optionsLoading: boolean
	optionsError: string
	onRetryOptions: () => void
}

export default function FinancialRecordFields({
	contractOptions,
	optionsLoading,
	optionsError,
	onRetryOptions,
}: Props) {
	const { watch, setValue } = useFormContext<FinancialFormValues>()
	const income = watch("transaction_type") === "Income"

	return (
		<>
			<div className="flex rounded border border-[#252525] overflow-hidden">
				{(["Income", "Expense"] as const).map((type) => (
					<Button
						type="button"
						key={type}
						onClick={() =>
							setValue("transaction_type", type, {
								shouldDirty: true,
							})
						}
						className={`flex-1 py-2.5 text-sm font-medium transition-colors ${watch("transaction_type") === type
							? type === "Income"
								? "bg-[#a3e635] text-black"
								: "bg-red-500 text-white"
							: "text-[#717171] bg-[#0d0d0d]"
							}`}
					>
						{transactionTypeLabels[type]}
					</Button>
				))}
			</div>

			<FormInput
				name="description"
				label="Descrição *"
				required
				placeholder={
					income
						? "Ex: Avaliação física avulsa"
						: "Ex: Conta de energia de setembro"
				}
			/>

			<div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
				<FormInput
					name="amountUnits"
					label="Valor (R$) *"
					type="number"
					mask="currency"
					min={0.01}
					step="0.01"
					required
				/>

				<FormInput
					name="due_date"
					label="Vencimento"
					type="date"
					required
				/>

				<FormSelect
					name="status_transaction"
					label="Situação"
					required
					options={Object.entries(transactionStatusLabels).map(
						([value, label]) => ({ value, label }),
					)}
				/>

				<FormInput
					name="payment_date"
					label="Data do pagamento"
					type="date"
					placeholder="dd/mm/aaaa"
					nullable
					required={watch("status_transaction") === "Paid"}
				/>
				{income &&
					<FormSelect
						name="payment_method"
						label="Forma de pagamento"
						required
						options={Object.entries(paymentLabels).map(
							([value, label]) => ({ value, label }),
						)}
					/>

				}
			</div>
			{income &&
				<FormSelect
					name="contract_id"
					label="Contrato (opcional)"
					nullable
					numeric
					options={contractOptions}
				/>
			}
			<ErrorMessage message={optionsError} />

			{optionsLoading && (
				<p role="status">Carregando contratos...</p>
			)}

			{optionsError && (
				<Button type="button" onClick={onRetryOptions}>
					Tentar carregar contratos novamente
				</Button>
			)}
		</>
	)
}