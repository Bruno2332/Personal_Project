import { useRef, useState } from "react"
import { useForm } from "react-hook-form"
import { open } from "@tauri-apps/plugin-dialog"

import {
    restartAfterRestore,
    restoreBackup,
} from "../../../services/backupService"

import { errorMessage } from "../../../utils/errors"


export default function useRestoreBackup() {
    const [path, setPath] = useState<string | null>(null)
    const [error, setError] = useState("")
    const [selecting, setSelecting] = useState(false)
    const [busy, setBusy] = useState(false)
    const [restored, setRestored] = useState(false)
    const [restartRequired, setRestartRequired] = useState(false)

    const operationRunning = useRef(false)

    const methods = useForm<{ password: string }>({
        defaultValues: {
            password: "",
        },
    })

    async function selectBackup() {
        if (operationRunning.current || restartRequired) return

        operationRunning.current = true
        setSelecting(true)
        setError("")

        try {
            const selected = await open({
                title: "Selecionar backup",
                directory: false,
                multiple: false,
                filters: [
                    {
                        name: "Backup do aplicativo",
                        extensions: ["ptbackup"],
                    },
                ],
            })

            if (typeof selected === "string") {
                setPath(selected)
                setRestored(false)
                methods.reset()
            }
        } catch (err) {
            setError(errorMessage(err))
        } finally {
            operationRunning.current = false
            setSelecting(false)
        }
    }

    async function restore(values: { password: string }) {
        if (operationRunning.current || restartRequired) return

        if (!path) {
            setError("Selecione o arquivo de backup.")
            return
        }

        operationRunning.current = true
        setError("")
        setBusy(true)
        setRestored(false)

        try {
            const result = await restoreBackup(path, values.password)

            setRestored(result.restored)
            setRestartRequired(result.restart_required)

            if (result.error) {
                setError(result.error)
            }
        } catch (err) {
            setError(errorMessage(err))
        } finally {
            methods.reset()
            operationRunning.current = false
            setBusy(false)
        }
    }

    async function restart() {
        if (operationRunning.current || !restartRequired) return

        operationRunning.current = true
        setBusy(true)

        try {
            await restartAfterRestore()
        } catch (err) {
            setError(
                `${errorMessage(err)} Feche e abra o aplicativo novamente.`,
            )

            operationRunning.current = false
            setBusy(false)
        }
    }

    return {
        methods,
        path,
        error,
        selecting,
        busy,
        restored,
        restartRequired,
        selectBackup,
        restore,
        restart,
    }
}