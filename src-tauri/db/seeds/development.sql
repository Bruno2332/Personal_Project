-- ContractStatus: 0 = Active, 1 = Completed, 2 = Cancelled.
PRAGMA foreign_keys = ON;
BEGIN TRANSACTION;

INSERT INTO students (
    name, cpf, gender, email, birth_date, phone,
    zip_code, address, address_number, address_complement,
    city, state, objective, notes, is_active
) VALUES
      ('Rafael Mendes', '12345678901', 0, 'rafael.mendes@example.com', '1990-03-15', '(11) 99812-3456', '01310100', 'Avenida Paulista', '1000', 'Apto 82', 'São Paulo', 24, 0, 'Prefere treinar no período da manhã.', 1),
      ('Camila Ferreira', '23456789012', 1, 'camila.ferreira@example.com', '1995-07-22', '(11) 98765-4321', '05402000', 'Rua Teodoro Sampaio', '620', NULL, 'São Paulo', 24, 1, 'Disponível às segundas e quartas.', 1),
      ('Lucas Oliveira', '34567890123', 0, 'lucas.oliveira@example.com', '1988-11-08', '(21) 97654-3210', '22041080', 'Rua Santa Clara', '215', 'Apto 401', 'Rio de Janeiro', 18, 2, 'Retornando aos treinos após um período parado.', 1),
      ('Isabela Santos', '45678901234', 1, 'isabela.santos@example.com', '2000-04-12', '(31) 98123-7890', '30130110', 'Rua da Bahia', '1420', NULL, 'Belo Horizonte', 12, 3, 'Pratica natação. Cadastro inativo por viagem.', 0),
      ('Thiago Costa', '56789012345', 0, 'thiago.costa@example.com', '1993-09-30', '(85) 99333-8877', '60175055', 'Avenida Beira Mar', '850', 'Bloco B', 'Fortaleza', 5, 4, 'Trabalha em home office e busca uma rotina de exercícios.', 1),
      ('Juliana Almeida', '67890123456', 1, 'juliana.almeida@example.com', '1987-12-03', '(41) 99222-1100', '80010000', 'Rua XV de Novembro', '300', 'Apto 12', 'Curitiba', 15, 5, 'Quer melhorar a flexibilidade e a mobilidade.', 1),
      ('Pedro Ribeiro', '78901234567', 0, 'pedro.ribeiro@example.com', '1998-06-18', '(51) 99111-2233', NULL, NULL, NULL, NULL, 'Porto Alegre', 20, 0, NULL, 1),
      ('Mariana Lima', '89012345678', 1, 'mariana.lima@example.com', '1992-02-25', '(71) 99444-5566', NULL, NULL, NULL, NULL, 'Salvador', 4, 1, 'Prefere treinos no final da tarde.', 1),
      ('Bruno Martins', '90123456789', 0, 'bruno.martins@example.com', '1985-08-10', NULL, NULL, NULL, NULL, NULL, 'Brasília', 6, 4, 'Cadastro inativo. Aguardando retorno.', 0),
      ('Fernanda Rocha', '01234567890', 1, 'fernanda.rocha@example.com', '1996-05-14', NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, 1)
    ON CONFLICT(cpf) DO NOTHING;

-- Tabela TEMP apenas para montar este seed; não integra o modelo do app.
CREATE TEMP TABLE seed_contracts (
    cpf TEXT PRIMARY KEY,
    description TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT,
    billing_type INTEGER NOT NULL,
    amount_cents INTEGER NOT NULL,
    payment_method INTEGER NOT NULL,
    installment_count INTEGER NOT NULL,
    first_due_date TEXT NOT NULL,
    due_day INTEGER NOT NULL,
    sessions_per_week INTEGER NOT NULL,
    status INTEGER NOT NULL

);

INSERT INTO seed_contracts VALUES
    ('12345678901', 'Pacote de hipertrofia — setembro/2026', '2026-09-01', '2026-11-30', 1, 120000, 0, 3, '2026-09-10', 10, 3, 0),
    ('23456789012', 'Pacote à vista de condicionamento — setembro/2026', '2026-09-01', '2026-10-31', 1, 80000, 0, 1, '2026-09-01', 1, 2, 0),
    ('34567890123', 'Pacote de fortalecimento — setembro/2026', '2026-09-01', '2027-02-28', 1, 150000, 0, 6, '2026-09-20', 20, 2, 0),
    ('45678901234', 'Pacote de preparação esportiva — encerrado', '2026-06-01', '2026-08-31', 1, 120000, 0, 3, '2026-06-01', 1, 2, 1),
    ('56789012345', 'Mensalidade de condicionamento — setembro/2026', '2026-09-01', NULL, 0, 60000, 0, 1, '2026-09-05', 5, 2, 0),
    ('67890123456', 'Mensalidade de mobilidade — setembro/2026', '2026-09-01', NULL, 0, 50000, 1, 1, '2026-09-10', 10, 2, 0),
    ('78901234567', 'Pacote de hipertrofia — setembro/2026', '2026-09-01', '2026-12-31', 1, 160000, 1, 4, '2026-09-15', 15, 3, 0),
    ('89012345678', 'Mensalidade de condicionamento — setembro/2026', '2026-09-01', NULL, 0, 65000, 1, 1, '2026-09-15', 15, 2, 0),
    ('90123456789', 'Pacote de saúde — cancelado', '2026-08-01', '2026-08-31', 1, 60000, 1, 2, '2026-08-10', 10, 2, 2),
    ('01234567890', 'Mensalidade inicial — setembro/2026', '2026-09-01', NULL, 0, 45000, 1, 1, '2026-09-25', 25, 1, 0);

INSERT INTO contracts (
    student_id, contract_description, start_date, end_date, service_type, billing_type,
    amount_cents, payment_method, installment_count, first_due_date, due_day,
    sessions_per_week, contract_status
)
SELECT s.id, t.description, t.start_date, t.end_date, 0, t.billing_type,
       t.amount_cents, t.payment_method, t.installment_count, t.first_due_date, t.due_day,
       t.sessions_per_week, t.status
FROM seed_contracts t
         JOIN students s ON s.cpf = t.cpf
WHERE NOT EXISTS (
    SELECT 1 FROM contracts c
    WHERE c.student_id = s.id AND c.contract_description = t.description
);

-- Pacotes: todas as parcelas. Mensal: somente setembro, sem antecipar 12 meses.
WITH RECURSIVE numbers(n) AS (
    SELECT 1
    UNION ALL
    SELECT n + 1 FROM numbers
    WHERE n < (SELECT MAX(installment_count) FROM seed_contracts)
), charges AS (
    SELECT c.id AS contract_id, t.description, t.billing_type,
           t.payment_method, t.status AS contract_status, numbers.n,
           t.amount_cents / t.installment_count
               + CASE WHEN numbers.n = t.installment_count
                          THEN t.amount_cents % t.installment_count ELSE 0 END AS amount,
    date(t.first_due_date, '+' || (numbers.n - 1) || ' months') AS due_date
FROM seed_contracts t
    JOIN students s ON s.cpf = t.cpf
    JOIN contracts c ON c.student_id = s.id AND c.contract_description = t.description
    JOIN numbers ON numbers.n <= t.installment_count
    )
