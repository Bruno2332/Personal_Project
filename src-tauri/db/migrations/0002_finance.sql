ALTER TABLE financial_records
    ADD COLUMN competence_month TEXT;

CREATE UNIQUE INDEX idx_financial_contract_competence
    ON financial_records (contract_id, competence_month);