import { invoke } from "@tauri-apps/api/core"
import type { StudentDto, StudentProjectionDto } from "../model/student"
import type { PageResponse } from "../model/pagination"

export function findAllStudents(
    page = 0,
    size = 20,
    name: string | null = null,
) {
    return invoke<PageResponse<StudentProjectionDto>>("find_all_students", {
        page,
        size,
        name,
    })
}
export function findStudentById(id: number) {
    return invoke<StudentDto>("find_student_by_id", { id })
}
export function insertStudent(dto: StudentDto) {
    return invoke<StudentDto>("insert_student", { dto })
}
export function updateStudent(id: number, dto: StudentDto) {
    return invoke<StudentDto>("update_student", { id, dto })
}
