-- Program: no fixed duration. Agenda = ordered list of workout/rest days.
-- Sessions: one per program day (day_index); session_date = when they completed it (default today, editable).

-- 1) Remove duration_days from programs
ALTER TABLE programs DROP COLUMN IF EXISTS duration_days;

-- 2) workout_sessions: add day_index (position in program, 0-based); one session per (patient_program_id, day_index)
ALTER TABLE workout_sessions ADD COLUMN IF NOT EXISTS day_index INT;

-- Migrate existing: assign distinct day_index per patient_program (order by session_date)
WITH ranked AS (
    SELECT id, (row_number() OVER (PARTITION BY patient_program_id ORDER BY session_date) - 1)::INT AS rn
    FROM workout_sessions
    WHERE day_index IS NULL
)
UPDATE workout_sessions w SET day_index = r.rn FROM ranked r WHERE w.id = r.id;
UPDATE workout_sessions SET day_index = 0 WHERE day_index IS NULL;

ALTER TABLE workout_sessions ALTER COLUMN day_index SET DEFAULT 0;
ALTER TABLE workout_sessions ALTER COLUMN day_index SET NOT NULL;
CREATE INDEX IF NOT EXISTS idx_workout_sessions_day_index ON workout_sessions(patient_program_id, day_index);

-- Drop old unique so we can have unique on (patient_program_id, day_index)
ALTER TABLE workout_sessions DROP CONSTRAINT IF EXISTS uq_patient_program_session_date;
ALTER TABLE workout_sessions ADD CONSTRAINT uq_patient_program_day_index UNIQUE (patient_program_id, day_index);

COMMENT ON COLUMN workout_sessions.day_index IS 'Position in the program (0-based). Session = completion of that day.';
COMMENT ON COLUMN workout_sessions.session_date IS 'Date when the patient completed this day (default today, editable).';