INSERT INTO financial_records (
    contract_id, transaction_type, description, amount_cent, due_date,
    payment_date, payment_method, installment_number, status_transaction
)
SELECT contract_id, 0,
       CASE WHEN billing_type = 0 THEN 'Mensalidade — setembro/2026'
            ELSE description || ' — parcela ' || n END,
       amount, due_date,
       CASE WHEN contract_status <> 2 AND n = 1 THEN due_date
            WHEN contract_status = 1 THEN due_date ELSE NULL END,
       payment_method, n,
       CASE WHEN contract_status = 2 THEN 2
            WHEN contract_status = 1 OR n = 1 THEN 1 ELSE 0 END
FROM charges x
WHERE NOT EXISTS (
    SELECT 1 FROM financial_records f
    WHERE f.contract_id = x.contract_id
      AND f.installment_number = x.n
      AND f.due_date = x.due_date
      AND f.transaction_type = 0
);

-- Despesa manual sem contrato.
INSERT INTO financial_records (
    contract_id, transaction_type, description, amount_cent, due_date,
    payment_date, payment_method, installment_number, status_transaction
)
SELECT NULL, 1, 'Compra de elásticos — teste setembro/2026', 18000,
       '2026-09-15', '2026-09-15', 0, 1, 1
    WHERE NOT EXISTS (
    SELECT 1 FROM financial_records
    WHERE description = 'Compra de elásticos — teste setembro/2026'
      AND transaction_type = 1 AND contract_id IS NULL
);

-- Uma linha por horário semanal, conforme a quantidade de aulas do contrato.
WITH slots(cpf, weekday, start_time, end_time) AS (
    VALUES
        ('12345678901', 1, '08:00:00', '09:00:00'),
        ('12345678901', 3, '08:00:00', '09:00:00'),
        ('12345678901', 5, '08:00:00', '09:00:00'),
        ('23456789012', 1, '09:00:00', '10:00:00'),
        ('23456789012', 3, '09:00:00', '10:00:00'),
        ('34567890123', 2, '08:00:00', '09:00:00'),
        ('34567890123', 4, '08:00:00', '09:00:00'),
        ('56789012345', 2, '09:00:00', '10:00:00'),
        ('56789012345', 4, '09:00:00', '10:00:00'),
        ('67890123456', 2, '10:00:00', '11:00:00'),
        ('67890123456', 4, '10:00:00', '11:00:00'),
        ('78901234567', 1, '10:00:00', '11:00:00'),
        ('78901234567', 3, '10:00:00', '11:00:00'),
        ('78901234567', 5, '10:00:00', '11:00:00'),
        ('89012345678', 1, '17:00:00', '18:00:00'),
        ('89012345678', 3, '17:00:00', '18:00:00'),
        ('01234567890', 5, '09:00:00', '10:00:00')
)
INSERT INTO agenda (
    student_id, appointment_type, is_recurring, scheduled_date, week_day,
    start_time, end_time, recurrence_start_date, recurrence_end_date, notes
)
SELECT s.id, 0, 1, NULL, x.weekday, x.start_time, x.end_time,
       t.start_date, t.end_date, 'Horário semanal de teste'
FROM slots x
         JOIN students s ON s.cpf = x.cpf
         JOIN seed_contracts t ON t.cpf = x.cpf
WHERE NOT EXISTS (
    SELECT 1 FROM agenda a
    WHERE a.student_id = s.id AND a.appointment_type = 0
      AND a.is_recurring = 1 AND a.week_day = x.weekday
      AND a.start_time = x.start_time AND a.recurrence_start_date = t.start_date
);

-- Agendamentos únicos de avaliação.
WITH appointments(cpf, scheduled_date, start_time, end_time, notes) AS (
    VALUES
        ('12345678901', '2026-10-03', '08:00:00', '09:00:00', 'Avaliação inicial de teste'),
        ('23456789012', '2026-10-03', '09:00:00', '10:00:00', 'Avaliação inicial de teste')
)
INSERT INTO agenda (
    student_id, appointment_type, is_recurring, scheduled_date, week_day,
    start_time, end_time, recurrence_start_date, recurrence_end_date, notes
)
SELECT s.id, 1, 0, x.scheduled_date, NULL, x.start_time, x.end_time,
       NULL, NULL, x.notes
FROM appointments x JOIN students s ON s.cpf = x.cpf
WHERE NOT EXISTS (
    SELECT 1 FROM agenda a
    WHERE a.student_id = s.id AND a.appointment_type = 1
      AND a.scheduled_date = x.scheduled_date AND a.start_time = x.start_time
);

DROP TABLE seed_contracts;

-- Avaliações com contrato próprio e cobrança à vista de R$ 150,00.
-- users não é preenchida; cadastro feito pela tela inicial.


-- 12345678901 | 2026-09-01
INSERT INTO contracts (
    student_id, contract_description, start_date, end_date, service_type,
    billing_type, amount_cents, payment_method, installment_count, first_due_date,
    due_day, sessions_per_week, contract_status
)
SELECT s.id, 'Avaliação física — 2026-09-01', '2026-09-01', '2026-09-01', 1,
       1, 15000, 1,1, '2026-09-01', NULL, NULL, 1
FROM students s
WHERE s.cpf = '12345678901'
  AND NOT EXISTS (
    SELECT 1 FROM contracts c
    WHERE c.student_id = s.id AND c.service_type = 1
      AND c.contract_description = 'Avaliação física — 2026-09-01'
);

INSERT INTO financial_records (
    contract_id, transaction_type, description, amount_cent, due_date,
    payment_date, payment_method, installment_number, status_transaction
)
SELECT c.id, 0, 'Avaliação física — 2026-09-01', 15000, '2026-09-01',
       '2026-09-01', 0, 1, 1
FROM contracts c
         JOIN students s ON s.id = c.student_id
WHERE s.cpf = '12345678901' AND c.service_type = 1
  AND c.contract_description = 'Avaliação física — 2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM financial_records f
    WHERE f.contract_id = c.id AND f.installment_number = 1
      AND f.transaction_type = 0
);

