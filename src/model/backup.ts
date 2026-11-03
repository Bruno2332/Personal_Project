export type BackupSettingsRequestDto = {
    folder: string
    password: string | null
}

export type BackupSettingsDto = {
    folder: string
    last_automatic_backup: string | null
}

export type BackupResultDto = {
    path: string
    warning: string | null
}

export type RestoreBackupResponse = {
    restored: boolean
    restart_required: boolean
    previous_database_path: string | null
    error: string | null
}