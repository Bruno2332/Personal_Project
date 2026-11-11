ALTER TABLE physical_assessments
    ADD COLUMN right_extensor_strength_new REAL;

ALTER TABLE physical_assessments
    ADD COLUMN left_extensor_strength_new REAL;

ALTER TABLE physical_assessments
    ADD COLUMN right_flexor_strength_new REAL;

ALTER TABLE physical_assessments
    ADD COLUMN left_flexor_strength_new REAL;

ALTER TABLE physical_assessments
    ADD COLUMN right_biceps_strength_new REAL;

ALTER TABLE physical_assessments
    ADD COLUMN left_biceps_strength_new REAL;

ALTER TABLE physical_assessments
    ADD COLUMN right_triceps_strength_new REAL;

ALTER TABLE physical_assessments
    ADD COLUMN left_triceps_strength_new REAL;


UPDATE physical_assessments
SET
    right_extensor_strength_new = CAST(right_extensor_strength AS REAL),
    left_extensor_strength_new = CAST(left_extensor_strength AS REAL),
    right_flexor_strength_new = CAST(right_flexor_strength AS REAL),
    left_flexor_strength_new = CAST(left_flexor_strength AS REAL),
    right_biceps_strength_new = CAST(right_biceps_strength AS REAL),
    left_biceps_strength_new = CAST(left_biceps_strength AS REAL),
    right_triceps_strength_new = CAST(right_triceps_strength AS REAL),
    left_triceps_strength_new = CAST(left_triceps_strength AS REAL);


ALTER TABLE physical_assessments
DROP COLUMN right_extensor_strength;

ALTER TABLE physical_assessments
DROP COLUMN left_extensor_strength;

ALTER TABLE physical_assessments
DROP COLUMN right_flexor_strength;

ALTER TABLE physical_assessments
DROP COLUMN left_flexor_strength;

ALTER TABLE physical_assessments
DROP COLUMN right_biceps_strength;

ALTER TABLE physical_assessments
DROP COLUMN left_biceps_strength;

ALTER TABLE physical_assessments
DROP COLUMN right_triceps_strength;

ALTER TABLE physical_assessments
DROP COLUMN left_triceps_strength;


ALTER TABLE physical_assessments
    RENAME COLUMN right_extensor_strength_new TO right_extensor_strength;

ALTER TABLE physical_assessments
    RENAME COLUMN left_extensor_strength_new TO left_extensor_strength;

ALTER TABLE physical_assessments
    RENAME COLUMN right_flexor_strength_new TO right_flexor_strength;

ALTER TABLE physical_assessments
    RENAME COLUMN left_flexor_strength_new TO left_flexor_strength;

ALTER TABLE physical_assessments
    RENAME COLUMN right_biceps_strength_new TO right_biceps_strength;

ALTER TABLE physical_assessments
    RENAME COLUMN left_biceps_strength_new TO left_biceps_strength;

ALTER TABLE physical_assessments
    RENAME COLUMN right_triceps_strength_new TO right_triceps_strength;

ALTER TABLE physical_assessments
    RENAME COLUMN left_triceps_strength_new TO left_triceps_strength;