INSERT INTO physical_assessments (
    student_id, contract_id, assessment_date, weight_kg, height_m, systolic_pressure, diastolic_pressure, resting_heart_rate_bpm, right_handgrip_strength, left_handgrip_strength, neck_circumference_cm, right_arm_circumference_cm, left_arm_circumference_cm, right_forearm_circumference_cm, left_forearm_circumference_cm, shoulder_circumference_cm, chest_circumference_cm, waist_circumference_cm, abdomen_circumference_cm, hip_circumference_cm, right_thigh_circumference_cm, left_thigh_circumference_cm, right_calf_circumference_cm, left_calf_circumference_cm, triceps_skinfold_mm, chest_skinfold_mm, midaxillary_skinfold_mm, subscapular_skinfold_mm, abdominal_skinfold_mm, suprailiac_skinfold_mm, thigh_skinfold_mm, right_extensor_strength, left_extensor_strength, right_flexor_strength, left_flexor_strength, right_biceps_strength, left_biceps_strength, right_triceps_strength, left_triceps_strength, bioimpedance_fat_percentage, vo2_final_stage_seconds
)
SELECT
    s.id, c.id, '2026-09-01', 82.5, 1.78, 120, 78, 68, 32.0, 30.0, 37.0, 32.0, 31.5, 27.0, 26.5, 108.0, 98.0, 85.0, 90.0, 100.0, 55.0, 54.5, 37.0, 36.5, 16.0, 12.0, 14.0, 18.0, 22.0, 20.0, 24.0, 40, 38, 25, 24, 18, 17, 16, 15, 20.0, 30
FROM students s
         JOIN contracts c ON c.student_id = s.id AND c.service_type = 1
    AND c.contract_description = 'Avaliação física — 2026-09-01'
WHERE s.cpf = '12345678901'
  AND NOT EXISTS (
    SELECT 1 FROM physical_assessments a
    WHERE a.student_id = s.id AND a.assessment_date = '2026-09-01'
);


-- 23456789012 | 2026-09-01
INSERT INTO contracts (
    student_id, contract_description, start_date, end_date, service_type,
    billing_type, amount_cents, payment_method, installment_count, first_due_date,
    due_day, sessions_per_week, contract_status
)
SELECT s.id, 'Avaliação física — 2026-09-01', '2026-09-01', '2026-09-01', 1,
       1, 15000, 1,1, '2026-09-01', NULL, NULL, 1
FROM students s
WHERE s.cpf = '23456789012'
  AND NOT EXISTS (
    SELECT 1 FROM contracts c
    WHERE c.student_id = s.id AND c.service_type = 1
      AND c.contract_description = 'Avaliação física — 2026-09-01'
);

INSERT INTO financial_records (
    contract_id, transaction_type, description, amount_cent, due_date,
    payment_date, payment_method, installment_number, status_transaction
)
SELECT c.id, 0, 'Avaliação física — 2026-09-01', 15000, '2026-09-01',
       '2026-09-01', 0, 1, 1
FROM contracts c
         JOIN students s ON s.id = c.student_id
WHERE s.cpf = '23456789012' AND c.service_type = 1
  AND c.contract_description = 'Avaliação física — 2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM financial_records f
    WHERE f.contract_id = c.id AND f.installment_number = 1
      AND f.transaction_type = 0
);

INSERT INTO physical_assessments (
    student_id, contract_id, assessment_date, weight_kg, height_m, systolic_pressure, diastolic_pressure, resting_heart_rate_bpm, right_handgrip_strength, left_handgrip_strength, neck_circumference_cm, right_arm_circumference_cm, left_arm_circumference_cm, right_forearm_circumference_cm, left_forearm_circumference_cm, shoulder_circumference_cm, chest_circumference_cm, waist_circumference_cm, abdomen_circumference_cm, hip_circumference_cm, right_thigh_circumference_cm, left_thigh_circumference_cm, right_calf_circumference_cm, left_calf_circumference_cm, triceps_skinfold_mm, chest_skinfold_mm, midaxillary_skinfold_mm, subscapular_skinfold_mm, abdominal_skinfold_mm, suprailiac_skinfold_mm, thigh_skinfold_mm, right_extensor_strength, left_extensor_strength, right_flexor_strength, left_flexor_strength, right_biceps_strength, left_biceps_strength, right_triceps_strength, left_triceps_strength, bioimpedance_fat_percentage, vo2_final_stage_seconds
)
SELECT
    s.id, c.id, '2026-09-01', 74.2, 1.65, 122, 80, 69, 33.4, 31.3, 37.4, 32.4, 31.9, 27.4, 26.9, 108.4, 98.4, 85.4, 90.4, 100.4, 55.4, 54.9, 37.4, 36.9, 16.5, 12.5, 14.5, 18.5, 22.5, 20.5, 24.5, 40, 38, 25, 24, 18, 17, 16, 15, 20.7, 30
FROM students s
         JOIN contracts c ON c.student_id = s.id AND c.service_type = 1
    AND c.contract_description = 'Avaliação física — 2026-09-01'
WHERE s.cpf = '23456789012'
  AND NOT EXISTS (
    SELECT 1 FROM physical_assessments a
    WHERE a.student_id = s.id AND a.assessment_date = '2026-09-01'
);


-- 34567890123 | 2026-09-01
INSERT INTO contracts (
    student_id, contract_description, start_date, end_date, service_type,
    billing_type, amount_cents, payment_method, installment_count, first_due_date,
    due_day, sessions_per_week, contract_status
)
SELECT s.id, 'Avaliação física — 2026-09-01', '2026-09-01', '2026-09-01', 1,
       1, 15000, 1,1, '2026-09-01', NULL, NULL, 1
FROM students s
WHERE s.cpf = '34567890123'
  AND NOT EXISTS (
    SELECT 1 FROM contracts c
    WHERE c.student_id = s.id AND c.service_type = 1
      AND c.contract_description = 'Avaliação física — 2026-09-01'
);

INSERT INTO financial_records (
    contract_id, transaction_type, description, amount_cent, due_date,
    payment_date, payment_method, installment_number, status_transaction
)
SELECT c.id, 0, 'Avaliação física — 2026-09-01', 15000, '2026-09-01',
       '2026-09-01', 0, 1, 1
FROM contracts c
         JOIN students s ON s.id = c.student_id
WHERE s.cpf = '34567890123' AND c.service_type = 1
  AND c.contract_description = 'Avaliação física — 2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM financial_records f
    WHERE f.contract_id = c.id AND f.installment_number = 1
      AND f.transaction_type = 0
);

