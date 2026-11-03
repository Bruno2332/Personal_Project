CREATE TABLE students (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    cpf TEXT NOT NULL UNIQUE,
    gender INTEGER NOT NULL CHECK (gender IN (0, 1)),
    email TEXT NOT NULL,
    birth_date TEXT NOT NULL,
    phone TEXT,
    zip_code TEXT,
    address TEXT,
    address_number TEXT,
    address_complement TEXT,
    city TEXT,
    state INTEGER CHECK (state BETWEEN 0 AND 26),
    objective INTEGER CHECK (objective BETWEEN 0 AND 5),
    notes TEXT,
    is_active INTEGER NOT NULL CHECK (is_active IN (0, 1)),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE contracts (
    id INTEGER PRIMARY KEY,
    student_id INTEGER NOT NULL REFERENCES students(id),
    contract_description TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT,
    service_type INTEGER NOT NULL CHECK (service_type IN (0, 1)),
    billing_type INTEGER NOT NULL CHECK (billing_type IN (0, 1)),
    amount_cents INTEGER NOT NULL,
    payment_method INTEGER NOT NULL CHECK (payment_method IN (0, 1)),
    installment_count INTEGER NOT NULL,
    first_due_date TEXT,
    due_day INTEGER CHECK (due_day BETWEEN 1 AND 28),
    sessions_per_week INTEGER,
    contract_status INTEGER NOT NULL CHECK (contract_status IN (0, 1, 2)),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE financial_records (
    id INTEGER PRIMARY KEY,
    contract_id INTEGER REFERENCES contracts(id),
    transaction_type INTEGER NOT NULL CHECK (transaction_type IN (0, 1)),
    description TEXT NOT NULL,
    amount_cent INTEGER NOT NULL,
    due_date TEXT NOT NULL,
    payment_date TEXT,
    payment_method INTEGER NOT NULL CHECK (payment_method IN (0, 1)),
    installment_number INTEGER NOT NULL,
    status_transaction INTEGER NOT NULL CHECK (status_transaction IN (0, 1, 2)),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE agenda (
    id INTEGER PRIMARY KEY,
    student_id INTEGER NOT NULL REFERENCES students(id),
    appointment_type INTEGER NOT NULL CHECK (appointment_type IN (0, 1)),
    is_recurring INTEGER NOT NULL CHECK (is_recurring IN (0, 1)),
    scheduled_date TEXT,
    week_day INTEGER CHECK (week_day BETWEEN 0 AND 6),
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    recurrence_start_date TEXT,
    recurrence_end_date TEXT,
    notes TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE physical_assessments (
    id INTEGER PRIMARY KEY,
    student_id INTEGER NOT NULL REFERENCES students(id),
    contract_id INTEGER NOT NULL REFERENCES contracts(id),
    assessment_date TEXT NOT NULL,
    weight_kg REAL NOT NULL,
    height_m REAL NOT NULL,

    systolic_pressure INTEGER,
    diastolic_pressure INTEGER,
    resting_heart_rate_bpm INTEGER,

    right_handgrip_strength REAL,
    left_handgrip_strength REAL,

    neck_circumference_cm REAL,
    right_arm_circumference_cm REAL,
    left_arm_circumference_cm REAL,
    right_forearm_circumference_cm REAL,
    left_forearm_circumference_cm REAL,
    shoulder_circumference_cm REAL,
    chest_circumference_cm REAL,
    waist_circumference_cm REAL,
    abdomen_circumference_cm REAL,
    hip_circumference_cm REAL,
    right_thigh_circumference_cm REAL,
    left_thigh_circumference_cm REAL,
    right_calf_circumference_cm REAL,
    left_calf_circumference_cm REAL,

    triceps_skinfold_mm REAL,
    chest_skinfold_mm REAL,
    midaxillary_skinfold_mm REAL,
    subscapular_skinfold_mm REAL,
    abdominal_skinfold_mm REAL,
    suprailiac_skinfold_mm REAL,
    thigh_skinfold_mm REAL,

    bioimpedance_fat_percentage REAL,

    right_extensor_strength INTEGER,
    left_extensor_strength INTEGER,
    right_flexor_strength INTEGER,
    left_flexor_strength INTEGER,
    right_biceps_strength INTEGER,
    left_biceps_strength INTEGER,
    right_triceps_strength INTEGER,
    left_triceps_strength INTEGER,

    vo2_final_stage_seconds INTEGER,

    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE vo2_assessments (
    id INTEGER PRIMARY KEY,
    physical_assessment_id INTEGER NOT NULL
     REFERENCES physical_assessments(id) ON DELETE CASCADE,
    treadmill_speed_kmh INTEGER,
    heart_rate_bpm INTEGER,
    perceived_exertion INTEGER CHECK (perceived_exertion IN (0, 1, 2, 3, 4)),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    cref TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX idx_contracts_student
    ON contracts(student_id);

CREATE INDEX idx_financial_records_contract
    ON financial_records(contract_id);

CREATE INDEX idx_agenda_student
    ON agenda(student_id);

CREATE INDEX idx_physical_assessments_student_date
    ON physical_assessments(student_id, assessment_date);

CREATE INDEX idx_physical_assessments_contract
    ON physical_assessments(contract_id);

CREATE INDEX idx_vo2_assessments_physical_assessment
    ON vo2_assessments(physical_assessment_id);