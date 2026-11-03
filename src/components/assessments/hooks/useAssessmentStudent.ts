import { useEffect, useState } from "react"
import type { StudentDto } from "../../../model/student"
import { findStudentById } from "../../../services/studentService"
import { errorMessage } from "../../../utils/errors"


export default function useAssessmentStudent(studentId: number) {
    const [student, setStudent] = useState<StudentDto | null>(null)
    const [loading, setLoading] = useState(true)
    const [error, setError] = useState("")
    const [retry, setRetry] = useState(0)

    useEffect(() => {
        let cancelled = false

        setStudent(null)
        setLoading(true)
        setError("")

        void findStudentById(studentId)
            .then((result) => {
                if (!cancelled) {
                    setStudent(result)
                }
            })
            .catch((error) => {
                if (!cancelled) {
                    setError(errorMessage(error))
                }
            })
            .finally(() => {
                if (!cancelled) {
                    setLoading(false)
                }
            })

        return () => {
            cancelled = true
        }
    }, [studentId, retry])

    function handleRetry() {
        setRetry((value) => value + 1)
    }

    return {
        student,
        loading,
        error,
        handleRetry,
    }
}