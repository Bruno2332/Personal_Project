use crate::models::physical_assessment::PhysicalAssessment;
use async_trait::async_trait;
use sqlx::{SqliteConnection, SqlitePool};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PhysicalAssessmentPoolRepository: Send + Sync {
    async fn find_all_by_student_id(
        &self,
        student_id: i64,
    ) -> Result<Vec<PhysicalAssessment>, sqlx::Error>;

    async fn delete_by_id(&self, id: i64) -> Result<u64, sqlx::Error>;
}

#[async_trait]
impl PhysicalAssessmentPoolRepository for SqlitePool {
    async fn find_all_by_student_id(
        &self,
        student_id: i64,
    ) -> Result<Vec<PhysicalAssessment>, sqlx::Error> {
        sqlx::query_as::<_, PhysicalAssessment>(
            r#"
            SELECT *
            FROM physical_assessments
            WHERE student_id = $1
            ORDER BY assessment_date
            "#,
        )
        .bind(student_id)
        .fetch_all(self)
        .await
    }

    async fn delete_by_id(&self, id: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM physical_assessments
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(self)
        .await?;

        Ok(result.rows_affected())
    }
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PhysicalAssessmentConnectionRepository: Send + Sync {
    async fn insert(
        &mut self,
        entity: PhysicalAssessment,
    ) -> Result<PhysicalAssessment, sqlx::Error>;

    async fn update(
        &mut self,
        entity: PhysicalAssessment,
        id: i64,
    ) -> Result<PhysicalAssessment, sqlx::Error>;
}

#[async_trait]
impl PhysicalAssessmentConnectionRepository for SqliteConnection {
    async fn insert(
        &mut self,
        entity: PhysicalAssessment,
    ) -> Result<PhysicalAssessment, sqlx::Error> {
        sqlx::query_as::<_, PhysicalAssessment>(
            r#"
            INSERT INTO physical_assessments (
                student_id,
                contract_id,
                assessment_date,
                weight_kg,
                height_m,
                systolic_pressure,
                diastolic_pressure,
                resting_heart_rate_bpm,
                notes,
                right_handgrip_strength,
                left_handgrip_strength,
                neck_circumference_cm,
                right_arm_circumference_cm,
                left_arm_circumference_cm,
                right_forearm_circumference_cm,
                left_forearm_circumference_cm,
                shoulder_circumference_cm,
                chest_circumference_cm,
                waist_circumference_cm,
                abdomen_circumference_cm,
                hip_circumference_cm,
                right_thigh_circumference_cm,
                left_thigh_circumference_cm,
                right_calf_circumference_cm,
                left_calf_circumference_cm,
                triceps_skinfold_mm,
                chest_skinfold_mm,
                midaxillary_skinfold_mm,
                subscapular_skinfold_mm,
                abdominal_skinfold_mm,
                suprailiac_skinfold_mm,
                thigh_skinfold_mm,
                bioimpedance_fat_percentage,
                right_extensor_strength,
                left_extensor_strength,
                right_flexor_strength,
                left_flexor_strength,
                right_biceps_strength,
                left_biceps_strength,
                right_triceps_strength,
                left_triceps_strength,
                vo2_final_stage_seconds,
                created_at,
                updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21,
                      $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37, $38, $39, $40, $41, $42, $43, $44)
            RETURNING *
            "#
        )
            .bind(entity.student_id)
            .bind(entity.contract_id)
            .bind(entity.assessment_date)
            .bind(entity.weight_kg)
            .bind(entity.height_m)
            .bind(entity.systolic_pressure)
            .bind(entity.diastolic_pressure)
            .bind(entity.resting_heart_rate_bpm)
            .bind(entity.notes)
            .bind(entity.right_handgrip_strength)
            .bind(entity.left_handgrip_strength)
            .bind(entity.neck_circumference_cm)
            .bind(entity.right_arm_circumference_cm)
            .bind(entity.left_arm_circumference_cm)
            .bind(entity.right_forearm_circumference_cm)
            .bind(entity.left_forearm_circumference_cm)
            .bind(entity.shoulder_circumference_cm)
            .bind(entity.chest_circumference_cm)
            .bind(entity.waist_circumference_cm)
            .bind(entity.abdomen_circumference_cm)
            .bind(entity.hip_circumference_cm)
            .bind(entity.right_thigh_circumference_cm)
            .bind(entity.left_thigh_circumference_cm)
            .bind(entity.right_calf_circumference_cm)
            .bind(entity.left_calf_circumference_cm)
            .bind(entity.triceps_skinfold_mm)
            .bind(entity.chest_skinfold_mm)
            .bind(entity.midaxillary_skinfold_mm)
            .bind(entity.subscapular_skinfold_mm)
            .bind(entity.abdominal_skinfold_mm)
            .bind(entity.suprailiac_skinfold_mm)
            .bind(entity.thigh_skinfold_mm)
            .bind(entity.bioimpedance_fat_percentage)
            .bind(entity.right_extensor_strength)
            .bind(entity.left_extensor_strength)
            .bind(entity.right_flexor_strength)
            .bind(entity.left_flexor_strength)
            .bind(entity.right_biceps_strength)
            .bind(entity.left_biceps_strength)
            .bind(entity.right_triceps_strength)
            .bind(entity.left_triceps_strength)
            .bind(entity.vo2_final_stage_seconds)
            .bind(entity.created_at)
            .bind(entity.updated_at)
            .fetch_one(&mut *self)
            .await
    }

