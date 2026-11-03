import { useEffect, useRef, useState } from "react"
import { useForm } from "react-hook-form"
import type {
	ContractDto,
	CreateContractRequest,
} from "../../../model/contract"
import type { FinancialRecordsProjectionDto } from "../../../model/financialRecord"
import {
	findContractsByStudentId,
	insertContract,
	updateContract,
} from "../../../services/contractService"
import { findAllFinancialRecords } from "../../../services/financialRecordService"
import { findAssessmentsByStudentId } from "../../../services/physicalAssessmentService"
import { collectPages } from "../../../services/pagination"
import { errorMessage } from "../../../utils/errors"
import { localDate } from "../../../utils/dates"
import type { AssessmentFormValues } from "../../assessments/forms/types"



export type ContractFormValues = ContractDto & {
	amount_reais: number
}

function defaults(studentId: number): ContractFormValues {
	const today = localDate()
	const now = new Date().toISOString()

	return {
		id: null,
		student_id: studentId,
		contract_description: "",
		start_date: today,
		end_date: null,
        duration_months: null,
		service_type: "Training",
		billing_type: "Monthly",
		amount_cents: 0,
		amount_reais: 0,
		payment_method: "Pix",
		installment_count: 1,
		first_due_date: today,
		due_day: Math.min(new Date().getDate(), 28),
		sessions_per_week: 1,
		contract_status: "Active",
		created_at: now,
		updated_at: now,
	}
}

function formValues(contract: ContractDto): ContractFormValues {
	return {
		...contract,
		amount_reais: contract.amount_cents / 100,
	}
}

