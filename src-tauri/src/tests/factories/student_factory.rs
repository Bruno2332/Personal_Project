use chrono::{NaiveDate, Utc};

use crate::{dtos::student_dto::StudentDto, models::students::Gender};

pub fn valid_student_dto() -> StudentDto {
    let now = Utc::now();

    StudentDto {
        id: None,
        name: "Aluno de teste".to_string(),
        cpf: "12345678909".to_string(),
        gender: Gender::Male,
        email: "aluno@example.com".to_string(),
        birth_date: NaiveDate::from_ymd_opt(1990, 5, 15).unwrap(),
        phone: Some("(11) 99999-9999".to_string()),
        zip_code: Some("14098-374".to_string()),
        address: None,
        address_number: None,
        address_complement: None,
        city: None,
        state: None,
        objective: None,
        notes: None,
        is_active: true,
        created_at: now,
        updated_at: now,
    }
}
