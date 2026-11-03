import { useEffect, useState, type ReactNode } from "react"
import type { UserDto } from "../../model/user"
import { existsUser, findUserById } from "../../services/userService"
import { Button } from "../ui/Button"
import { ErrorMessage } from "../ui/Feedback"
import { errorMessage } from "../../utils/errors"
import UserForm from "./userForm"
import logo from "../../assets/logo_1.png"


export default function UserSetup({
	children,
}: {
	children: (onEditUser: () => void) => ReactNode
}) {
	const [user, setUser] = useState<UserDto | null>(null)
	const [loading, setLoading] = useState(true)
	const [editing, setEditing] = useState(false)
	const [error, setError] = useState("")
	const [attempt, setAttempt] = useState(0)

	useEffect(() => {
		let cancelled = false

		async function loadUser() {
			setLoading(true)
			setError("")

			try {
				const exists = await existsUser()
				const registeredUser = exists
					? await findUserById(1)
					: null

				if (!cancelled) {
					setUser(registeredUser)
				}
			} catch (error) {
				if (!cancelled) {
					setError(errorMessage(error))
				}
			} finally {
				if (!cancelled) {
					setLoading(false)
				}
			}
		}

		void loadUser()

		return () => {
			cancelled = true
		}
	}, [attempt])

	if (!loading && !error && user && !editing) {
		return <>{children(() => setEditing(true))}</>
	}

	return (
		<div className="min-h-screen bg-[#0d0d0d] flex items-center justify-center p-4">
			<div className="w-full max-w-md bg-[#111] border border-[#252525] rounded-lg shadow-2xl overflow-hidden">
				<div className="px-6 py-6 border-b border-[#252525]">
					<div className="flex items-center gap-2">
						<div className="w-12 h-12 rounded flex items-center justify-center shrink-0">
							<img src={logo} alt="" />
						</div>

						<span className="font-display text-xl font-700 uppercase tracking-wide text-white">
							Jonatas Sciencia Personal Trainer
						</span>
					</div>

					<h1 className="font-display text-3xl font-700 uppercase text-white mt-5">
						{loading
							? "Carregando"
							: error
								? "Cadastro"
								: user
									? "Editar cadastro"
									: "Primeiro acesso"}
					</h1>
				</div>

				<div className="p-6">
					{loading ? (
						<p role="status" className="text-sm text-[#717171]">
							Carregando...
						</p>
					) : error ? (
						<div className="space-y-3">
							<ErrorMessage message={error} />

							<Button
								type="button"
								onClick={() => setAttempt((value) => value + 1)}
								className="text-sm text-[#a3e635]"
							>
								Tentar novamente
							</Button>
						</div>
					) : (
						<UserForm
							user={user}
							onSaved={(saved) => {
								setUser(saved)
								setEditing(false)
							}}
							onCancel={() => setEditing(false)}
						/>
					)}
				</div>
			</div>
		</div>
	)
}