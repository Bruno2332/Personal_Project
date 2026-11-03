import { useState } from "react"
import type { CreateContractRequest } from "../../../model/contract"
import AssessmentForm from "../forms/AssessmentForm"
import type { AssessmentFormValues } from "./types"
import NewAssessmentContractStep from "./NewAssessmentContractStep"
import useAssessmentStudentOptions from "../hooks/useAssessmentStudentOptions"
import useNewAssessmentContract from "../hooks/useNewAssessmentContract"


type Props = {
	studentId: number | null
	onClose: () => void
	onSaved: (id: number, studentId: number) => Promise<void>
}


export default function NewAssessment({
	studentId,
	onClose,
	onSaved,
}: Props) {
	const [draft, setDraft] = useState<CreateContractRequest | null>(null)

	const [measurements, setMeasurements] =
		useState<AssessmentFormValues | undefined>()

	const {
		studentOptions,
		loading: studentsLoading,
		error: studentsError,
		handleRetry,
	} = useAssessmentStudentOptions(studentId)

	const { methods, next } = useNewAssessmentContract({
		studentId,
		studentsLoading,
		studentsError,
		measurements,
		setMeasurements,
		setDraft,
	})

	if (draft) {
		return (
			<AssessmentForm
				studentId={draft.contract_dto.student_id}
				initial={null}
				contractRequest={draft}
				initialValues={measurements}
				onBack={(values) => {
					setMeasurements(values)
					setDraft(null)
				}}
				onClose={onClose}
				onSaved={(id) =>
					onSaved(id, draft.contract_dto.student_id)
				}
			/>
		)
	}

	return (
		<NewAssessmentContractStep
			methods={methods}
			studentOptions={studentOptions}
			studentsLoading={studentsLoading}
			studentsError={studentsError}
			onRetry={handleRetry}
			onClose={onClose}
			onNext={next}
		/>
	)
}