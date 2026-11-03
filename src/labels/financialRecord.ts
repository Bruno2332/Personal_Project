import type {
  FinancialStatusFilter,
  PaymentMethod,
  TransactionType,
  TransactionStatus,
} from "../model/financialRecord"

export const paymentLabels: Record<PaymentMethod, string> = {
  Pix: "PIX",
  CreditCard: "Cartão de crédito",
}

export const transactionTypeLabels: Record<TransactionType, string> = {
  Income: "Entrada",
  Expense: "Saída",
}

export const transactionStatusLabels: Record<TransactionStatus, string> = {
  Pending: "Pendente",
  Paid: "Pago",
  Cancelled: "Cancelado",
}

export const financialStatusLabels: Record<FinancialStatusFilter, string> = {
  ...transactionStatusLabels,
  Overdue: "Atrasado",
}
