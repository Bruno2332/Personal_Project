import { invoke } from "@tauri-apps/api/core"
import type { UserDto } from "../model/user"

export function existsUser() {
    return invoke<boolean>("exists_user")
}

export function findUserById(id: number) {
    return invoke<UserDto>("find_user_by_id", { id })
}

export function insertUser(dto: UserDto) {
    return invoke<UserDto>("insert_user", { dto })
}

export function updateUser(id: number, dto: UserDto) {
    return invoke<UserDto>("update_user", { id, dto })
}
