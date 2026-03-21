-- Workouts become specialist library (like exercises). Programs reference library workouts via program_schedule.

-- 1) Add specialist_id to workouts, migrate from program
ALTER TABLE workouts ADD COLUMN IF NOT EXISTS specialist_id UUID REFERENCES profiles(id) ON DELETE CASCADE;

UPDATE workouts w
SET specialist_id = (SELECT p.specialist_id FROM programs p WHERE p.id = w.program_id LIMIT 1)
WHERE specialist_id IS NULL;

ALTER TABLE workouts ALTER COLUMN specialist_id SET NOT NULL;
CREATE INDEX IF NOT EXISTS idx_workouts_specialist ON workouts(specialist_id);

-- 2) Drop all policies that depend on workouts.program_id (must do before dropping column)
DROP POLICY IF EXISTS "workouts_select_for_specialist" ON workouts;
DROP POLICY IF EXISTS "workouts_select_for_patient" ON workouts;
DROP POLICY IF EXISTS "workouts_insert_for_specialist" ON workouts;
DROP POLICY IF EXISTS "workouts_update_for_specialist" ON workouts;
DROP POLICY IF EXISTS "workouts_delete_for_specialist" ON workouts;
DROP POLICY IF EXISTS "exercises_select_for_patient" ON exercises;
DROP POLICY IF EXISTS "workout_exercises_for_specialist" ON workout_exercises;
DROP POLICY IF EXISTS "workout_exercises_select_for_patient" ON workout_exercises;

-- 3) Drop program_id (workouts are no longer per-program)
ALTER TABLE workouts DROP CONSTRAINT IF EXISTS workouts_program_id_fkey;
DROP INDEX IF EXISTS idx_workouts_program;
DROP INDEX IF EXISTS idx_workouts_program_order;
ALTER TABLE workouts DROP COLUMN IF EXISTS program_id;

-- 4) RLS: create new policies by specialist_id
ALTER TABLE workouts ENABLE ROW LEVEL SECURITY;

CREATE POLICY "workouts_select_for_specialist"
    ON workouts FOR SELECT
    USING (specialist_id = auth.uid());

CREATE POLICY "workouts_insert_for_specialist"
    ON workouts FOR INSERT
    WITH CHECK (specialist_id = auth.uid());

CREATE POLICY "workouts_update_for_specialist"
    ON workouts FOR UPDATE
    USING (specialist_id = auth.uid())
    WITH CHECK (specialist_id = auth.uid());

CREATE POLICY "workouts_delete_for_specialist"
    ON workouts FOR DELETE
    USING (specialist_id = auth.uid());

-- Patients can see workouts that appear in their assigned programs' schedule
CREATE POLICY "workouts_select_for_patient"
    ON workouts FOR SELECT
    USING (
        id IN (
            SELECT workout_id FROM program_schedule
            WHERE program_id IN (SELECT patient_assigned_program_ids())
            AND workout_id IS NOT NULL
        )
    );

COMMENT ON TABLE workouts IS 'Specialist workout library (entrenamientos). Reusable across programs via program_schedule.';

-- 5) Recreate RLS on exercises and workout_exercises (no longer use workouts.program_id)
CREATE POLICY "exercises_select_for_patient"
    ON exercises FOR SELECT
    USING (
        id IN (
            SELECT we.exercise_id FROM workout_exercises we
            WHERE we.workout_id IN (
                SELECT workout_id FROM program_schedule
                WHERE program_id IN (SELECT patient_assigned_program_ids())
                AND workout_id IS NOT NULL
            )
        )
    );

DROP POLICY IF EXISTS "workout_exercises_for_specialist" ON workout_exercises;
CREATE POLICY "workout_exercises_for_specialist"
    ON workout_exercises FOR ALL
    USING (workout_id IN (SELECT id FROM workouts WHERE specialist_id = auth.uid()))
    WITH CHECK (workout_id IN (SELECT id FROM workouts WHERE specialist_id = auth.uid()));

DROP POLICY IF EXISTS "workout_exercises_select_for_patient" ON workout_exercises;
CREATE POLICY "workout_exercises_select_for_patient"
    ON workout_exercises FOR SELECT
    USING (
        workout_id IN (
            SELECT workout_id FROM program_schedule
            WHERE program_id IN (SELECT patient_assigned_program_ids())
            AND workout_id IS NOT NULL
        )
    );
