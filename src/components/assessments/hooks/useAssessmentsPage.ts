import { useCallback, useEffect, useRef, useState } from "react"
import { useForm, useWatch } from "react-hook-form"
import type { PhysicalAssessmentCompleteDto } from "../../../model/physicalAssessment"
import {
    assessmentView,
    type AssessmentView,
} from "../../../utils/assessments"
import type { AssessmentTab } from "../forms/assessmentFields"
import useAssessmentStudentOptions from "./useAssessmentStudentOptions"
import useAssessmentRecords from "./useAssessmentRecords"
import useAssessmentOperations from "./useAssessmentOperations"



export default function useAssessmentsPage() {
    const methods = useForm<{ student_id: number | null }>({
        defaultValues: {
            student_id: null,
        },
    })

    const selectedStudentId = useWatch({
        control: methods.control,
        name: "student_id",
    })

    const { setValue } = methods

    const [modalOpen, setModalOpen] = useState(false)
    const [editing, setEditing] =
        useState<PhysicalAssessmentCompleteDto | null>(null)
    const [selectedAssessment, setSelectedAssessment] =
        useState<AssessmentView | null>(null)
    const [tab, setTab] = useState<AssessmentTab>("general")

    const detailsRef = useRef<HTMLDivElement>(null)
    const initialStudentSelected = useRef(false)

    const students = useAssessmentStudentOptions(selectedStudentId)

    const openAssessment = useCallback((assessment: AssessmentView) => {
        setSelectedAssessment(assessment)
        setTab("general")
    }, [])

    const records = useAssessmentRecords({
        studentId: selectedStudentId,
        onOpenAssessment: openAssessment,
    })

    const operations = useAssessmentOperations({
        selectedAssessment,
        setSelectedAssessment,
        setResults: records.setResults,
        reload: records.reload,
        setError: records.setError,
    })

    const {
        studentOptions,
        loading: studentsLoading,
        error: studentsError,
    } = students

    const { setPdfError, setPdfMessage } = operations

    const currentStudent = studentOptions.find(
        (student) => student.value === selectedStudentId,
    )

    useEffect(() => {
        if (
            initialStudentSelected.current ||
            studentsLoading ||
            studentsError
        ) {
            return
        }

        if (selectedStudentId !== null) {
            initialStudentSelected.current = true
            return
        }

        const firstStudent = studentOptions[0]

        if (firstStudent) {
            initialStudentSelected.current = true
            setValue("student_id", firstStudent.value)
        }
    }, [
        studentOptions,
        studentsLoading,
        studentsError,
        selectedStudentId,
        setValue,
    ])

    useEffect(() => {
        setSelectedAssessment(null)
        setTab("general")
        setPdfError("")
        setPdfMessage("")
    }, [selectedStudentId, setPdfError, setPdfMessage])

    useEffect(() => {
        if (!selectedAssessment) return

        detailsRef.current?.scrollIntoView({
            behavior: "smooth",
            block: "start",
        })
    }, [selectedAssessment])

    function handleStudentChange() {
        records.resultToOpen.current = null
    }

    function closeModal() {
        setModalOpen(false)
        setEditing(null)
    }

    function openNewAssessment() {
        setEditing(null)
        setModalOpen(true)
    }

    function openEditAssessment(assessment: PhysicalAssessmentCompleteDto) {
        setEditing(assessment)
        setModalOpen(true)
    }

    function selectAssessment(assessment: AssessmentView) {
        if (operations.busy) return

        operations.clearPdfMessages()

        setSelectedAssessment((current) =>
            current?.id === assessment.id ? null : assessment,
        )
    }

    function retryAssessments() {
        void records.reload().catch(() => { })
    }

    async function saved(id: number) {
        const response = await records.reload()

        const assessment = response.find(
            (item) => item.physical_assessment_dto.id === id,
        )

        if (!assessment) {
            throw new Error(
                "A avaliação foi salva, mas não foi encontrada ao recarregar.",
            )
        }

        operations.clearPdfMessages()
        openAssessment(assessmentView(assessment))
    }

    async function savedNew(id: number, studentId: number) {
        if (studentId === selectedStudentId) {
            await saved(id)
            return
        }

        operations.clearPdfMessages()
        setSelectedAssessment(null)

        records.resultToOpen.current = id
        setValue("student_id", studentId)
    }

    return {
        methods,
        selectedStudentId,
        currentStudent,
        selectedAssessment,
        tab,
        setTab,
        modalOpen,
        editing,
        detailsRef,

        studentOptions,
        studentsLoading,
        studentsError,
        retryStudents: students.handleRetry,

        studentAssessments: records.studentAssessments,
        loading: records.loading,
        error: records.error,
        retryAssessments,

        busy: operations.busy,
        exportingPdf: operations.exportingPdf,
        pdfError: operations.pdfError,
        pdfMessage: operations.pdfMessage,
        generatePdf: operations.generatePdf,
        removeAssessment: operations.removeAssessment,

        handleStudentChange,
        closeModal,
        openNewAssessment,
        openEditAssessment,
        selectAssessment,
        saved,
        savedNew,
    }
}