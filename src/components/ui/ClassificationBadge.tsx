import type { ReactNode } from "react"


export type ClassificationTone = "normal" | "attention" | "risk"
const styles: Record<ClassificationTone, string> = {
	normal: "text-[#a3e635] bg-[#a3e635]/10",
	attention: "text-yellow-400 bg-yellow-400/10",
	risk: "text-red-400 bg-red-400/10",
}


export function ClassificationBadge({
	children,
	tone,
}: {
	children: ReactNode
	tone: ClassificationTone
}) {
	return (
		<span
			className={`inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ${styles[tone]}`}
		>
			{children}
		</span>
	)
}
