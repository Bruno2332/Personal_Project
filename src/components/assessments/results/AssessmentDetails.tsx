import type { PhysicalAssessmentCompleteDto } from "../../../model/physicalAssessment"
import type { AssessmentView } from "../../../utils/assessments"
import {
	assessmentTabs,
	type AssessmentTab,
} from "../forms/assessmentFields"
import AssessmentResults from "../results/AssessmentResults"
import { Tabs } from "../../ui/Tabs"
import { Button } from "../../ui/Button"
import { ErrorMessage } from "../../ui/Feedback"
import { dateLabel } from "../../../utils/dates"

type Props = {
	assessment: AssessmentView
	tab: AssessmentTab
	busy: boolean
	loading: boolean
	exportingPdf: boolean
	pdfError: string
	pdfMessage: string
	onTabChange: (tab: AssessmentTab) => void
	onGeneratePdf: () => void
	onEdit: (assessment: PhysicalAssessmentCompleteDto) => void
	onRemove: () => void
	detailsRef: React.RefObject<HTMLDivElement | null>
}


export default function AssessmentDetails({
	assessment,
	tab,
	busy,
	loading,
	exportingPdf,
	pdfError,
	pdfMessage,
	onTabChange,
	onGeneratePdf,
	onEdit,
	onRemove,
	detailsRef,
}: Props) {
	return (
		<div
			ref={detailsRef}
			className="scroll-mt-16 bg-[#161616] border border-[#252525] rounded overflow-hidden"
		>
			<div className="px-4 py-3 flex flex-wrap justify-between items-center gap-3">
				<h4 className="text-xs text-[#717171] uppercase tracking-wider font-medium">
					Avaliação de {dateLabel(assessment.data)}
				</h4>

				<div className="flex items-center gap-3">
					<Button
						type="button"
						disabled={busy || loading}
						onClick={onGeneratePdf}
						className="px-3 py-1.5 text-sm bg-[#a3e635] text-black font-semibold rounded hover:bg-[#bef264] transition-colors"
					>
						{exportingPdf ? "Gerando PDF..." : "Gerar PDF"}
					</Button>

					<Button
						type="button"
						disabled={busy}
						onClick={() => onEdit(assessment.raw)}
						className="text-sm text-[#a3e635]"
					>
						Editar
					</Button>

					<Button
						type="button"
						disabled={busy}
						onClick={onRemove}
						className="text-sm text-red-400"
					>
						Excluir
					</Button>
				</div>
			</div>

			{pdfError && (
				<div className="px-4 pb-3">
					<ErrorMessage message={pdfError} />
				</div>
			)}

			{pdfMessage && (
				<p
					role="status"
					className="px-4 pb-3 text-sm text-[#a3e635]"
				>
					{pdfMessage}
				</p>
			)}

			<Tabs
				items={assessmentTabs}
				value={tab}
				onChange={onTabChange}
			/>

			<div className="p-4">
				<AssessmentResults
					result={assessment.raw}
					tab={tab}
				/>
			</div>
		</div>
	)
}