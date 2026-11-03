import { Button } from "./Button"


export function Tabs<T extends string>({
	items,
	value,
	onChange,
}: {
	items: readonly { id: T; label: string }[]
	value: T
	onChange: (value: T) => void
}) {
	return (
		<div
			className="flex border-b border-[#252525] overflow-x-auto"
			role="tablist"
		>
			{items.map((tab) => (
				<Button
					key={tab.id}
					role="tab"
					aria-selected={value === tab.id}
					onClick={() => onChange(tab.id)}
					className={`py-3 px-4 text-sm font-medium border-b-2 -mb-px transition-colors whitespace-nowrap ${value === tab.id
						? "border-[#a3e635] text-[#a3e635]"
						: "border-transparent text-[#717171] hover:text-white"
						}`}
				>
					{tab.label}
				</Button>
			))}
		</div>
	)
}
