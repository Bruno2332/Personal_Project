import { jsPDF } from "jspdf"
import {
    autoTable,
    type CellDef,
    type RowInput,
} from "jspdf-autotable"
import type { PhysicalAssessmentCompleteDto } from "../model/physicalAssessment"
import type { StudentDto } from "../model/student"
import type { UserDto } from "../model/user"
import type { ClassificationTone } from "../components/ui/ClassificationBadge"
import {
    circumferenceFields,
    skinfoldFields,
} from "../components/assessments/forms/assessmentFields"
import * as labels from "../labels/physicalAssessment"
import * as tones from "../labels/classificationTone"
import logo from "../assets/logo_1.png"

const colors: Record<ClassificationTone, [number, number, number]> = {
    normal: [35, 112, 51],
    attention: [139, 91, 0],
    risk: [177, 35, 35],
}

function classification<T extends string>(
    value: T | null | undefined,
    names: Record<T, string>,
    severity: Record<T, ClassificationTone>,
): CellDef {
    if (value == null) return { content: "" }

    return {
        content: names[value],
        styles: {
            textColor: colors[severity[value]],
            fontStyle: "bold",
        },
    }
}

function formatNumber(
    value: number | null | undefined,
    unit = "",
): string {
    if (value == null || !Number.isFinite(value)) return "—"

    return `${value.toLocaleString("pt-BR", {
        maximumFractionDigits: 2,
    })}${unit}`
}

function formatDate(value: string): string {
    const [year, month, day] = value.slice(0, 10).split("-")
    return `${day}/${month}/${year}`
}

function measurement(
    label: string,
    value: number | null | undefined,
    unit = "",
    result: CellDef = { content: "" },
): RowInput[] {
    if (value == null) return []

    return [[label, formatNumber(value, unit), result]]
}

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

