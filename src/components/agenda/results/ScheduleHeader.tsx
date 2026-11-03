import { addDays, pad } from "../../../utils/agenda"
import { Button } from "../../ui/Button"

type Props = {
	weekStart: Date
	disabled: boolean
	onToday: () => void
	onPreviousWeek: () => void
	onNextWeek: () => void
	onNew: () => void
}

export default function ScheduleHeader({
	weekStart,
	disabled,
	onToday,
	onPreviousWeek,
	onNextWeek,
	onNew,
}: Props) {
	const weekEnd = addDays(weekStart, 6)

	const sameMonth =
		weekStart.getMonth() === weekEnd.getMonth() &&
		weekStart.getFullYear() === weekEnd.getFullYear()

	const rangeLabel = sameMonth
		? `${pad(weekStart.getDate())}–${pad(weekEnd.getDate())} de ${weekEnd.toLocaleDateString("pt-BR", {
			month: "long",
			year: "numeric",
		})
		}`
		: `${weekStart.toLocaleDateString("pt-BR", {
			day: "2-digit",
			month: "short",
		})} – ${weekEnd.toLocaleDateString("pt-BR", {
			day: "2-digit",
			month: "short",
			year: "numeric",
		})}`

	return (
		<div className="flex flex-col gap-4 lg:flex-row lg:items-end lg:justify-between">
			<div>
				<h1 className="font-display text-4xl font-800 tracking-wide text-white uppercase">
					Agenda
				</h1>

				<p className="text-[#717171] text-sm mt-0.5">
					Aulas semanais e avaliações físicas em um só lugar
				</p>
			</div>

			<div className="flex flex-wrap items-center gap-2">
				<Button
					type="button"
					disabled={disabled}
					onClick={onToday}
					className="px-3 py-2 text-xs text-[#d0d0d0] bg-[#161616] border border-[#252525] rounded hover:border-[#3a3a3a] transition-colors"
				>
					Hoje
				</Button>

				<div className="flex items-center bg-[#161616] border border-[#252525] rounded">
					<Button
						type="button"
						disabled={disabled}
						onClick={onPreviousWeek}
						className="px-3 py-2 text-[#717171] hover:text-white"
						aria-label="Semana anterior"
					>
						‹
					</Button>

					<span className="min-w-[190px] px-2 text-center text-xs text-white capitalize">
						{rangeLabel}
					</span>

					<Button
						type="button"
						disabled={disabled}
						onClick={onNextWeek}
						className="px-3 py-2 text-[#717171] hover:text-white"
						aria-label="Próxima semana"
					>
						›
					</Button>
				</div>

				<Button
					type="button"
					disabled={disabled}
					onClick={onNew}
					className="flex items-center gap-2 px-5 py-2.5 bg-[#a3e635] text-black font-semibold text-sm rounded hover:bg-[#bef264] transition-colors"
				>
					<span className="text-lg leading-none">+</span>
					Novo Horário
				</Button>
			</div>
		</div>
	)
}