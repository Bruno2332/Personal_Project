import { useEffect, useMemo, useRef, useState } from "react"
import { useForm, useWatch } from "react-hook-form"

import type {
    FinancialMonthDto,
    FinancialRecordsDto,
    FinancialRecordsProjectionDto,
    FinancialStatusFilter,
    TransactionType,
} from "../../../model/financialRecord"
import {
    findAllFinancialRecords,
    findFinancialSummary,
    updateFinancialRecord,
} from "../../../services/financialRecordService"
import { financialRecordDto } from "../../../utils/financialRecord"
import { errorMessage } from "../../../utils/errors"
import { localDate } from "../../../utils/dates"
import { financialTabs, type FinancialTab } from "../forms/financialFields"

type FilterValues = {
    status: "all" | FinancialStatusFilter
    period: string
}

const pageSize = 20

function formatMonth(month: string) {
    return new Date(`${month}-01T12:00:00`).toLocaleDateString("pt-BR", {
        month: "short",
        year: "2-digit",
    })
}

export default function useFinance() {
    const [records, setRecords] = useState<FinancialRecordsProjectionDto[]>([])
    const [summary, setSummary] = useState<FinancialMonthDto[] | null>(null)
    const [total, setTotal] = useState(0)
    const [tab, setTab] = useState<FinancialTab>("monthly")
    const [typeFilter, setTypeFilter] = useState<"all" | TransactionType>("all")
    const [search, setSearch] = useState("")
    const [page, setPage] = useState(0)
    const [revision, setRevision] = useState(0)
    const [loading, setLoading] = useState(true)
    const [listError, setListError] = useState("")
    const [summaryError, setSummaryError] = useState("")
    const [error, setError] = useState("")
    const [busyId, setBusyId] = useState<number | null>(null)
    const [modalOpen, setModalOpen] = useState(false)
    const [selected, setSelected] = useState<FinancialRecordsDto | null>(null)
    const [kind, setKind] = useState<TransactionType>("Income")
    const mutationBusy = useRef(false)
    const previousFilters = useRef({ status: "all", period: "1m" })
    const filters = useForm<FilterValues>({
        defaultValues: { status: "all", period: "1m" },
    })
    const [statusFilter, period] = useWatch({
        control: filters.control,
        name: ["status", "period"],
    })
    const monthly = tab === "monthly"

    useEffect(() => {
        if (
            previousFilters.current.status !== statusFilter ||
            previousFilters.current.period !== period
        ) {
            previousFilters.current = { status: statusFilter, period }
            setPage(0)
        }
    }, [statusFilter, period])

    useEffect(() => {
        let cancelled = false
        setLoading(true)
        setListError("")
        const timer = window.setTimeout(async () => {
            try {
                const result = await findAllFinancialRecords(
                    page,
                    pageSize,
                    search.trim() || null,
                    monthly,
                    {
                        transactionType: typeFilter === "all" ? null : typeFilter,
                        status: statusFilter === "all" ? null : statusFilter,
                        period: period === "all" ? null : period,
                    },
                )
                if (cancelled) return
                const lastPage = Math.max(0, Math.ceil(result.total / pageSize) - 1)
                setTotal(result.total)
                if (page > lastPage) {
                    setRecords([])
                    setPage(lastPage)
                    return
                }
                setRecords(result.items)
            } catch (err) {
                if (!cancelled) {
                    setRecords([])
                    setTotal(0)
                    setListError(errorMessage(err))
                }
            } finally {
                if (!cancelled) setLoading(false)
            }
        }, 200)
        return () => {
            cancelled = true
            window.clearTimeout(timer)
        }
    }, [page, search, monthly, typeFilter, statusFilter, period, revision])

    useEffect(() => {
        let cancelled = false
        setSummaryError("")
        void findFinancialSummary()
            .then((data) => {
                if (!cancelled) setSummary(data)
            })
            .catch((err) => {
                if (!cancelled) {
                    setSummary(null)
                    setSummaryError(errorMessage(err))
                }
            })
        return () => {
            cancelled = true
        }
    }, [revision])


    function cutoffMonth(period: string): string | null {
        const monthsBack: Record<string, number> = {
            "1m": 1,
            "3m": 3,
            "6m": 6,
            "12m": 12,
        }

        const months = monthsBack[period]
        if (months === undefined) return null

        const date = new Date()
        date.setDate(1)
        date.setMonth(date.getMonth() - months)

        const year = date.getFullYear()
        const month = String(date.getMonth() + 1).padStart(2, "0")

        return `${year}-${month}`
    }

    const periodSummary = useMemo(() => {
        const cutoff = cutoffMonth(period)
        const selectedMonths = (summary ?? []).filter(
            (item) => cutoff === null || item.month >= cutoff,
        )

        const income =
            selectedMonths.reduce((sum, item) => sum + item.income_cents, 0) / 100
        const expense =
            selectedMonths.reduce((sum, item) => sum + item.expense_cents, 0) / 100

        return {
            income,
            expense,
            receivable:
                selectedMonths.reduce((sum, item) => sum + item.receivable_cents, 0) /
                100,
            payable:
                selectedMonths.reduce((sum, item) => sum + item.payable_cents, 0) /
                100,
            balance: income - expense,
            cashFlow: selectedMonths.slice(-8).map((item) => ({
                month: formatMonth(item.month),
                income: item.income_cents / 100,
                expense: item.expense_cents / 100,
            })),
        }
    }, [summary, period])

    function openRecord(transactionType: TransactionType) {
        setSelected(null)
        setKind(transactionType)
        setModalOpen(true)
    }

    function editRecord(record: FinancialRecordsDto) {
        setSelected(record)
        setKind(record.transaction_type)
        setModalOpen(true)
    }

    function resetPage() {
        setPage(0)
    }

    async function changeStatus(record: FinancialRecordsProjectionDto, cancel: boolean) {
        if (record.id === null || mutationBusy.current) return
        if (cancel && !window.confirm(`Cancelar o lançamento “${record.description}”?`)) return

        mutationBusy.current = true
        setBusyId(record.id)
        setError("")
        try {
            await updateFinancialRecord(record.id, {
                ...financialRecordDto(record),
                status_transaction: cancel ? "Cancelled" : "Paid",
                payment_date: cancel ? record.payment_date : localDate(),
                updated_at: new Date().toISOString(),
            })
            setRevision((value) => value + 1)
        } catch (err) {
            setError(errorMessage(err))
        } finally {
            mutationBusy.current = false
            setBusyId(null)
        }
    }

    function markAsPaid(id: number) {
        const record = records.find((item) => item.id === id)
        if (record) void changeStatus(record, false)
    }

    return {
        records,
        summary,
        total,
        tab,
        setTab,
        tabs: financialTabs,
        typeFilter,
        setTypeFilter,
        search,
        setSearch,
        page,
        setPage,
        pageSize,
        filters,
        statusFilter,
        period,
        periodSummary,
        loading,
        listError,
        summaryError,
        error,
        busyId,
        modalOpen,
        selected,
        kind,
        openRecord,
        editRecord,
        closeModal: () => setModalOpen(false),
        onSaved: () => setRevision((value) => value + 1),
        changeStatus,
        markAsPaid,
        retry: () => setRevision((value) => value + 1),
        resetPage,
    }
}
