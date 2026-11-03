import { FormProvider } from "react-hook-form"

import useAssessmentsPage from "../components/assessments/hooks/useAssessmentsPage"

import AssessmentCharts from "../components/assessments/charts/AssessmentCharts"
import AssessmentHistory from "../components/assessments/results/AssessmentHistory"
import AssessmentDetails from "../components/assessments/results/AssessmentDetails"
import NewAssessment from "../components/assessments/forms/NewAssessment"
import AssessmentForm from "../components/assessments/forms/AssessmentForm"

import { FormSelect } from "../components/ui/Fields"
import { Button } from "../components/ui/Button"
import {
	ErrorMessage,
	LoadingMessage,
} from "../components/ui/Feedback"



export default function Assessments() {
	const page = useAssessmentsPage()

	return (
		<div className="flex flex-col h-full gap-6">
			<ErrorMessage message={page.error} />
			<ErrorMessage message={page.studentsError} />

			{page.studentsError && (
				<Button
					type="button"
					disabled={page.studentsLoading || page.busy}
					onClick={page.retryStudents}
					className="text-sm text-[#a3e635] self-start"
				>
					Recarregar alunos
				</Button>
			)}

			{page.error && (
				<Button
					type="button"
					disabled={page.loading || page.busy}
					onClick={page.retryAssessments}
					className="text-sm text-[#a3e635] self-start"
				>
					Recarregar avaliações
				</Button>
			)}

			{(page.loading || page.studentsLoading) && <LoadingMessage />}

			<div className="flex items-center justify-between">
				<div>
					<h1 className="font-display text-4xl font-800 tracking-wide text-white uppercase">
						Avaliações Físicas
					</h1>

					<p className="text-[#717171] text-sm mt-0.5">
						{page.studentAssessments.length} avaliações registradas
						{" "}para este aluno
					</p>
				</div>

				<Button
					type="button"
					disabled={
						!page.currentStudent ||
						page.studentsLoading ||
						page.busy
					}
					onClick={page.openNewAssessment}
					className="flex items-center gap-2 px-5 py-2.5 bg-[#a3e635] text-black font-semibold text-sm rounded hover:bg-[#bef264] transition-colors"
				>
					<span className="text-lg leading-none">+</span>
					Nova Avaliação
				</Button>
			</div>

			<FormProvider {...page.methods}>
				<fieldset
					disabled={
						page.studentsLoading ||
						page.busy ||
						page.modalOpen
					}
					className="min-w-0 w-full max-w-md"
					onChange={page.handleStudentChange}
				>
					<FormSelect
						name="student_id"
						label="Aluno"
						numeric
						nullable
						options={page.studentOptions}
					/>
				</fieldset>
			</FormProvider>

			<AssessmentCharts assessments={page.studentAssessments} />

			<AssessmentHistory
				assessments={page.studentAssessments}
				studentName={page.currentStudent?.label ?? null}
				loading={page.loading}
				busy={page.busy}
				selectedAssessmentId={page.selectedAssessment?.id ?? null}
				onSelect={page.selectAssessment}
			/>

			{page.selectedAssessment && (
				<AssessmentDetails
					assessment={page.selectedAssessment}
					tab={page.tab}
					busy={page.busy}
					loading={page.loading}
					exportingPdf={page.exportingPdf}
					pdfError={page.pdfError}
					pdfMessage={page.pdfMessage}
					onTabChange={page.setTab}
					onGeneratePdf={() => void page.generatePdf()}
					onEdit={page.openEditAssessment}
					onRemove={() => void page.removeAssessment()}
					detailsRef={page.detailsRef}
				/>
			)}

			{page.modalOpen && !page.editing && (
				<NewAssessment
					studentId={page.selectedStudentId}
					onClose={page.closeModal}
					onSaved={page.savedNew}
				/>
			)}

			{page.modalOpen &&
				page.editing &&
				page.selectedStudentId !== null && (
					<AssessmentForm
						key={page.editing.physical_assessment_dto.id ?? "edit"}
						studentId={page.selectedStudentId}
						initial={page.editing}
						onClose={page.closeModal}
						onSaved={page.saved}
					/>
				)}
		</div>
	)
}