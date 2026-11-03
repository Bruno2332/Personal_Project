use crate::domain::abdominal_circ_classification::abdominal_circumference_calculate;
use crate::domain::bmi::adult_classification::{
    adult_bmi_classification, calculate_ideal_weight_adult,
};
use crate::domain::bmi::bmi::bmi_calculate;
use crate::domain::bmi::child_classification::{
    bmi_child_classification, calculate_ideal_weight_child,
};
use crate::domain::bmi::elderly_classification::{
    calculate_ideal_weight_elderly, elderly_bmi_classification,
};
use crate::domain::calculate_age::{calculate_age_in_months, calculate_age_in_years};
use crate::domain::dynamometer_strength::dynamometer_classification;
use crate::domain::fc_classification::{
    exercise_heart_rate_classification, fc_percentage, fc_rest_classification,
};
use crate::domain::hypertension_classification::hypertension_classification;
use crate::domain::skin_folds::{
    calculate_kilos_fat, classify_fat_percentage, fat_percentage, ideal_weight_by_fat_percentage,
};
use crate::domain::strength_asymmetry::{asymmetry_classification, strength_asymmetry};
use crate::domain::vo2_classification::{calculate_vo2_max, vo2_classification};
use crate::domain::waist_to_hip_ratio::{
    waist_hip_ratio_calculate, waist_hip_ratio_classification,
};
use crate::dtos::contract_dto::CreateContractRequest;
use crate::dtos::physical_assessment_dto::{PhysicalAssessmentCompleteDto, PhysicalAssessmentDto};
use crate::dtos::vo2_assessment_dto::{Vo2AssessmentCompleteDto, Vo2AssessmentDto};
use crate::errors::app_errors::{AppError, FieldError};
use crate::models::physical_assessment::PhysicalAssessment;
use crate::models::students::Student;
use crate::models::vo2_assessment::Vo2Assessment;
use crate::repositories::contract_repository::ContractRepository;
use crate::repositories::financial_records_repository::FinancialRecordsConnectionRepository;
use crate::repositories::physical_assessment_repository::{
    PhysicalAssessmentConnectionRepository, PhysicalAssessmentPoolRepository,
};
use crate::repositories::student_repository::{StudentRepository, StudentStatusRepository};
use crate::repositories::vo2_assessment_repository::{
    Vo2AssessmentConnectionRepository, Vo2AssessmentPoolRepository,
};
use crate::services::contract_service;
use chrono::Utc;
use std::collections::HashMap;
use crate::utils::validations::validate_dto;

pub async fn find_all_by_student_id<R>(
    repository: &R,
    id: i64,
) -> Result<Vec<PhysicalAssessmentCompleteDto>, AppError>
where
    R: StudentRepository + PhysicalAssessmentPoolRepository + Vo2AssessmentPoolRepository,
{
    let student = StudentRepository::find_by_id(repository, id)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao buscar aluno".to_string(),
        })?
        .ok_or_else(|| AppError::NotFound {
            message: "Aluno não encontrado".to_string(),
        })?;

    let entities = PhysicalAssessmentPoolRepository::find_all_by_student_id(repository, id)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao buscar avaliações físicas".to_string(),
        })?;

    let vo2_assessments = Vo2AssessmentPoolRepository::find_all_by_student_id(repository, id)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao buscar etapas de VO₂".to_string(),
        })?;

    let vo2_assessments: Vec<Vo2AssessmentDto> = vo2_assessments
        .into_iter()
        .map(Vo2AssessmentDto::from)
        .collect();

    let mut vo2_by_assessment: HashMap<i64, Vec<Vo2AssessmentDto>> = HashMap::new();

    for stage in vo2_assessments {
        vo2_by_assessment
            .entry(stage.physical_assessment_id)
            .or_default()
            .push(stage);
    }

    let items = entities
        .into_iter()
        .map(|entity| {
            let stages = entity
                .id
                .and_then(|assessment_id| vo2_by_assessment.remove(&assessment_id))
                .unwrap_or_default();

            build_complete_dto(entity, &student, stages)
        })
        .collect();

    Ok(items)
}

