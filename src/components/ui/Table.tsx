import type { ComponentPropsWithoutRef } from "react"

export function Table(props: ComponentPropsWithoutRef<"table">) {
	return <table {...props} />
}

export function TableHeader({
	columns,
	className = "text-left px-4 py-3 text-xs font-medium text-[#717171] uppercase tracking-wider",
}: {
	columns: string[]
	className?: string
}) {
	return (
		<thead>
			<tr className="border-b border-[#252525]">
				{columns.map((column, index) => (
					<th key={`${column}-${index}`} scope="col" className={className}>
						{column}
					</th>
				))}
			</tr>
		</thead>
	)
}

export function EmptyTableRow({
	columns,
	children,
}: {
	columns: number
	children: React.ReactNode
}) {
	return (
		<tr>
			<td colSpan={columns} className="px-4 py-12 text-center text-[#717171]">
				{children}
			</td>
		</tr>
	)
}
