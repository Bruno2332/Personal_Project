import { useEffect, useState } from "react"
import { Button } from "../ui/Button"
import type { UserDto } from "../../model/user"
import { findUserById } from "../../services/userService"
import logo from "../../assets/logo_1.png"

type Page =
	| "agenda"
	| "alunos"
	| "avaliacoes"
	| "financeiro"
	| "backup"

const NAV = [
	{ id: "agenda", label: "Agenda", icon: "▦" },
	{ id: "alunos", label: "Alunos", icon: "◈" },
	{ id: "avaliacoes", label: "Avaliações", icon: "◎" },
	{ id: "financeiro", label: "Financeiro", icon: "◇" },
	{ id: "backup", label: "Backup", icon: "↥" },
] as const

export function SidebarContent({
	page,
	navTo,
	onEditUser,
}: {
	page: Page
	navTo: (page: Page) => void
	onEditUser: () => void
}) {
	const [user, setUser] = useState<UserDto | null>(null)

	useEffect(() => {
		let cancelled = false

		findUserById(1)
			.then((result) => {
				if (!cancelled) {
					setUser(result)
				}
			})
			.catch((error) => {
				if (!cancelled) {
					console.error("Erro ao buscar usuário:", error)
				}
			})

		return () => {
			cancelled = true
		}
	}, [])

	return (
		<>
			<div className="px-5 py-6 border-b border-[#1e1e1e]">
				<div className="flex items-center gap-2">
					<div className="w-25 mx-auto rounded flex items-center justify-center shrink-0">
						<img src={logo} alt="" />
					</div>
				</div>
			</div>

			<nav className="flex-1 px-3 py-4 space-y-0.5">
				{NAV.map((item) => (
					<Button
						key={item.id}
						type="button"
						onClick={() => navTo(item.id)}
						className={`w-full flex items-center gap-3 px-3 py-2.5 rounded text-sm font-medium transition-all ${page === item.id
								? "bg-[#a3e635]/10 text-[#a3e635] border border-[#a3e635]/20"
								: "text-[#717171] hover:text-white hover:bg-[#1a1a1a]"
							}`}
					>
						<span className="text-base leading-none">
							{item.icon}
						</span>

						{item.label}
					</Button>
				))}
			</nav>

			{user && (
				<Button
					type="button"
					onClick={onEditUser}
					aria-label="Editar meu cadastro"
					className="block w-full px-5 py-4 border-t border-[#1e1e1e] text-left hover:bg-[#161616] transition-colors"
				>
					<div className="flex items-center gap-2">
						<div className="w-7 h-7 rounded-full bg-[#252525] flex items-center justify-center text-[#717171] text-xs font-bold font-display shrink-0">
							{user.username
								.trim()
								.split(/\s+/)
								.map((name) => name[0])
								.slice(0, 2)
								.join("")
								.toUpperCase()}
						</div>

						<div className="min-w-0">
							<p className="text-xs text-white font-medium truncate">
								{user.username}
							</p>

							<p className="text-[10px] text-[#717171] truncate">
								Personal · CREF {user.cref}
							</p>
						</div>
					</div>
				</Button>
			)}
		</>
	)
}