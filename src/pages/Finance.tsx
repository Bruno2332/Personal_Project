import { FormProvider } from "react-hook-form"

import FinancialModal from "../components/financial/forms/FinancialModal"
import useFinance from "../components/financial/hooks/useFinance"
import FinancialFilters from "../components/financial/results/FinancialFilters"
import FinancialSummary from "../components/financial/results/FinancialSummary"
import FinancialTable from "../components/financial/results/FinancialTable"
import { Button } from "../components/ui/Button"
import { ErrorMessage, LoadingMessage } from "../components/ui/Feedback"
import { Pagination } from "../components/ui/Pagination"



export default function Finance() {
	const finance = useFinance()

	return (
		<div className="flex flex-col h-full gap-6">
			<ErrorMessage message={finance.error} />
			<ErrorMessage message={finance.listError} />
			<ErrorMessage message={finance.summaryError} />
			{(finance.listError || finance.summaryError) && (
				<Button type="button" onClick={finance.retry}>
					Tentar novamente
				</Button>
			)}

			<div className="flex flex-col lg:flex-row lg:items-end justify-between gap-4">
				<div>
					<div className="flex items-center gap-3">
						<h1 className="font-display text-4xl font-800 tracking-wide text-white uppercase">
							Financeiro
						</h1>
						<span className="text-[10px] uppercase tracking-[0.18em] px-2 py-1 rounded bg-[#a3e635]/10 text-[#a3e635] border border-[#a3e635]/20">
							Livro-caixa
						</span>
					</div>
					<p className="text-[#717171] text-sm mt-1">
						Receitas dos alunos e todas as despesas do negócio no mesmo lugar
					</p>
				</div>

				<div className="flex flex-col sm:flex-row gap-2">
					<Button
						type="button"
						onClick={() => finance.openRecord("Income")}
						className="flex items-center justify-center gap-2 px-4 py-2.5 border border-[#a3e635]/50 text-[#a3e635] font-semibold text-sm rounded hover:bg-[#a3e635]/10 transition-colors"
					>
						<span className="text-lg leading-none">+</span> Nova entrada
					</Button>
					<Button
						type="button"
						onClick={() => finance.openRecord("Expense")}
						className="flex items-center justify-center gap-2 px-4 py-2.5 bg-red-500 text-white font-semibold text-sm rounded hover:bg-red-400 transition-colors"
					>
						<span className="text-lg leading-none">−</span> Nova despesa
					</Button>
				</div>
			</div>

			{finance.summary !== null && (
				<FinancialSummary {...finance.periodSummary} period={finance.period} />
			)}

			<FormProvider {...finance.filters}>
				<FinancialFilters
					typeFilter={finance.typeFilter}
					search={finance.search}
					onTypeFilter={(value) => {
						finance.setTypeFilter(value)
						finance.resetPage()
					}}
					onSearch={(value) => {
						finance.setSearch(value)
						finance.resetPage()
					}}
				/>
			</FormProvider>

			<div
				role="tablist"
				aria-label="Categorias financeiras"
				className="flex border-b border-[#252525]"
			>
				{finance.tabs.map((item) => (
					<button
						key={item.id}
						type="button"
						role="tab"
						aria-selected={finance.tab === item.id}
						onClick={() => {
							finance.setTab(item.id)
							finance.resetPage()
						}}
						className={`px-4 py-3 text-sm font-medium border-b-2 transition-colors ${finance.tab === item.id
							? "border-[#a3e635] text-[#a3e635]"
							: "border-transparent text-[#717171] hover:text-white"
							}`}
					>
						{item.label}
					</button>
				))}
			</div>

			<div className="relative" aria-busy={finance.loading}>
				<FinancialTable
					items={finance.records}
					total={finance.total}
					busyId={finance.busyId}
					onMarkAsPaid={finance.markAsPaid}
					onCancel={(record) => void finance.changeStatus(record, true)}
					onEdit={finance.editRecord}
				/>

				{finance.loading && (
					<div className="absolute inset-0 z-10 bg-[#161616]/70 cursor-wait">
						<LoadingMessage />
					</div>
				)}
			</div>

			<Pagination
				page={finance.page}
				size={finance.pageSize}
				total={finance.total}
				onPage={finance.setPage}
			/>

			{finance.modalOpen && (
				<FinancialModal
					record={finance.selected}
					kind={finance.kind}
					onSaved={finance.onSaved}
					onClose={finance.closeModal}
				/>
			)}
		</div>
	)
}
