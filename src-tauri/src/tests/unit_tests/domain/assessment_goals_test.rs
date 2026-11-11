use crate::{
    domain::{
        abdominal_circ_classification::values_abdominal_circumference,
        skin_folds::ideal_weight_by_fat_percentage,
        waist_to_hip_ratio::waist_hip_ratio_values,
    },
    models::students::Gender,
};

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 0.000001,
        "Esperado {expected}, recebido {actual}",
    );
}

#[test]
fn goals_are_absent_without_fat_percentage() {
    assert!(
        ideal_weight_by_fat_percentage(
            Gender::Male,
            35,
            None,
            100.0,
        )
            .is_none()
    );
}

#[test]
fn obese_man_receives_all_weight_targets() {
    // 100 kg e 30% de gordura: 70 kg de massa magra.
    let goals = ideal_weight_by_fat_percentage(
        Gender::Male,
        35,
        Some(30.0),
        100.0,
    )
        .unwrap();

    assert_close(goals.ideal_weight_min, 81.3953488372093);
    assert_close(goals.ideal_weight_max, 85.3658536585366);
    assert_close(goals.normal_weight_target, 88.6075949367089);
    assert_close(goals.overweight_target, 93.3333333333333);

    assert_eq!(goals.ideal_percentage_min, 14.0);
    assert_eq!(goals.ideal_percentage_max, 18.0);
    assert_eq!(goals.ideal_percentage_health, 21.0);
    assert_eq!(goals.ideal_percentage_overweight, 25.0);
}

#[test]
fn obese_woman_receives_female_reference_targets() {
    // 100 kg e 40% de gordura: 60 kg de massa magra.
    let goals = ideal_weight_by_fat_percentage(
        Gender::Female,
        35,
        Some(40.0),
        100.0,
    )
        .unwrap();

    assert_close(goals.ideal_weight_min, 74.0740740740741);
    assert_close(goals.ideal_weight_max, 80.0);
    assert_close(goals.normal_weight_target, 84.5070422535211);
    assert_close(goals.overweight_target, 89.5522388059701);

    assert_eq!(goals.ideal_percentage_min, 19.0);
    assert_eq!(goals.ideal_percentage_max, 25.0);
    assert_eq!(goals.ideal_percentage_health, 29.0);
    assert_eq!(goals.ideal_percentage_overweight, 33.0);
}

#[test]
fn overweight_person_has_no_overweight_target_remaining() {
    let goals = ideal_weight_by_fat_percentage(
        Gender::Male,
        35,
        Some(23.0),
        100.0,
    )
        .unwrap();

    assert_eq!(goals.overweight_target, 0.0);
    assert_close(goals.normal_weight_target, 97.4683544303797);
    assert_close(goals.ideal_weight_min, 89.5348837209302);
    assert_close(goals.ideal_weight_max, 93.9024390243902);
}

#[test]
fn normal_person_above_reference_keeps_only_reference_targets() {
    let goals = ideal_weight_by_fat_percentage(
        Gender::Male,
        35,
        Some(20.0),
        100.0,
    )
        .unwrap();

    assert_eq!(goals.overweight_target, 0.0);
    assert_eq!(goals.normal_weight_target, 0.0);
    assert_close(goals.ideal_weight_min, 93.0232558139535);
    assert_close(goals.ideal_weight_max, 97.5609756097561);
}

#[test]
fn current_weight_is_inside_targets_when_inside_reference_range() {
    for (gender, percentage) in [
        (Gender::Male, 16.0),
        (Gender::Female, 22.0),
    ] {
        let goals = ideal_weight_by_fat_percentage(
            gender,
            35,
            Some(percentage),
            100.0,
        )
            .unwrap();

        assert_eq!(goals.overweight_target, 0.0);
        assert_eq!(goals.normal_weight_target, 0.0);

        assert!(goals.ideal_weight_min < 100.0);
        assert!(goals.ideal_weight_max > 100.0);
    }
}

#[test]
fn below_reference_minimum_requires_higher_target_weight() {
    let goals = ideal_weight_by_fat_percentage(
        Gender::Male,
        35,
        Some(10.0),
        100.0,
    )
        .unwrap();

    assert_close(goals.ideal_weight_min, 104.6511627906977);
    assert_close(goals.ideal_weight_max, 109.7560975609756);
    assert_eq!(goals.normal_weight_target, 0.0);
    assert_eq!(goals.overweight_target, 0.0);
}

