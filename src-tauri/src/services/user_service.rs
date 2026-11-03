use chrono::Utc;
use crate::dtos::user_dto::UserDto;
use crate::errors::app_errors::AppError;
use crate::models::user::User;
use crate::repositories::user_repository::UserRepository;

pub async fn find_by_id<R: UserRepository>(repository: &R, id: i64) -> Result<UserDto, AppError> {
    let user = repository
        .find_by_id(id)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao buscar usuário".to_string(),
        })?
        .ok_or_else(|| AppError::NotFound {
            message: "Usuário não encontrado".to_string(),
        })?;

    Ok(UserDto::from(user))
}

pub async fn exists<R: UserRepository>(repository: &R) -> Result<bool, AppError> {
    let exists = repository.exists().await.map_err(|_| AppError::Database {
        message: "Erro ao verificar se existe usuário cadastrado".to_string(),
    })?;

    Ok(exists)
}

pub async fn insert<R: UserRepository>(repository: &R, dto: UserDto) -> Result<UserDto, AppError> {
    let mut result = User::from(dto);
    let now = Utc::now();
    result.created_at = now;
    result.updated_at = now;

    let created = repository
        .insert(result)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao criar usuário".to_string(),
        })?;

    Ok(UserDto::from(created))
}

pub async fn update<R: UserRepository>(
    repository: &R,
    id: i64,
    dto: UserDto,
) -> Result<UserDto, AppError> {
    let mut result = User::from(dto);
    result.updated_at = Utc::now();
    
    let updated = repository
        .update(result, id)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => AppError::NotFound {
                message: "Usuário não encontrado".to_string(),
            },
            _ => AppError::Database {
                message: "Erro ao atualizar usuário".to_string(),
            },
        })?;

    Ok(UserDto::from(updated))
}
