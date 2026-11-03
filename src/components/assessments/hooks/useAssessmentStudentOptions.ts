import { useEffect, useRef, useState } from "react"
import {
    findAllStudents,
    findStudentById,
} from "../../../services/studentService"
import { collectPages } from "../../../services/pagination"
import { errorMessage } from "../../../utils/errors"

type StudentOption = {
    value: number
    label: string
}


export default function useAssessmentStudentOptions(
    studentId: number | null,
) {
    const [studentOptions, setStudentOptions] = useState<StudentOption[]>([])
    const [listLoading, setListLoading] = useState(true)
    const [listError, setListError] = useState("")
    const [listLoaded, setListLoaded] = useState(false)
    const [retry, setRetry] = useState(0)

    const [selectedRequest, setSelectedRequest] = useState<{
        id: number
        loading: boolean
        error: string
    } | null>(null)

    const optionsRef = useRef<StudentOption[]>([])

    useEffect(() => {
        let cancelled = false

        setListLoading(true)
        setListLoaded(false)
        setListError("")

        async function loadStudents() {
            try {
                const students = await collectPages((page, size) =>
                    findAllStudents(page, size, null),
                )

                if (cancelled) return

                const uniqueStudents = new Map<number, string>()

                for (const student of students) {
                    if (student.id !== null) {
                        uniqueStudents.set(student.id, student.name)
                    }
                }

                // Preserva alunos incluídos por uma busca individual.
                for (const option of optionsRef.current) {
                    if (!uniqueStudents.has(option.value)) {
                        uniqueStudents.set(option.value, option.label)
                    }
                }

                const options = Array.from(
                    uniqueStudents,
                    ([value, label]) => ({ value, label }),
                ).sort((a, b) =>
                    a.label.localeCompare(b.label, "pt-BR"),
                )

                optionsRef.current = options
                setStudentOptions(options)
                setListLoaded(true)
            } catch (error) {
                if (!cancelled) {
                    setListError(errorMessage(error))
                }
            } finally {
                if (!cancelled) setListLoading(false)
            }
        }

        void loadStudents()

        return () => {
            cancelled = true
        }
    }, [retry])

    useEffect(() => {
        let cancelled = false

        setSelectedRequest(null)

        if (
            studentId === null ||
            !listLoaded ||
            studentOptions.some((option) => option.value === studentId)
        ) {
            return
        }

        const id = studentId

        setSelectedRequest({
            id,
            loading: true,
            error: "",
        })

        async function loadSelectedStudent() {
            try {
                const student = await findStudentById(id)

                if (cancelled) return

                if (student.id !== id) {
                    throw new Error("O aluno retornado não possui o ID esperado.")
                }

                const options = [
                    ...optionsRef.current.filter(
                        (option) => option.value !== id,
                    ),
                    { value: id, label: student.name },
                ].sort((a, b) =>
                    a.label.localeCompare(b.label, "pt-BR"),
                )

                optionsRef.current = options
                setStudentOptions(options)
                setSelectedRequest(null)
            } catch (error) {
                if (!cancelled) {
                    setSelectedRequest({
                        id,
                        loading: false,
                        error: errorMessage(error),
                    })
                }
            }
        }

        void loadSelectedStudent()

        return () => {
            cancelled = true
        }
    }, [studentId, listLoaded, studentOptions, retry])

    function handleRetry() {
        setListLoading(true)
        setListLoaded(false)
        setListError("")
        setSelectedRequest(null)
        setRetry((value) => value + 1)
    }

    const currentRequest =
        selectedRequest?.id === studentId ? selectedRequest : null

    return {
        studentOptions,
        loading: listLoading || (currentRequest?.loading ?? false),
        error: listError || currentRequest?.error || "",
        handleRetry,
    }
}