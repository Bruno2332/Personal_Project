use crate::{
    dtos::contract_dto::ContractDto,
    models::{
        contract::{BillingType, Contract, ContractStatus, ServiceType},
        financial_records::PaymentMethod,
    },
    tests::factories::contract_factory::{valid_contract, valid_contract_dto},
};

#[test]
fn converts_contract_model_to_dto_without_losing_fields() {
    let dto = ContractDto::from(valid_contract());

    assert_eq!(dto.id, None);
    assert_eq!(dto.student_id, 7);
    assert_eq!(dto.contract_description, "Pacote de treinamento");
    assert_eq!(dto.service_type, ServiceType::Training);
    assert_eq!(dto.billing_type, BillingType::Package);
    assert_eq!(dto.amount_cents, 10_000);
    assert_eq!(dto.payment_method, PaymentMethod::Pix);
    assert_eq!(dto.installment_count, 3);
    assert_eq!(dto.due_day, Some(10));
    assert_eq!(dto.sessions_per_week, Some(3));
    assert_eq!(dto.contract_status, ContractStatus::Active);
}

#[test]
fn converts_contract_dto_to_model_without_losing_fields() {
    let model = Contract::from(valid_contract_dto());

    assert_eq!(model.id, None);
    assert_eq!(model.student_id, 7);
    assert_eq!(model.contract_description, "Pacote de treinamento");
    assert_eq!(model.service_type, ServiceType::Training);
    assert_eq!(model.billing_type, BillingType::Package);
    assert_eq!(model.amount_cents, 10_000);
    assert_eq!(model.payment_method, PaymentMethod::Pix);
    assert_eq!(model.installment_count, 3);
    assert_eq!(model.due_day, Some(10));
    assert_eq!(model.sessions_per_week, Some(3));
    assert_eq!(model.contract_status, ContractStatus::Active);
}
