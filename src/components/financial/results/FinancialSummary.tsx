import {
	AreaChart,
	Area,
	XAxis,
	YAxis,
	CartesianGrid,
	Tooltip,
	ResponsiveContainer,
	Legend,
} from "recharts"
import { money } from "../../../utils/dates"


function periodLabel(period: string) {
	switch (period) {
		case "all":
			return "Todo o histórico"
		case "1m":
			return "Último mês"
		case "3m":
			return "Últimos 3 meses"
		case "6m":
			return "Últimos 6 meses"
		case "12m":
			return "Último ano"
		default:
			return period
	}
}


const CustomTooltip = ({
	active,
	payload,
	label,
}: {
	active?: boolean
	payload?: Array<{ name: string; value: number; color: string }>
	label?: string
}) => {
	if (!active || !payload?.length) return null
	return (
		<div className="bg-[#1e1e1e] border border-[#252525] rounded p-3 text-xs shadow-xl">
			<p className="text-[#9a9a9a] mb-1.5 font-mono">{label}</p>
			{payload.map((item) => (
				<p key={item.name} style={{ color: item.color }}>
					{item.name}:{" "}
					<span className="font-mono font-medium">
						{money(Number(item.value))}
					</span>
				</p>
			))}
		</div>
	)
}

type Props = {
	balance: number
	income: number
	expense: number
	receivable: number
	payable: number
	period: string
	cashFlow: { month: string; income: number; expense: number }[]
}

export default function FinancialSummary({
	balance,
	income,
	expense,
	receivable,
	payable,
	period,
	cashFlow,
}: Props) {
	return (
		<>
			<div className="grid grid-cols-2 xl:grid-cols-5 gap-3">
				<KPICard
					label="Saldo realizado"
					value={balance}
					color={balance >= 0 ? "#a3e635" : "#f87171"}
					featured
				/>
				<KPICard
					label="Entradas recebidas"
					value={income}
					color="#a3e635"
					icon="↑"
				/>
				<KPICard
					label="Despesas pagas"
					value={expense}
					color="#f87171"
					icon="↓"
				/>
				<KPICard
					label="Contas a receber"
					value={receivable}
					color="#facc15"
					icon="○"
				/>
				<KPICard
					label="Contas a pagar"
					value={payable}
					color="#fb923c"
					icon="○"
				/>
			</div>

			<div className="grid grid-cols-1 xl:grid-cols-[1fr_260px] gap-3">
				<div className="bg-[#161616] border border-[#252525] rounded overflow-hidden">
					<div className="flex items-center justify-between border-b border-[#252525] pr-4">
						<div className="flex">
							<span className="px-5 py-3 text-sm font-medium border-b-2 -mb-px border-[#a3e635] text-[#a3e635]">
								Fluxo de caixa
							</span>
						</div>
						<span className="hidden sm:block text-[10px] text-[#5f5f5f] uppercase tracking-wider">
							Somente valores realizados
						</span>
					</div>
					<div className="p-4" style={{ height: 240 }}>
						<ResponsiveContainer width="100%" height="100%">
							<AreaChart data={cashFlow}>
								<defs>
									<linearGradient id="gEntrada" x1="0" y1="0" x2="0" y2="1">
										<stop offset="5%" stopColor="#a3e635" stopOpacity={0.3} />
										<stop offset="95%" stopColor="#a3e635" stopOpacity={0} />
									</linearGradient>
									<linearGradient id="gSaida" x1="0" y1="0" x2="0" y2="1">
										<stop offset="5%" stopColor="#f87171" stopOpacity={0.3} />
										<stop offset="95%" stopColor="#f87171" stopOpacity={0} />
									</linearGradient>
								</defs>
								<CartesianGrid strokeDasharray="3 3" stroke="#252525" />
								<XAxis
									dataKey="month"
									tick={{ fill: "#717171", fontSize: 11 }}
									axisLine={false}
									tickLine={false}
								/>
								<YAxis
									tick={{ fill: "#717171", fontSize: 11 }}
									axisLine={false}
									tickLine={false}
								/>
								<Tooltip content={<CustomTooltip />} />
								<Legend wrapperStyle={{ fontSize: 11, color: "#717171" }} />
								<Area
									type="monotone"
									dataKey="income"
									name="Entradas"
									stroke="#a3e635"
									fill="url(#gEntrada)"
									strokeWidth={2}
								/>
								<Area
									type="monotone"
									dataKey="expense"
									name="Saídas"
									stroke="#f87171"
									fill="url(#gSaida)"
									strokeWidth={2}
								/>
							</AreaChart>
						</ResponsiveContainer>
					</div>
				</div>

				<div className="bg-[#161616] border border-[#252525] rounded p-5 flex flex-col justify-between">
					<div>
						<p className="text-[10px] text-[#717171] uppercase tracking-[0.16em]">
							Resumo do período
						</p>
						<p className="text-white font-display text-xl mt-1 capitalize">
							{period === "all" ? "Todo o histórico" : periodLabel(period)}
						</p>
					</div>
					<div className="space-y-4 my-5">
						<SummaryRow label="Entrou" value={income} color="#a3e635" />
						<SummaryRow label="Saiu" value={expense} color="#f87171" />
						<div className="h-px bg-[#252525]" />
						<SummaryRow
							label="Resultado"
							value={balance}
							color={balance >= 0 ? "#a3e635" : "#f87171"}
							strong
						/>
					</div>
					<p className="text-[11px] leading-relaxed text-[#5f5f5f]">
						Cobranças dos contratos e lançamentos manuais são exibidos conforme
						os registros salvos.
					</p>
				</div>
			</div>
		</>
	)
}
function KPICard({
	label,
	value,
	color,
	icon,
	featured,
}: {
	label: string
	value: number
	color: string
	icon?: string
	featured?: boolean
}) {
	return (
		<div
			className={`border rounded p-4 ${featured
				? "bg-[#a3e635]/[0.04] border-[#a3e635]/25"
				: "bg-[#161616] border-[#252525]"
				}`}
		>
			<div className="flex items-center justify-between">
				<p className="text-[10px] text-[#717171] uppercase tracking-wider font-medium">
					{label}
				</p>
				{icon && (
					<span style={{ color }} className="text-sm">
						{icon}
					</span>
				)}
			</div>
			<p
				className="text-xl 2xl:text-2xl font-display font-700 mt-2 whitespace-nowrap"
				style={{ color }}
			>
				{money(value)}
			</p>
		</div>
	)
}

function SummaryRow({
	label,
	value,
	color,
	strong,
}: {
	label: string
	value: number
	color: string
	strong?: boolean
}) {
	return (
		<div className="flex items-center justify-between gap-3">
			<span
				className={`${strong ? "text-white font-medium" : "text-[#717171]"
					} text-sm`}
			>
				{label}
			</span>
			<span
				className={`font-mono ${strong ? "text-lg font-bold" : "text-sm font-semibold"
					}`}
				style={{ color }}
			>
				{money(value)}
			</span>
		</div>
	)
}
