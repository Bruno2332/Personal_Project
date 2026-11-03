ALTER TABLE physical_assessment_photos
    ADD COLUMN position INTEGER
        CHECK (position IN (0, 1, 2));

CREATE UNIQUE INDEX uq_assessment_photo_position
    ON physical_assessment_photos (physical_assessment_id, position);