
import { useForm } from "react-hook-form"

import type {
    ContractDto,
    CreateContractRequest,
} from "../../../model/contract"

import type { AssessmentFormValues } from "../forms/types"

import { localDate } from "../../../utils/dates"

type FormValues = ContractDto & {
    amount_reais: number
}

type Props = {
    studentId: number | null
    studentsLoading: boolean
    studentsError: string
    measurements: AssessmentFormValues | undefined
    setMeasurements: (values: AssessmentFormValues | undefined) => void
    setDraft: (request: CreateContractRequest) => void
}

export type { FormValues }

export default function useNewAssessmentContract({
    studentId,
    studentsLoading,
    studentsError,
    measurements,
    setMeasurements,
    setDraft,
}: Props) {
    const today = localDate()
    const now = new Date().toISOString()

    const methods = useForm<FormValues>({
        defaultValues: {
            id: null,
            student_id: studentId ?? 0,
            contract_description: "Avaliação física",
            start_date: today,
            end_date: today,
            service_type: "PhysicalAssessment",
            billing_type: "Package",
            amount_cents: 0,
            amount_reais: 0,
            payment_method: "Pix",
            installment_count: 1,
            first_due_date: null,
            due_day: null,
            sessions_per_week: null,
            contract_status: "Completed",
            created_at: now,
            updated_at: now,
        },
    })

    function next(values: FormValues) {
        if (studentsLoading || studentsError) return

        const { amount_reais, ...contract } = values
        const amountCents = Math.round(amount_reais * 100)

        if (!values.student_id || values.student_id <= 0) {
            methods.setError("student_id", {
                message: "Selecione um aluno.",
            })
            return
        }

        if (
            !Number.isSafeInteger(amountCents) ||
            amountCents <= 0 ||
            amountCents > 2147483647
        ) {
            methods.setError("amount_reais", {
                message: "Informe um valor positivo dentro do limite permitido.",
            })
            return
        }

        if (
            !Number.isInteger(values.installment_count) ||
            values.installment_count < 1 ||
            values.installment_count > 12
        ) {
            methods.setError("installment_count", {
                message: "Selecione entre 1 e 12 parcelas.",
            })
            return
        }

        const timestamp = new Date().toISOString()

        const contractDto: ContractDto = {
            ...contract,
            id: null,
            amount_cents: amountCents,
            contract_description: contract.contract_description.trim(),
            end_date: contract.start_date,
            first_due_date: null,
            service_type: "PhysicalAssessment",
            billing_type: "Package",
            contract_status: "Completed",
            due_day: null,
            sessions_per_week: null,
            created_at: timestamp,
            updated_at: timestamp,
        }

        const request: CreateContractRequest = {
            contract_dto: contractDto,
            financial_record: {
                id: null,
                contract_id: null,
                transaction_type: "Income",
                description: contractDto.contract_description,
                amount_cent: amountCents,
                due_date: contractDto.start_date,
                payment_date: null,
                payment_method: contractDto.payment_method,
                installment_number: 1,
                status_transaction: "Pending",
                created_at: timestamp,
                updated_at: timestamp,
            },
        }

        // Não reaproveita medidas se o aluno foi trocado.
        if (
            measurements &&
            measurements.assessment.student_id !== contractDto.student_id
        ) {
            setMeasurements(undefined)
        }

        setDraft(request)
    }

    return {
        methods,
        next,
    }
}