INSERT INTO physical_assessments (
    student_id, contract_id, assessment_date, weight_kg, height_m, systolic_pressure, diastolic_pressure, resting_heart_rate_bpm, right_handgrip_strength, left_handgrip_strength, neck_circumference_cm, right_arm_circumference_cm, left_arm_circumference_cm, right_forearm_circumference_cm, left_forearm_circumference_cm, shoulder_circumference_cm, chest_circumference_cm, waist_circumference_cm, abdomen_circumference_cm, hip_circumference_cm, right_thigh_circumference_cm, left_thigh_circumference_cm, right_calf_circumference_cm, left_calf_circumference_cm, triceps_skinfold_mm, chest_skinfold_mm, midaxillary_skinfold_mm, subscapular_skinfold_mm, abdominal_skinfold_mm, suprailiac_skinfold_mm, thigh_skinfold_mm, right_extensor_strength, left_extensor_strength, right_flexor_strength, left_flexor_strength, right_biceps_strength, left_biceps_strength, right_triceps_strength, left_triceps_strength, bioimpedance_fat_percentage, vo2_final_stage_seconds
)
SELECT
    s.id, c.id, '2026-09-01', 86.0, 1.80, 124, 78, 70, 34.8, 32.6, 37.8, 32.8, 32.3, 27.8, 27.3, 108.8, 98.8, 85.8, 90.8, 100.8, 55.8, 55.3, 37.8, 37.3, 17.0, 13.0, 15.0, 19.0, 23.0, 21.0, 25.0, 41, 39, 26, 25, 19, 18, 17, 16, 21.4, 30
FROM students s
         JOIN contracts c ON c.student_id = s.id AND c.service_type = 1
    AND c.contract_description = 'Avaliação física — 2026-09-01'
WHERE s.cpf = '34567890123'
  AND NOT EXISTS (
    SELECT 1 FROM physical_assessments a
    WHERE a.student_id = s.id AND a.assessment_date = '2026-09-01'
);


-- 45678901234 | 2026-08-01
INSERT INTO contracts (
    student_id, contract_description, start_date, end_date, service_type,
    billing_type, amount_cents, payment_method, installment_count, first_due_date,
    due_day, sessions_per_week, contract_status
)
SELECT s.id, 'Avaliação física — 2026-08-01', '2026-08-01', '2026-08-01', 1,
       1, 15000, 1, 1, '2026-08-01', NULL, NULL, 1
FROM students s
WHERE s.cpf = '45678901234'
  AND NOT EXISTS (
    SELECT 1 FROM contracts c
    WHERE c.student_id = s.id AND c.service_type = 1
      AND c.contract_description = 'Avaliação física — 2026-08-01'
);

INSERT INTO financial_records (
    contract_id, transaction_type, description, amount_cent, due_date,
    payment_date, payment_method, installment_number, status_transaction
)
SELECT c.id, 0, 'Avaliação física — 2026-08-01', 15000, '2026-08-01',
       '2026-08-01', 0, 1, 1
FROM contracts c
         JOIN students s ON s.id = c.student_id
WHERE s.cpf = '45678901234' AND c.service_type = 1
  AND c.contract_description = 'Avaliação física — 2026-08-01'
  AND NOT EXISTS (
    SELECT 1 FROM financial_records f
    WHERE f.contract_id = c.id AND f.installment_number = 1
      AND f.transaction_type = 0
);

INSERT INTO physical_assessments (
    student_id, contract_id, assessment_date, weight_kg, height_m, systolic_pressure, diastolic_pressure, resting_heart_rate_bpm, right_handgrip_strength, left_handgrip_strength, neck_circumference_cm, right_arm_circumference_cm, left_arm_circumference_cm, right_forearm_circumference_cm, left_forearm_circumference_cm, shoulder_circumference_cm, chest_circumference_cm, waist_circumference_cm, abdomen_circumference_cm, hip_circumference_cm, right_thigh_circumference_cm, left_thigh_circumference_cm, right_calf_circumference_cm, left_calf_circumference_cm, triceps_skinfold_mm, chest_skinfold_mm, midaxillary_skinfold_mm, subscapular_skinfold_mm, abdominal_skinfold_mm, suprailiac_skinfold_mm, thigh_skinfold_mm, right_extensor_strength, left_extensor_strength, right_flexor_strength, left_flexor_strength, right_biceps_strength, left_biceps_strength, right_triceps_strength, left_triceps_strength, bioimpedance_fat_percentage, vo2_final_stage_seconds
)
SELECT
    s.id, c.id, '2026-08-01', 62.0, 1.70, 120, 80, 71, 36.2, 33.9, 38.2, 33.2, 32.7, 28.2, 27.7, 109.2, 99.2, 86.2, 91.2, 101.2, 56.2, 55.7, 38.2, 37.7, 17.5, 13.5, 15.5, 19.5, 23.5, 21.5, 25.5, 41, 39, 26, 25, 19, 18, 17, 16, 22.1, 30
FROM students s
         JOIN contracts c ON c.student_id = s.id AND c.service_type = 1
    AND c.contract_description = 'Avaliação física — 2026-08-01'
WHERE s.cpf = '45678901234'
  AND NOT EXISTS (
    SELECT 1 FROM physical_assessments a
    WHERE a.student_id = s.id AND a.assessment_date = '2026-08-01'
);


-- 56789012345 | 2026-09-01
INSERT INTO contracts (
    student_id, contract_description, start_date, end_date, service_type,
    billing_type, amount_cents, payment_method, installment_count, first_due_date,
    due_day, sessions_per_week, contract_status
)
SELECT s.id, 'Avaliação física — 2026-09-01', '2026-09-01', '2026-09-01', 1,
       1, 15000, 1,1, '2026-09-01', NULL, NULL, 1
FROM students s
WHERE s.cpf = '56789012345'
  AND NOT EXISTS (
    SELECT 1 FROM contracts c
    WHERE c.student_id = s.id AND c.service_type = 1
      AND c.contract_description = 'Avaliação física — 2026-09-01'
);

INSERT INTO financial_records (
    contract_id, transaction_type, description, amount_cent, due_date,
    payment_date, payment_method, installment_number, status_transaction
)
SELECT c.id, 0, 'Avaliação física — 2026-09-01', 15000, '2026-09-01',
       '2026-09-01', 0, 1, 1
FROM contracts c
         JOIN students s ON s.id = c.student_id
WHERE s.cpf = '56789012345' AND c.service_type = 1
  AND c.contract_description = 'Avaliação física — 2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM financial_records f
    WHERE f.contract_id = c.id AND f.installment_number = 1
      AND f.transaction_type = 0
);

