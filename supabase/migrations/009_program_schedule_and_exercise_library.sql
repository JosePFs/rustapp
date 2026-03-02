-- Program defines schedule (workout blocks + rest blocks). Workouts are one-day templates (no days_count/rest_days).
-- Exercises are reusable: specialist's library, linked to workouts via workout_exercises.

-- 1) program_schedule: program defines which days are which workout or rest
CREATE TABLE program_schedule (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    program_id UUID NOT NULL REFERENCES programs(id) ON DELETE CASCADE,
    order_index INT NOT NULL DEFAULT 0,
    workout_id UUID REFERENCES workouts(id) ON DELETE CASCADE,
    days_count INT NOT NULL DEFAULT 1 CHECK (days_count > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT chk_schedule_days_positive CHECK (days_count > 0)
);

CREATE INDEX idx_program_schedule_program ON program_schedule(program_id);
COMMENT ON TABLE program_schedule IS 'Program defines the agenda: blocks of workout days or rest days. workout_id NULL = rest.';

-- Populate from current workouts (each workout: one block workout, one block rest)
INSERT INTO program_schedule (program_id, order_index, workout_id, days_count)
SELECT program_id, (row_number() OVER (PARTITION BY program_id ORDER BY order_index) - 1) * 2, id, days_count
FROM workouts;

INSERT INTO program_schedule (program_id, order_index, workout_id, days_count)
SELECT program_id, (row_number() OVER (PARTITION BY program_id ORDER BY order_index) - 1) * 2 + 1, NULL, rest_days_after
FROM workouts
WHERE rest_days_after > 0;

-- 2) Workouts: remove days_count and rest_days_after (each workout = one day)
ALTER TABLE workouts DROP COLUMN IF EXISTS days_count;
ALTER TABLE workouts DROP COLUMN IF EXISTS rest_days_after;

-- 3) Exercises: add specialist_id (exercise library per specialist)
ALTER TABLE exercises ADD COLUMN IF NOT EXISTS specialist_id UUID REFERENCES profiles(id) ON DELETE CASCADE;

UPDATE exercises e
SET specialist_id = (SELECT p.specialist_id FROM workouts w JOIN programs p ON p.id = w.program_id WHERE w.id = e.workout_id LIMIT 1);

ALTER TABLE exercises ALTER COLUMN specialist_id SET NOT NULL;
CREATE INDEX idx_exercises_specialist ON exercises(specialist_id);

-- 4) workout_exercises: many-to-many (reusable exercises in workouts)
CREATE TABLE workout_exercises (
    workout_id UUID NOT NULL REFERENCES workouts(id) ON DELETE CASCADE,
    exercise_id UUID NOT NULL REFERENCES exercises(id) ON DELETE CASCADE,
    order_index INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (workout_id, exercise_id)
);

CREATE INDEX idx_workout_exercises_workout ON workout_exercises(workout_id);
CREATE INDEX idx_workout_exercises_exercise ON workout_exercises(exercise_id);

-- Copy current assignment (each exercise is in one workout)
INSERT INTO workout_exercises (workout_id, exercise_id, order_index)
SELECT workout_id, id, order_index FROM exercises;

-- 5) Drop exercises policies that depend on workout_id, then drop column
DROP POLICY IF EXISTS "exercises_select_for_specialist" ON exercises;
DROP POLICY IF EXISTS "exercises_select_for_patient" ON exercises;
DROP POLICY IF EXISTS "exercises_insert_for_specialist" ON exercises;
DROP POLICY IF EXISTS "exercises_update_for_specialist" ON exercises;
DROP POLICY IF EXISTS "exercises_delete_for_specialist" ON exercises;

ALTER TABLE exercises DROP CONSTRAINT IF EXISTS exercises_workout_id_fkey;
ALTER TABLE exercises DROP COLUMN IF EXISTS workout_id;

-- 6) RLS exercises: by specialist (library); patient reads via assigned programs
ALTER TABLE exercises ENABLE ROW LEVEL SECURITY;

CREATE POLICY "exercises_all_for_specialist"
    ON exercises FOR ALL
    USING (specialist_id = auth.uid())
    WITH CHECK (specialist_id = auth.uid());

CREATE POLICY "exercises_select_for_patient"
    ON exercises FOR SELECT
    USING (
        id IN (
            SELECT we.exercise_id FROM workout_exercises we
            JOIN workouts w ON w.id = we.workout_id
            WHERE w.program_id IN (SELECT patient_assigned_program_ids())
        )
    );

-- 7) RLS workout_exercises
ALTER TABLE workout_exercises ENABLE ROW LEVEL SECURITY;

CREATE POLICY "workout_exercises_for_specialist"
    ON workout_exercises FOR ALL
    USING (
        workout_id IN (SELECT w.id FROM workouts w JOIN programs p ON p.id = w.program_id WHERE p.specialist_id = auth.uid())
    )
    WITH CHECK (
        workout_id IN (SELECT w.id FROM workouts w JOIN programs p ON p.id = w.program_id WHERE p.specialist_id = auth.uid())
    );

CREATE POLICY "workout_exercises_select_for_patient"
    ON workout_exercises FOR SELECT
    USING (
        workout_id IN (
            SELECT w.id FROM workouts w WHERE w.program_id IN (SELECT patient_assigned_program_ids())
        )
    );

-- 8) RLS program_schedule
ALTER TABLE program_schedule ENABLE ROW LEVEL SECURITY;

CREATE POLICY "program_schedule_for_specialist"
    ON program_schedule FOR ALL
    USING (program_id IN (SELECT id FROM programs WHERE specialist_id = auth.uid()))
    WITH CHECK (program_id IN (SELECT id FROM programs WHERE specialist_id = auth.uid()));

CREATE POLICY "program_schedule_select_for_patient"
    ON program_schedule FOR SELECT
    USING (program_id IN (SELECT patient_assigned_program_ids()));
