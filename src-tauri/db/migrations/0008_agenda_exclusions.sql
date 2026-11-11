CREATE TABLE agenda_exclusions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    agenda_id INTEGER NOT NULL,
    occurrence_date TEXT NOT NULL,
    created_at TEXT NOT NULL,

    FOREIGN KEY (agenda_id)
       REFERENCES agenda(id)
       ON DELETE CASCADE,

    UNIQUE (agenda_id, occurrence_date)
    );

CREATE INDEX idx_agenda_exclusions_occurrence_date
    ON agenda_exclusions(occurrence_date);