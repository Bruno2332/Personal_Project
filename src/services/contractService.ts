import { invoke } from "@tauri-apps/api/core"
import type {
	ContractDto,
	CreateContractRequest,
} from "../model/contract"

export function findContractsByStudentId(id: number) {
	return invoke<ContractDto[]>("find_all_contracts_by_student_id", {
		id,
	})
}

export function insertContract(dto: CreateContractRequest) {
	return invoke<ContractDto>("insert_contract", {
		dto,
	})
}

export function updateContract(id: number, dto: CreateContractRequest) {
	return invoke<ContractDto>("update_contract", {
		id,
		dto,
	})
}