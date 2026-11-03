use chrono::NaiveDate;

use crate::{
    domain::{
        bmi::{
            adult_classification::{adult_bmi_classification, calculate_ideal_weight_adult},
            bmi::{bmi_calculate, BmiClassification},
            child_classification::{
                bmi_child_classification, calculate_ideal_weight_child, get_bmi_child_z_scores,
            },
            elderly_classification::{calculate_ideal_weight_elderly, elderly_bmi_classification},
        },
        calculate_age::{calculate_age_in_months, calculate_age_in_years},
    },
    models::students::Gender,
};

#[test]
fn age_in_years_does_not_count_birthday_until_it_arrives() {
    let birth_date = NaiveDate::from_ymd_opt(2000, 5, 15).unwrap();

    assert_eq!(
        calculate_age_in_years(birth_date, NaiveDate::from_ymd_opt(2026, 5, 14).unwrap()),
        25
    );
    assert_eq!(
        calculate_age_in_years(birth_date, NaiveDate::from_ymd_opt(2026, 5, 15).unwrap()),
        26
    );
}

#[test]
fn age_in_months_counts_only_completed_months() {
    let birth_date = NaiveDate::from_ymd_opt(2020, 1, 15).unwrap();

    assert_eq!(
        calculate_age_in_months(birth_date, NaiveDate::from_ymd_opt(2025, 1, 14).unwrap()),
        59
    );
    assert_eq!(
        calculate_age_in_months(birth_date, NaiveDate::from_ymd_opt(2025, 1, 15).unwrap()),
        60
    );
}

#[test]
fn bmi_calculate_uses_weight_divided_by_height_squared() {
    assert!((bmi_calculate(80.0, 2.0) - 20.0).abs() < 1e-10);
}

#[test]
fn adult_bmi_classification_covers_category_boundaries() {
    assert!(matches!(
        adult_bmi_classification(18.49),
        BmiClassification::Thinness
    ));
    assert!(matches!(
        adult_bmi_classification(18.5),
        BmiClassification::NormalWeight
    ));
    assert!(matches!(
        adult_bmi_classification(24.9),
        BmiClassification::NormalWeight
    ));
    assert!(matches!(
        adult_bmi_classification(25.0),
        BmiClassification::OverWeight
    ));
    assert!(matches!(
        adult_bmi_classification(30.0),
        BmiClassification::ObesityClassI
    ));
    assert!(matches!(
        adult_bmi_classification(35.0),
        BmiClassification::ObesityClassII
    ));
    assert!(matches!(
        adult_bmi_classification(40.0),
        BmiClassification::ObesityClassIII
    ));
}

#[test]
fn adult_ideal_weight_returns_expected_range() {
    assert_eq!(calculate_ideal_weight_adult(2.0), [74.0, 88.8, 99.6]);
}

#[test]
fn elderly_bmi_classification_uses_elderly_cutoffs() {
    assert!(matches!(
        elderly_bmi_classification(21.99),
        BmiClassification::Thinness
    ));
    assert!(matches!(
        elderly_bmi_classification(22.0),
        BmiClassification::NormalWeight
    ));
    assert!(matches!(
        elderly_bmi_classification(26.9),
        BmiClassification::NormalWeight
    ));
    assert!(matches!(
        elderly_bmi_classification(27.0),
        BmiClassification::OverWeight
    ));
}

#[test]
fn elderly_ideal_weight_returns_expected_range() {
    assert_eq!(calculate_ideal_weight_elderly(2.0), [88.0, 98.0, 108.0]);
}

#[test]
fn child_bmi_classification_uses_age_and_gender_growth_reference() {
    assert!(matches!(
        bmi_child_classification(60, Gender::Male, 12.0),
        BmiClassification::SevereThinness
    ));
    assert!(matches!(
        bmi_child_classification(60, Gender::Male, 17.0),
        BmiClassification::OverWeight
    ));
    assert!(matches!(
        bmi_child_classification(60, Gender::Female, 22.0),
        BmiClassification::SevereObesity
    ));
}

#[test]
fn child_ideal_weight_uses_age_specific_z_scores() {
    let (_, z_minus_2, _, z_zero, z_plus_1, _, _) = get_bmi_child_z_scores(60, Gender::Male);

    assert_eq!(
        calculate_ideal_weight_child(60, Gender::Male, 2.0),
        [z_minus_2 * 4.0, z_zero * 4.0, z_plus_1 * 4.0]
    );
}
