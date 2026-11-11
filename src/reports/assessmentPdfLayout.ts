import type { jsPDF } from "jspdf"
import { autoTable, type RowInput } from "jspdf-autotable"

import type { UserDto } from "../model/user"
import { formatDate } from "./assessmentPdfFormatters"
import logo from "../assets/logo_1.png"

export type PdfSection = (
    title: string,
    rows: RowInput[],
    columns?: string[],
) => void

async function loadLogo(): Promise<HTMLImageElement> {
    return new Promise((resolve, reject) => {
        const image = new Image()

        image.onload = () => resolve(image)
        image.onerror = () =>
            reject(
                new Error("Não foi possível carregar o logo do relatório."),
            )

        image.src = logo
    })
}

export default async function createAssessmentPdfLayout(
    doc: jsPDF,
    professional: UserDto,
) {
    const margin = 16
    const pageWidth = doc.internal.pageSize.getWidth()
    const pageHeight = doc.internal.pageSize.getHeight()
    const contentWidth = pageWidth - margin * 2
    const contentBottom = pageHeight - 22

    const image = await loadLogo()
    const logoScale = Math.min(
        22 / image.naturalWidth,
        22 / image.naturalHeight,
    )

    const logoWidth = image.naturalWidth * logoScale
    const logoHeight = image.naturalHeight * logoScale

    doc.addImage(
        image,
        "PNG",
        margin + (22 - logoWidth) / 2,
        14 + (22 - logoHeight) / 2,
        logoWidth,
        logoHeight,
    )

    doc.setFont("helvetica", "bold")
    doc.setFontSize(16)
    doc.setTextColor(0, 0, 0)
    doc.text("AVALIAÇÃO FÍSICA", 44, 23)

    doc.setFont("helvetica", "normal")
    doc.setFontSize(9)
    doc.setTextColor(100, 100, 100)

    const professionalLines: string[] = doc.splitTextToSize(
        `${professional.username} · CREF ${professional.cref}`,
        pageWidth - margin - 44,
    )

    doc.text(professionalLines, 44, 30)

    const dividerY =
        Math.max(36, 30 + (professionalLines.length - 1) * 4) + 7

    doc.setDrawColor(220, 224, 228)
    doc.setLineWidth(0.3)
    doc.line(margin, dividerY, pageWidth - margin, dividerY)

    let y = dividerY + 10

    function newPage() {
        doc.addPage()
        y = 18
    }

    const section: PdfSection = (
        title,
        rows,
        columns = ["Medida", "Resultado", "Classificação"],
    ) => {
        if (!rows.length) return

        if (y + 30 > contentBottom) {
            newPage()
        }

        doc.setFont("helvetica", "bold")
        doc.setFontSize(11)
        doc.setTextColor(0, 0, 0)
        doc.text(title, margin, y)

        autoTable(doc, {
            startY: y + 4,
            margin: {
                top: 18,
                right: margin,
                bottom: 22,
                left: margin,
            },
            head: [columns],
            body: rows,
            theme: "striped",
            styles: {
                font: "helvetica",
                fontSize: 9,
                cellPadding: 2.5,
                textColor: [35, 35, 35],
                overflow: "linebreak",
            },
            headStyles: {
                fillColor: [15, 76, 92],
                textColor: [255, 255, 255],
            },
            alternateRowStyles: {
                fillColor: [245, 247, 248],
            },
            rowPageBreak: "avoid",
            showHead: "everyPage",
            didDrawPage: ({ cursor }) => {
                y = (cursor?.y ?? 18) + 11
            },
        })
    }

    function chart(title: string, source: string | null) {
        if (!source) return

        const properties = doc.getImageProperties(source)

        if (properties.width <= 0 || properties.height <= 0) {
            throw new Error("O gráfico possui dimensões inválidas.")
        }

        let imageWidth = contentWidth
        let imageHeight =
            (properties.height * imageWidth) / properties.width

        const maxImageHeight = contentBottom - 18 - 14

        if (imageHeight > maxImageHeight) {
            imageWidth *= maxImageHeight / imageHeight
            imageHeight = maxImageHeight
        }

        if (y + imageHeight + 14 > contentBottom) {
            newPage()
        }

        doc.setFont("helvetica", "bold")
        doc.setFontSize(11)
        doc.setTextColor(0, 0, 0)
        doc.text(title, margin, y)

        doc.addImage(
            source,
            "PNG",
            margin + (contentWidth - imageWidth) / 2,
            y + 6,
            imageWidth,
            imageHeight,
        )

        y += imageHeight + 14
    }

    function footer(assessmentDate: string) {
        const pageCount = doc.getNumberOfPages()

        for (let page = 1; page <= pageCount; page++) {
            doc.setPage(page)

            doc.setDrawColor(215, 220, 225)
            doc.setLineWidth(0.3)
            doc.line(
                margin,
                pageHeight - 15,
                pageWidth - margin,
                pageHeight - 15,
            )

            doc.setFont("helvetica", "normal")
            doc.setFontSize(8)
            doc.setTextColor(95, 95, 95)

            doc.text(
                `Avaliação de ${formatDate(assessmentDate)}`,
                margin,
                pageHeight - 9,
            )

            doc.text(
                `${page} / ${pageCount}`,
                pageWidth - margin,
                pageHeight - 9,
                { align: "right" },
            )
        }
    }

    return { section, chart, footer }
}