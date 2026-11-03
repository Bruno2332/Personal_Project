export type TransactionType = "Income" | "Expense"

export type PaymentMethod = "Pix" | "CreditCard"

export type TransactionStatus = "Pending" | "Paid" | "Cancelled"

export type FinancialRecordsDto = {
    id: number | null
    contract_id: number | null
    transaction_type: TransactionType
    description: string
    amount_cent: number
    due_date: string
    payment_date: string | null
    payment_method: PaymentMethod
    installment_number: number
    status_transaction: TransactionStatus
    created_at: string
    updated_at: string
}

export type FinancialRecordsProjectionDto = FinancialRecordsDto & {
    student_id: number | null
    student_name: string | null
}

export type FinancialStatusFilter = TransactionStatus | "Overdue"


export type FinancialMonthDto = {
    month: string
    income_cents: number
    expense_cents: number
    receivable_cents: number
    payable_cents: number
}
export type FinancialContractOptionDto = { value: number; label: string }


export type FinancialFilters = {
    transactionType?: TransactionType | null
    status?: FinancialStatusFilter | null
    period?: string | null
    contractId?: number | null
}

