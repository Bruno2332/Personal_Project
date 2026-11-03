use chrono::Utc;
use crate::dtos::pagination::PageResponse;
use crate::dtos::student_dto::{StudentDto, StudentProjectionDto};
use crate::errors::app_errors::AppError;
use crate::models::students::Student;
use crate::repositories::student_repository::StudentRepository;
use crate::utils::validations::validate_dto;

pub async fn find_all<R: StudentRepository>(
    repository: &R,
    page: i64,
    size: i64,
    name: Option<String>,
) -> Result<PageResponse<StudentProjectionDto>, AppError> {
    let students = repository
        .find_all_paginated(page, size, name.clone())
        .await
        .map_err(|error| {
            eprintln!("Erro em find_all_paginated: {error:?}");
            AppError::Database {
                message: "Erro ao buscar alunos".to_string(),
            }
        })?;

    let total = repository.count_all(name).await.map_err(|error| {
        eprintln!("Erro em count_all: {error:?}");
        AppError::Database {
            message: "Erro ao contar alunos".to_string(),
        }
    })?;

    let items = students
        .into_iter()
        .map(StudentProjectionDto::from)
        .collect();

    Ok(PageResponse {
        items,
        page,
        size,
        total,
    })
}

pub async fn find_by_id<R: StudentRepository>(
    repository: &R,
    id: i64,
) -> Result<StudentDto, AppError> {
    let student = repository
        .find_by_id(id)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao buscar aluno".to_string(),
        })?
        .ok_or_else(|| AppError::NotFound {
            message: "Aluno não encontrado".to_string(),
        })?;

    Ok(StudentDto::from(student))
}

pub async fn insert<R: StudentRepository>(
    repository: &R,
    dto: StudentDto,
) -> Result<StudentDto, AppError> {

    validate_dto(&dto)?;

    let mut result = Student::from(dto);
    let now = Utc::now();
    result.created_at = now;
    result.updated_at = now;

    let created = repository
        .insert(result)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao criar aluno".to_string(),
        })?;

    Ok(StudentDto::from(created))
}

pub async fn update<R: StudentRepository>(
    repository: &R,
    id: i64,
    dto: StudentDto,
) -> Result<StudentDto, AppError> {

    validate_dto(&dto)?;

    let mut result = Student::from(dto);
    result.updated_at = Utc::now();

    let updated = repository
        .update(id, result)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => AppError::NotFound {
                message: "Aluno não encontrado".to_string(),
            },
            _ => AppError::Database {
                message: "Erro ao atualizar aluno".to_string(),
            },
        })?;

    Ok(StudentDto::from(updated))
}