INSERT INTO physical_assessments (
    student_id, contract_id, assessment_date, weight_kg, height_m, systolic_pressure, diastolic_pressure, resting_heart_rate_bpm, right_handgrip_strength, left_handgrip_strength, neck_circumference_cm, right_arm_circumference_cm, left_arm_circumference_cm, right_forearm_circumference_cm, left_forearm_circumference_cm, shoulder_circumference_cm, chest_circumference_cm, waist_circumference_cm, abdomen_circumference_cm, hip_circumference_cm, right_thigh_circumference_cm, left_thigh_circumference_cm, right_calf_circumference_cm, left_calf_circumference_cm, triceps_skinfold_mm, chest_skinfold_mm, midaxillary_skinfold_mm, subscapular_skinfold_mm, abdominal_skinfold_mm, suprailiac_skinfold_mm, thigh_skinfold_mm, right_extensor_strength, left_extensor_strength, right_flexor_strength, left_flexor_strength, right_biceps_strength, left_biceps_strength, right_triceps_strength, left_triceps_strength, bioimpedance_fat_percentage, vo2_final_stage_seconds
)
SELECT
    s.id, c.id, '2026-09-01', 91.3, 1.75, 122, 78, 72, 37.6, 35.2, 38.6, 33.6, 33.1, 28.6, 28.1, 109.6, 99.6, 86.6, 91.6, 101.6, 56.6, 56.1, 38.6, 38.1, 18.0, 14.0, 16.0, 20.0, 24.0, 22.0, 26.0, 42, 40, 27, 26, 20, 19, 18, 17, 22.8, 30
FROM students s
         JOIN contracts c ON c.student_id = s.id AND c.service_type = 1
    AND c.contract_description = 'Avaliação física — 2026-09-01'
WHERE s.cpf = '56789012345'
  AND NOT EXISTS (
    SELECT 1 FROM physical_assessments a
    WHERE a.student_id = s.id AND a.assessment_date = '2026-09-01'
);


-- 67890123456 | 2026-09-01
INSERT INTO contracts (
    student_id, contract_description, start_date, end_date, service_type,
    billing_type, amount_cents, payment_method, installment_count, first_due_date,
    due_day, sessions_per_week, contract_status
)
SELECT s.id, 'Avaliação física — 2026-09-01', '2026-09-01', '2026-09-01', 1,
       1, 15000, 1,1, '2026-09-01', NULL, NULL, 1
FROM students s
WHERE s.cpf = '67890123456'
  AND NOT EXISTS (
    SELECT 1 FROM contracts c
    WHERE c.student_id = s.id AND c.service_type = 1
      AND c.contract_description = 'Avaliação física — 2026-09-01'
);

INSERT INTO financial_records (
    contract_id, transaction_type, description, amount_cent, due_date,
    payment_date, payment_method, installment_number, status_transaction
)
SELECT c.id, 0, 'Avaliação física — 2026-09-01', 15000, '2026-09-01',
       '2026-09-01', 0, 1, 1
FROM contracts c
         JOIN students s ON s.id = c.student_id
WHERE s.cpf = '67890123456' AND c.service_type = 1
  AND c.contract_description = 'Avaliação física — 2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM financial_records f
    WHERE f.contract_id = c.id AND f.installment_number = 1
      AND f.transaction_type = 0
);

INSERT INTO physical_assessments (
    student_id, contract_id, assessment_date, weight_kg, height_m, systolic_pressure, diastolic_pressure, resting_heart_rate_bpm, right_handgrip_strength, left_handgrip_strength, neck_circumference_cm, right_arm_circumference_cm, left_arm_circumference_cm, right_forearm_circumference_cm, left_forearm_circumference_cm, shoulder_circumference_cm, chest_circumference_cm, waist_circumference_cm, abdomen_circumference_cm, hip_circumference_cm, right_thigh_circumference_cm, left_thigh_circumference_cm, right_calf_circumference_cm, left_calf_circumference_cm, triceps_skinfold_mm, chest_skinfold_mm, midaxillary_skinfold_mm, subscapular_skinfold_mm, abdominal_skinfold_mm, suprailiac_skinfold_mm, thigh_skinfold_mm, right_extensor_strength, left_extensor_strength, right_flexor_strength, left_flexor_strength, right_biceps_strength, left_biceps_strength, right_triceps_strength, left_triceps_strength, bioimpedance_fat_percentage, vo2_final_stage_seconds
)
SELECT
    s.id, c.id, '2026-09-01', 65.8, 1.62, 124, 80, 73, 39.0, 36.5, 39.0, 34.0, 33.5, 29.0, 28.5, 110.0, 100.0, 87.0, 92.0, 102.0, 57.0, 56.5, 39.0, 38.5, 18.5, 14.5, 16.5, 20.5, 24.5, 22.5, 26.5, 43, 41, 28, 27, 21, 20, 19, 18, 23.5, 30
FROM students s
         JOIN contracts c ON c.student_id = s.id AND c.service_type = 1
    AND c.contract_description = 'Avaliação física — 2026-09-01'
WHERE s.cpf = '67890123456'
  AND NOT EXISTS (
    SELECT 1 FROM physical_assessments a
    WHERE a.student_id = s.id AND a.assessment_date = '2026-09-01'
);


-- 78901234567 | 2026-09-01
INSERT INTO contracts (
    student_id, contract_description, start_date, end_date, service_type,
    billing_type, amount_cents, payment_method, installment_count, first_due_date,
    due_day, sessions_per_week, contract_status
)
SELECT s.id, 'Avaliação física — 2026-09-01', '2026-09-01', '2026-09-01', 1,
       1, 15000, 1,1, '2026-09-01', NULL, NULL, 1
FROM students s
WHERE s.cpf = '78901234567'
  AND NOT EXISTS (
    SELECT 1 FROM contracts c
    WHERE c.student_id = s.id AND c.service_type = 1
      AND c.contract_description = 'Avaliação física — 2026-09-01'
);

INSERT INTO financial_records (
    contract_id, transaction_type, description, amount_cent, due_date,
    payment_date, payment_method, installment_number, status_transaction
)
SELECT c.id, 0, 'Avaliação física — 2026-09-01', 15000, '2026-09-01',
       '2026-09-01', 0, 1, 1
FROM contracts c
         JOIN students s ON s.id = c.student_id
WHERE s.cpf = '78901234567' AND c.service_type = 1
  AND c.contract_description = 'Avaliação física — 2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM financial_records f
    WHERE f.contract_id = c.id AND f.installment_number = 1
      AND f.transaction_type = 0
);

INSERT INTO physical_assessments (
    student_id, contract_id, assessment_date, weight_kg, height_m, systolic_pressure, diastolic_pressure, resting_heart_rate_bpm, right_handgrip_strength, left_handgrip_strength, neck_circumference_cm, right_arm_circumference_cm, left_arm_circumference_cm, right_forearm_circumference_cm, left_forearm_circumference_cm, shoulder_circumference_cm, chest_circumference_cm, waist_circumference_cm, abdomen_circumference_cm, hip_circumference_cm, right_thigh_circumference_cm, left_thigh_circumference_cm, right_calf_circumference_cm, left_calf_circumference_cm, triceps_skinfold_mm, chest_skinfold_mm, midaxillary_skinfold_mm, subscapular_skinfold_mm, abdominal_skinfold_mm, suprailiac_skinfold_mm, thigh_skinfold_mm, right_extensor_strength, left_extensor_strength, right_flexor_strength, left_flexor_strength, right_biceps_strength, left_biceps_strength, right_triceps_strength, left_triceps_strength, bioimpedance_fat_percentage, vo2_final_stage_seconds
)
SELECT
    s.id, c.id, '2026-09-01', 78.4, 1.82, 120, 78, 74, 40.4, 37.8, 39.4, 34.4, 33.9, 29.4, 28.9, 110.4, 100.4, 87.4, 92.4, 102.4, 57.4, 56.9, 39.4, 38.9, 19.0, 15.0, 17.0, 21.0, 25.0, 23.0, 27.0, 43, 41, 28, 27, 21, 20, 19, 18, 24.2, 30
