import {
    useCallback,
    useEffect,
    useMemo,
    useRef,
    useState,
} from "react"

import type { PhysicalAssessmentCompleteDto } from "../../../model/physicalAssessment"

import { findAssessmentsByStudentId } from "../../../services/physicalAssessmentService"

import {
    assessmentView,
    type AssessmentView,
} from "../../../utils/assessments"

import { errorMessage } from "../../../utils/errors"

type Options = {
    studentId: number | null
    onOpenAssessment: (assessment: AssessmentView) => void
}

export default function useAssessmentRecords({
    studentId,
    onOpenAssessment,
}: Options) {
    const [results, setResults] = useState<
        PhysicalAssessmentCompleteDto[]
    >([])

    const [loading, setLoading] = useState(false)
    const [error, setError] = useState("")

    const requestId = useRef(0)
    const resultToOpen = useRef<number | null>(null)

    const reload = useCallback(async () => {
        const currentRequest = ++requestId.current

        if (studentId === null) {
            setResults([])
            setLoading(false)
            setError("")
            return []
        }

        setLoading(true)
        setError("")

        try {
            const response =
                await findAssessmentsByStudentId(studentId)

            if (currentRequest !== requestId.current) {
                return response
            }

            setResults(response)

            if (resultToOpen.current !== null) {
                const assessment = response.find(
                    (item) =>
                        item.physical_assessment_dto.id ===
                        resultToOpen.current,
                )

                if (assessment) {
                    onOpenAssessment(assessmentView(assessment))
                }

                resultToOpen.current = null
            }

            return response
        } catch (err) {
            if (currentRequest === requestId.current) {
                setError(errorMessage(err))
            }

            throw err
        } finally {
            if (currentRequest === requestId.current) {
                setLoading(false)
            }
        }
    }, [studentId, onOpenAssessment])

    useEffect(() => {
        setResults([])
        void reload().catch(() => { })

        return () => {
            requestId.current++
        }
    }, [reload])

    const studentAssessments = useMemo(
        () =>
            results
                .map(assessmentView)
                .sort(
                    (a, b) =>
                        a.data.localeCompare(b.data) ||
                        (a.id ?? 0) - (b.id ?? 0),
                ),
        [results],
    )

    return {
        results,
        setResults,
        studentAssessments,
        loading,
        error,
        setError,
        reload,
        resultToOpen,
    }
}