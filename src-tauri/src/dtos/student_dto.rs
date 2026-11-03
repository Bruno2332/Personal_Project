use crate::models::contract::BillingType;
use crate::models::students::{Gender, Objective, State, Student, StudentProjection};
use crate::utils::validations::{
    validate_birth_date, validate_student_name, CEP_REGEX, CPF_REGEX, PHONE_REGEX,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct StudentDto {
    pub id: Option<i64>,
    #[validate(custom(
        function = "validate_student_name",
        message = "O nome deve ter entre 3 e 80 caracteres e não pode ficar em branco."
    ))]
    pub name: String,
    #[validate(regex(path = *CPF_REGEX, message = "O CPF deve conter exatamente 11 dígitos numéricos."))]
    pub cpf: String,
    pub gender: Gender,
    #[validate(email(message = "Informe um email válido."))]
    pub email: String,
    #[validate(custom(
        function = "validate_birth_date",
        message = "O aluno deve ter pelo menos 5 anos completos."
    ))]
    pub birth_date: NaiveDate,
    #[validate(regex(path = *PHONE_REGEX, message = "Informe o telefone no formato (11) 99999-9999 ou (11) 9999-9999."))]
    pub phone: Option<String>,
    #[validate(regex(path = *CEP_REGEX, message = "Informe o CEP no formato 00000-000."))]
    pub zip_code: Option<String>,
    pub address: Option<String>,
    pub address_number: Option<String>,
    pub address_complement: Option<String>,
    pub city: Option<String>,
    pub state: Option<State>,
    pub objective: Option<Objective>,
    pub notes: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Student> for StudentDto {
    fn from(student: Student) -> Self {
        Self {
            id: student.id,
            name: student.name,
            cpf: student.cpf,
            gender: student.gender,
            email: student.email,
            birth_date: student.birth_date,
            phone: student.phone,
            zip_code: student.zip_code,
            address: student.address,
            address_number: student.address_number,
            address_complement: student.address_complement,
            city: student.city,
            state: student.state,
            objective: student.objective,
            notes: student.notes,
            is_active: student.is_active,
            created_at: student.created_at,
            updated_at: student.updated_at,
        }
    }
}

impl From<StudentDto> for Student {
    fn from(dto: StudentDto) -> Self {
        Self {
            id: None,
            name: dto.name,
            cpf: dto.cpf,
            gender: dto.gender,
            email: dto.email,
            birth_date: dto.birth_date,
            phone: dto.phone,
            zip_code: dto.zip_code,
            address: dto.address,
            address_number: dto.address_number,
            address_complement: dto.address_complement,
            city: dto.city,
            state: dto.state,
            objective: dto.objective,
            notes: dto.notes,
            is_active: dto.is_active,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct StudentProjectionDto {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub objective: Objective,
    pub is_active: bool,
    pub first_due_date: Option<NaiveDate>,
    pub amount_cents: i32,
    pub billing_type: BillingType,
}

impl From<StudentProjection> for StudentProjectionDto {
    fn from(projection: StudentProjection) -> Self {
        Self {
            id: projection.id,
            name: projection.name,
            email: projection.email,
            phone: projection.phone,
            objective: projection.objective,
            is_active: projection.is_active,
            first_due_date: projection.first_due_date,
            amount_cents: projection.amount_cents,
            billing_type: projection.billing_type,
        }
    }
}
