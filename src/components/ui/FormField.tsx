import { type ReactNode } from "react"

type FormFieldProps = {
	label: string
	children: ReactNode
	className?: string
	htmlFor?: string
	error?: string
}

export const inputClass =
	"w-full px-3 py-2 bg-[#0d0d0d] border border-[#252525] rounded text-sm text-white placeholder:text-[#717171] focus:outline-none focus:border-[#a3e635] transition-colors"
export const agendaInputClass =
	"w-full px-3 py-2.5 bg-[#161616] border border-[#252525] rounded text-sm text-white placeholder:text-[#5f5f5f] focus:outline-none focus:border-[#a3e635] transition-colors"

export function FormField({
	label,
	children,
	className = "",
	htmlFor,
	error,
}: FormFieldProps) {
	return (
		<div className={className}>
			<label
				htmlFor={htmlFor}
				className="block text-xs text-[#717171] uppercase tracking-wider font-medium mb-1.5 pl-2"
			>
				{label}
			</label>
			{children}
			{error && (
				<p role="alert" className="text-xs text-red-400 mt-1">
					{error}
				</p>
			)}
		</div>
	)
}
