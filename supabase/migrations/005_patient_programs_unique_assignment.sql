-- Ensure a patient can only be assigned once to the same program.
-- If duplicates exist, keep the row with the earliest assigned_at (first assignment).

-- 1. Remove duplicate assignments, keeping one per (patient_id, program_id)
DELETE FROM patient_programs a
USING patient_programs b
WHERE a.patient_id = b.patient_id
  AND a.program_id = b.program_id
  AND a.id <> b.id
  AND a.assigned_at > b.assigned_at;

-- 2. Add unique constraint
ALTER TABLE patient_programs
ADD CONSTRAINT uq_patient_programs_patient_program UNIQUE (patient_id, program_id);
