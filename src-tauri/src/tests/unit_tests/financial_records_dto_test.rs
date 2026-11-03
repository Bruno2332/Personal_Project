use crate::{
    dtos::financial_records_dto::FinancialRecordsDto,
    models::financial_records::{
        FinancialRecords, PaymentMethod, TransactionStatus, TransactionType,
    },
    tests::factories::financial_records_factory::{
        financial_record, financial_record_projection, valid_financial_record_dto,
    },
};

#[test]
fn converts_financial_record_model_to_dto_without_losing_fields() {
    let model = financial_record(Some(8), 3, TransactionStatus::Paid);

    let dto = FinancialRecordsDto::from(model);

    assert_eq!(dto.id, Some(8));
    assert_eq!(dto.contract_id, Some(20));
    assert_eq!(dto.transaction_type, TransactionType::Income);
    assert_eq!(dto.description, "Parcela 3");
    assert_eq!(dto.amount_cent, 5_000);
    assert_eq!(dto.payment_method, PaymentMethod::Pix);
    assert_eq!(dto.installment_number, 3);
    assert_eq!(dto.status_transaction, TransactionStatus::Paid);
}

#[test]
fn converts_financial_record_dto_to_model_without_losing_fields() {
    let model = FinancialRecords::from(valid_financial_record_dto());

    assert_eq!(model.id, None);
    assert_eq!(model.contract_id, None);
    assert_eq!(model.transaction_type, TransactionType::Income);
    assert_eq!(model.description, "Mensalidade de teste");
    assert_eq!(model.amount_cent, 12_500);
    assert_eq!(model.payment_method, PaymentMethod::Pix);
    assert_eq!(model.installment_number, 1);
    assert_eq!(model.status_transaction, TransactionStatus::Pending);
}

#[test]
fn converts_financial_projection_to_dto_including_student_fields() {
    let dto = crate::dtos::financial_records_dto::FinancialRecordsProjectionDto::from(
        financial_record_projection(),
    );

    assert_eq!(dto.id, Some(5));
    assert_eq!(dto.student_id, Some(7));
    assert_eq!(dto.student_name.as_deref(), Some("Aluno de teste"));
    assert_eq!(dto.installment_number, 2);
    assert_eq!(dto.status_transaction, TransactionStatus::Paid);
}
