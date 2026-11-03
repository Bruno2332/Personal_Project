use crate::dtos::agenda_dto::AgendaDto;
use crate::errors::app_errors::AppError;
use crate::models::agenda::Agenda;
use crate::repositories::agenda_repository::AgendaRepository;
use chrono::{NaiveDate, Utc};

pub async fn find_by_period<R: AgendaRepository>(
    repository: &R,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<AgendaDto>, AppError> {
    let agenda = repository
        .find_by_period(start_date, end_date)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao buscar agendamento".to_string(),
        })?;

    let items = agenda.into_iter().map(AgendaDto::from).collect();

    Ok(items)
}

pub async fn insert<R: AgendaRepository>(
    repository: &R,
    dto: AgendaDto,
) -> Result<AgendaDto, AppError> {

    let mut result = Agenda::from(dto);
    let now = Utc::now();
    result.created_at = now;
    result.updated_at = now;

    let created = repository
        .insert(result)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao criar agendamento".to_string(),
        })?;
    Ok(AgendaDto::from(created))
}

pub async fn update<R: AgendaRepository>(
    repository: &R,
    id: i64,
    dto: AgendaDto,
) -> Result<AgendaDto, AppError> {

    let mut result = Agenda::from(dto);
    result.updated_at = Utc::now();

    let updated = repository
        .update(result, id)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => AppError::NotFound {
                message: "Agendamento não encontrado".to_string(),
            },
            _ => AppError::Database {
                message: "Erro ao atualizar agendamento".to_string(),
            },
        })?;

    Ok(AgendaDto::from(updated))
}

pub async fn delete<R: AgendaRepository>(repository: &R, id: i64) -> Result<(), AppError> {
    let rows_affected = repository
        .delete_by_id(id)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao deletar agendamento".to_string(),
        })?;

    if rows_affected == 0 {
        return Err(AppError::NotFound {
            message: "Agendamento não encontrado".to_string(),
        });
    }
    Ok(())
}