FROM students s
         JOIN contracts c ON c.student_id = s.id AND c.service_type = 1
    AND c.contract_description = 'Avaliação física — 2026-09-01'
WHERE s.cpf = '78901234567'
  AND NOT EXISTS (
    SELECT 1 FROM physical_assessments a
    WHERE a.student_id = s.id AND a.assessment_date = '2026-09-01'
);


-- 89012345678 | 2026-09-01
INSERT INTO contracts (
    student_id, contract_description, start_date, end_date, service_type,
    billing_type, amount_cents, payment_method, installment_count, first_due_date,
    due_day, sessions_per_week, contract_status
)
SELECT s.id, 'Avaliação física — 2026-09-01', '2026-09-01', '2026-09-01', 1,
       1, 15000, 1, 1, '2026-09-01', NULL, NULL, 1
FROM students s
WHERE s.cpf = '89012345678'
  AND NOT EXISTS (
    SELECT 1 FROM contracts c
    WHERE c.student_id = s.id AND c.service_type = 1
      AND c.contract_description = 'Avaliação física — 2026-09-01'
);

INSERT INTO financial_records (
    contract_id, transaction_type, description, amount_cent, due_date,
    payment_date, payment_method, installment_number, status_transaction
)
SELECT c.id, 0, 'Avaliação física — 2026-09-01', 15000, '2026-09-01',
       '2026-09-01', 0, 1, 1
FROM contracts c
         JOIN students s ON s.id = c.student_id
WHERE s.cpf = '89012345678' AND c.service_type = 1
  AND c.contract_description = 'Avaliação física — 2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM financial_records f
    WHERE f.contract_id = c.id AND f.installment_number = 1
      AND f.transaction_type = 0
);

INSERT INTO physical_assessments (
    student_id, contract_id, assessment_date, weight_kg, height_m, systolic_pressure, diastolic_pressure, resting_heart_rate_bpm, right_handgrip_strength, left_handgrip_strength, neck_circumference_cm, right_arm_circumference_cm, left_arm_circumference_cm, right_forearm_circumference_cm, left_forearm_circumference_cm, shoulder_circumference_cm, chest_circumference_cm, waist_circumference_cm, abdomen_circumference_cm, hip_circumference_cm, right_thigh_circumference_cm, left_thigh_circumference_cm, right_calf_circumference_cm, left_calf_circumference_cm, triceps_skinfold_mm, chest_skinfold_mm, midaxillary_skinfold_mm, subscapular_skinfold_mm, abdominal_skinfold_mm, suprailiac_skinfold_mm, thigh_skinfold_mm, right_extensor_strength, left_extensor_strength, right_flexor_strength, left_flexor_strength, right_biceps_strength, left_biceps_strength, right_triceps_strength, left_triceps_strength, bioimpedance_fat_percentage, vo2_final_stage_seconds
)
SELECT
    s.id, c.id, '2026-09-01', 80.5, 1.68, 122, 80, 75, 41.8, 39.1, 39.8, 34.8, 34.3, 29.8, 29.3, 110.8, 100.8, 87.8, 92.8, 102.8, 57.8, 57.3, 39.8, 39.3, 19.5, 15.5, 17.5, 21.5, 25.5, 23.5, 27.5, 44, 42, 29, 28, 22, 21, 20, 19, 24.9, 30
FROM students s
         JOIN contracts c ON c.student_id = s.id AND c.service_type = 1
    AND c.contract_description = 'Avaliação física — 2026-09-01'
WHERE s.cpf = '89012345678'
  AND NOT EXISTS (
    SELECT 1 FROM physical_assessments a
    WHERE a.student_id = s.id AND a.assessment_date = '2026-09-01'
);


-- 90123456789 | 2026-08-01
INSERT INTO contracts (
    student_id, contract_description, start_date, end_date, service_type,
    billing_type, amount_cents, payment_method, installment_count, first_due_date,
    due_day, sessions_per_week, contract_status
)
SELECT s.id, 'Avaliação física — 2026-08-01', '2026-08-01', '2026-08-01', 1,
       1, 15000, 1,1, '2026-08-01', NULL, NULL, 1
FROM students s
WHERE s.cpf = '90123456789'
  AND NOT EXISTS (
    SELECT 1 FROM contracts c
    WHERE c.student_id = s.id AND c.service_type = 1
      AND c.contract_description = 'Avaliação física — 2026-08-01'
);

INSERT INTO financial_records (
    contract_id, transaction_type, description, amount_cent, due_date,
    payment_date, payment_method, installment_number, status_transaction
)
SELECT c.id, 0, 'Avaliação física — 2026-08-01', 15000, '2026-08-01',
       '2026-08-01', 0, 1, 1
FROM contracts c
         JOIN students s ON s.id = c.student_id
WHERE s.cpf = '90123456789' AND c.service_type = 1
  AND c.contract_description = 'Avaliação física — 2026-08-01'
  AND NOT EXISTS (
    SELECT 1 FROM financial_records f
    WHERE f.contract_id = c.id AND f.installment_number = 1
      AND f.transaction_type = 0
);

INSERT INTO physical_assessments (
    student_id, contract_id, assessment_date, weight_kg, height_m, systolic_pressure, diastolic_pressure, resting_heart_rate_bpm, right_handgrip_strength, left_handgrip_strength, neck_circumference_cm, right_arm_circumference_cm, left_arm_circumference_cm, right_forearm_circumference_cm, left_forearm_circumference_cm, shoulder_circumference_cm, chest_circumference_cm, waist_circumference_cm, abdomen_circumference_cm, hip_circumference_cm, right_thigh_circumference_cm, left_thigh_circumference_cm, right_calf_circumference_cm, left_calf_circumference_cm, triceps_skinfold_mm, chest_skinfold_mm, midaxillary_skinfold_mm, subscapular_skinfold_mm, abdominal_skinfold_mm, suprailiac_skinfold_mm, thigh_skinfold_mm, right_extensor_strength, left_extensor_strength, right_flexor_strength, left_flexor_strength, right_biceps_strength, left_biceps_strength, right_triceps_strength, left_triceps_strength, bioimpedance_fat_percentage, vo2_final_stage_seconds
)
SELECT
    s.id, c.id, '2026-08-01', 94.2, 1.77, 124, 78, 76, 43.2, 40.4, 40.2, 35.2, 34.7, 30.2, 29.7, 111.2, 101.2, 88.2, 93.2, 103.2, 58.2, 57.7, 40.2, 39.7, 20.0, 16.0, 18.0, 22.0, 26.0, 24.0, 28.0, 44, 42, 29, 28, 22, 21, 20, 19, 25.6, 30
