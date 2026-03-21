-- Workouts (entrenamientos): each program has one or more workouts; each workout has days_count and rest_days_after.
-- Exercises belong to a workout instead of directly to a program.

-- 1) Create workouts table
CREATE TABLE workouts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    program_id UUID NOT NULL REFERENCES programs(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    order_index INT NOT NULL DEFAULT 0,
    days_count INT NOT NULL DEFAULT 1 CHECK (days_count > 0),
    rest_days_after INT NOT NULL DEFAULT 0 CHECK (rest_days_after >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_workouts_program ON workouts(program_id);
CREATE INDEX idx_workouts_program_order ON workouts(program_id, order_index);

COMMENT ON TABLE workouts IS 'Daily workouts (entrenamientos) within a program; each has days_count and rest_days_after for the agenda.';
COMMENT ON COLUMN workouts.days_count IS 'Number of consecutive days this workout is done.';
COMMENT ON COLUMN workouts.rest_days_after IS 'Rest days after this workout block.';

-- 2) One workout per existing program (so we can migrate exercises)
INSERT INTO workouts (id, program_id, name, order_index, days_count, rest_days_after, created_at, updated_at)
SELECT gen_random_uuid(), id, 'Entrenamiento 1', 0, duration_days, 0, now(), now() FROM programs;

-- 3) Add workout_id to exercises (nullable first)
ALTER TABLE exercises ADD COLUMN IF NOT EXISTS workout_id UUID;

-- 4) Assign each exercise to its program's single workout
UPDATE exercises e
SET workout_id = (SELECT w.id FROM workouts w WHERE w.program_id = e.program_id ORDER BY w.order_index LIMIT 1);

-- 5) Drop exercises RLS policies that depend on program_id (must happen before dropping the column)
DROP POLICY IF EXISTS "exercises_select_for_specialist_program" ON exercises;
DROP POLICY IF EXISTS "exercises_select_for_patient_program" ON exercises;
DROP POLICY IF EXISTS "exercises_insert_for_specialist" ON exercises;
DROP POLICY IF EXISTS "exercises_update_for_specialist" ON exercises;
DROP POLICY IF EXISTS "exercises_delete_for_specialist" ON exercises;

-- 6) Make workout_id NOT NULL and drop program_id
ALTER TABLE exercises ALTER COLUMN workout_id SET NOT NULL;
ALTER TABLE exercises DROP CONSTRAINT IF EXISTS exercises_program_id_fkey;
ALTER TABLE exercises DROP COLUMN IF EXISTS program_id;
ALTER TABLE exercises ADD CONSTRAINT exercises_workout_id_fkey
    FOREIGN KEY (workout_id) REFERENCES workouts(id) ON DELETE CASCADE;

-- 7) Indexes for exercises by workout
DROP INDEX IF EXISTS idx_exercises_program;
DROP INDEX IF EXISTS idx_exercises_program_order;
CREATE INDEX idx_exercises_workout ON exercises(workout_id);
CREATE INDEX idx_exercises_workout_order ON exercises(workout_id, order_index);

-- 8) RLS for workouts (same pattern as programs: specialist CRUD, patient read if assigned)
ALTER TABLE workouts ENABLE ROW LEVEL SECURITY;

CREATE POLICY "workouts_select_for_specialist"
    ON workouts FOR SELECT
    USING (program_id IN (SELECT id FROM programs WHERE specialist_id = auth.uid()));

CREATE POLICY "workouts_select_for_patient"
    ON workouts FOR SELECT
    USING (program_id IN (SELECT patient_assigned_program_ids()));

CREATE POLICY "workouts_insert_for_specialist"
    ON workouts FOR INSERT
    WITH CHECK (program_id IN (SELECT id FROM programs WHERE specialist_id = auth.uid()));

CREATE POLICY "workouts_update_for_specialist"
    ON workouts FOR UPDATE
    USING (program_id IN (SELECT id FROM programs WHERE specialist_id = auth.uid()))
    WITH CHECK (program_id IN (SELECT id FROM programs WHERE specialist_id = auth.uid()));

CREATE POLICY "workouts_delete_for_specialist"
    ON workouts FOR DELETE
    USING (program_id IN (SELECT id FROM programs WHERE specialist_id = auth.uid()));

-- 9) Exercises RLS: new policies using workout_id (via workout.program_id)

CREATE POLICY "exercises_select_for_specialist"
    ON exercises FOR SELECT
    USING (
        workout_id IN (SELECT w.id FROM workouts w JOIN programs p ON p.id = w.program_id WHERE p.specialist_id = auth.uid())
    );

CREATE POLICY "exercises_select_for_patient"
    ON exercises FOR SELECT
    USING (
        workout_id IN (
            SELECT w.id FROM workouts w
            WHERE w.program_id IN (SELECT patient_assigned_program_ids())
        )
    );

CREATE POLICY "exercises_insert_for_specialist"
    ON exercises FOR INSERT
    WITH CHECK (
        workout_id IN (SELECT w.id FROM workouts w JOIN programs p ON p.id = w.program_id WHERE p.specialist_id = auth.uid())
    );

CREATE POLICY "exercises_update_for_specialist"
    ON exercises FOR UPDATE
    USING (
        workout_id IN (SELECT w.id FROM workouts w JOIN programs p ON p.id = w.program_id WHERE p.specialist_id = auth.uid())
    )
    WITH CHECK (
        workout_id IN (SELECT w.id FROM workouts w JOIN programs p ON p.id = w.program_id WHERE p.specialist_id = auth.uid())
    );

CREATE POLICY "exercises_delete_for_specialist"
    ON exercises FOR DELETE
    USING (
        workout_id IN (SELECT w.id FROM workouts w JOIN programs p ON p.id = w.program_id WHERE p.specialist_id = auth.uid())
    );

-- 10) updated_at trigger for workouts
CREATE TRIGGER tr_workouts_updated_at
    BEFORE UPDATE ON workouts
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();
