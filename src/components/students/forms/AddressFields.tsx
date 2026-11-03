import { useEffect, useRef, useState } from "react"
import { useFormContext } from "react-hook-form"
import { states, type StudentDto } from "../../../model/student"
import { findAddressByCep } from "../../../services/cepService"
import { formatCEP } from "../../../utils/formatters"
import { FormInput, FormSelect } from "../../ui/Fields"

const stateOptions = states.map((state) => ({
    value: state,
    label: state,
}))

export default function AddressFields() {
    const { getValues, setValue } = useFormContext<StudentDto>()
    const [error, setError] = useState("")
    const request = useRef<AbortController | null>(null)

    useEffect(() => {
        return () => request.current?.abort()
    }, [])

    async function searchCep() {
        request.current?.abort()
        setError("")

        const cep = (getValues("zip_code") ?? "").replace(/\D/g, "")

        if (!cep) return

        if (cep.length !== 8) {
            setError("Informe um CEP com 8 dígitos.")
            return
        }

        const controller = new AbortController()
        request.current = controller

        try {
            const data = await findAddressByCep(cep, controller.signal)

            
            const currentCep = (getValues("zip_code") ?? "").replace(/\D/g, "")

            if (controller.signal.aborted || currentCep !== cep) return

            const state = states.find((value) => value === data.uf)

            if (!state) {
                throw new Error("A consulta retornou uma UF inválida.")
            }

            const options = {
                shouldDirty: true,
                shouldValidate: true,
            }

            setValue("address", data.logradouro || null, options)
            setValue("city", data.localidade || null, options)
            setValue("state", state, options)
        } catch (error) {
            if (controller.signal.aborted) return

            setError(
                error instanceof Error
                    ? error.message
                    : "Erro ao buscar endereço pelo CEP.",
            )
        }
    }

    return (
        <>
            <div className="sm:col-span-2 flex items-center gap-3 pt-2">
                <p className="text-xs text-[#a3e635] uppercase tracking-[0.16em] font-medium">
                    Endereço
                </p>
                <div className="h-px flex-1 bg-[#252525]" />
            </div>

            <div onBlur={() => void searchCep()}>
                <FormInput
                    name="zip_code"
                    label="CEP"
                    placeholder="00000-000"
                    mask={formatCEP}
                    nullable
                />

                {error && (
                    <p role="alert" className="mt-1 text-xs text-red-400">
                        {error}
                    </p>
                )}
            </div>

            <FormInput
                name="address"
                label="Logradouro"
                placeholder="Rua, avenida..."
                nullable
            />

            <FormInput
                name="address_number"
                label="Número"
                placeholder="123 ou S/N"
                nullable
            />

            <FormInput
                name="address_complement"
                label="Complemento"
                placeholder="Apartamento, bloco..."
                nullable
            />

            <div className="grid grid-cols-[1fr_80px] gap-3">
                <FormInput
                    name="city"
                    label="Cidade"
                    placeholder="Cidade"
                    nullable
                />

                <FormSelect
                    name="state"
                    label="UF"
                    options={stateOptions}
                    nullable
                />
            </div>
        </>
    )
}