FROM students s
         JOIN contracts c ON c.student_id = s.id AND c.service_type = 1
    AND c.contract_description = 'Avaliação física — 2026-08-01'
WHERE s.cpf = '90123456789'
  AND NOT EXISTS (
    SELECT 1 FROM physical_assessments a
    WHERE a.student_id = s.id AND a.assessment_date = '2026-08-01'
);


-- 01234567890 | 2026-09-01
INSERT INTO contracts (
    student_id, contract_description, start_date, end_date, service_type,
    billing_type, amount_cents, payment_method, installment_count, first_due_date,
    due_day, sessions_per_week, contract_status
)
SELECT s.id, 'Avaliação física — 2026-09-01', '2026-09-01', '2026-09-01', 1,
       1, 15000, 1, 1, '2026-09-01', NULL, NULL, 1
FROM students s
WHERE s.cpf = '01234567890'
  AND NOT EXISTS (
    SELECT 1 FROM contracts c
    WHERE c.student_id = s.id AND c.service_type = 1
      AND c.contract_description = 'Avaliação física — 2026-09-01'
);

INSERT INTO financial_records (
    contract_id, transaction_type, description, amount_cent, due_date,
    payment_date, payment_method, installment_number, status_transaction
)
SELECT c.id, 0, 'Avaliação física — 2026-09-01', 15000, '2026-09-01',
       '2026-09-01', 0, 1, 1
FROM contracts c
         JOIN students s ON s.id = c.student_id
WHERE s.cpf = '01234567890' AND c.service_type = 1
  AND c.contract_description = 'Avaliação física — 2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM financial_records f
    WHERE f.contract_id = c.id AND f.installment_number = 1
      AND f.transaction_type = 0
);

INSERT INTO physical_assessments (
    student_id, contract_id, assessment_date, weight_kg, height_m, systolic_pressure, diastolic_pressure, resting_heart_rate_bpm, right_handgrip_strength, left_handgrip_strength, neck_circumference_cm, right_arm_circumference_cm, left_arm_circumference_cm, right_forearm_circumference_cm, left_forearm_circumference_cm, shoulder_circumference_cm, chest_circumference_cm, waist_circumference_cm, abdomen_circumference_cm, hip_circumference_cm, right_thigh_circumference_cm, left_thigh_circumference_cm, right_calf_circumference_cm, left_calf_circumference_cm, triceps_skinfold_mm, chest_skinfold_mm, midaxillary_skinfold_mm, subscapular_skinfold_mm, abdominal_skinfold_mm, suprailiac_skinfold_mm, thigh_skinfold_mm, right_extensor_strength, left_extensor_strength, right_flexor_strength, left_flexor_strength, right_biceps_strength, left_biceps_strength, right_triceps_strength, left_triceps_strength, bioimpedance_fat_percentage, vo2_final_stage_seconds
)
SELECT
    s.id, c.id, '2026-09-01', 68.0, 1.64, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL
FROM students s
         JOIN contracts c ON c.student_id = s.id AND c.service_type = 1
    AND c.contract_description = 'Avaliação física — 2026-09-01'
WHERE s.cpf = '01234567890'
  AND NOT EXISTS (
    SELECT 1 FROM physical_assessments a
    WHERE a.student_id = s.id AND a.assessment_date = '2026-09-01'
);


-- 12345678901 | 2026-10-01
INSERT INTO contracts (
    student_id, contract_description, start_date, end_date, service_type,
    billing_type, amount_cents, payment_method, installment_count, first_due_date,
    due_day, sessions_per_week, contract_status
)
SELECT s.id, 'Avaliação física — 2026-10-01', '2026-10-01', '2026-10-01', 1,
       1, 15000, 1,1, '2026-10-01', NULL, NULL, 1
FROM students s
WHERE s.cpf = '12345678901'
  AND NOT EXISTS (
    SELECT 1 FROM contracts c
    WHERE c.student_id = s.id AND c.service_type = 1
      AND c.contract_description = 'Avaliação física — 2026-10-01'
);

INSERT INTO financial_records (
    contract_id, transaction_type, description, amount_cent, due_date,
    payment_date, payment_method, installment_number, status_transaction
)
SELECT c.id, 0, 'Avaliação física — 2026-10-01', 15000, '2026-10-01',
       '2026-10-01', 0, 1, 1
FROM contracts c
         JOIN students s ON s.id = c.student_id
WHERE s.cpf = '12345678901' AND c.service_type = 1
  AND c.contract_description = 'Avaliação física — 2026-10-01'
  AND NOT EXISTS (
    SELECT 1 FROM financial_records f
    WHERE f.contract_id = c.id AND f.installment_number = 1
      AND f.transaction_type = 0
);

INSERT INTO physical_assessments (
    student_id, contract_id, assessment_date, weight_kg, height_m, systolic_pressure, diastolic_pressure, resting_heart_rate_bpm, right_handgrip_strength, left_handgrip_strength, neck_circumference_cm, right_arm_circumference_cm, left_arm_circumference_cm, right_forearm_circumference_cm, left_forearm_circumference_cm, shoulder_circumference_cm, chest_circumference_cm, waist_circumference_cm, abdomen_circumference_cm, hip_circumference_cm, right_thigh_circumference_cm, left_thigh_circumference_cm, right_calf_circumference_cm, left_calf_circumference_cm, triceps_skinfold_mm, chest_skinfold_mm, midaxillary_skinfold_mm, subscapular_skinfold_mm, abdominal_skinfold_mm, suprailiac_skinfold_mm, thigh_skinfold_mm, right_extensor_strength, left_extensor_strength, right_flexor_strength, left_flexor_strength, right_biceps_strength, left_biceps_strength, right_triceps_strength, left_triceps_strength, bioimpedance_fat_percentage, vo2_final_stage_seconds
)
SELECT
    s.id, c.id, '2026-10-01', 81.3, 1.78, 120, 78, 66, 32.0, 30.0, 36.5, 31.5, 31.0, 26.5, 26.0, 107.5, 97.5, 84.5, 89.5, 99.5, 54.5, 54.0, 36.5, 36.0, 15.0, 11.0, 13.0, 17.0, 21.0, 19.0, 23.0, 41, 39, 26, 25, 19, 18, 17, 16, 27.0, 30
FROM students s
         JOIN contracts c ON c.student_id = s.id AND c.service_type = 1
    AND c.contract_description = 'Avaliação física — 2026-10-01'
