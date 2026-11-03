import { useEffect, useRef, type ReactNode } from "react"

type Props = {
	label: string
	onClose: () => void
	busy?: boolean
	backdropClassName?: string
	children: ReactNode
}

export function Modal({
	label,
	onClose,
	busy = false,
	backdropClassName = "bg-black/70 backdrop-blur-sm",
	children,
}: Props) {
	const root = useRef<HTMLDivElement>(null)
	const closeRef = useRef(onClose)
	const busyRef = useRef(busy)
	closeRef.current = onClose
	busyRef.current = busy
	useEffect(() => {
		const previous = document.activeElement as HTMLElement | null
		const overflow = document.body.style.overflow
		document.body.style.overflow = "hidden"
		const focusables = () =>
			Array.from(
				root.current?.querySelectorAll<HTMLElement>(
					'button:not([disabled]),input:not([disabled]),select:not([disabled]),textarea:not([disabled]),a[href],[tabindex="0"]',
				) ?? [],
			).filter(
				(element) =>
					!element.closest("[hidden]") &&
					element.getAttribute("aria-hidden") !== "true",
			)
		const timer = window.setTimeout(() => {
			; (focusables()[0] ?? root.current)?.focus()
		}, 0)
		function key(event: KeyboardEvent) {
			if (event.key === "Escape" && !busyRef.current) {
				event.preventDefault()
				closeRef.current()
			}
			if (event.key === "Tab") {
				const items = focusables()
				if (!items.length) {
					event.preventDefault()
					return
				}
				if (event.shiftKey && document.activeElement === items[0]) {
					event.preventDefault()
					items[items.length - 1].focus()
				} else if (
					!event.shiftKey &&
					document.activeElement === items[items.length - 1]
				) {
					event.preventDefault()
					items[0].focus()
				}
			}
		}
		document.addEventListener("keydown", key)
		return () => {
			window.clearTimeout(timer)
			document.removeEventListener("keydown", key)
			document.body.style.overflow = overflow
			previous?.focus()
		}
	}, [])
	return (
		<div
			ref={root}
			tabIndex={-1}
			className="fixed inset-0 z-50 flex items-center justify-center p-4"
			role="dialog"
			aria-modal="true"
			aria-label={label}
		>
			<div
				className={`absolute inset-0 ${backdropClassName}`}
				onClick={() => !busy && onClose()}
			/>
			{children}
		</div>
	)
}
