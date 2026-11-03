import { save } from "@tauri-apps/plugin-dialog"
import { writeFile } from "@tauri-apps/plugin-fs"
import type { PhysicalAssessmentCompleteDto } from "../model/physicalAssessment"
import { findStudentById } from "./studentService"
import { findUserById } from "./userService"
import { findAssessmentsByStudentId } from "./physicalAssessmentService"
import { createAssessmentPdf } from "../reports/assessmentPdf"
import { captureAssessmentChart } from "../reports/captureAssessmentChart"
import { assessmentView } from "../utils/assessments"

export async function exportAssessmentPdf(
    result: PhysicalAssessmentCompleteDto,
): Promise<string | null> {
    const assessment = result.physical_assessment_dto

    const [student, user] = await Promise.all([
        findStudentById(assessment.student_id),
        findUserById(1),
    ])

    const studentName =
        student.name
            .normalize("NFD")
            .replace(/[\u0300-\u036f]/g, "")
            .replace(/[^a-zA-Z0-9-]+/g, "-")
            .replace(/^-+|-+$/g, "")
            .slice(0, 80) || "Aluno"

    const path = await save({
        title: "Salvar avaliação em PDF",
        defaultPath: `Avaliacao-${studentName}-${assessment.assessment_date}.pdf`,
        filters: [
            {
                name: "Documento PDF",
                extensions: ["pdf"],
            },
        ],
    })

    if (!path) return null

    const history = await findAssessmentsByStudentId(
        assessment.student_id,
    )

    const previousAssessment = history
        .filter((item) => {
            const candidate = item.physical_assessment_dto

            if (
                candidate.student_id !== assessment.student_id ||
                candidate.id === assessment.id
            ) {
                return false
            }

            if (candidate.assessment_date < assessment.assessment_date) {
                return true
            }

            return (
                candidate.assessment_date === assessment.assessment_date &&
                candidate.id !== null &&
                assessment.id !== null &&
                candidate.id < assessment.id
            )
        })
        .sort((a, b) => {
            const first = a.physical_assessment_dto
            const second = b.physical_assessment_dto

            return (
                second.assessment_date.localeCompare(
                    first.assessment_date,
                ) ||
                (second.id ?? 0) - (first.id ?? 0)
            )
        })[0]

    const chartImage = previousAssessment
        ? await captureAssessmentChart([
            assessmentView(previousAssessment),
            assessmentView(result),
        ])
        : null

    const bytes = await createAssessmentPdf(
        result,
        student,
        user,
        chartImage,
    )

    await writeFile(path, bytes)

    return path
}