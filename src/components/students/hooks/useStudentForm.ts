import { useState } from "react"
import { useForm } from "react-hook-form"

import type { StudentDto } from "../../../model/student"
import { insertStudent, updateStudent } from "../../../services/studentService"
import { localDate } from "../../../utils/dates"
import { errorMessage } from "../../../utils/errors"

export default function useStudentForm(student: StudentDto | null) {
	const [saved, setSaved] = useState<StudentDto | null>(student)
	const [tab, setTab] = useState<"personal" | "contract">("personal")
	const [error, setError] = useState("")
	const now = new Date().toISOString()
	const methods = useForm<StudentDto>({
		defaultValues: student ?? {
			id: null,
			name: "",
			cpf: "",
			gender: "Male",
			email: "",
			birth_date: "",
			phone: null,
			zip_code: null,
			address: null,
			address_number: null,
			address_complement: null,
			city: null,
			state: null,
			objective: null,
			notes: null,
			is_active: true,
			created_at: now,
			updated_at: now,
		},
	})

	const active = methods.watch("is_active")
	const name = methods.watch("name")

	async function save(values: StudentDto) {
		setError("")
		const cpf = values.cpf.replace(/\D/g, "")

		if (cpf.length !== 11) {
			methods.setError("cpf", { message: "Informe um CPF com 11 dígitos." })
			return
		}

		if (values.birth_date > localDate()) {
			methods.setError("birth_date", { message: "Nascimento não pode ser futuro." })
			return
		}

		try {
			const payload: StudentDto = {
				...values,
				cpf,
				name: values.name.trim(),
				email: values.email.trim(),
				updated_at: new Date().toISOString(),
			}
			const result = saved?.id != null
				? await updateStudent(saved.id, payload)
				: await insertStudent(payload)

			setSaved(result)
			methods.reset(result)
			setTab("contract")
		} catch (err) {
			setError(errorMessage(err))
		}
	}

	return { methods, saved, tab, setTab, error, active, name, save }
}
