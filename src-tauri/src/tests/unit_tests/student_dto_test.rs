use crate::tests::factories::student_factory::valid_student_dto;
use chrono::{Duration, Local, Months};
use validator::Validate;

#[test]
fn accepts_valid_student() {
    let dto = valid_student_dto();

    assert!(dto.validate().is_ok());
}

#[test]
fn accepts_name_with_three_characters() {
    let mut dto = valid_student_dto();
    dto.name = "Ana".to_string();

    assert!(dto.validate().is_ok());
}

#[test]
fn accepts_name_with_eighty_characters() {
    let mut dto = valid_student_dto();
    dto.name = "a".repeat(80);

    assert!(dto.validate().is_ok());
}

#[test]
fn rejects_name_shorter_than_three_characters() {
    for name in ["", "A", "Al"] {
        let mut dto = valid_student_dto();
        dto.name = name.to_string();

        let errors = dto.validate().expect_err("O nome deveria ser rejeitado");

        assert!(errors.field_errors().contains_key("name"));
    }
}

#[test]
fn rejects_name_longer_than_eighty_characters() {
    let mut dto = valid_student_dto();
    dto.name = "a".repeat(81);

    let errors = dto.validate().expect_err("O nome deveria ser rejeitado");

    assert!(errors.field_errors().contains_key("name"));
}

#[test]
fn rejects_cpf_with_incorrect_length() {
    for cpf in ["", "1234567890", "123456789012"] {
        let mut dto = valid_student_dto();
        dto.cpf = cpf.to_string();

        let errors = dto.validate().expect_err("O CPF deveria ser rejeitado");

        assert!(errors.field_errors().contains_key("cpf"));
    }
}

#[test]
fn rejects_invalid_email() {
    for email in ["", "aluno", "aluno@", "@example.com"] {
        let mut dto = valid_student_dto();
        dto.email = email.to_string();

        let errors = dto.validate().expect_err("O e-mail deveria ser rejeitado");

        assert!(errors.field_errors().contains_key("email"));
    }
}

#[test]
fn rejects_future_birth_date() {
    let mut dto = valid_student_dto();

    dto.birth_date = Local::now()
        .date_naive()
        .checked_add_signed(Duration::days(1))
        .unwrap();

    let errors = dto
        .validate()
        .expect_err("Nascimento futuro deveria ser rejeitado");

    assert!(errors.field_errors().contains_key("birth_date"));
}

#[test]
fn rejects_birth_date_today() {
    let mut dto = valid_student_dto();
    dto.birth_date = Local::now().date_naive();

    let errors = dto
        .validate()
        .expect_err("Aluno sem cinco anos deveria ser rejeitado");

    assert!(errors.field_errors().contains_key("birth_date"));
}

#[test]
fn accepts_student_exactly_five_years_old() {
    let mut dto = valid_student_dto();

    dto.birth_date = Local::now()
        .date_naive()
        .checked_sub_months(Months::new(60))
        .unwrap();

    assert!(dto.validate().is_ok());
}

#[test]
fn rejects_student_one_day_younger_than_five_years() {
    let mut dto = valid_student_dto();

    dto.birth_date = Local::now()
        .date_naive()
        .checked_sub_months(Months::new(60))
        .unwrap()
        .checked_add_signed(Duration::days(1))
        .unwrap();

    let errors = dto
        .validate()
        .expect_err("Aluno com menos de cinco anos deveria ser rejeitado");

    assert!(errors.field_errors().contains_key("birth_date"));
}

#[test]
fn accepts_mobile_and_landline_phone_numbers() {
    for phone in ["(11) 99999-9999", "(11) 3333-4444"] {
        let mut dto = valid_student_dto();
        dto.phone = Some(phone.to_string());

        assert!(
            dto.validate().is_ok(),
            "O telefone {phone} deveria ser aceito",
        );
    }
}

#[test]
fn accepts_missing_phone() {
    let mut dto = valid_student_dto();
    dto.phone = None;

    assert!(dto.validate().is_ok());
}

#[test]
fn rejects_invalid_phone_format() {
    for phone in [
        "",
        "11999999999",
        "(1) 99999-9999",
        "(11) 999-9999",
        "(11) 999999-9999",
        "(11) ABCDE-9999",
    ] {
        let mut dto = valid_student_dto();
        dto.phone = Some(phone.to_string());

        let errors = dto
            .validate()
            .expect_err("O telefone deveria ser rejeitado");

        assert!(
            errors.field_errors().contains_key("phone"),
            "Deveria existir erro para o telefone {phone}",
        );
    }
}

#[test]
fn accepts_missing_zip_code() {
    let mut dto = valid_student_dto();
    dto.zip_code = None;

    assert!(dto.validate().is_ok());
}

#[test]
fn accepts_zip_code_with_mask() {
    let mut dto = valid_student_dto();
    dto.zip_code = Some("14098-374".to_string());

    assert!(dto.validate().is_ok());
}

#[test]
fn rejects_invalid_zip_code_format() {
    for zip_code in [
        "",
        "14098374",
        "1409-374",
        "14098-37",
        "14098-3740",
        "ABCDE-123",
    ] {
        let mut dto = valid_student_dto();
        dto.zip_code = Some(zip_code.to_string());

        let errors = dto.validate().expect_err("O CEP deveria ser rejeitado");

        assert!(
            errors.field_errors().contains_key("zip_code"),
            "Deveria existir erro para o CEP {zip_code}",
        );
    }
}

#[test]
fn reports_multiple_invalid_fields() {
    let mut dto = valid_student_dto();

    dto.name = "A".to_string();
    dto.cpf = "123".to_string();
    dto.email = "invalido".to_string();
    dto.phone = Some("invalido".to_string());
    dto.zip_code = Some("invalido".to_string());

    let errors = dto
        .validate()
        .expect_err("Os campos inválidos deveriam ser rejeitados");

    let fields = errors.field_errors();

    for field in ["name", "cpf", "email", "phone", "zip_code"] {
        assert!(
            fields.contains_key(field),
            "Deveria existir erro no campo {field}",
        );
    }
}

// Estes dois testes verificam as regras que você pediu,
// mas que ainda não estão completamente implementadas no DTO.

#[test]
fn rejects_blank_name() {
    let mut dto = valid_student_dto();
    dto.name = "   ".to_string();

    let errors = dto
        .validate()
        .expect_err("Nome contendo somente espaços deveria ser rejeitado");

    assert!(errors.field_errors().contains_key("name"));
}

#[test]
fn rejects_cpf_with_letters() {
    let mut dto = valid_student_dto();
    dto.cpf = "1234567890A".to_string();

    let errors = dto
        .validate()
        .expect_err("CPF contendo letras deveria ser rejeitado");

    assert!(errors.field_errors().contains_key("cpf"));
}
