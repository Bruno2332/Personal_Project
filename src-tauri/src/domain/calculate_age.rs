use chrono::{Datelike, NaiveDate};

pub fn calculate_age_in_years(birth_date: NaiveDate, assessment_date: NaiveDate) -> i32 {
    let mut years = assessment_date.year() - birth_date.year();

    if (assessment_date.month(), assessment_date.day()) < (birth_date.month(), birth_date.day()) {
        years -= 1;
    }

    years
}

pub fn calculate_age_in_months(birth_date: NaiveDate, assessment_date: NaiveDate) -> i32 {
    let mut months = (assessment_date.year() - birth_date.year()) * 12
        + assessment_date.month() as i32
        - birth_date.month() as i32;

    if assessment_date.day() < birth_date.day() {
        months -= 1;
    }

    months
}