export async function createAssessmentPdf(
    result: PhysicalAssessmentCompleteDto,
    student: StudentDto,
    professional: UserDto,
    chartImage: string | null = null,
): Promise<Uint8Array> {
    const assessment = result.physical_assessment_dto

    if (student.id !== assessment.student_id) {
        throw new Error(
            "O aluno não corresponde à avaliação selecionada.",
        )
    }

    const doc = new jsPDF({
        orientation: "portrait",
        unit: "mm",
        format: "a4",
    })

    doc.setProperties({
        title: `Avaliação física — ${student.name} — ${formatDate(
            assessment.assessment_date,
        )}`,
        author: professional.username,
        subject: "Relatório de avaliação física",
    })

    const image = await loadLogo()

    const logoScale = Math.min(
        22 / image.naturalWidth,
        22 / image.naturalHeight,
    )

    const logoWidth = image.naturalWidth * logoScale
    const logoHeight = image.naturalHeight * logoScale

    // Logo centralizado em uma área de 22 × 22 mm.
    doc.addImage(
        image,
        "PNG",
        16 + (22 - logoWidth) / 2,
        14 + (22 - logoHeight) / 2,
        logoWidth,
        logoHeight,
    )

    // Título ao lado do logo.
    doc.setFont("helvetica", "bold")
    doc.setFontSize(16)
    doc.setTextColor(0, 0, 0)
    doc.text("AVALIAÇÃO FÍSICA", 44, 23)

    // Identificação do profissional.
    doc.setFont("helvetica", "normal")
    doc.setFontSize(9)
    doc.setTextColor(100, 100, 100)

    const professionalLines = doc.splitTextToSize(
        `${professional.username} · CREF ${professional.cref}`,
        150,
    )

    doc.text(professionalLines, 44, 30)

    // Linha separadora abaixo do cabeçalho.
    const dividerY = Math.max(
        36,
        30 + (professionalLines.length - 1) * 4,
    ) + 7

    doc.setDrawColor(220, 224, 228)
    doc.setLineWidth(0.3)
    doc.line(16, dividerY, 194, dividerY)

    // Início da identificação.
    let y = dividerY + 10

    function section(
        title: string,
        rows: RowInput[],
        columns = ["Medida", "Resultado", "Classificação"],
    ) {
        if (!rows.length) return

        if (y > 248) {
            doc.addPage()
            y = 18
        }

        doc.setFont("helvetica", "bold")
        doc.setFontSize(11)
        doc.setTextColor(0, 0, 0)
        doc.text(title, 16, y)

        autoTable(doc, {
            startY: y + 4,
            margin: {
                top: 18,
                right: 16,
                bottom: 18,
                left: 16,
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

    section(
        "Identificação",
        [
            ["Aluno", student.name],
            [
                "Data da avaliação",
                formatDate(assessment.assessment_date),
            ],
            ["Idade na avaliação", `${result.age} anos`],
            [
                "Sexo",
                student.gender === "Male" ? "Masculino" : "Feminino",
            ],
        ],
        ["Informação", "Dados"],
    )

    section("Dados gerais", [
        ...measurement("Peso", assessment.weight_kg, " kg"),
        ...measurement("Altura", assessment.height_m, " m"),
        ...measurement(
            "IMC",
            result.bmi,
            " kg/m²",
            classification(
                result.bmi_classification,
                labels.bmiLabels,
                tones.bmiTones,
            ),
        ),
        ...measurement(
            "Peso ideal mínimo",
            result.min_ideal_weight,
            " kg",
        ),
        ...measurement(
            "Peso ideal médio",
            result.medium_ideal_weight,
            " kg",
        ),
        ...measurement(
            "Peso ideal máximo",
            result.max_ideal_weight,
            " kg",
        ),
        ...(assessment.systolic_pressure != null &&
            assessment.diastolic_pressure != null
            ? [
                [
                    "Pressão arterial",
                    `${assessment.systolic_pressure}/${assessment.diastolic_pressure} mmHg`,
                    classification(
                        result.hypertension_classification,
                        labels.pressureLabels,
                        tones.pressureTones,
                    ),
                ] as RowInput,
            ]
            : [
                ...measurement(
                    "Pressão sistólica",
                    assessment.systolic_pressure,
                    " mmHg",
                ),
                ...measurement(
                    "Pressão diastólica",
                    assessment.diastolic_pressure,
                    " mmHg",
                ),
            ]),
        ...measurement(
            "Frequência cardíaca em repouso",
            assessment.resting_heart_rate_bpm,
            " bpm",
            classification(
                result.fc_heart_rest_classification,
                labels.fcLabels,
                tones.fcTones,
            ),
        ),
    ])

    section("Circunferências", [
        ...circumferenceFields.flatMap((field) => {
            const value = assessment[field.name]

            if (typeof value !== "number") return []

            return measurement(
                field.label,
                value,
                " cm",
                field.name === "abdomen_circumference_cm"
                    ? classification(
                        result.circumference_abdominal_classification,
                        labels.abdomenLabels,
                        tones.abdomenTones,
                    )
                    : { content: "" },
            )
        }),
        ...measurement(
            "Relação cintura/quadril",
            result.waist_hip_calculate,
            "",
            classification(
                result.waist_hip_classification,
                labels.waistHipLabels,
                tones.waistHipTones,
            ),
        ),
    ])

    section("Composição corporal — pregas cutâneas", [
        ...measurement(
            "Gordura corporal",
            result.fat_percentage_skinfolds,
            " %",
            classification(
                result.fat_percentage_classification_skinfolds,
                labels.fatLabels,
                tones.fatTones,
            ),
        ),
        ...measurement(
            "Massa gorda",
            result.kilos_fat_skinfolds,
            " kg",
        ),
        ...measurement(
            "Massa magra",
            result.kilos_lean_body_mass_skinfolds,
            " kg",
        ),
        ...measurement(
            "Percentual de massa magra",
            result.lean_body_mass_percentage_skinfolds,
            " %",
        ),
        ...skinfoldFields.flatMap((field) => {
            const value = assessment[field.name]

            return typeof value === "number"
                ? measurement(field.label, value, " mm")
                : []
        }),
    ])

    section("Composição corporal — bioimpedância", [
        ...measurement(
            "Gordura corporal",
            assessment.bioimpedance_fat_percentage,
            " %",
            classification(
                result.fat_percentage_classification_bioimpedance,
                labels.fatLabels,
                tones.fatTones,
            ),
        ),
        ...measurement(
            "Massa gorda",
            result.kilos_fat_bioimpedance,
            " kg",
        ),
        ...measurement(
            "Massa magra",
            result.kilos_lean_body_mass_bioimpedance,
            " kg",
        ),
        ...measurement(
            "Percentual de massa magra",
            result.lean_body_mass_percentage_bioimpedance,
            " %",
        ),
    ])

    section("Força — dinamometria", [
        ...measurement(
            "Dinamômetro direito",
            assessment.right_handgrip_strength,
            " kgf",
            classification(
                result.right_dynamometer_strength_classification,
                labels.dynamometerLabels,
                tones.dynamometerTones,
            ),
        ),
        ...measurement(
            "Dinamômetro esquerdo",
            assessment.left_handgrip_strength,
            " kgf",
            classification(
                result.left_dynamometer_strength_classification,
                labels.dynamometerLabels,
                tones.dynamometerTones,
            ),
        ),
    ])

    section("Força muscular e assimetrias", [
        ...measurement(
            "Bíceps direito",
            assessment.right_biceps_strength,
            " kg",
        ),
        ...measurement(
            "Bíceps esquerdo",
            assessment.left_biceps_strength,
            " kg",
        ),
        
        ...measurement(
            "Tríceps direito",
            assessment.right_triceps_strength,
            " kg",
        ),
        ...measurement(
            "Tríceps esquerdo",
            assessment.left_triceps_strength,
            " kg",
        ),
        
        ...measurement(
            "Extensora direita",
            assessment.right_extensor_strength,
            " kg",
        ),
        ...measurement(
            "Extensora esquerda",
            assessment.left_extensor_strength,
            " kg",
        ),
        
        ...measurement(
            "Flexora direita",
            assessment.right_flexor_strength,
            " kg",
        ),
        ...measurement(
            "Flexora esquerda",
            assessment.left_flexor_strength,
            " kg",
        ),
        ...measurement(
            "Assimetria de bíceps",
            result.biceps_asymmetry_percentage,
            " %",
            classification(
                result.biceps_asymmetry_classification,
                labels.asymmetryLabels,
                tones.asymmetryTones,
            ),
        ),
        ...measurement(
            "Assimetria de tríceps",
            result.triceps_asymmetry_percentage,
            " %",
            classification(
                result.triceps_asymmetry_classification,
                labels.asymmetryLabels,
                tones.asymmetryTones,
            ),
        ),
        ...measurement(
            "Assimetria de extensores",
            result.extensor_asymmetry_percentage,
            " %",
            classification(
                result.extensor_asymmetry_classification,
                labels.asymmetryLabels,
                tones.asymmetryTones,
            ),
        ),
        ...measurement(
            "Assimetria de flexores",
            result.flexor_asymmetry_percentage,
            " %",
            classification(
                result.flexor_asymmetry_classification,
                labels.asymmetryLabels,
                tones.asymmetryTones,
            ),
        ),
    ])

    section(
        "VO2 — etapas da avaliação",
        result.vo2_assessments.map((stage, index) => {
            const stageAssessment = stage.vo2_assessment_dto

            return [
                String(index + 1),
                formatNumber(stageAssessment.treadmill_speed_kmh),
                formatNumber(stageAssessment.heart_rate_bpm),
                formatNumber(stage.fc_heart_percentage, " %"),
                classification(
                    stage.fc_heart_classification,
                    labels.fcExerciseLabels,
                    tones.fcExerciseTones,
                ),
                classification(
                    stageAssessment.perceived_exertion,
                    labels.perceivedExertionLabels,
                    tones.perceivedExertionTones,
                ),
            ]
        }),
        [
            "Etapa",
            "km/h",
            "FC (bpm)",
            "% FC máx.",
            "Intensidade FC",
            "Esforço",
        ],
    )

    section("VO2 — resultado", [
        ...measurement(
            "VO2 máximo",
            result.vo2_parameter,
            " mL/kg/min",
            classification(
                result.vo2_classification,
                labels.vo2Labels,
                tones.vo2Tones,
            ),
        ),
    ])

    if (chartImage) {
        const imageWidth = 178
        const imageProperties = doc.getImageProperties(chartImage)
        const imageHeight =
            (imageProperties.height * imageWidth) /
            imageProperties.width

        const sectionHeight = imageHeight + 14

        if (y + sectionHeight > 275) {
            doc.addPage()
            y = 18
        }

        doc.setFont("helvetica", "bold")
        doc.setFontSize(11)
        doc.setTextColor(0, 0, 0)
        doc.text(
            "Evolução — avaliação anterior e atual",
            16,
            y,
        )

        doc.addImage(
            chartImage,
            "PNG",
            16,
            y + 6,
            imageWidth,
            imageHeight,
        )

        y += sectionHeight
    }

    const pageCount = doc.getNumberOfPages()

    for (let page = 1; page <= pageCount; page++) {
        doc.setPage(page)

        doc.setDrawColor(215, 220, 225)
        doc.line(16, 282, 194, 282)

        doc.setFont("helvetica", "normal")
        doc.setFontSize(8)
        doc.setTextColor(95, 95, 95)

        doc.text(
            `Avaliação de ${formatDate(assessment.assessment_date)}`,
            16,
            288,
        )

        doc.text(`${page} / ${pageCount}`, 194, 288, {
            align: "right",
        })
    }

    return new Uint8Array(doc.output("arraybuffer"))
}