import { invoke } from "@tauri-apps/api/core"

import type {
    BackupResultDto,
    BackupSettingsDto,
    BackupSettingsRequestDto,
    RestoreBackupResponse,
} from "../model/backup"


export function getBackupSettings() {
    return invoke<BackupSettingsDto | null>(
        "get_backup_settings",
    )
}

export function configureBackup(dto: BackupSettingsRequestDto) {
    return invoke<BackupSettingsDto>(
        "configure_backup",
        { dto },
    )
}

export function createManualBackup() {
    return invoke<BackupResultDto>(
        "create_manual_backup",
    )
}

export function restoreBackup(
    path: string,
    password: string,
) {
    return invoke<RestoreBackupResponse>(
        "restore_backup",
        { path, password },
    )
}

export function restartAfterRestore() {
    return invoke<void>("restart_after_restore")
}