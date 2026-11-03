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
fn ideal_weight_is_only_provided_when_fat_is_above_healthy_maximum() {
    assert_eq!(
        ideal_weight_by_fat_percentage(Gender::Male, 30, None, 100.0),
        None
    );
    assert_eq!(
        ideal_weight_by_fat_percentage(Gender::Male, 30, Some(20.0), 100.0),
        None
    );

    let ideal = ideal_weight_by_fat_percentage(Gender::Male, 30, Some(25.0), 100.0).unwrap();
    approximately(ideal, 100.0 * 0.75 / 0.75);

    let obesity_reference =
        ideal_weight_by_fat_percentage(Gender::Male, 30, Some(30.0), 100.0).unwrap();
    approximately(obesity_reference, 70.0 / 0.75);
}
