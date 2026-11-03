import { useState } from "react"
import type { StudentDto } from "../../../model/student"
import type {
    PhysicalAssessmentDto,
    PhysicalAssessmentCompleteDto,
} from "../../../model/physicalAssessment"
import type { CreateContractRequest } from "../../../model/contract"
import type { AssessmentFormValues } from "../forms/types"
import {
    insertPhysicalAssessment,
    updatePhysicalAssessment,
} from "../../../services/physicalAssessmentService"
import { errorMessage } from "../../../utils/errors"
import type { AssessmentTab } from "../forms/assessmentFields"

type Props = {
    studentId: number
    student: StudentDto | null
    studentLoading: boolean
    studentError: string
    initial: PhysicalAssessmentCompleteDto | null
    contractRequest?: CreateContractRequest
    onSaved: (id: number) => Promise<void>
    onClose: () => void
    setTab: (tab: AssessmentTab) => void
}


export default function useAssessmentSubmission({
    studentId,
    student,
    studentLoading,
    studentError,
    initial,
    contractRequest,
    onSaved,
    onClose,
    setTab,
}: Props) {
    const [error, setError] = useState("")
    const [persistedId, setPersistedId] = useState<number | null>(null)
    const [removedVo2Ids, setRemovedVo2Ids] = useState<number[]>([])

    function handleRemoveSavedStage(id: number) {
        setRemovedVo2Ids((current) =>
            current.includes(id) ? current : [...current, id],
        )
    }

    async function loadResults(id: number) {
        try {
            await onSaved(id)
            onClose()
        } catch {
            setError(
                "Avaliação salva. Não foi possível buscar os resultados; tente carregar novamente.",
            )
        }
    }

    async function save(values: AssessmentFormValues) {
        setError("")

        if (persistedId !== null) {
            await loadResults(persistedId)
            return
        }

        if (studentLoading || !student || studentError) {
            setError("Carregue os dados do aluno antes de salvar.")
            return
        }

        const existingAssessment = initial?.physical_assessment_dto
        let assessmentDate = values.assessment.assessment_date

        if (existingAssessment) {
            if (existingAssessment.id === null) {
                setError("A avaliação não possui um ID válido.")
                return
            }

            if (existingAssessment.student_id !== studentId) {
                setError("A avaliação não pertence ao aluno selecionado.")
                return
            }
        } else {
            if (!contractRequest) {
                setError("Informe os dados do contrato e do financeiro.")
                return
            }

            const contract = contractRequest.contract_dto

            if (contract.student_id !== studentId) {
                setError("O contrato deve pertencer ao aluno desta avaliação.")
                return
            }

            if (contract.service_type !== "PhysicalAssessment") {
                setError("O serviço do contrato deve ser avaliação física.")
                return
            }

            if (contract.id !== null) {
                setError("O cadastro conjunto deve receber um novo contrato.")
                return
            }

            assessmentDate = contract.start_date
        }

        if (!assessmentDate) {
            setError("Informe a data da avaliação.")
            return
        }

        if (assessmentDate < student.birth_date) {
            setError("A avaliação não pode ser anterior ao nascimento do aluno.")
            return
        }

        const dto: PhysicalAssessmentDto = {
            ...values.assessment,
            id: existingAssessment?.id ?? null,
            student_id: studentId,
            contract_id: existingAssessment?.contract_id ?? 0,
            assessment_date: assessmentDate,
            updated_at: new Date().toISOString(),
        }

        const stages = values.vo2_assessments

        if (
            stages.some(
                (stage) =>
                    stage.treadmill_speed_kmh == null &&
                    stage.heart_rate_bpm == null &&
                    stage.perceived_exertion == null,
            )
        ) {
            setTab("vo2")
            setError("Preencha ou remova os estágios vazios.")
            return
        }

        if (!stages.length) {
            dto.vo2_final_stage_seconds = null
        }

        try {
            let saved: PhysicalAssessmentDto

            if (dto.id !== null) {
                saved = await updatePhysicalAssessment(
                    dto.id,
                    dto,
                    stages,
                    removedVo2Ids,
                )
            } else {
                if (!contractRequest) {
                    setError("Informe os dados do contrato e do financeiro.")
                    return
                }

                saved = await insertPhysicalAssessment(
                    contractRequest,
                    dto,
                    stages,
                )
            }

            if (saved.id === null) {
                throw new Error("A avaliação salva não retornou um ID.")
            }

            setRemovedVo2Ids([])
            setPersistedId(saved.id)

            await loadResults(saved.id)
        } catch (error) {
            setError(errorMessage(error))
        }
    }

    return {
        error,
        persistedId,
        removedVo2Ids,
        handleRemoveSavedStage,
        save,
    }
}