use crate::{
    domain::skin_folds::{
        calculate_kilos_fat, classify_fat_percentage, fat_percentage,
        ideal_weight_by_fat_percentage, FatPercentageClassification,
    },
    models::students::Gender,
};

fn approximately(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 0.01, "{actual} != {expected}");
}

#[test]
fn skinfold_calculation_requires_all_seven_measurements() {
    let complete = fat_percentage(
        Gender::Male,
        30,
        Some(10.0),
        Some(10.0),
        Some(10.0),
        Some(10.0),
        Some(10.0),
        Some(10.0),
        Some(10.0),
    );
    let missing = fat_percentage(
        Gender::Male,
        30,
        Some(10.0),
        Some(10.0),
        Some(10.0),
        Some(10.0),
        Some(10.0),
        Some(10.0),
        None,
    );

    assert!(complete.is_some());
    assert_eq!(missing, None);
}

#[test]
fn fat_mass_calculation_converts_percentage_to_kilograms() {
    approximately(calculate_kilos_fat(80.0, Some(25.0)).unwrap(), 20.0);
    assert_eq!(calculate_kilos_fat(80.0, None), None);
}

#[test]
fn fat_percentage_classification_uses_gender_and_age_ranges() {
    assert_eq!(
        classify_fat_percentage(Gender::Male, 25, Some(10.0)),
        Some(FatPercentageClassification::Excellent)
    );
    assert_eq!(
        classify_fat_percentage(Gender::Male, 25, Some(20.0)),
        Some(FatPercentageClassification::Healthy)
    );
    assert_eq!(
        classify_fat_percentage(Gender::Male, 25, Some(22.0)),
        Some(FatPercentageClassification::Overweight)
    );
    assert_eq!(
        classify_fat_percentage(Gender::Female, 25, Some(32.0)),
        Some(FatPercentageClassification::Obesity)
    );
    assert_eq!(classify_fat_percentage(Gender::Male, 25, None), None);
}

#[test]
fn body_composition_goals_return_reference_range_and_applicable_targets() {
    assert!(
        ideal_weight_by_fat_percentage(
            Gender::Male,
            30,
            None,
            100.0,
        )
            .is_none()
    );

    // 20%: abaixo do limite de normalidade para essa idade.
    // Mantém a faixa de referência, mas zera os outros alvos.
    let normal = ideal_weight_by_fat_percentage(
        Gender::Male,
        30,
        Some(20.0),
        100.0,
    )
        .unwrap();

    approximately(normal.ideal_weight_min, 80.0 / 0.86);
    approximately(normal.ideal_weight_max, 80.0 / 0.82);
    assert_eq!(normal.normal_weight_target, 0.0);
    assert_eq!(normal.overweight_target, 0.0);

    // 25%: exatamente no limite de obesidade.
    let boundary = ideal_weight_by_fat_percentage(
        Gender::Male,
        30,
        Some(25.0),
        100.0,
    )
        .unwrap();

    approximately(boundary.overweight_target, 100.0);
    approximately(boundary.normal_weight_target, 75.0 / 0.79);

    // 30%: retorna todos os pesos-alvo.
    let obesity = ideal_weight_by_fat_percentage(
        Gender::Male,
        30,
        Some(30.0),
        100.0,
    )
        .unwrap();

    approximately(obesity.overweight_target, 70.0 / 0.75);
    approximately(obesity.normal_weight_target, 70.0 / 0.79);
    approximately(obesity.ideal_weight_min, 70.0 / 0.86);
    approximately(obesity.ideal_weight_max, 70.0 / 0.82);

    assert_eq!(obesity.ideal_percentage_min, 14.0);
    assert_eq!(obesity.ideal_percentage_max, 18.0);
    assert_eq!(obesity.ideal_percentage_health, 21.0);
    assert_eq!(obesity.ideal_percentage_overweight, 25.0);
}
