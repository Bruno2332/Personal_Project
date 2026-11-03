import type {
    BillingType,
    ContractStatus,
    ServiceType,
} from "../model/contract"



export const billingLabels: Record<BillingType, string> = {
    Monthly: "Mensal",
    Package: "Pacote",
}

export const serviceLabels: Record<ServiceType, string> = {
    Training: "Treinamento",
    PhysicalAssessment: "Avaliação física",
}

export const contractStatusLabels: Record<ContractStatus, string> = {
    Active: "Ativo",
    Completed: "Encerrado",
    Cancelled: "Cancelado",
}