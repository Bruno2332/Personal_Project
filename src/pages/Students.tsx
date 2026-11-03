import StudentModal from "../components/students/forms/StudentModal"
import useStudentsPage from "../components/students/hooks/useStudentsPage"
import StudentFilters from "../components/students/results/StudentFilters"
import StudentTable from "../components/students/results/StudentTable"
import { Button } from "../components/ui/Button"
import { Pagination } from "../components/ui/Pagination"



export default function Students() {
	const page = useStudentsPage()

	return (
		<div className="flex flex-col h-full gap-6">
			<div className="flex items-center justify-between">
				<div>
					<h1 className="font-display text-4xl font-800 tracking-wide text-white uppercase">
						Alunos
					</h1>
					<p className="text-[#717171] text-sm mt-0.5">
						{page.loading
							? "Carregando..."
							: `${page.total} ${page.search.trim() ? "encontrados" : "total"}`}
					</p>
				</div>

				<Button
					type="button"
					disabled={page.loadingStudent}
					onClick={page.openNewStudent}
					className="flex items-center gap-2 px-5 py-2.5 bg-[#a3e635] text-black font-semibold text-sm rounded hover:bg-[#bef264] transition-colors"
				>
					<span className="text-lg leading-none">+</span>
					Novo Aluno
				</Button>
			</div>

			<StudentFilters search={page.search} onSearch={page.updateSearch} />

			{page.detailsError && (
				<p role="alert" className="text-sm text-red-400">
					{page.detailsError}
				</p>
			)}

			{page.loadingStudent && (
				<p role="status" className="text-sm text-[#717171]">
					Carregando dados do aluno...
				</p>
			)}

			{page.loading ? (
				<p role="status" className="text-sm text-[#717171]">
					Carregando alunos...
				</p>
			) : page.error ? (
				<div className="flex items-center gap-3">
					<p role="alert" className="text-sm text-red-400">
						{page.error}
					</p>
					<Button
						type="button"
						onClick={page.retry}
						className="px-3 py-1.5 rounded text-sm border border-[#252525] text-[#717171] hover:text-white"
					>
						Tentar novamente
					</Button>
				</div>
			) : (
				<>
					<StudentTable
						students={page.students}
						onSelect={(student) => void page.openStudent(student)}
					/>
					<Pagination
						page={page.page}
						size={page.pageSize}
						total={page.total}
						onPage={page.setPage}
					/>
				</>
			)}

			{page.modalOpen && (
				<StudentModal
					key={page.selectedStudent?.id ?? "new"}
					student={page.selectedStudent}
					onClose={page.closeModal}
				/>
			)}
		</div>
	)
}
