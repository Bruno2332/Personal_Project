import type { ComponentPropsWithoutRef } from "react"

type SearchInputProps = ComponentPropsWithoutRef<"input"> & {
	containerClassName?: string
	showIcon?: boolean
}

export function SearchInput({
	containerClassName = "relative flex-1 min-w-[180px] max-w-sm",
	showIcon = true,
	className = "w-full pl-8 pr-4 py-2 bg-[#161616] border border-[#252525] rounded text-sm text-white placeholder:text-[#717171] focus:outline-none focus:border-[#a3e635] transition-colors",
	...props
}: SearchInputProps) {
	return (
		<div className={containerClassName}>
			{showIcon && (
				<span
					aria-hidden="true"
					className="absolute left-3 top-1/2 -translate-y-1/2 text-[#717171] text-sm"
				>
					⌕
				</span>
			)}
			<input
				aria-label={props.placeholder ?? "Buscar"}
				{...props}
				className={className}
			/>
		</div>
	)
}