    async fn update(
        &mut self,
        entity: PhysicalAssessment,
        id: i64,
    ) -> Result<PhysicalAssessment, sqlx::Error> {
        sqlx::query_as::<_, PhysicalAssessment>(
            r#"
            UPDATE physical_assessments
            SET
                student_id = $1,
                contract_id = $2,
                assessment_date = $3,
                weight_kg = $4,
                height_m = $5,
                systolic_pressure = $6,
                diastolic_pressure = $7,
                resting_heart_rate_bpm = $8,
                right_handgrip_strength = $9,
                left_handgrip_strength = $10,
                neck_circumference_cm = $11,
                right_arm_circumference_cm = $12,
                left_arm_circumference_cm = $13,
                right_forearm_circumference_cm = $14,
                left_forearm_circumference_cm = $15,
                shoulder_circumference_cm = $16,
                chest_circumference_cm = $17,
                waist_circumference_cm = $18,
                abdomen_circumference_cm = $19,
                hip_circumference_cm = $20,
                right_thigh_circumference_cm = $21,
                left_thigh_circumference_cm = $22,
                right_calf_circumference_cm = $23,
                left_calf_circumference_cm = $24,
                triceps_skinfold_mm = $25,
                chest_skinfold_mm = $26,
                midaxillary_skinfold_mm = $27,
                subscapular_skinfold_mm = $28,
                abdominal_skinfold_mm = $29,
                suprailiac_skinfold_mm = $30,
                thigh_skinfold_mm = $31,
                bioimpedance_fat_percentage = $32,
                right_extensor_strength = $33,
                left_extensor_strength = $34,
                right_flexor_strength = $35,
                left_flexor_strength = $36,
                right_biceps_strength = $37,
                left_biceps_strength = $38,
                right_triceps_strength = $39,
                left_triceps_strength = $40,
                vo2_final_stage_seconds = $41,
                updated_at = $42,
                notes = $43
            WHERE id = $44
            RETURNING *
            "#,
        )
        .bind(entity.student_id)
        .bind(entity.contract_id)
        .bind(entity.assessment_date)
        .bind(entity.weight_kg)
        .bind(entity.height_m)
        .bind(entity.systolic_pressure)
        .bind(entity.diastolic_pressure)
        .bind(entity.resting_heart_rate_bpm)
        .bind(entity.right_handgrip_strength)
        .bind(entity.left_handgrip_strength)
        .bind(entity.neck_circumference_cm)
        .bind(entity.right_arm_circumference_cm)
        .bind(entity.left_arm_circumference_cm)
        .bind(entity.right_forearm_circumference_cm)
        .bind(entity.left_forearm_circumference_cm)
        .bind(entity.shoulder_circumference_cm)
        .bind(entity.chest_circumference_cm)
        .bind(entity.waist_circumference_cm)
        .bind(entity.abdomen_circumference_cm)
        .bind(entity.hip_circumference_cm)
        .bind(entity.right_thigh_circumference_cm)
        .bind(entity.left_thigh_circumference_cm)
        .bind(entity.right_calf_circumference_cm)
        .bind(entity.left_calf_circumference_cm)
        .bind(entity.triceps_skinfold_mm)
        .bind(entity.chest_skinfold_mm)
        .bind(entity.midaxillary_skinfold_mm)
        .bind(entity.subscapular_skinfold_mm)
        .bind(entity.abdominal_skinfold_mm)
        .bind(entity.suprailiac_skinfold_mm)
        .bind(entity.thigh_skinfold_mm)
        .bind(entity.bioimpedance_fat_percentage)
        .bind(entity.right_extensor_strength)
        .bind(entity.left_extensor_strength)
        .bind(entity.right_flexor_strength)
        .bind(entity.left_flexor_strength)
        .bind(entity.right_biceps_strength)
        .bind(entity.left_biceps_strength)
        .bind(entity.right_triceps_strength)
        .bind(entity.left_triceps_strength)
        .bind(entity.vo2_final_stage_seconds)
        .bind(entity.updated_at)
        .bind(entity.notes)
        .bind(id)
        .fetch_one(&mut *self)
        .await
        .map_err(|error| {
            eprintln!("Erro ao inserir avaliação física: {error:?}");
            error
        })
    }
}
