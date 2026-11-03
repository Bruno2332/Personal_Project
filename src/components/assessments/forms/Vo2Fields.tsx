import { useEffect, useState } from "react"
import { useFieldArray, useFormContext, useWatch } from "react-hook-form"
import type { AssessmentFormValues } from "./types"
import { FormInput, FormSelect } from "../../ui/Fields"
import { Button } from "../../ui/Button"
import { perceivedExertionLabels } from "../../../labels/physicalAssessment"
import { findStudentById } from "../../../services/studentService"


const options = (labels: Record<string, string>) =>
	Object.entries(labels).map(([value, label]) => ({ value, label }))



export default function Vo2Fields({
	onRemoveSavedStage,
}: {
	onRemoveSavedStage: (id: number) => void
}) {
	const { control, getValues, setValue } =
		useFormContext<AssessmentFormValues>()

	const studentId = useWatch({
		control,
		name: "assessment.student_id",
	})

	const assessmentDate = useWatch({
		control,
		name: "assessment.assessment_date",
	})

	const [studentBirth, setStudentBirth] = useState<{
		studentId: number
		birthDate: string
	} | null>(null)

	const [studentLoading, setStudentLoading] = useState(false)
	const [studentError, setStudentError] = useState("")

	const { fields, append, remove } = useFieldArray({
		control,
		name: "vo2_assessments",
		keyName: "fieldKey",
	})

	function handleRemoveStage(index: number) {
		const stages = getValues("vo2_assessments")

		if (index < 0 || index !== stages.length - 1) return

		const stage = stages[index]

		if (stage.id !== null) {
			onRemoveSavedStage(stage.id)
		}

		remove(index)

		// Os segundos anteriores pertenciam ao estágio removido.
		setValue("assessment.vo2_final_stage_seconds", null, {
			shouldDirty: true,
			shouldValidate: true,
		})
	}

	useEffect(() => {
		let cancelled = false

		setStudentBirth(null)
		setStudentError("")

		if (!studentId) {
			setStudentLoading(false)
			return
		}

		setStudentLoading(true)

		async function loadStudent() {
			try {
				const student = await findStudentById(studentId)

				if (!cancelled) {
					setStudentBirth({
						studentId,
						birthDate: student.birth_date,
					})
				}
			} catch {
				if (!cancelled) {
					setStudentError(
						"Não foi possível buscar a data de nascimento do aluno.",
					)
				}
			} finally {
				if (!cancelled) setStudentLoading(false)
			}
		}

		void loadStudent()

		return () => {
			cancelled = true
		}
	}, [studentId])

	let maximumHeartRate: number | null = null

	if (
		studentBirth?.studentId === studentId &&
		studentBirth?.birthDate &&
		assessmentDate
	) {
		const [birthYear, birthMonth, birthDay] =
			studentBirth.birthDate.split("-").map(Number)

		const [year, month, day] =
			assessmentDate.split("-").map(Number)

		if (
			[birthYear, birthMonth, birthDay, year, month, day]
				.every(Number.isFinite)
		) {
			let age = year - birthYear

			if (
				month < birthMonth ||
				(month === birthMonth && day < birthDay)
			) {
				age--
			}

			if (age >= 0) {
				maximumHeartRate = 220 - age
			}
		}
	}

	return (
		<div className="space-y-4">
			<div className="p-3 bg-[#161616] border border-[#252525] rounded">
				<p className="text-xs text-[#717171] uppercase tracking-wider">
					Frequência cardíaca máxima estimada
				</p>

				{studentError ? (
					<p role="alert" className="mt-1 text-sm text-red-400">
						{studentError}
					</p>
				) : (
					<p className="mt-1 text-xl font-semibold text-[#3697ff]">
						{studentLoading
							? "Carregando..."
							: maximumHeartRate !== null
								? `${maximumHeartRate} bpm`
								: "—"}
					</p>
				)}
			</div>

			{fields.map((field, index) => (
				<div
					key={field.fieldKey}
					className="p-3 bg-[#161616] border border-[#252525] rounded space-y-3"
				>
					<div className="flex justify-between items-center">
						<p className="text-xs text-[#717171] uppercase tracking-wider">
							Estágio {index + 1}
						</p>

						{index === fields.length - 1 && (
							<Button
								type="button"
								onClick={() => handleRemoveStage(index)}
								className="text-xs text-red-400"
							>
								Remover
							</Button>
						)}
					</div>

					<div className="grid grid-cols-1 sm:grid-cols-3 gap-3">
						<FormInput
							name={`vo2_assessments.${index}.treadmill_speed_kmh`}
							label="Velocidade (km/h)"
							type="number"
							decimalPlaces={1}
							maxLength={2}
							min={1}
							max={25}
							step="0.1"
							nullable
						/>

						<FormInput
							name={`vo2_assessments.${index}.heart_rate_bpm`}
							label="FC (bpm)"
							type="number"
							maxLength={3}
							min={1}
							max={250}
							step={1}
							nullable
						/>

						<FormSelect
							name={`vo2_assessments.${index}.perceived_exertion`}
							label="Percepção de esforço"
							nullable
							options={options(perceivedExertionLabels)}
						/>
					</div>
				</div>
			))}

			<Button
				type="button"
				onClick={() => {
					const now = new Date().toISOString()
					const stages = getValues("vo2_assessments")
					const lastSpeed = stages[stages.length - 1]?.treadmill_speed_kmh

					append({
						id: null,
						physical_assessment_id: getValues("assessment.id") ?? 0,
						treadmill_speed_kmh:
							lastSpeed != null ? Number(lastSpeed) + 1 : null,
						heart_rate_bpm: null,
						perceived_exertion: null,
						created_at: now,
						updated_at: now,
					})
				}}
				className="px-4 py-2 text-sm text-[#a3e635] border border-[#a3e635]/30 rounded hover:bg-[#a3e635]/10"
			>
				+ Adicionar estágio
			</Button>

			{fields.length > 0 && (
				<FormInput
					name="assessment.vo2_final_stage_seconds"
					label="Segundos do último estágio"
					type="number"
					min={0}
					max={59}
					step={1}
					required
				/>
			)}
		</div>
	)
}