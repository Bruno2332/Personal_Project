import { useEffect } from "react"
import { useFormContext } from "react-hook-form"
import type { ContractDto } from "../../../model/contract"
import {
	billingLabels,
	contractStatusLabels,
	serviceLabels,
} from "../../../labels/contract"
import { paymentLabels } from "../../../labels/financialRecord"
import { FormInput, FormSelect } from "../../ui/Fields"

const options = (labels: Record<string, string>) =>
	Object.entries(labels).map(([value, label]) => ({ value, label }))

export default function ContractFields({
	assessmentOnly = false,
}: {
	assessmentOnly?: boolean
}) {
	const { watch, setValue } = useFormContext<
		ContractDto & { amount_reais: number }
	>()
	const serviceType = watch("service_type")
	const billingType = watch("billing_type")
	const paymentMethod = watch("payment_method")
	const isAssessment = assessmentOnly || serviceType === "PhysicalAssessment"
	const isMonthly = !isAssessment && billingType === "Monthly"
	const isCreditCard = paymentMethod === "CreditCard"


	useEffect(() => {
		if (isMonthly) {
			setValue("installment_count", 1, { shouldValidate: true })
		}
	}, [isMonthly, setValue])

	return (
		<div className="grid grid-cols-1 sm:grid-cols-3 gap-4">

			{!assessmentOnly && (
				<FormSelect
					name="service_type"
					label="Serviço"
					options={options(serviceLabels)}
					required
				/>
			)}
			{!isAssessment && (
				<FormSelect
					name="billing_type"
					label="Tipo de Faturamento"
					options={options(billingLabels)}
					required
				/>

			)}
			{!isMonthly &&
				<FormInput
					name="duration_months"
					label="Duração em meses"
					type="number"
					min={1}
					max={99}
					maxLength={2}
					placeholder="Duração"
					nullable
				/>
			}
			<FormInput
				name="amount_reais"
				label={isMonthly ? "Valor Mensal (R$)" : "Valor Total (R$)"}
				type="number"
				mask="currency"
				min={0.01}
				step="0.01"
				required
			/>

			<FormSelect
				name="payment_method"
				label="Forma de Pagamento"
				options={options(paymentLabels)}
				required
			/>
			{!isMonthly &&
				<FormSelect
					name="installment_count"
					label="Parcelas"
					numeric
					required
					options={Array.from({ length: isMonthly ? 1 : 12 }, (_, index) => ({
						value: index + 1,
						label: `${index + 1}x`,
					}))}
				/>
			}
			{!isAssessment && (
				<FormSelect
					name="contract_status"
					label="Status"
					options={options(contractStatusLabels)}
					required
				/>

			)}

			<FormInput
				name="contract_description"
				label="Descrição do serviço"
				className="sm:col-span-3"
				placeholder="Plano Mensal, Pacote, Avaliação..."
				required
			/>

			<FormInput
				name="start_date"
				label={isAssessment ? "Data da avaliação" : "Início"}
				type="date"
				required
			/>

			{!isAssessment && (
				<>
					{!isCreditCard &&
						<>
							<FormInput
								name="first_due_date"
								label="Primeiro vencimento"
								type="date"
								nullable
							/>
							<FormInput
								name="due_day"
								label="Dia do vencimento"
								type="number"
								min={1}
								max={28}
								step={1}
								maxLength={2}
								required
							/>
						</>
					}
					<FormInput
						name="sessions_per_week"
						label="Aulas por semana"
						type="number"
						min={1}
						max={7}
						step={1}
						maxLength={1}
						required
					/>
				</>
			)}
		</div>
	)
}