import { invoke } from "@tauri-apps/api/core"
import type {
    FinancialRecordsDto,
    FinancialRecordsProjectionDto,
    FinancialFilters,
    FinancialMonthDto,
    FinancialContractOptionDto,
} from "../model/financialRecord"
import type { PageResponse } from "../model/pagination"


export function findAllFinancialRecords(
    page = 0,
    size = 20,
    name: string | null = null,
    monthly: boolean | null = null,
    filters: FinancialFilters = {},
) {
    return invoke<PageResponse<FinancialRecordsProjectionDto>>(
        "find_all_financial_records",
        {
            page,
            size,
            name,
            monthly,
            transactionType: filters.transactionType ?? null,
            status: filters.status ?? null,
            period: filters.period ?? null,
            contractId: filters.contractId ?? null,
        },
    )
}


export function insertFinancialRecord(dto: FinancialRecordsDto) {
    return invoke<FinancialRecordsDto>("insert_financial_record", { dto })
}


export function updateFinancialRecord(id: number, dto: FinancialRecordsDto) {
    return invoke<FinancialRecordsDto>("update_financial_record", { id, dto })
}


export function findFinancialSummary() {
    return invoke<FinancialMonthDto[]>("find_financial_summary")
}


export function findFinancialContractOptions() {
    return invoke<FinancialContractOptionDto[]>("find_financial_contract_options")
}

export function generateMonthlyCharges() {
	return invoke<number>("generate_monthly_charges")
}
