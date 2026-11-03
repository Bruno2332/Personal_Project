use chrono::Utc;
use std::path::{Path, PathBuf};

use crate::{
    dtos::physical_assessment_photos_dto::PhysicalAssessmentPhotosDto,
    errors::app_errors::{AppError, FieldError},
    models::physical_assessment_photos::PhysicalAssessmentPhotos,
    repositories::physical_assessment_photos_repository::PhysicalAssessmentPhotosRepository,
    utils::{
        assessment_photo_files::{self, run_file_task},
        assessment_photo_key,
    },
};

pub async fn find_all_by_assessment_id<R: PhysicalAssessmentPhotosRepository>(
    repository: &R,
    assessment_id: i64,
) -> Result<Vec<PhysicalAssessmentPhotosDto>, AppError> {
    let photos = repository
        .find_all_by_assessment_id(assessment_id)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao buscar fotos da avaliação.".to_string(),
        })?;

    Ok(photos
        .into_iter()
        .map(PhysicalAssessmentPhotosDto::from)
        .collect())
}

pub async fn find_by_id<R: PhysicalAssessmentPhotosRepository>(
    repository: &R,
    id: i64,
) -> Result<PhysicalAssessmentPhotosDto, AppError> {
    let photo = repository
        .find_by_id(id)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao buscar foto.".to_string(),
        })?
        .ok_or_else(|| AppError::NotFound {
            message: "Foto não encontrada.".to_string(),
        })?;

    Ok(PhysicalAssessmentPhotosDto::from(photo))
}

pub async fn read_content<R: PhysicalAssessmentPhotosRepository>(
    repository: &R,
    data_dir: &Path,
    id: i64,
) -> Result<Vec<u8>, AppError> {
    let photo = repository
        .find_by_id(id)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao buscar foto.".to_string(),
        })?
        .ok_or_else(|| AppError::NotFound {
            message: "Foto não encontrada.".to_string(),
        })?;

    let directory = data_dir.to_path_buf();
    let relative_path = photo.file_path;

    run_file_task(move || {
        // Na leitura nunca gera outra chave.
        let key = assessment_photo_key::load_key()?;

        assessment_photo_files::decrypt_photo(&directory, &relative_path, &key)
    })
    .await
    .map_err(file_error)
}

pub async fn insert<R: PhysicalAssessmentPhotosRepository>(
    repository: &R,
    data_dir: &Path,
    dto: PhysicalAssessmentPhotosDto,
) -> Result<PhysicalAssessmentPhotosDto, AppError> {
    if dto.physical_assessment_id <= 0 {
        return Err(AppError::Validation {
            errors: vec![FieldError {
                field: "physical_assessment_id".to_string(),
                message: "Informe uma avaliação válida.".to_string(),
            }],
        });
    }

    let source_path = PathBuf::from(&dto.file_path);
    let directory = data_dir.to_path_buf();

    let relative_path = run_file_task(move || {
        let key = assessment_photo_key::load_or_create_key(&directory)?;

        assessment_photo_files::encrypt_photo(&directory, &source_path, &key)
    })
    .await
    .map_err(file_error)?;

    let now = Utc::now();

    let entity = PhysicalAssessmentPhotos {
        id: None,
        physical_assessment_id: dto.physical_assessment_id,
        file_path: relative_path.clone(),
        position: dto.position,
        description: dto.description,
        created_at: now,
        updated_at: now,
    };

    match repository.insert(entity).await {
        Ok(created) => Ok(PhysicalAssessmentPhotosDto::from(created)),

        Err(_) => {
            let directory = data_dir.to_path_buf();

            let cleanup = run_file_task(move || {
                assessment_photo_files::remove_photo(&directory, &relative_path)
            })
            .await;

            let message = match cleanup {
                Ok(()) => {
                    "Erro ao salvar o registro da foto.".to_string()
                }

                Err(error) => format!(
                    "Erro ao salvar o registro da foto. A limpeza do arquivo criptografado também falhou: {error}"
                ),
            };

            Err(AppError::Database { message })
        }
    }
}

pub async fn update<R: PhysicalAssessmentPhotosRepository>(
    repository: &R,
    id: i64,
    dto: PhysicalAssessmentPhotosDto,
) -> Result<PhysicalAssessmentPhotosDto, AppError> {
    let updated = repository
        .update(id, dto.position, dto.description, Utc::now())
        .await
        .map_err(|error| match error {
            sqlx::Error::RowNotFound => AppError::NotFound {
                message: "Foto não encontrada.".to_string(),
            },

            _ => AppError::Database {
                message: "Erro ao atualizar a descrição da foto.".to_string(),
            },
        })?;

    Ok(PhysicalAssessmentPhotosDto::from(updated))
}

pub async fn delete_by_id<R: PhysicalAssessmentPhotosRepository>(
    repository: &R,
    data_dir: &Path,
    id: i64,
) -> Result<(), AppError> {
    let photo = repository
        .find_by_id(id)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao buscar foto para exclusão.".to_string(),
        })?
        .ok_or_else(|| AppError::NotFound {
            message: "Foto não encontrada.".to_string(),
        })?;

    let directory = data_dir.to_path_buf();
    let relative_path = photo.file_path;

    let staged =
        run_file_task(move || assessment_photo_files::stage_deletion(&directory, &relative_path))
            .await
            .map_err(file_error)?;

    match repository.delete_by_id(id).await {
        Ok(affected) => {
            if let Some((_, temporary)) = staged {
                run_file_task(move || assessment_photo_files::finish_deletion(&temporary))
                    .await
                    .map_err(file_error)?;
            }

            if affected == 0 {
                return Err(AppError::NotFound {
                    message: "Foto não encontrada.".to_string(),
                });
            }

            Ok(())
        }

        Err(_) => {
            if let Some((original, temporary)) = staged {
                run_file_task(move || assessment_photo_files::undo_deletion(&original, &temporary))
                    .await
                    .map_err(file_error)?;
            }

            Err(AppError::Database {
                message: "Erro ao excluir o registro da foto.".to_string(),
            })
        }
    }
}

fn file_error(message: String) -> AppError {
    AppError::Validation {
        errors: vec![FieldError {
            field: "file_path".to_string(),
            message,
        }],
    }
}