pub async fn insert<R>(
    repository: &mut R,
    contract_request: CreateContractRequest,
    mut dto: PhysicalAssessmentDto,
    vo2_assessments: Vec<Vo2AssessmentDto>,
) -> Result<PhysicalAssessmentDto, AppError>
where
    R: PhysicalAssessmentConnectionRepository
    + Vo2AssessmentConnectionRepository
    + ContractRepository
    + FinancialRecordsConnectionRepository
    + StudentStatusRepository,
{
    validate_dto(&dto)?;

    let contract =
        contract_service::insert(&mut *repository, contract_request)
            .await?;

    dto.contract_id = contract.id.ok_or_else(|| AppError::Database {
        message: "O contrato criado não retornou um ID".to_string(),
    })?;

    dto.student_id = contract.student_id;

    let now = Utc::now();

    let mut entity = PhysicalAssessment::from(dto);
    entity.created_at = now;
    entity.updated_at = now;

    let created = PhysicalAssessmentConnectionRepository::insert(
        &mut *repository,
        entity,
    )
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao criar avaliação física".to_string(),
        })?;

    let assessment_id = created.id.ok_or_else(|| AppError::Database {
        message: "A avaliação criada não retornou um ID".to_string(),
    })?;

    let stages = vo2_assessments
        .into_iter()
        .map(|stage| Vo2Assessment {
            id: None,
            physical_assessment_id: assessment_id,
            treadmill_speed_kmh: stage.treadmill_speed_kmh,
            heart_rate_bpm: stage.heart_rate_bpm,
            perceived_exertion: stage.perceived_exertion,
            created_at: now,
            updated_at: now,
        })
        .collect();

    Vo2AssessmentConnectionRepository::insert_many(
        &mut *repository,
        stages,
    )
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao criar etapas de VO₂".to_string(),
        })?;

    Ok(PhysicalAssessmentDto::from(created))
}

pub async fn update<R>(
    repository: &mut R,
    id: i64,
    dto: PhysicalAssessmentDto,
    vo2_assessments: Vec<Vo2AssessmentDto>,
    removed_vo2_ids: Vec<i64>,
) -> Result<PhysicalAssessmentDto, AppError>
where
    R: PhysicalAssessmentConnectionRepository
    + Vo2AssessmentConnectionRepository
    + ContractRepository,
{
    validate_dto(&dto)?;

    validate_contract_student(
        &mut *repository,
        dto.contract_id,
        dto.student_id,
    )
        .await?;

    let now = Utc::now();

    let mut entity = PhysicalAssessment::from(dto);
    entity.updated_at = now;

    let updated = PhysicalAssessmentConnectionRepository::update(
        &mut *repository,
        entity,
        id,
    )
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => AppError::NotFound {
                message: "Avaliação física não encontrada".to_string(),
            },
            _ => AppError::Database {
                message: "Erro ao atualizar avaliação física".to_string(),
            },
        })?;

    Vo2AssessmentConnectionRepository::delete_by_ids(
        &mut *repository,
        removed_vo2_ids,
        id,
    )
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao excluir etapas de VO₂".to_string(),
        })?;

    let mut new_stages = Vec::new();
    let mut existing_stages = Vec::new();

    for stage in vo2_assessments {
        let entity = Vo2Assessment {
            id: stage.id,
            physical_assessment_id: id,
            treadmill_speed_kmh: stage.treadmill_speed_kmh,
            heart_rate_bpm: stage.heart_rate_bpm,
            perceived_exertion: stage.perceived_exertion,

            // Gravado apenas no insert. O update_many deve
            // preservar o created_at que já está no banco.
            created_at: now,
            updated_at: now,
        };

        if entity.id.is_some() {
            existing_stages.push(entity);
        } else {
            new_stages.push(entity);
        }
    }

    Vo2AssessmentConnectionRepository::update_many(
        &mut *repository,
        id,
        existing_stages,
    )
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => AppError::NotFound {
                message:
                "Uma ou mais etapas de VO₂ não foram encontradas nesta avaliação."
                    .to_string(),
            },
            _ => AppError::Database {
                message: "Erro ao atualizar etapas de VO₂".to_string(),
            },
        })?;

    Vo2AssessmentConnectionRepository::insert_many(
        &mut *repository,
        new_stages,
    )
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao criar etapas de VO₂".to_string(),
        })?;

    Ok(PhysicalAssessmentDto::from(updated))
}

