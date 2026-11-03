use crate::models::students::{Student, StudentProjection};
use crate::utils::search::name_search_pattern;
use async_trait::async_trait;
use sqlx::{SqliteConnection, SqlitePool};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait StudentRepository: Send + Sync {
    async fn find_all_paginated(
        &self,
        page: i64,
        size: i64,
        name: Option<String>,
    ) -> Result<Vec<StudentProjection>, sqlx::Error>;

    async fn count_all(&self, name: Option<String>) -> Result<i64, sqlx::Error>;

    async fn find_by_id(&self, id: i64) -> Result<Option<Student>, sqlx::Error>;

    async fn insert(&self, student: Student) -> Result<Student, sqlx::Error>;

    async fn update(&self, id: i64, student: Student) -> Result<Student, sqlx::Error>;
}

#[async_trait]
impl StudentRepository for SqlitePool {
    async fn find_all_paginated(
        &self,
        page: i64,
        size: i64,
        name: Option<String>,
    ) -> Result<Vec<StudentProjection>, sqlx::Error> {
        let offset = page * size;

        let pattern = name
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .map(name_search_pattern);

        sqlx::query_as::<_, StudentProjection>(
            r#"
        SELECT
            s.id,
            s.name,
            s.email,
            s.phone,
            s.objective,
            s.is_active,
            c.first_due_date,
            c.amount_cents,
            c.billing_type
        FROM students s
        LEFT JOIN contracts c ON c.id = (
            SELECT selected_contract.id
            FROM contracts selected_contract
            WHERE selected_contract.student_id = s.id
              AND selected_contract.service_type = 0
            ORDER BY
                (selected_contract.contract_status = 0) DESC,
                selected_contract.start_date DESC,
                selected_contract.id DESC
            LIMIT 1
        )
        WHERE ($1 IS NULL OR s.name REGEXP $1)
        ORDER BY s.is_active DESC, s.name ASC, s.id ASC
        LIMIT $2 OFFSET $3
        "#,
        )
        .bind(pattern)
        .bind(size)
        .bind(offset)
        .fetch_all(self)
        .await
    }

    async fn count_all(&self, name: Option<String>) -> Result<i64, sqlx::Error> {
        let pattern = name
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .map(name_search_pattern);

        sqlx::query_scalar::<_, i64>(
            r#"
        SELECT COUNT(*)
        FROM students s
        WHERE ($1 IS NULL OR s.name REGEXP $1)
        "#,
        )
        .bind(pattern)
        .fetch_one(self)
        .await
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Student>, sqlx::Error> {
        sqlx::query_as::<_, Student>(
            r#"
        SELECT *
        FROM students
        WHERE id = $1
        "#,
        )
        .bind(id)
        .fetch_optional(self)
        .await
    }

    async fn insert(&self, student: Student) -> Result<Student, sqlx::Error> {
        sqlx::query_as::<_, Student>(
            r#"
            INSERT INTO students(
                 name,
                 cpf,
                 gender,
                 email,
                 birth_date,
                 phone,
                 zip_code,
                 address,
                 address_number,
                 address_complement,
                 city,
                 state,
                 objective,
                 notes,
                 is_active,
                 created_at,
                 updated_at
                 )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            RETURNING *
            "#,
        )
        .bind(student.name)
        .bind(student.cpf)
        .bind(student.gender)
        .bind(student.email)
        .bind(student.birth_date)
        .bind(student.phone)
        .bind(student.zip_code)
        .bind(student.address)
        .bind(student.address_number)
        .bind(student.address_complement)
        .bind(student.city)
        .bind(student.state)
        .bind(student.objective)
        .bind(student.notes)
        .bind(student.is_active)
        .bind(student.created_at)
        .bind(student.updated_at)
        .fetch_one(self)
        .await
        .map_err(|error| {
            eprintln!("Erro ao inserir aluno: {error:?}");
            error
        })
    }

    async fn update(&self, id: i64, student: Student) -> Result<Student, sqlx::Error> {
        sqlx::query_as::<_, Student>(
            r#"
            UPDATE students
            SET
                name = $1,
                cpf = $2,
                gender = $3,
                email = $4,
                birth_date = $5,
                phone = $6,
                zip_code = $7,
                address = $8,
                address_number = $9,
                address_complement = $10,
                city = $11,
                state = $12,
                objective = $13,
                notes = $14,
                is_active = $15,
                updated_at = $16
            WHERE id = $17
            RETURNING *
            "#,
        )
        .bind(student.name)
        .bind(student.cpf)
        .bind(student.gender)
        .bind(student.email)
        .bind(student.birth_date)
        .bind(student.phone)
        .bind(student.zip_code)
        .bind(student.address)
        .bind(student.address_number)
        .bind(student.address_complement)
        .bind(student.city)
        .bind(student.state)
        .bind(student.objective)
        .bind(student.notes)
        .bind(student.is_active)
        .bind(student.updated_at)
        .bind(id)
        .fetch_one(self)
        .await
    }
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait StudentStatusRepository: Send {
    async fn update_is_active(
        &mut self,
        student_id: i64,
        is_active: bool,
    ) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl StudentStatusRepository for SqliteConnection {
    async fn update_is_active(
        &mut self,
        student_id: i64,
        is_active: bool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE students SET is_active = $1 WHERE id = $2")
            .bind(is_active)
            .bind(student_id)
            .execute(&mut *self)
            .await?;

        Ok(())
    }
}
