use crate::models::contract::BillingType;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum Objective {
    HypertrophyAndDefinition = 0,
    WeightLoss = 1,
    RehabilitationStrengthening = 2,
    SportsPerformance = 3,
    GeneralConditioningHealth = 4,
    FlexibilityMobility = 5,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum Gender {
    Male = 0,
    Female = 1,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
#[serde(rename_all = "UPPERCASE")]
pub enum State {
    Ac = 0,  // Acre
    Al = 1,  // Alagoas
    Ap = 2,  // Amapá
    Am = 3,  // Amazonas
    Ba = 4,  // Bahia
    Ce = 5,  // Ceará
    Df = 6,  // Distrito Federal
    Es = 7,  // Espírito Santo
    Go = 8,  // Goiás
    Ma = 9,  // Maranhão
    Mt = 10, // Mato Grosso
    Ms = 11, // Mato Grosso do Sul
    Mg = 12, // Minas Gerais
    Pa = 13, // Pará
    Pb = 14, // Paraíba
    Pr = 15, // Paraná
    Pe = 16, // Pernambuco
    Pi = 17, // Piauí
    Rj = 18, // Rio de Janeiro
    Rn = 19, // Rio Grande do Norte
    Rs = 20, // Rio Grande do Sul
    Ro = 21, // Rondônia
    Rr = 22, // Roraima
    Sc = 23, // Santa Catarina
    Sp = 24, // São Paulo
    Se = 25, // Sergipe
    To = 26, // Tocantins
}

#[derive(Debug, FromRow, Deserialize)]
pub struct Student {
    pub id: Option<i64>,
    pub name: String,
    pub cpf: String,
    pub gender: Gender,
    pub email: String,
    pub birth_date: NaiveDate,
    pub phone: Option<String>,
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

#[derive(Debug, FromRow, Deserialize)]
pub struct StudentProjection {
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