pub async fn delete<R: PhysicalAssessmentPoolRepository>(
    repository: &R,
    id: i64,
) -> Result<(), AppError> {
    let rows_affected = repository
        .delete_by_id(id)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao deletar avaliação física".to_string(),
        })?;

    if rows_affected == 0 {
        return Err(AppError::NotFound {
            message: "Avaliação física não encontrada".to_string(),
        });
    }
    Ok(())
}

fn build_complete_dto(
    entity: PhysicalAssessment,
    student: &Student,
    vo2_assessment: Vec<Vo2AssessmentDto>,
) -> PhysicalAssessmentCompleteDto {
    let gender = student.gender;

    let age = calculate_age_in_years(student.birth_date, entity.assessment_date);

    let age_months = calculate_age_in_months(student.birth_date, entity.assessment_date);

    let bmi = bmi_calculate(entity.weight_kg, entity.height_m);

    let (bmi_classification, ideal_weights) = match age {
        0..=18 => (
            bmi_child_classification(age_months, gender, bmi),
            calculate_ideal_weight_child(age_months, gender, entity.height_m),
        ),

        19..=59 => (
            adult_bmi_classification(bmi),
            calculate_ideal_weight_adult(entity.height_m),
        ),

        _ => (
            elderly_bmi_classification(bmi),
            calculate_ideal_weight_elderly(entity.height_m),
        ),
    };

    let fat_percentage_skinfolds = fat_percentage(
        gender,
        age,
        entity.triceps_skinfold_mm,
        entity.chest_skinfold_mm,
        entity.midaxillary_skinfold_mm,
        entity.subscapular_skinfold_mm,
        entity.abdominal_skinfold_mm,
        entity.suprailiac_skinfold_mm,
        entity.thigh_skinfold_mm,
    );
    let ideal_weight =
        ideal_weight_by_fat_percentage(gender, age, fat_percentage_skinfolds, entity.weight_kg);

    let vo2_parameter = vo2_assessment.last().and_then(|stage| {
        calculate_vo2_max(stage.treadmill_speed_kmh, entity.vo2_final_stage_seconds)
    });

    let kilos_fat_skinfolds = calculate_kilos_fat(entity.weight_kg, fat_percentage_skinfolds);
    let kilos_fat_bioimpedance =
        calculate_kilos_fat(entity.weight_kg, entity.bioimpedance_fat_percentage);
    let ratio_hundredths =
        waist_hip_ratio_calculate(entity.waist_circumference_cm, entity.hip_circumference_cm);
    let biceps_asymmetry =
        strength_asymmetry(entity.left_biceps_strength, entity.right_biceps_strength);
    let triceps_asymmetry =
        strength_asymmetry(entity.left_triceps_strength, entity.right_triceps_strength);
    let extensor_asymmetry = strength_asymmetry(
        entity.left_extensor_strength,
        entity.right_extensor_strength,
    );
    let flexor_asymmetry =
        strength_asymmetry(entity.left_flexor_strength, entity.right_flexor_strength);

    let mut vo2_assessment_complete: Vec<Vo2AssessmentCompleteDto> =
        Vec::with_capacity(vo2_assessment.len());

    for i in &vo2_assessment {
        let fc_percentage = fc_percentage(age, i.heart_rate_bpm);
        let result = Vo2AssessmentCompleteDto {
            vo2_assessment_dto: i.clone(),
            fc_heart_percentage: fc_percentage,
            fc_heart_classification: exercise_heart_rate_classification(fc_percentage),
        };
        vo2_assessment_complete.push(result);
    }

    PhysicalAssessmentCompleteDto {
        physical_assessment_dto: PhysicalAssessmentDto::from(entity.clone()),
        age,
        bmi,
        bmi_classification,
        min_ideal_weight: ideal_weights[0],
        medium_ideal_weight: ideal_weights[1],
        max_ideal_weight: ideal_weights[2],

        hypertension_classification: hypertension_classification(
            entity.systolic_pressure,
            entity.diastolic_pressure,
        ),
        circumference_abdominal_classification: abdominal_circumference_calculate(
            gender,
            entity.abdomen_circumference_cm,
        ),

        fc_heart_rest_classification: fc_rest_classification(entity.resting_heart_rate_bpm),

        left_dynamometer_strength_classification: dynamometer_classification(
            gender,
            age,
            entity.left_handgrip_strength,
        ),
        right_dynamometer_strength_classification: dynamometer_classification(
            gender,
            age,
            entity.right_handgrip_strength,
        ),

        kilos_lean_body_mass_skinfolds: kilos_fat_skinfolds.map(|fat| entity.weight_kg - fat),
        kilos_fat_skinfolds,
        lean_body_mass_percentage_skinfolds: fat_percentage_skinfolds.map(|fat| 100.0 - fat),
        fat_percentage_skinfolds,

        ideal_weight_fat_percentage: ideal_weight,
        weight_to_lose: ideal_weight.map(|ideal| entity.weight_kg - ideal),

        kilos_lean_body_mass_bioimpedance: kilos_fat_bioimpedance.map(|fat| entity.weight_kg - fat),
        kilos_fat_bioimpedance,
        lean_body_mass_percentage_bioimpedance: entity
            .bioimpedance_fat_percentage
            .map(|fat| 100.0 - fat),

        fat_percentage_classification_skinfolds: classify_fat_percentage(
            gender,
            age,
            fat_percentage_skinfolds,
        ),
        fat_percentage_classification_bioimpedance: classify_fat_percentage(
            gender,
            age,
            entity.bioimpedance_fat_percentage,
        ),

        biceps_asymmetry_percentage: biceps_asymmetry,
        triceps_asymmetry_percentage: triceps_asymmetry,
        extensor_asymmetry_percentage: extensor_asymmetry,
        flexor_asymmetry_percentage: flexor_asymmetry,

        biceps_asymmetry_classification: asymmetry_classification(biceps_asymmetry),
        triceps_asymmetry_classification: asymmetry_classification(triceps_asymmetry),
        extensor_asymmetry_classification: asymmetry_classification(extensor_asymmetry),
        flexor_asymmetry_classification: asymmetry_classification(flexor_asymmetry),

        waist_hip_calculate: ratio_hundredths,
        waist_hip_classification: waist_hip_ratio_classification(ratio_hundredths, age, gender),

        maximum_heart_rate: 220 - age,
        vo2_assessments: vo2_assessment_complete,
        vo2_parameter,
        vo2_classification: vo2_classification(gender, age, vo2_parameter),
    }
}

async fn validate_contract_student<R: ContractRepository>(
    repository: &mut R,
    contract_id: i64,
    student_id: i64,
) -> Result<(), AppError> {
    let belongs = repository
        .belongs_to_student(contract_id, student_id)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao verificar contrato do aluno".to_string(),
        })?;

    if !belongs {
        return Err(AppError::Validation {
            errors: vec![FieldError {
                field: "contract_id".to_string(),
                message: "O contrato não existe ou não pertence ao aluno informado.".to_string(),
            }],
        });
    }

    Ok(())
}
