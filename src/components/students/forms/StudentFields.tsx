import { FormInput, FormSelect, FormTextArea } from "../../ui/Fields"
import { genderLabels, objectiveLabels } from "../../../labels/student"
import { formatCPF, formatPhone, formatDate } from "../../../utils/formatters"
import AddressFields from "./AddressFields"

export default function StudentFields() {
	return (
		<div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
			<FormInput
				name="name"
				label="Nome Completo"
				className="sm:col-span-2"
				required
				placeholder="Nome e sobrenome"
			/>
			<FormInput
				name="birth_date"
				label="Data de Nascimento"
				type="date"
				mask={formatDate}
				placeholder="dd/mm/aaaa"
				required
			/>
			<FormSelect
				name="gender"
				label="Sexo"
				required
				options={Object.entries(genderLabels).map(([value, label]) => ({
					value,
					label,
				}))}
			/>
			<FormInput
				name="email"
				label="E-mail"
				type="email"
				required
				placeholder="email@exemplo.com"
			/>
			<FormInput
				name="phone"
				label="Telefone"
				mask={formatPhone}
				nullable
				placeholder="(11) 99999-9999"
			/>
			<FormInput 
				name="cpf" 
				label="CPF"
				mask={formatCPF}
				required 
				placeholder="000.000.000-00" 
			/>
			
			<AddressFields />

			<FormSelect
				name="objective"
				label="Objetivo"
				nullable
				options={Object.entries(objectiveLabels).map(([value, label]) => ({
					value,
					label,
				}))}
			/>
			<FormTextArea
				name="notes"
				label="Observações"
				nullable
				className="sm:col-span-2"
				placeholder="Lesões, restrições, preferências..."
			/>
		</div>
	)
}
