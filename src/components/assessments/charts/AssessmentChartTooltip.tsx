
type Props = {
	active?: boolean
	payload?: Array<{
		name?: string | number
		value?: number | string | Array<number | string>
		color?: string
	}>
	label?: string | number
}

export default function AssessmentChartTooltip({
	active,
	payload,
	label,
}: Props) {
	if (!active || !payload?.length) return null

	return (
		<div className="bg-[#1e1e1e] border border-[#252525] rounded p-3 text-xs">
			<p className="text-[#717171] mb-1.5 font-mono">
				{label}
			</p>

			{payload.map((item, index) => (
				<p
					key={`${item.name}-${index}`}
					style={{ color: item.color }}
				>
					{item.name}:{" "}
					<span className="font-mono font-medium">
						{Array.isArray(item.value)
							? item.value.join(" – ")
							: item.value}
					</span>
				</p>
			))}
		</div>
	)
}