export default function useContractPanel(
	studentId: number,
	onClose: () => void,
) {
	const [contracts, setContracts] = useState<ContractDto[]>([])
	const [selected, setSelected] = useState<number | null>(null)
	const [contractsLoading, setContractsLoading] = useState(true)
	const [contractsError, setContractsError] = useState("")
	const [retry, setRetry] = useState(0)

	const [error, setError] = useState("")
	const [notice, setNotice] = useState("")

	const [charges, setCharges] = useState<FinancialRecordsProjectionDto[]>([])
	const [chargesRevision, setChargesRevision] = useState(0)
	const [chargesLoading, setChargesLoading] = useState(false)
	const [chargesError, setChargesError] = useState("")

	const [assessmentRequest, setAssessmentRequest] =
		useState<CreateContractRequest | null>(null)
	const [measurements, setMeasurements] =
		useState<AssessmentFormValues | undefined>()

	const generation = useRef(0)

	const methods = useForm<ContractFormValues>({
		defaultValues: defaults(studentId),
	})

	const isSubmitting = methods.formState.isSubmitting
	const serviceType = methods.watch("service_type")
	const isNewAssessment =
		selected === null && serviceType === "PhysicalAssessment"

	const studentContracts = [...contracts].sort(
		(a, b) =>
			b.start_date.localeCompare(a.start_date) ||
			(b.id ?? 0) - (a.id ?? 0),
	)

	useEffect(() => {
		const currentGeneration = ++generation.current
		let cancelled = false

		setContractsLoading(true)
		setContractsError("")
		setContracts([])
		setSelected(null)
		setError("")
		setNotice("")
		setAssessmentRequest(null)
		setMeasurements(undefined)
		methods.reset(defaults(studentId))

		async function loadContracts() {
			try {
				const response = await findContractsByStudentId(studentId)

				if (cancelled || currentGeneration !== generation.current) return

				const sorted = [...response].sort(
					(a, b) =>
						b.start_date.localeCompare(a.start_date) ||
						(b.id ?? 0) - (a.id ?? 0),
				)

				const initialContract =
					sorted.find(
						(contract) =>
							contract.service_type === "Training" &&
							contract.contract_status === "Active",
					) ??
					sorted[0] ??
					null

				setContracts(sorted)
				setSelected(initialContract?.id ?? null)

				methods.reset(
					initialContract
						? formValues(initialContract)
						: defaults(studentId),
				)
			} catch (err) {
				if (!cancelled) {
					setContractsError(errorMessage(err))
				}
			} finally {
				if (!cancelled) {
					setContractsLoading(false)
				}
			}
		}

		void loadContracts()

		return () => {
			cancelled = true
			generation.current += 1
		}
	}, [studentId, retry, methods.reset])

	useEffect(() => {
		let cancelled = false

		setCharges([])
		setChargesError("")

		if (selected === null) {
			setChargesLoading(false)
			return
		}

		setChargesLoading(true)

		void collectPages((page, size) =>
			findAllFinancialRecords(page, size, null, null, {
				contractId: selected,
			}),
		)
			.then((records) => {
				if (!cancelled) {
					setCharges(
						[...records].sort(
							(a, b) =>
								a.installment_number - b.installment_number,
						),
					)
				}
			})
			.catch((err) => {
				if (!cancelled) {
					setChargesError(errorMessage(err))
				}
			})
			.finally(() => {
				if (!cancelled) {
					setChargesLoading(false)
				}
			})

		return () => {
			cancelled = true
		}
	}, [selected, chargesRevision, studentId])

	function selectContract(id: number | null) {
		setError("")
		setNotice("")
		setAssessmentRequest(null)
		setMeasurements(undefined)

		if (id === null) {
			setSelected(null)
			methods.reset(defaults(studentId))
			return
		}

		const contract = contracts.find((item) => item.id === id)

		if (!contract) return

		setSelected(id)
		methods.reset(formValues(contract))
	}

	async function assessmentSaved(assessmentId: number) {
		const currentGeneration = generation.current

		const [updatedContracts, assessments] = await Promise.all([
			findContractsByStudentId(studentId),
			findAssessmentsByStudentId(studentId),
		])

		if (currentGeneration !== generation.current) return

		const assessment = assessments.find(
			(item) => item.physical_assessment_dto.id === assessmentId,
		)

		const contract = updatedContracts.find(
			(item) =>
				item.id === assessment?.physical_assessment_dto.contract_id,
		)

		if (!contract) {
			throw new Error(
				"A avaliação foi salva, mas não foi possível carregar seu contrato.",
			)
		}

		setContracts(updatedContracts)
		setSelected(contract.id)
		methods.reset(formValues(contract))
		setChargesRevision((value) => value + 1)
		setNotice("Contrato e avaliação salvos.")
		onClose()
	}

	async function save(values: ContractFormValues) {
		const currentGeneration = generation.current

		setError("")
		setNotice("")

		if (
			values.service_type !== "PhysicalAssessment" &&
			values.end_date &&
			values.end_date < values.start_date
		) {
			methods.setError("end_date", {
				message: "Término deve ser igual ou posterior ao início.",
			})
			return
		}

		const { amount_reais, ...contract } = values
		const now = new Date().toISOString()

		const dto: ContractDto = {
			...contract,
			id: selected,
			amount_cents: Math.round(amount_reais * 100),
			student_id: studentId,
			updated_at: now,
		}

		if (dto.service_type === "PhysicalAssessment") {
			dto.end_date = dto.start_date
			dto.first_due_date = null
			dto.billing_type = "Package"
			dto.due_day = null
			dto.sessions_per_week = null

			if (dto.id === null) {
				dto.contract_status = "Completed"
			}
		}

		if (dto.billing_type === "Monthly") {
			dto.installment_count = 1
		}

		if (
			!Number.isSafeInteger(dto.amount_cents) ||
			dto.amount_cents <= 0 ||
			dto.amount_cents > 2147483647
		) {
			methods.setError("amount_reais", {
				message: "Informe um valor positivo dentro do limite permitido.",
			})
			return
		}

		if (
			!Number.isInteger(dto.installment_count) ||
			dto.installment_count < 1 ||
			dto.installment_count > 12
		) {
			methods.setError("installment_count", {
				message: "Selecione entre 1 e 12 parcelas.",
			})
			return
		}

		const request: CreateContractRequest = {
			contract_dto: dto,
			financial_record: {
				id: null,
				contract_id: dto.id,
				transaction_type: "Income",
				description: dto.contract_description,
				amount_cent: dto.amount_cents,
				due_date: dto.first_due_date ?? dto.start_date,
				payment_date: null,
				payment_method: dto.payment_method,
				installment_number: 1,
				status_transaction: "Pending",
				created_at: now,
				updated_at: now,
			},
		}

		if (
			dto.id === null &&
			dto.service_type === "PhysicalAssessment"
		) {
			setAssessmentRequest(request)
			return
		}

		try {
			const saved =
				dto.id !== null
					? await updateContract(dto.id, request)
					: await insertContract(request)

			if (currentGeneration !== generation.current) return

			setContracts((current) => [
				saved,
				...current.filter((item) => item.id !== saved.id),
			])

			setSelected(saved.id)
			methods.reset(formValues(saved))
			setMeasurements(undefined)
			setChargesRevision((value) => value + 1)

			try {
				const response = await findContractsByStudentId(studentId)

				if (currentGeneration !== generation.current) return

				setContracts(response)
				setNotice("Contrato salvo.")
			} catch {
				if (currentGeneration === generation.current) {
					setError(
						"Contrato salvo, mas não foi possível recarregar a lista de contratos.",
					)
				}
			}

			onClose()
		} catch (err) {
			if (currentGeneration === generation.current) {
				setError(errorMessage(err))
			}
		}
	}

	return {
		methods,
		contractsLoading,
		contractsError,
		error,
		notice,
		charges,
		chargesLoading,
		chargesError,
		selected,
		studentContracts,
		isSubmitting,
		isNewAssessment,
		assessmentRequest,
		measurements,
		save,
		selectContract,
		assessmentSaved,
		retryContracts: () => setRetry((value) => value + 1),
		retryCharges: () => setChargesRevision((value) => value + 1),
		backToContract: (values: AssessmentFormValues) => {
			setMeasurements(values)
			setAssessmentRequest(null)
		},
		closeAssessment: () => {
			setAssessmentRequest(null)
			setMeasurements(undefined)
		},
	}
}