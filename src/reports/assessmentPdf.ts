import { jsPDF } from "jspdf"

import type { PhysicalAssessmentCompleteDto } from "../model/physicalAssessment"
import type { StudentDto } from "../model/student"
import type { UserDto } from "../model/user"

import { formatDate } from "./assessmentPdfFormatters"
import createAssessmentPdfLayout from "./assessmentPdfLayout"

import assessmentGeneralSection from "./sections/assessmentGeneralSection"
import assessmentCircumferencesSection from "./sections/assessmentCircumferencesSection"
import assessmentCompositionSection from "./sections/assessmentCompositionSection"
import assessmentStrengthSection from "./sections/assessmentStrengthSection"
import assessmentVo2Section from "./sections/assessmentVo2Section"
import assessmentGoalsSection from "./sections/assessmentGoalsSection"

export async function createAssessmentPdf(
	result: PhysicalAssessmentCompleteDto,
	student: StudentDto,
	professional: UserDto,
	chartImage: string | null = null,
	vo2ChartImage: string | null = null,
): Promise<Uint8Array> {
	const assessment = result.physical_assessment_dto

	if (student.id !== assessment.student_id) {
		throw new Error(
			"O aluno não corresponde à avaliação selecionada.",
		)
	}

	const doc = new jsPDF({
		orientation: "portrait",
		unit: "mm",
		format: "a4",
	})

	doc.setProperties({
		title: `Avaliação física — ${student.name} — ${formatDate(
			assessment.assessment_date,
		)}`,
		author: professional.username,
		subject: "Relatório de avaliação física",
	})

	const layout = await createAssessmentPdfLayout(doc, professional)

	assessmentGeneralSection(layout.section, result, student)
	assessmentCircumferencesSection(layout.section, result)
	assessmentCompositionSection(layout.section, result)
	assessmentStrengthSection(layout.section, result)
	assessmentVo2Section(layout.section, result)
	assessmentGoalsSection(layout.section, result)

	layout.chart(
		"Evolução — avaliação anterior e atual",
		chartImage,
	)

	layout.chart(
		"VO2 — evolução nas avaliações selecionadas",
		vo2ChartImage,
	)

	layout.footer(assessment.assessment_date)

	return new Uint8Array(doc.output("arraybuffer"))
}