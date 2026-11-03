import { FormProvider, useForm } from "react-hook-form"
import type { CreateContractRequest } from "../../../model/contract"
import type { PhysicalAssessmentCompleteDto } from "../../../model/physicalAssessment"
import { Modal } from "../../ui/Modal"
import { Tabs } from "../../ui/Tabs"
import { Button } from "../../ui/Button"
import { ErrorMessage } from "../../ui/Feedback"
import { defaultAssessment } from "./defaultAssessment"
import { assessmentFormTabs } from "./assessmentFields"

import AssessmentGeneralFields from "./AssessmentGeneralFields"
import AssessmentCircumferenceFields from "./AssessmentCircumferenceFields"
import AssessmentCompositionFields from "./AssessmentCompositionFields"
import AssessmentStrengthFields from "./AssessmentStrengthFields"
import Vo2Fields from "./Vo2Fields"

import useAssessmentStudent from "../hooks/useAssessmentStudent"
import useAssessmentNavigation from "../hooks/useAssessmentNavigation"
import useAssessmentSubmission from "../hooks/useAssessmentSubmission"

import type { AssessmentFormValues } from "./types"
import { useEffect } from "react"



type Props = {
	studentId: number
	contractRequest?: CreateContractRequest
	initial: PhysicalAssessmentCompleteDto | null
	initialValues?: AssessmentFormValues
	onBack?: (values: AssessmentFormValues) => void
	onClose: () => void
	onSaved: (id: number) => Promise<void>
}

export default function AssessmentForm({
	studentId,
	contractRequest,
	initial,
	initialValues,
	onBack,
	onClose,
	onSaved,
}: Props) {


	const {
		student,
		loading: studentLoading,
		error: studentError,
		handleRetry,
	} = useAssessmentStudent(studentId)


	const {
		tab,
		setTab,
		handleValidationErrors,
	} = useAssessmentNavigation()


	const methods = useForm<AssessmentFormValues>({
		defaultValues: initialValues ?? {
			assessment: initial?.physical_assessment_dto ?? {
				...defaultAssessment(studentId),
				weight_kg: undefined,
				height_m: undefined,
			},
			vo2_assessments:
				initial?.vo2_assessments.map(
					(stage) => stage.vo2_assessment_dto,
				) ?? [],
		},
	})

	useEffect(() => {
		if (tab === "vo2" && methods.getValues("vo2_assessments").length === 0) {
			return
		}

		const frame = window.requestAnimationFrame(() => {
			switch (tab) {
				case "general":
					methods.setFocus("assessment.weight_kg")
					break
				case "circumferences":
					methods.setFocus("assessment.neck_circumference_cm")
					break
				case "composition":
					methods.setFocus("assessment.triceps_skinfold_mm")
					break
				case "strength":
					methods.setFocus("assessment.right_handgrip_strength")
					break
				case "vo2":
					methods.setFocus("vo2_assessments.0.treadmill_speed_kmh")
					break
			}
		})

		return () => window.cancelAnimationFrame(frame)
	}, [tab, methods])

	const isSubmitting = methods.formState.isSubmitting


	const {
		error,
		persistedId,
		handleRemoveSavedStage,
		save,
	} = useAssessmentSubmission({
		studentId,
		student,
		studentLoading,
		studentError,
		initial,
		contractRequest,
		onSaved,
		onClose,
		setTab,
	})

	return (
		<Modal
			label="Avaliação física"
			onClose={onClose}
			busy={isSubmitting}
			backdropClassName="bg-black/70 backdrop-blur-sm"
		>
			<div className="relative bg-[#111] border border-[#252525] rounded-lg w-full max-w-2xl max-h-[90vh] overflow-y-auto shadow-2xl mx-2">

				<div className="sticky top-0 bg-[#111] border-b border-[#252525] px-6 py-4 flex items-center justify-between z-10">
					<h2 className="font-display text-2xl font-700 uppercase tracking-wide text-white">
						{initial ? "Editar Avaliação" : "Nova Avaliação"}
						{" — "}
						{student?.name ??
							(studentLoading
								? "Carregando aluno..."
								: `Aluno #${studentId}`)}
					</h2>

					<Button
						type="button"
						disabled={isSubmitting}
						onClick={onClose}
						className="text-[#717171] hover:text-white text-xl transition-colors"
					>
						×
					</Button>
				</div>


				{studentError && (
					<div className="px-6 pt-4 space-y-2">
						<ErrorMessage message={studentError} />

						<Button
							type="button"
							disabled={studentLoading || isSubmitting}
							onClick={handleRetry}
							className="text-sm text-[#a3e635]"
						>
							Tentar carregar aluno novamente
						</Button>
					</div>
				)}
				<Tabs
					items={assessmentFormTabs}
					value={tab}
					onChange={setTab}
				/>

				<FormProvider {...methods}>
					<form
						noValidate
						onSubmit={methods.handleSubmit(
							save,
							handleValidationErrors,
						)}
					>
						<fieldset
							disabled={persistedId !== null || isSubmitting}
							className="p-6 space-y-5"
						>
							<div hidden={tab !== "general"}>
								<AssessmentGeneralFields />
							</div>

							<div hidden={tab !== "circumferences"}>
								<AssessmentCircumferenceFields />
							</div>

							<div hidden={tab !== "composition"}>
								<AssessmentCompositionFields />
							</div>

							<div hidden={tab !== "strength"}>
								<AssessmentStrengthFields />
							</div>

							<div hidden={tab !== "vo2"}>
								<Vo2Fields
									onRemoveSavedStage={handleRemoveSavedStage}
								/>
							</div>

						</fieldset>


						<div className="mx-6 mb-4">
							<ErrorMessage message={error} />
						</div>


						<div className="relative bg-[#111] border-t border-[#252525] px-4 sm:px-6 py-4">
							<div className="grid grid-cols-3 items-center gap-2 sm:gap-3">
								<div>
									{onBack && persistedId === null && (
										<Button
											type="button"
											onClick={() =>
												onBack(methods.getValues())
											}
											disabled={isSubmitting}
											className="w-full px-2 sm:px-4 py-2 text-sm text-[#717171] hover:text-white border border-[#252525] rounded"
										>
											Voltar ao contrato
										</Button>
									)}
								</div>

								<Button
									type="button"
									onClick={onClose}
									disabled={isSubmitting}
									className="w-full px-2 sm:px-4 py-2 text-sm text-[#717171] hover:text-white border border-[#252525] rounded"
								>
									Cancelar
								</Button>

								<Button
									type="submit"
									disabled={
										isSubmitting ||
										(persistedId === null &&
											(studentLoading || !student))
									}
									className="w-full px-2 sm:px-5 py-2 text-sm bg-[#a3e635] text-black font-semibold rounded hover:bg-[#bef264]"
								>
									{isSubmitting
										? "Salvando..."
										: persistedId !== null
											? "Carregar resultados"
											: "Salvar avaliação"}
								</Button>
							</div>
						</div>
					</form>
				</FormProvider>
			</div>
		</Modal>
	)
}
