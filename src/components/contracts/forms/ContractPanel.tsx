import { FormProvider } from "react-hook-form"
import AssessmentForm from "../../assessments/forms/AssessmentForm"
import { Button } from "../../ui/Button"
import { ErrorMessage } from "../../ui/Feedback"
import ContractCharges from "../results/ContractCharges"
import useContractPanel from "../hooks/useContractPanel"
import ContractFields from "./ContractFields"



type Props = {
	studentId: number
	onClose: () => void
}

export default function ContractPanel({ studentId, onClose }: Props) {
	const contract = useContractPanel(studentId, onClose)
	const { methods } = contract

	if (contract.assessmentRequest) {
		return (
			<AssessmentForm
				studentId={studentId}
				initial={null}
				contractRequest={contract.assessmentRequest}
				initialValues={contract.measurements}
				onBack={contract.backToContract}
				onClose={contract.closeAssessment}
				onSaved={contract.assessmentSaved}
			/>
		)
	}

	return (
		<FormProvider {...methods}>
			<form noValidate onSubmit={methods.handleSubmit(contract.save)}>
				<div className="p-6 space-y-5">
					<div className="flex items-center gap-3">
						<select
							aria-label="Contrato"
							value={contract.selected ?? ""}
							disabled={
								contract.isSubmitting ||
								contract.contractsLoading ||
								!!contract.contractsError
							}
							onChange={(event) =>
								contract.selectContract(
									event.target.value === ""
										? null
										: Number(event.target.value),
								)
							}
							className="flex-1 px-3 py-2 bg-[#0d0d0d] border border-[#252525] rounded text-sm text-white focus:outline-none focus:border-[#a3e635]"
						>
							<option value="">
								{contract.contractsLoading
									? "Carregando contratos..."
									: "Novo contrato"}
							</option>

							{contract.studentContracts.map((item) => (
								<option key={item.id} value={item.id ?? ""}>
									{item.contract_description}
								</option>
							))}
						</select>

						<Button
							type="button"
							disabled={
								contract.isSubmitting ||
								contract.contractsLoading ||
								!!contract.contractsError
							}
							onClick={() => contract.selectContract(null)}
							className="text-sm text-[#a3e635]"
						>
							+ Novo contrato
						</Button>
					</div>

					{contract.contractsError && (
						<div className="space-y-2">
							<ErrorMessage message={contract.contractsError} />
							<Button
								type="button"
								onClick={contract.retryContracts}
								className="text-sm text-[#a3e635]"
							>
								Tentar novamente
							</Button>
						</div>
					)}

					{!contract.contractsLoading &&
						!contract.contractsError && <ContractFields />}

					{contract.error && <ErrorMessage message={contract.error} />}

					{contract.notice && (
						<p role="status" className="text-sm text-[#a3e635]">
							{contract.notice}
						</p>
					)}

					{contract.selected !== null && (
						<ContractCharges
							charges={contract.charges}
							loading={contract.chargesLoading}
							error={contract.chargesError}
							onReload={contract.retryCharges}
						/>
					)}
				</div>

				<div className="sticky bottom-0 bg-[#111] border-t border-[#252525] px-6 py-4 flex justify-end gap-3">
					<Button
						type="submit"
						disabled={
							contract.isSubmitting ||
							contract.contractsLoading ||
							!!contract.contractsError
						}
						className="px-5 py-2 text-sm bg-[#a3e635] text-black font-semibold rounded hover:bg-[#bef264] transition-colors"
					>
						{contract.isSubmitting
							? "Salvando..."
							: contract.isNewAssessment
								? "Continuar para avaliação"
								: "Salvar Contrato"}
					</Button>
				</div>
			</form>
		</FormProvider>
	)
}