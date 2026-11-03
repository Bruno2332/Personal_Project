import { useState } from "react"

import type { Dispatch, SetStateAction } from "react"

import type { PhysicalAssessmentCompleteDto } from "../../../model/physicalAssessment"

import { deletePhysicalAssessment } from "../../../services/physicalAssessmentService"
import { exportAssessmentPdf } from "../../../services/assessmentPdfService"

import type { AssessmentView } from "../../../utils/assessments"
import { errorMessage } from "../../../utils/errors"



type Options = {
    selectedAssessment: AssessmentView | null
    setSelectedAssessment: Dispatch<
        SetStateAction<AssessmentView | null>
    >
    setResults: Dispatch<
        SetStateAction<PhysicalAssessmentCompleteDto[]>
    >
    reload: () => Promise<PhysicalAssessmentCompleteDto[]>
    setError: Dispatch<SetStateAction<string>>
}



export default function useAssessmentOperations({
    selectedAssessment,
    setSelectedAssessment,
    setResults,
    reload,
    setError,
}: Options) {
    const [busy, setBusy] = useState(false)
    const [exportingPdf, setExportingPdf] = useState(false)
    const [pdfError, setPdfError] = useState("")
    const [pdfMessage, setPdfMessage] = useState("")

    function clearPdfMessages() {
        setPdfError("")
        setPdfMessage("")
    }

    async function generatePdf() {
        if (!selectedAssessment || busy) return

        setBusy(true)
        setExportingPdf(true)
        clearPdfMessages()

        try {
            const path = await exportAssessmentPdf(
                selectedAssessment.raw,
            )

            if (path) {
                setPdfMessage("PDF salvo com sucesso.")
            }
        } catch (err) {
            setPdfError(errorMessage(err))
        } finally {
            setExportingPdf(false)
            setBusy(false)
        }
    }

    async function removeAssessment() {
        if (selectedAssessment?.id == null || busy) return

        if (
            !window.confirm(
                "Excluir esta avaliação e suas etapas de VO₂?",
            )
        ) {
            return
        }

        const id = selectedAssessment.id

        setBusy(true)
        setError("")
        clearPdfMessages()

        try {
            await deletePhysicalAssessment(id)

            setSelectedAssessment(null)

            setResults((current) =>
                current.filter(
                    (item) => item.physical_assessment_dto.id !== id,
                ),
            )

            try {
                await reload()
            } catch {
                setError(
                    "Avaliação excluída, mas não foi possível recarregar a lista.",
                )
            }
        } catch (err) {
            setError(errorMessage(err))
        } finally {
            setBusy(false)
        }
    }

    return {
        busy,
        exportingPdf,
        pdfError,
        pdfMessage,
        setPdfError,
        setPdfMessage,
        clearPdfMessages,
        generatePdf,
        removeAssessment,
    }
}