use chrono::{Local, NaiveDate};
use regex::Regex;
use std::sync::LazyLock;
use validator::{Validate, ValidationError};
use crate::errors::app_errors::{AppError, FieldError};

pub fn validate_birth_date(birth_date: &NaiveDate) -> Result<(), ValidationError> {
    let today = Local::now().date_naive();

    match today.years_since(*birth_date) {
        Some(age) if age >= 5 => Ok(()),
        _ => Err(ValidationError::new("minimum_age")),
    }
}

pub fn validate_student_name(name: &str) -> Result<(), ValidationError> {
    let length = name.trim().chars().count();

    if (3..=80).contains(&length) {
        Ok(())
    } else {
        Err(ValidationError::new("student_name"))
    }
}

pub static CPF_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[0-9]{11}$").unwrap());

pub static CEP_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[0-9]{5}-[0-9]{3}$").unwrap());

pub static PHONE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\([0-9]{2}\) [0-9]{4,5}-[0-9]{4}$").unwrap());

pub fn validate_dto<T: Validate>(dto: &T) -> Result<(), AppError> {
    dto.validate().map_err(|validation_errors| AppError::Validation {
        errors: validation_errors
            .field_errors()
            .into_iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |error| FieldError {
                    field: field.to_string(),
                    message: error
                        .message
                        .as_ref()
                        .map(|message| message.to_string())
                        .unwrap_or_else(|| "Valor inválido.".to_string()),
                })
            })
            .collect(),
    })
}