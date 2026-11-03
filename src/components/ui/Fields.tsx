import { useId } from "react"
import { Controller, useFormContext } from "react-hook-form"
import { FormField, inputClass } from "./FormField"
import { formatDate, isValidDate } from "../../utils/formatters"

type Props = {
	name: string
	label: string
	type?: string
	required?: boolean
	nullable?: boolean
	className?: string
	placeholder?: string
	min?: number
	max?: number
	step?: number | string
	readOnly?: boolean
	inputStyle?: string
	mask?: "currency" | "decimal" | "integer" | ((value: string) => string)
}

export function FormInput({
	name,
	label,
	type = "text",
	required,
	nullable,
	className,
	placeholder,
	min,
	max,
	step,
	readOnly,
	inputStyle = inputClass,
	mask,
	maxLength,
	decimalPlaces = 2,
}: Props & {
	maxLength?: number
	decimalPlaces?: number
}) {
	const { control } = useFormContext()
	const id = useId()

	const isCurrency = mask === "currency"
	const isDecimal = mask === "decimal"
	const isInteger = mask === "integer"
	const isDate = type === "date"
	const isTime = type === "time"
	const isMasked = isCurrency || isDecimal || isInteger
	const places = isCurrency ? 2 : isInteger ? 0 : decimalPlaces
	const factor = 10 ** places
	const requiresInteger = isInteger || step === 1 || step === "1"

	const formatNumber = (value: number) =>
		value.toLocaleString("pt-BR", {
			minimumFractionDigits: places,
			maximumFractionDigits: places,
			useGrouping: isCurrency,
		})

	const formatTime = (value: string) =>
		value
			.replace(/\D/g, "")
			.slice(0, 4)
			.replace(/(\d{2})(\d)/, "$1:$2")

	return (
		<Controller
			name={name}
			control={control}
			rules={{
				required: required ? "Campo obrigatório." : false,
				...(min !== undefined
					? { min: { value: min, message: `Valor mínimo: ${min}.` } }
					: {}),
				...(max !== undefined
					? { max: { value: max, message: `Valor máximo: ${max}.` } }
					: {}),
				validate: (value) => {
					if (value == null || value === "") {
						return !required || "Campo obrigatório."
					}

					if (isDate) {
						return (
							isValidDate(String(value)) ||
							"Informe uma data válida no formato dd/mm/aaaa."
						)
					}

					if (isTime) {
						return (
							/^([01]\d|2[0-3]):[0-5]\d(:[0-5]\d(\.\d{1,9})?)?$/.test(
								String(value),
							) ||
							"Informe um horário válido no formato HH:mm."
						)
					}

					if (type === "number" || isMasked) {
						return (
							(Number.isFinite(value) &&
								(!requiresInteger || Number.isInteger(value))) ||
							"Informe um número válido."
						)
					}

					if (typeof value === "string" && required && !value.trim()) {
						return "Campo obrigatório."
					}

					if (type === "email") {
						return (
							/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value) ||
							"Informe um e-mail válido."
						)
					}

					return true
				},
			}}
			render={({ field, fieldState }) => (
				<FormField
					label={label}
					className={className}
					htmlFor={id}
					error={fieldState.error?.message}
				>
					<input
						{...field}
						id={id}
						className={inputStyle}
						type={isMasked || isDate || isTime ? "text" : type}
						inputMode={
							isMasked || isDate || isTime ? "numeric" : undefined
						}
						value={
							field.value == null || field.value === ""
								? ""
								: isDate
									? formatDate(String(field.value))
									: isTime
										? formatTime(String(field.value))
										: typeof mask === "function"
											? mask(String(field.value))
											: isMasked
												? formatNumber(Number(field.value))
												: field.value
						}
						readOnly={readOnly}
						placeholder={placeholder ?? (isTime ? "HH:mm" : undefined)}
						min={min}
						max={max}
						step={step}
						aria-invalid={!!fieldState.error}
						onChange={(event) => {
							const value = event.target.value
							const emptyValue = nullable ? null : ""

							if (isTime) {
								const formatted = formatTime(value)

								field.onChange(
									formatted.length === 5
										? `${formatted}:00`
										: formatted || emptyValue,
								)
								return
							}

							if (isDate) {
								const formatted = formatDate(value)
								const digits = formatted.replace(/\D/g, "")

								if (digits.length === 8) {
									const day = digits.slice(0, 2)
									const month = digits.slice(2, 4)
									const year = digits.slice(4, 8)

									field.onChange(`${year}-${month}-${day}`)
								} else {
									field.onChange(formatted || emptyValue)
								}

								return
							}

							if (typeof mask === "function") {
								const formatted = mask(value)
								field.onChange(formatted === "" ? emptyValue : formatted)
								return
							}

							if (isMasked) {
								const digits = value.replace(/\D/g, "")
								const inputEvent = event.nativeEvent as InputEvent
								const isDeleting =
									inputEvent.inputType?.startsWith("delete") ?? false

								if (
									digits === "" ||
									(isDeleting && Number(digits) === 0)
								) {
									field.onChange(emptyValue)
									return
								}

								const number = Number(digits) / factor

								if (!Number.isFinite(number)) return

								const formatted = formatNumber(number)

								if (
									maxLength !== undefined &&
									formatted.length > maxLength
								) {
									const digit = inputEvent.data

									if (
										isDecimal &&
										inputEvent.inputType === "insertText" &&
										digit &&
										/^\d$/.test(digit) &&
										event.target.selectionStart === value.length
									) {
										const currentDigits = String(
											Math.round(Number(field.value ?? 0) * factor),
										)

										const updatedDigits =
											currentDigits.slice(0, -1) + digit

										const updatedNumber = Number(updatedDigits) / factor

										if (
											Number.isFinite(updatedNumber) &&
											formatNumber(updatedNumber).length <= maxLength
										) {
											field.onChange(updatedNumber)
										}
									}

									return
								}

								field.onChange(number)
								return
							}

							if (
								maxLength !== undefined &&
								value.length > maxLength
							) {
								return
							}

							if (value === "") {
								field.onChange(emptyValue)
								return
							}

							field.onChange(
								type === "number" ? Number(value) : value,
							)
						}}
					/>
				</FormField>
			)}
		/>
	)
}


