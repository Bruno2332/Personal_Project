import type { BillingType } from "./contract"

export type Objective = "HypertrophyAndDefinition" | "WeightLoss" | "RehabilitationStrengthening" | "SportsPerformance" | "GeneralConditioningHealth" | "FlexibilityMobility"
export type Gender = "Male" | "Female"

export const states = [
    "AC", "AL", "AP", "AM", "BA", "CE", "DF",
    "ES", "GO", "MA", "MT", "MS", "MG", "PA",
    "PB", "PR", "PE", "PI", "RJ", "RN", "RS",
    "RO", "RR", "SC", "SP", "SE", "TO",
] as const

export type State = (typeof states)[number]

export type StudentDto = {
    id: number | null
    name: string
    cpf: string
    gender: Gender
    email: string
    birth_date: string
    phone: string | null
    zip_code: string | null
    address: string | null
    address_number: string | null
    address_complement: string | null
    city: string | null
    state: State | null
    objective: Objective | null
    notes: string | null
    is_active: boolean
    created_at: string
    updated_at: string
}

export type StudentProjectionDto = {
    id: number | null
    name: string
    email: string
    phone: string
    objective: Objective
    is_active: boolean
    first_due_date: string
    amount_cents: number
    billing_type: BillingType
}

