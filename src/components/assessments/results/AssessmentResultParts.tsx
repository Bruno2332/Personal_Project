import type { ReactNode } from "react"
import {
	ClassificationBadge,
	type ClassificationTone,
} from "../../ui/ClassificationBadge"

export const textColors = {
	normal: "text-[#a3e635]",
	attention: "text-yellow-400",
	risk: "text-red-400",
}

export function value(number: number | null, unit = "") {
	return number === null
		? "—"
		: `${number.toLocaleString("pt-BR", { maximumFractionDigits: 2 })}${unit}`
}

export default function Result({
	label,
	children,
	classification,
}: {
	label: string
	children: ReactNode
	classification?: ReactNode
}) {
	return (
		<div className="bg-[#1a1a1a] rounded p-3">
			<div className="text-[10px] text-[#717171] uppercase tracking-wider">
				{label}
			</div>

			<div className="text-lg font-display font-700 text-white mt-1">
				{children}
			</div>

			{classification && (
				<div className="mt-2">{classification}</div>
			)}
		</div>
	)
}

export function badge<T extends string>(
	classification: T | null,
	labels: Record<T, string>,
	tones: Record<T, ClassificationTone>,
) {
	return classification === null ? null : (
		<ClassificationBadge tone={tones[classification]}>
			{labels[classification]}
		</ClassificationBadge>
	)
}