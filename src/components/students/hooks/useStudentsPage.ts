import { useEffect, useRef, useState } from "react"

import type { StudentDto, StudentProjectionDto } from "../../../model/student"
import { findAllStudents, findStudentById } from "../../../services/studentService"
import { errorMessage } from "../../../utils/errors"

const pageSize = 20

export default function useStudentsPage() {
	const [students, setStudents] = useState<StudentProjectionDto[]>([])
	const [search, setSearch] = useState("")
	const [page, setPage] = useState(0)
	const [total, setTotal] = useState(0)
	const [modalOpen, setModalOpen] = useState(false)
	const [selectedStudent, setSelectedStudent] = useState<StudentDto | null>(null)
	const [loading, setLoading] = useState(true)
	const [loadingStudent, setLoadingStudent] = useState(false)
	const [error, setError] = useState("")
	const [detailsError, setDetailsError] = useState("")
	const [reloadKey, setReloadKey] = useState(0)
	const openingStudent = useRef(false)

	useEffect(() => {
		if (modalOpen) return

		let cancelled = false
		setLoading(true)
		setError("")

		const timer = window.setTimeout(async () => {
			try {
				const response = await findAllStudents(
					page,
					pageSize,
					search.trim() || null,
				)
				if (cancelled) return

				const lastPage = Math.max(0, Math.ceil(response.total / pageSize) - 1)
				if (page > lastPage) {
					setPage(lastPage)
					return
				}

				setStudents(response.items)
				setTotal(response.total)
			} catch (err) {
				if (!cancelled) setError(errorMessage(err))
			} finally {
				if (!cancelled) setLoading(false)
			}
		}, 300)

		return () => {
			cancelled = true
			window.clearTimeout(timer)
		}
	}, [page, search, modalOpen, reloadKey])

	async function openStudent(student: StudentProjectionDto) {
		if (openingStudent.current) return
		if (student.id === null) {
			setDetailsError("O aluno retornado não possui ID.")
			return
		}

		openingStudent.current = true
		setLoadingStudent(true)
		setDetailsError("")
		try {
			const result = await findStudentById(student.id)
			setSelectedStudent(result)
			setModalOpen(true)
		} catch (err) {
			setDetailsError(errorMessage(err))
		} finally {
			openingStudent.current = false
			setLoadingStudent(false)
		}
	}

	function openNewStudent() {
		setSelectedStudent(null)
		setDetailsError("")
		setModalOpen(true)
	}

	function closeModal() {
		setModalOpen(false)
		setSelectedStudent(null)
	}

	function updateSearch(value: string) {
		setSearch(value)
		setPage(0)
		setDetailsError("")
	}

	return {
		students,
		search,
		page,
		setPage,
		pageSize,
		total,
		modalOpen,
		selectedStudent,
		loading,
		loadingStudent,
		error,
		detailsError,
		updateSearch,
		openStudent,
		openNewStudent,
		closeModal,
		retry: () => setReloadKey((value) => value + 1),
	}
}