export function FormSelect({
	name,
	label,
	options,
	nullable,
	required,
	className,
	numeric,
	inputStyle = inputClass,
}: {
	name: string
	label: string
	options: { value: string | number; label: string }[]
	nullable?: boolean
	required?: boolean
	className?: string
	numeric?: boolean
	inputStyle?: string
}) {
	const { control } = useFormContext()
	const id = useId()
	return (
		<Controller
			name={name}
			control={control}
			rules={{
				required: required ? "Selecione uma opção." : false,
				validate: (value) => {
					if (value == null || value === "")
						return !required || "Selecione uma opção."
					return (
						options.some((option) => option.value === value) ||
						"Selecione uma opção válida."
					)
				},
			}}
			render={({ field, fieldState }) => (
				<FormField
					label={label}
					htmlFor={id}
					className={className}
					error={fieldState.error?.message}
				>
					<select
						{...field}
						id={id}
						className={inputStyle}
						value={field.value ?? ""}
						aria-invalid={!!fieldState.error}
						onChange={(event) =>
							field.onChange(
								event.target.value === ""
									? null
									: numeric
										? Number(event.target.value)
										: event.target.value,
							)
						}
					>
						{(nullable || !field.value) && (
							<option value="">Selecione...</option>
						)}
						{options.map((option) => (
							<option key={option.value} value={option.value}>
								{option.label}
							</option>
						))}
					</select>
				</FormField>
			)}
		/>
	)
}


export function FormTextArea({
	name,
	label,
	nullable,
	className,
	placeholder,
	inputStyle = inputClass,
}: {
	name: string
	label: string
	nullable?: boolean
	className?: string
	placeholder?: string
	inputStyle?: string
}) {
	const { control } = useFormContext()
	const id = useId()
	return (
		<Controller
			name={name}
			control={control}
			render={({ field, fieldState }) => (
				<FormField
					label={label}
					htmlFor={id}
					className={className}
					error={fieldState.error?.message}
				>
					<textarea
						{...field}
						id={id}
						value={field.value ?? ""}
						onChange={(event) =>
							field.onChange(event.target.value || (nullable ? null : ""))
						}
						className={`${inputStyle} resize-none h-20`}
						placeholder={placeholder}
					/>
				</FormField>
			)}
		/>
	)
}
