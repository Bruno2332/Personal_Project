import type { FinancialStatusFilter } from "../model/financialRecord"

export const financialStatusStyles: Record<FinancialStatusFilter, string> = {
  Pending: "text-yellow-400 bg-yellow-400/10",
  Paid: "text-[#a3e635] bg-[#a3e635]/10",
  Cancelled: "text-[#717171] bg-[#717171]/10",
  Overdue: "text-red-400 bg-red-400/10",
}
