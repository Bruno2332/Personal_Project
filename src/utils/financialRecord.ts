import type { FinancialRecordsDto } from "../model/financialRecord"

export function financialRecordDto(
  record: FinancialRecordsDto,
): FinancialRecordsDto {
  return {
    id: record.id,
    contract_id: record.contract_id,
    transaction_type: record.transaction_type,
    description: record.description,
    amount_cent: record.amount_cent,
    due_date: record.due_date,
    payment_date: record.payment_date,
    payment_method: record.payment_method,
    installment_number: record.installment_number,
    status_transaction: record.status_transaction,
    created_at: record.created_at,
    updated_at: record.updated_at,
  }
}
