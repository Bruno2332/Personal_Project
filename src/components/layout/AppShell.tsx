import { useEffect, useState } from "react"
import { Button } from "../ui/Button"
import { SidebarContent } from "./SidebarContent"
import Students from "../../pages/Students"
import Assessments from "../../pages/Assessments"
import Finance from "../../pages/Finance"
import Schedule from "../../pages/Schedule"
import Backup from "../../pages/Backup"
import { generateMonthlyCharges } from "../../services/financialRecordService"

type Page =
	| "agenda"
	| "alunos"
	| "avaliacoes"
	| "financeiro"
	| "backup"

export function AppShell({
	onEditUser,
}: {
	onEditUser: () => void
}) {
	const [page, setPage] = useState<Page>("agenda")
	const [sidebarOpen, setSidebarOpen] = useState(false)

	function navTo(id: Page) {
		setPage(id)
		setSidebarOpen(false)
	}

	function editUser() {
		setSidebarOpen(false)
		onEditUser()
	}

	useEffect(() => {
		generateMonthlyCharges()
			.then((quantity) => {
				console.log(`Mensalidades geradas: ${quantity}`)
			})
			.catch((error) => {
				console.error("Erro ao gerar mensalidades:", error)
			})
	}, [])

	return (
		<div className="min-h-screen bg-[#0d0d0d] flex">
			<aside className="hidden md:flex w-56 min-h-screen bg-[#0a0a0a] border-r border-[#1e1e1e] flex-col fixed top-0 left-0 z-20">
				<SidebarContent
					page={page}
					navTo={navTo}
					onEditUser={editUser}
				/>
			</aside>

			{sidebarOpen && (
				<div
					className="fixed inset-0 z-40 md:hidden"
					onClick={() => setSidebarOpen(false)}
				>
					<div className="absolute inset-0 bg-black/60 backdrop-blur-sm" />

					<aside
						className="absolute left-0 top-0 bottom-0 w-64 bg-[#0a0a0a] border-r border-[#1e1e1e] flex flex-col"
						onClick={(event) => event.stopPropagation()}
					>
						<SidebarContent
							page={page}
							navTo={navTo}
							onEditUser={editUser}
						/>
					</aside>
				</div>
			)}

			<div className="md:hidden fixed top-0 left-0 right-0 z-30 bg-[#0a0a0a] border-b border-[#1e1e1e] flex items-center justify-between px-4 h-14">
				<Button
					type="button"
					onClick={() => setSidebarOpen(true)}
					aria-label="Abrir menu"
					className="text-[#717171] hover:text-white transition-colors p-1"
				>
					<svg
						width="20"
						height="20"
						viewBox="0 0 20 20"
						fill="none"
					>
						<path
							d="M3 5h14M3 10h14M3 15h14"
							stroke="currentColor"
							strokeWidth="1.5"
							strokeLinecap="round"
						/>
					</svg>
				</Button>

				<div className="flex items-center gap-2">
					<div className="w-6 h-6 bg-[#a3e635] rounded flex items-center justify-center">
						<span className="text-black text-[10px] font-bold font-display">
							PT
						</span>
					</div>

					<span className="font-display text-sm font-700 uppercase tracking-wider text-white">
						Personal Trainer Pro
					</span>
				</div>

				<div className="w-8" />
			</div>

			<main className="flex-1 md:ml-56 min-h-screen pt-14 md:pt-0">
				<div className="max-w-6xl mx-auto p-4 md:p-8">
					{page === "agenda" && <Schedule />}
					{page === "alunos" && <Students />}
					{page === "avaliacoes" && <Assessments />}
					{page === "financeiro" && <Finance />}
					{page === "backup" && <Backup />}
				</div>
			</main>
		</div>
	)
}