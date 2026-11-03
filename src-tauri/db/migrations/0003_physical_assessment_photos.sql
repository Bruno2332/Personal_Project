CREATE TABLE physical_assessment_photos (
    id INTEGER PRIMARY KEY,
    physical_assessment_id INTEGER NOT NULL,
    file_path TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL DEFAULT (
        strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
        ),
    updated_at TEXT NOT NULL DEFAULT (
        strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
        ),

    FOREIGN KEY (physical_assessment_id)
        REFERENCES physical_assessments(id)
        ON DELETE CASCADE
    );

CREATE INDEX idx_physical_assessment_photos_assessment
    ON physical_assessment_photos(physical_assessment_id);