WHERE s.cpf = '12345678901'
  AND NOT EXISTS (
    SELECT 1 FROM physical_assessments a
    WHERE a.student_id = s.id AND a.assessment_date = '2026-10-01'
);


-- 23456789012 | 2026-10-01
INSERT INTO contracts (
    student_id, contract_description, start_date, end_date, service_type,
    billing_type, amount_cents, payment_method, installment_count, first_due_date,
    due_day, sessions_per_week, contract_status
)
SELECT s.id, 'Avaliação física — 2026-10-01', '2026-10-01', '2026-10-01', 1,
       1, 15000, 1, 1, '2026-10-01', NULL, NULL, 1
FROM students s
WHERE s.cpf = '23456789012'
  AND NOT EXISTS (
    SELECT 1 FROM contracts c
    WHERE c.student_id = s.id AND c.service_type = 1
      AND c.contract_description = 'Avaliação física — 2026-10-01'
);

INSERT INTO financial_records (
    contract_id, transaction_type, description, amount_cent, due_date,
    payment_date, payment_method, installment_number, status_transaction
)
SELECT c.id, 0, 'Avaliação física — 2026-10-01', 15000, '2026-10-01',
       '2026-10-01', 0, 1, 1
FROM contracts c
         JOIN students s ON s.id = c.student_id
WHERE s.cpf = '23456789012' AND c.service_type = 1
  AND c.contract_description = 'Avaliação física — 2026-10-01'
  AND NOT EXISTS (
    SELECT 1 FROM financial_records f
    WHERE f.contract_id = c.id AND f.installment_number = 1
      AND f.transaction_type = 0
);

INSERT INTO physical_assessments (
    student_id, contract_id, assessment_date, weight_kg, height_m, systolic_pressure, diastolic_pressure, resting_heart_rate_bpm, right_handgrip_strength, left_handgrip_strength, neck_circumference_cm, right_arm_circumference_cm, left_arm_circumference_cm, right_forearm_circumference_cm, left_forearm_circumference_cm, shoulder_circumference_cm, chest_circumference_cm, waist_circumference_cm, abdomen_circumference_cm, hip_circumference_cm, right_thigh_circumference_cm, left_thigh_circumference_cm, right_calf_circumference_cm, left_calf_circumference_cm, triceps_skinfold_mm, chest_skinfold_mm, midaxillary_skinfold_mm, subscapular_skinfold_mm, abdominal_skinfold_mm, suprailiac_skinfold_mm, thigh_skinfold_mm, right_extensor_strength, left_extensor_strength, right_flexor_strength, left_flexor_strength, right_biceps_strength, left_biceps_strength, right_triceps_strength, left_triceps_strength, bioimpedance_fat_percentage, vo2_final_stage_seconds
)
SELECT
    s.id, c.id, '2026-10-01', 73.0, 1.65, 122, 80, 67, 33.4, 31.3, 36.9, 31.9, 31.4, 26.9, 26.4, 107.9, 97.9, 84.9, 89.9, 99.9, 54.9, 54.4, 36.9, 36.4, 15.5, 11.5, 13.5, 17.5, 21.5, 19.5, 23.5, 41, 39, 26, 25, 19, 18, 17, 16, 27.7, 30
FROM students s
         JOIN contracts c ON c.student_id = s.id AND c.service_type = 1
    AND c.contract_description = 'Avaliação física — 2026-10-01'
WHERE s.cpf = '23456789012'
  AND NOT EXISTS (
    SELECT 1 FROM physical_assessments a
    WHERE a.student_id = s.id AND a.assessment_date = '2026-10-01'
);


INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 3, 100, 0
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '12345678901' AND a.assessment_date = '2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 3
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 4, 110, 0
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '12345678901' AND a.assessment_date = '2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 4
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 5, 120, 2
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '12345678901' AND a.assessment_date = '2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 5
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 3, 102, 1
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '23456789012' AND a.assessment_date = '2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 3
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 4, 112, 1
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '23456789012' AND a.assessment_date = '2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 4
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 5, 122, 1
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '23456789012' AND a.assessment_date = '2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 5
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 3, 104, 0
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '34567890123' AND a.assessment_date = '2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 3
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 4, 114, 4
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '34567890123' AND a.assessment_date = '2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 4
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 5, 124, 0
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '34567890123' AND a.assessment_date = '2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 5
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 4, 106, 4
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '45678901234' AND a.assessment_date = '2026-08-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 4
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 5, 116, 3
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '45678901234' AND a.assessment_date = '2026-08-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 5
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 6, 126, 0
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '45678901234' AND a.assessment_date = '2026-08-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 6
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 4, 108, 0
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '56789012345' AND a.assessment_date = '2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 4
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 5, 118, 0
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '56789012345' AND a.assessment_date = '2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 5
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 6, 128, 1
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '56789012345' AND a.assessment_date = '2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 6
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 4, 110, 1
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '67890123456' AND a.assessment_date = '2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 4
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 5, 120, 4
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '67890123456' AND a.assessment_date = '2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 5
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 6, 130, 4
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '67890123456' AND a.assessment_date = '2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 6
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 4, 112, 0
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '78901234567' AND a.assessment_date = '2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 4
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 5, 122, 4
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '78901234567' AND a.assessment_date = '2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 5
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 6, 132, 1
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '78901234567' AND a.assessment_date = '2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 6
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 4, 114, 4
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '89012345678' AND a.assessment_date = '2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 4
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 5, 124, 3
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '89012345678' AND a.assessment_date = '2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 5
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 6, 134, 1
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '89012345678' AND a.assessment_date = '2026-09-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 6
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 5, 116, 3
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '90123456789' AND a.assessment_date = '2026-08-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 5
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 6, 126, 4
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '90123456789' AND a.assessment_date = '2026-08-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 6
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 7, 136, 2
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '90123456789' AND a.assessment_date = '2026-08-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 7
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 3, 100, 0
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '12345678901' AND a.assessment_date = '2026-10-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 3
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 4, 110, 1
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '12345678901' AND a.assessment_date = '2026-10-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 4
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 5, 120, 3
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '12345678901' AND a.assessment_date = '2026-10-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 5
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 3, 102, 2
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '23456789012' AND a.assessment_date = '2026-10-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 3
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 4, 112, 2
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '23456789012' AND a.assessment_date = '2026-10-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 4
);

INSERT INTO vo2_assessments (
    physical_assessment_id, treadmill_speed_kmh, heart_rate_bpm, perceived_exertion
)
SELECT a.id, 5, 122, 1
FROM physical_assessments a
         JOIN students s ON s.id = a.student_id
WHERE s.cpf = '23456789012' AND a.assessment_date = '2026-10-01'
  AND NOT EXISTS (
    SELECT 1 FROM vo2_assessments v
    WHERE v.physical_assessment_id = a.id
      AND v.treadmill_speed_kmh = 5
);

COMMIT;