#[test]
fn reference_boundaries_return_current_weight_for_matching_target() {
    for (gender, minimum, maximum) in [
        (Gender::Male, 14.0, 18.0),
        (Gender::Female, 19.0, 25.0),
    ] {
        let at_minimum = ideal_weight_by_fat_percentage(
            gender,
            35,
            Some(minimum),
            100.0,
        )
            .unwrap();

        let at_maximum = ideal_weight_by_fat_percentage(
            gender,
            35,
            Some(maximum),
            100.0,
        )
            .unwrap();

        assert_close(at_minimum.ideal_weight_min, 100.0);
        assert_close(at_maximum.ideal_weight_max, 100.0);
    }
}

#[test]
fn classification_boundaries_return_current_weight_as_boundary_target() {
    let at_obesity = ideal_weight_by_fat_percentage(
        Gender::Male,
        35,
        Some(25.0),
        100.0,
    )
        .unwrap();

    assert_close(at_obesity.overweight_target, 100.0);
    assert!(at_obesity.normal_weight_target < 100.0);

    let at_normal_limit = ideal_weight_by_fat_percentage(
        Gender::Male,
        35,
        Some(21.0),
        100.0,
    )
        .unwrap();

    assert_close(at_normal_limit.normal_weight_target, 100.0);
    assert_eq!(at_normal_limit.overweight_target, 0.0);
}

#[test]
fn classification_targets_follow_age_bands_but_reference_stays_fixed() {
    let cases = [
        (Gender::Male, 29, 20.0, 25.0),
        (Gender::Male, 30, 21.0, 25.0),
        (Gender::Male, 39, 21.0, 25.0),
        (Gender::Male, 40, 23.0, 27.0),
        (Gender::Male, 49, 23.0, 27.0),
        (Gender::Male, 50, 24.0, 28.0),
        (Gender::Male, 59, 24.0, 28.0),
        (Gender::Male, 60, 25.0, 30.0),
        (Gender::Female, 29, 28.0, 32.0),
        (Gender::Female, 30, 29.0, 33.0),
        (Gender::Female, 39, 29.0, 33.0),
        (Gender::Female, 40, 32.0, 35.0),
        (Gender::Female, 49, 32.0, 35.0),
        (Gender::Female, 50, 33.0, 38.0),
        (Gender::Female, 59, 33.0, 38.0),
        (Gender::Female, 60, 35.0, 40.0),
    ];

    for (gender, age, normal, obesity) in cases {
        let goals = ideal_weight_by_fat_percentage(
            gender,
            age,
            Some(45.0),
            100.0,
        )
            .unwrap();

        assert_eq!(goals.ideal_percentage_health, normal);
        assert_eq!(goals.ideal_percentage_overweight, obesity);

        let reference = match gender {
            Gender::Male => (14.0, 18.0),
            Gender::Female => (19.0, 25.0),
        };

        assert_eq!(
            (goals.ideal_percentage_min, goals.ideal_percentage_max),
            reference,
        );
    }
}

#[test]
fn abdominal_reference_values_follow_gender() {
    assert_eq!(
        values_abdominal_circumference(Gender::Male),
        (94.0, 102.0),
    );

    assert_eq!(
        values_abdominal_circumference(Gender::Female),
        (80.0, 88.0),
    );
}

#[test]
fn waist_hip_reference_values_follow_gender_and_age() {
    let cases = [
        (Gender::Male, 29, (0.83, 0.88, 0.94)),
        (Gender::Male, 30, (0.84, 0.91, 0.96)),
        (Gender::Male, 39, (0.84, 0.91, 0.96)),
        (Gender::Male, 40, (0.88, 0.95, 1.00)),
        (Gender::Male, 49, (0.88, 0.95, 1.00)),
        (Gender::Male, 50, (0.90, 0.96, 1.02)),
        (Gender::Male, 59, (0.90, 0.96, 1.02)),
        (Gender::Male, 60, (0.91, 0.98, 1.03)),
        (Gender::Female, 29, (0.71, 0.77, 0.82)),
        (Gender::Female, 30, (0.72, 0.78, 0.84)),
        (Gender::Female, 39, (0.72, 0.78, 0.84)),
        (Gender::Female, 40, (0.73, 0.79, 0.87)),
        (Gender::Female, 49, (0.73, 0.79, 0.87)),
        (Gender::Female, 50, (0.74, 0.81, 0.88)),
        (Gender::Female, 59, (0.74, 0.81, 0.88)),
        (Gender::Female, 60, (0.76, 0.83, 0.90)),
    ];

    for (gender, age, expected) in cases {
        assert_eq!(
            waist_hip_ratio_values(gender, age),
            expected,
            "Referências incorretas para {gender:?}, idade {age}.",
        );
    }
}