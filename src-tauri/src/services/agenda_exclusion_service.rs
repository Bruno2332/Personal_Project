use chrono::{Datelike, NaiveDate};

use crate::{
    dtos::agenda_exclusion_dto::{
        AgendaExclusionDto,
        CreateAgendaExclusionDto,
    },
    errors::app_errors::{AppError, FieldError},
    models::agenda_exclusions::AgendaExclusion,
    repositories::{
        agenda_exclusion_repository::AgendaExclusionRepository,
        agenda_repository::AgendaRepository,
    },
};

pub async fn find_by_period<R: AgendaExclusionRepository>(
    repository: &R,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<AgendaExclusionDto>, AppError> {
    if end_date < start_date {
        return Err(AppError::Validation {
            errors: vec![FieldError {
                field: "end_date".to_string(),
                message: "O fim do período não pode ser anterior ao início."
                    .to_string(),
            }],
        });
    }

    let entities = AgendaExclusionRepository::find_by_period(
        repository,
        start_date,
        end_date,
    )
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao buscar exclusões da agenda.".to_string(),
        })?;

    Ok(
        entities
            .into_iter()
            .map(AgendaExclusionDto::from)
            .collect(),
    )
}

pub async fn insert<R>(
    repository: &R,
    dto: CreateAgendaExclusionDto,
) -> Result<AgendaExclusionDto, AppError>
where
    R: AgendaRepository + AgendaExclusionRepository,
{
    let appointment = AgendaRepository::find_by_id(
        repository,
        dto.agenda_id,
    )
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao buscar agendamento.".to_string(),
        })?
        .ok_or_else(|| AppError::NotFound {
            message: "Agendamento não encontrado.".to_string(),
        })?;

    if !appointment.is_recurring {
        return Err(AppError::Validation {
            errors: vec![FieldError {
                field: "agenda_id".to_string(),
                message: "Esta operação é exclusiva de agendamentos recorrentes."
                    .to_string(),
            }],
        });
    }

    let date = dto.occurrence_date;

    let inside_period = appointment
        .recurrence_start_date
        .is_some_and(|start| date >= start)
        && appointment
        .recurrence_end_date
        .map_or(true, |end| date <= end);

    let matches_weekday = appointment.week_day.is_some_and(|day| {
        day as u32 == date.weekday().num_days_from_sunday()
    });

    if !inside_period || !matches_weekday {
        return Err(AppError::Validation {
            errors: vec![FieldError {
                field: "occurrence_date".to_string(),
                message: "Esta data não corresponde a uma ocorrência da série."
                    .to_string(),
            }],
        });
    }

    let created = AgendaExclusionRepository::insert(
        repository,
        AgendaExclusion::from(dto),
    )
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao excluir a ocorrência da agenda.".to_string(),
        })?;

    Ok(AgendaExclusionDto::from(created))
}