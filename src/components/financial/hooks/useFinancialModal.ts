import { useEffect, useState } from "react"
import { useForm } from "react-hook-form"
import type {
    FinancialContractOptionDto,
    FinancialRecordsDto,
    TransactionType,
} from "../../../model/financialRecord"
import {
    findFinancialContractOptions,
    insertFinancialRecord,
    updateFinancialRecord,
} from "../../../services/financialRecordService"
import { errorMessage } from "../../../utils/errors"
import { localDate } from "../../../utils/dates"

export type FinancialFormValues = FinancialRecordsDto & {
    amountUnits: number
}

type Options = {
    record: FinancialRecordsDto | null
    kind: TransactionType
    onClose: () => void
    onSaved: (record: FinancialRecordsDto) => void
}

export default function useFinancialModal({
    record,
    kind,
    onClose,
    onSaved,
}: Options) {
    const [contractOptions, setContractOptions] = useState<
        FinancialContractOptionDto[]
    >([])
    const [optionsLoading, setOptionsLoading] = useState(true)
    const [optionsError, setOptionsError] = useState("")
    const [optionsRevision, setOptionsRevision] = useState(0)
    const [error, setError] = useState("")
    const [savedId, setSavedId] = useState(record?.id ?? null)

    const now = new Date().toISOString()

    const methods = useForm<FinancialFormValues>({
        defaultValues: record
            ? { ...record, amountUnits: record.amount_cent / 100 }
            : {
                id: null,
                contract_id: null,
                transaction_type: kind,
                description: "",
                amount_cent: 0,
                amountUnits: 0,
                due_date: localDate(),
                payment_date: null,
                payment_method: "Pix",
                installment_number: 1,
                status_transaction: "Pending",
                created_at: now,
                updated_at: now,
            },
    })

    const income = methods.watch("transaction_type") === "Income"

    useEffect(() => {
        let cancelled = false

        setOptionsLoading(true)
        setOptionsError("")

        void findFinancialContractOptions()
            .then((options) => {
                if (!cancelled) setContractOptions(options)
            })
            .catch((err) => {
                if (!cancelled) setOptionsError(errorMessage(err))
            })
            .finally(() => {
                if (!cancelled) setOptionsLoading(false)
            })

        return () => {
            cancelled = true
        }
    }, [optionsRevision])

    async function save(values: FinancialFormValues) {
        setError("")

        if (optionsLoading || optionsError) return

        if (
            values.status_transaction === "Paid" &&
            !values.payment_date
        ) {
            methods.setError("payment_date", {
                message: "Informe a data do pagamento.",
            })
            return
        }

        const { amountUnits, ...dto } = values
        dto.amount_cent = Math.round(amountUnits * 100)
        dto.updated_at = new Date().toISOString()

        if (
            !Number.isSafeInteger(dto.amount_cent) ||
            dto.amount_cent > 2147483647
        ) {
            methods.setError("amountUnits", {
                message: "Valor fora do limite permitido.",
            })
            return
        }

        try {
            const result =
                savedId !== null
                    ? await updateFinancialRecord(savedId, dto)
                    : await insertFinancialRecord(dto)

            setSavedId(result.id)
            onSaved(result)
            onClose()
        } catch (err) {
            setError(errorMessage(err))
        }
    }

    return {
        methods,
        income,
        contractOptions,
        optionsLoading,
        optionsError,
        error,
        save,
        retryOptions: () =>
            setOptionsRevision((revision) => revision + 1),
    }
}