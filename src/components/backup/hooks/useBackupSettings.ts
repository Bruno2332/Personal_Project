import { useEffect, useRef, useState } from "react"
import { useForm } from "react-hook-form"
import { open } from "@tauri-apps/plugin-dialog"

import type {
    BackupSettingsDto,
    BackupSettingsRequestDto,
} from "../../../model/backup"

import {
    configureBackup,
    createManualBackup,
    getBackupSettings,
} from "../../../services/backupService"

import { errorMessage } from "../../../utils/errors"


export default function useBackupSettings() {
    const [settings, setSettings] = useState<BackupSettingsDto | null>(null)
    const [loading, setLoading] = useState(true)
    const [loadFailed, setLoadFailed] = useState(false)
    const [busy, setBusy] = useState(false)
    const [error, setError] = useState("")
    const [message, setMessage] = useState("")
    const [attempt, setAttempt] = useState(0)

    const operationRunning = useRef(false)

    const methods = useForm<BackupSettingsRequestDto>({
        defaultValues: {
            folder: "",
            password: null,
        },
    })

    const { reset } = methods
    const folder = methods.watch("folder")
    const password = methods.watch("password")

    const unsavedChanges =
        folder !== (settings?.folder ?? "") || Boolean(password)

    useEffect(() => {
        let cancelled = false

        setLoading(true)
        setLoadFailed(false)
        setError("")

        async function load() {
            try {
                const result = await getBackupSettings()

                if (cancelled) return

                setSettings(result)
                reset({
                    folder: result?.folder ?? "",
                    password: null,
                })
            } catch (err) {
                if (!cancelled) {
                    setLoadFailed(true)
                    setError(errorMessage(err))
                }
            } finally {
                if (!cancelled) {
                    setLoading(false)
                }
            }
        }

        void load()

        return () => {
            cancelled = true
        }
    }, [reset, attempt])

    function beginOperation() {
        if (operationRunning.current || loading || loadFailed) {
            return false
        }

        operationRunning.current = true
        setBusy(true)
        setError("")
        setMessage("")

        return true
    }

    function finishOperation() {
        operationRunning.current = false
        setBusy(false)
    }

    function retry() {
        setAttempt((value) => value + 1)
    }

    async function selectFolder() {
        if (!beginOperation()) return

        try {
            const selected = await open({
                title: "Selecionar pasta dos backups",
                directory: true,
                multiple: false,
            })

            if (typeof selected === "string") {
                methods.setValue("folder", selected, {
                    shouldDirty: true,
                    shouldValidate: true,
                })
            }
        } catch (err) {
            setError(errorMessage(err))
        } finally {
            finishOperation()
        }
    }

    async function save(values: BackupSettingsRequestDto) {
        if (!values.folder) {
            setError("Selecione a pasta dos backups.")
            return
        }

        if (!beginOperation()) return

        try {
            const result = await configureBackup({
                folder: values.folder,
                password: values.password || null,
            })

            setSettings(result)

            reset({
                folder: result.folder,
                password: null,
            })

            setMessage(
                "Configuração salva. Você pode fazer um backup agora. O automático será verificado na próxima abertura do aplicativo.",
            )
        } catch (err) {
            setError(errorMessage(err))
        } finally {
            finishOperation()
        }
    }

    async function backupNow() {
        if (!settings || unsavedChanges) return
        if (!beginOperation()) return

        try {
            const result = await createManualBackup()

            setMessage(`Backup criado: ${result.path}`)

            if (result.warning) {
                setError(result.warning)
            }
        } catch (err) {
            setError(errorMessage(err))
        } finally {
            finishOperation()
        }
    }

    return {
        methods,
        settings,
        loading,
        loadFailed,
        busy,
        error,
        message,
        unsavedChanges,
        retry,
        selectFolder,
        save,
        backupNow,
    }
}