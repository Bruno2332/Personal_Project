import type { FinancialRecordsDto } from "./financialRecord"

export type BillingType = "Monthly" | "Package"

export type ServiceType = "Training" | "PhysicalAssessment"

export type ContractStatus = "Active" | "Completed" | "Cancelled"

export type PaymentMethod = "Pix" | "CreditCard"

export type ContractDto = {
	id: number | null
	student_id: number
	contract_description: string
	start_date: string
	end_date: string | null
    duration_months: number | null
	service_type: ServiceType
	billing_type: BillingType
	amount_cents: number
    payment_method: PaymentMethod
	installment_count: number
	first_due_date: string | null
	due_day: number | null
	sessions_per_week: number | null
	contract_status: ContractStatus
	created_at: string
	updated_at: string
}


export type CreateContractRequest = {
    contract_dto: ContractDto
    financial_record: FinancialRecordsDto
}