import { createRoot } from "react-dom/client"
import { flushSync } from "react-dom"
import { toPng } from "html-to-image"

import AssessmentVo2Chart from "../components/assessments/charts/AssessmentVo2Chart"
import type { AssessmentView } from "../utils/assessments"

const printColors: Record<string, string> = {
	"#22d3ee": "#0f4c5c",
	"rgb(34, 211, 238)": "#0f4c5c",

	"#717171": "#475569",
	"rgb(113, 113, 113)": "#475569",

	"#252525": "#dbe2e8",
	"rgb(37, 37, 37)": "#dbe2e8",
}

export async function captureAssessmentVo2Chart(
	assessments: AssessmentView[],
): Promise<string | null> {
	if (assessments.length < 2) return null

	const hasData = assessments.some(
		(assessment) =>
			assessment.raw.vo2_parameter != null &&
			Number.isFinite(assessment.raw.vo2_parameter),
	)

	if (!hasData) return null

	const orderedAssessments = [...assessments].sort(
		(a, b) =>
			a.data.localeCompare(b.data) ||
			(a.id ?? 0) - (b.id ?? 0),
	)

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
		height: "300px",
		padding: "16px",
		boxSizing: "border-box",
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
				<AssessmentVo2Chart
					assessments={orderedAssessments}
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

			// Com um único resultado, pode haver apenas um ponto.
			const plottedData = chartElement.querySelector(
				".recharts-line-curve, .recharts-line-dot",
			)

			if (
				svg &&
				svg.getBoundingClientRect().width > 0 &&
				svg.getBoundingClientRect().height > 0 &&
				plottedData
			) {
				ready = true
				break
			}
		}

		if (!ready) {
			throw new Error(
				"Não foi possível renderizar o gráfico de VO₂.",
			)
		}

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

			const textColor =
				printColors[computed.color.toLowerCase()]

			if (textColor) {
				element.style.setProperty(
					"color",
					textColor,
					"important",
				)
			}

			for (const attribute of ["fill", "stroke"] as const) {
				const currentColor =
					element.getAttribute(attribute) ??
					computed.getPropertyValue(attribute)

				const replacement =
					printColors[currentColor.trim().toLowerCase()]

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

		for (
			const text of chartElement.querySelectorAll<SVGTextElement>(
				"svg text",
			)
		) {
			text.style.setProperty("fill", "#000000", "important")
			text.style.setProperty(
				"font-family",
				"Arial, sans-serif",
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