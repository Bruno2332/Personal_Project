import { createRoot } from "react-dom/client"
import { flushSync } from "react-dom"
import { toPng } from "html-to-image"
import AssessmentCharts from "../components/assessments/charts/AssessmentCharts"
import type { AssessmentView } from "../utils/assessments"

const printColors: Record<string, string> = {
	
	"#a3e635": "#0f4c5c",
	"rgb(163, 230, 53)": "#0f4c5c",

	
	"#22d3ee": "#b45309",
	"rgb(34, 211, 238)": "#b45309",

	
	"#f59e0b": "#7c3aed",
	"rgb(245, 158, 11)": "#7c3aed",

	
	"#717171": "#475569",
	"rgb(113, 113, 113)": "#475569",

	
	"#252525": "#dbe2e8",
	"rgb(37, 37, 37)": "#dbe2e8",
}

export async function captureAssessmentChart(
	assessments: AssessmentView[],
): Promise<string | null> {
	if (assessments.length < 2) return null

	const container = document.createElement("div")
	container.setAttribute("aria-hidden", "true")

	Object.assign(container.style, {
		position: "fixed",
		left: "-10000px",
		top: "0",
		width: "900px",
		pointerEvents: "none",
	})

	const chartElement = document.createElement("div")

	Object.assign(chartElement.style, {
		width: "900px",
		backgroundColor: "#ffffff",
		color: "#475569",
		fontFamily: "Arial, sans-serif",
	})

	container.appendChild(chartElement)
	document.body.appendChild(container)

	const root = createRoot(chartElement)

	try {
		await document.fonts.ready

		flushSync(() => {
			root.render(
				<AssessmentCharts
					assessments={assessments}
					pdfOnly
				/>,
			)
		})

		let ready = false

		for (let attempt = 0; attempt < 60; attempt++) {
			await new Promise<void>((resolve) => {
				requestAnimationFrame(() => resolve())
			})

			const svg = chartElement.querySelector("svg")
			const line = chartElement.querySelector(
				".recharts-line-curve",
			)

			if (svg && svg.getBoundingClientRect().width > 0 && line) {
				ready = true
				break
			}
		}

		if (!ready) {
			throw new Error(
				"Não foi possível renderizar o gráfico de evolução.",
			)
		}

		// Altera apenas a cópia renderizada para o PDF.
		const elements =
			chartElement.querySelectorAll<HTMLElement | SVGElement>("*")

		for (const element of elements) {
			const computed = window.getComputedStyle(element)

			if (
				computed.backgroundColor !== "transparent" &&
				computed.backgroundColor !== "rgba(0, 0, 0, 0)"
			) {
				element.style.setProperty(
					"background-color",
					"#ffffff",
					"important",
				)
			}

			element.style.setProperty(
				"border-color",
				"#e2e8f0",
				"important",
			)

			const textColor = printColors[computed.color.toLowerCase()]

			if (textColor) {
				element.style.setProperty("color", textColor, "important")
			}

			for (const attribute of ["fill", "stroke"] as const) {
				const currentColor = element.getAttribute(attribute)
				const replacement = currentColor
					? printColors[currentColor.toLowerCase()]
					: undefined

				if (replacement) {
					element.setAttribute(attribute, replacement)
					element.style.setProperty(
						attribute,
						replacement,
						"important",
					)
				}
			}
		}

		// Eixos e datas com texto escuro.
		for (const text of chartElement.querySelectorAll<SVGTextElement>("svg text")) {
			text.style.setProperty("fill", "#475569", "important")
			text.style.setProperty("font-family", "Arial, sans-serif")
		}

		// Remove a moldura do card da tela.
		const chartCard = chartElement.firstElementChild

		if (chartCard instanceof HTMLElement) {
			chartCard.style.setProperty("border", "none", "important")
			chartCard.style.setProperty(
				"background-color",
				"#ffffff",
				"important",
			)
		}

		return await toPng(chartElement, {
			backgroundColor: "#ffffff",
			pixelRatio: 2,
			skipFonts: true,
		})
	} finally {
		root.unmount()
		container.remove()
	}
}