import { useState } from "react"
import { FormProvider, useForm } from "react-hook-form"
import type { UserDto } from "../../model/user"
import { insertUser, updateUser } from "../../services/userService"
import { FormInput } from "../ui/Fields"
import { Button } from "../ui/Button"
import { ErrorMessage } from "../ui/Feedback"
import { errorMessage } from "../../utils/errors"



export default function UserForm({
	user,
	onSaved,
	onCancel,
}: {
	user: UserDto | null
	onSaved: (user: UserDto) => void
	onCancel: () => void
}) {
	const [error, setError] = useState("")
	const now = new Date().toISOString()

	const methods = useForm<UserDto>({
		defaultValues: user ?? {
			id: null,
			username: "",
			email: "",
			cref: "",
			created_at: now,
			updated_at: now,
		},
	})

	async function save(values: UserDto) {
		setError("")

		const dto: UserDto = {
			...values,
			username: values.username.trim(),
			email: values.email.trim(),
			cref: values.cref.trim(),
			updated_at: new Date().toISOString(),
		}

		try {
			const saved =
				dto.id !== null
					? await updateUser(dto.id, dto)
					: await insertUser(dto)

			onSaved(saved)
		} catch (error) {
			setError(errorMessage(error))
		}
	}

	return (
		<FormProvider {...methods}>
			<form
				noValidate
				onSubmit={methods.handleSubmit(save)}
				className="space-y-4"
			>
				<FormInput name="username" label="Nome" required />

				<FormInput
					name="email"
					label="E-mail"
					type="email"
					required
				/>

				<FormInput name="cref" label="CREF" required />

				{error && <ErrorMessage message={error} />}

				<div className="flex gap-3">
					{user && (
						<Button
							type="button"
							onClick={onCancel}
							disabled={methods.formState.isSubmitting}
							className="px-4 py-2.5 text-sm text-[#717171] hover:text-white border border-[#252525] rounded"
						>
							Cancelar
						</Button>
					)}

					<Button
						type="submit"
						disabled={methods.formState.isSubmitting}
						className="flex-1 px-5 py-2.5 bg-[#a3e635] text-black font-semibold text-sm rounded hover:bg-[#bef264] transition-colors"
					>
						{methods.formState.isSubmitting
							? "Salvando..."
							: user
								? "Salvar alterações"
								: "Criar cadastro"}
					</Button>
				</div>
			</form>
		</FormProvider>
	)
}