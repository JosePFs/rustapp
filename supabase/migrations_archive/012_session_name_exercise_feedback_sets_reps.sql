-- 1) workout_sessions: add editable name; remove effort, pain, comment (moved to per-exercise feedback)
ALTER TABLE workout_sessions ADD COLUMN IF NOT EXISTS name TEXT;
COMMENT ON COLUMN workout_sessions.name IS 'Editable name for this training session (e.g. "Día 1 - Fuerza").';

ALTER TABLE workout_sessions DROP CONSTRAINT IF EXISTS chk_effort_range;
ALTER TABLE workout_sessions DROP CONSTRAINT IF EXISTS chk_pain_range;
ALTER TABLE workout_sessions DROP COLUMN IF EXISTS effort;
ALTER TABLE workout_sessions DROP COLUMN IF EXISTS pain;
ALTER TABLE workout_sessions DROP COLUMN IF EXISTS comment;

-- 2) workout_exercises: add sets and reps
ALTER TABLE workout_exercises ADD COLUMN IF NOT EXISTS sets INT NOT NULL DEFAULT 3;
ALTER TABLE workout_exercises ADD COLUMN IF NOT EXISTS reps INT NOT NULL DEFAULT 10;
COMMENT ON COLUMN workout_exercises.sets IS 'Number of sets for this exercise in the workout.';
COMMENT ON COLUMN workout_exercises.reps IS 'Number of repetitions per set.';

-- 3) session_exercise_feedback: pain, effort, comment per exercise per session
CREATE TABLE IF NOT EXISTS session_exercise_feedback (
    workout_session_id UUID NOT NULL REFERENCES workout_sessions(id) ON DELETE CASCADE,
    exercise_id UUID NOT NULL REFERENCES exercises(id) ON DELETE CASCADE,
    effort INT,
    pain INT,
    comment TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (workout_session_id, exercise_id),
    CONSTRAINT chk_sef_effort CHECK (effort IS NULL OR (effort >= 1 AND effort <= 10)),
    CONSTRAINT chk_sef_pain CHECK (pain IS NULL OR (pain >= 0 AND pain <= 10))
);

CREATE INDEX idx_session_exercise_feedback_session ON session_exercise_feedback(workout_session_id);
CREATE INDEX idx_session_exercise_feedback_exercise ON session_exercise_feedback(exercise_id);

CREATE TRIGGER tr_session_exercise_feedback_updated_at
    BEFORE UPDATE ON session_exercise_feedback
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

ALTER TABLE session_exercise_feedback ENABLE ROW LEVEL SECURITY;

-- Patient can manage feedback for their own sessions (via workout_sessions -> patient_programs)
CREATE POLICY "session_exercise_feedback_select_for_patient"
    ON session_exercise_feedback FOR SELECT
    USING (
        workout_session_id IN (
            SELECT id FROM workout_sessions
            WHERE patient_program_id IN (SELECT id FROM patient_programs WHERE patient_id = auth.uid())
        )
    );

CREATE POLICY "session_exercise_feedback_insert_for_patient"
    ON session_exercise_feedback FOR INSERT
    WITH CHECK (
        workout_session_id IN (
            SELECT id FROM workout_sessions
            WHERE patient_program_id IN (SELECT id FROM patient_programs WHERE patient_id = auth.uid())
        )
    );

CREATE POLICY "session_exercise_feedback_update_for_patient"
    ON session_exercise_feedback FOR UPDATE
    USING (
        workout_session_id IN (
            SELECT id FROM workout_sessions
            WHERE patient_program_id IN (SELECT id FROM patient_programs WHERE patient_id = auth.uid())
        )
    );

CREATE POLICY "session_exercise_feedback_delete_for_patient"
    ON session_exercise_feedback FOR DELETE
    USING (
        workout_session_id IN (
            SELECT id FROM workout_sessions
            WHERE patient_program_id IN (SELECT id FROM patient_programs WHERE patient_id = auth.uid())
        )
    );

-- Specialist can read feedback of their patients' sessions
CREATE POLICY "session_exercise_feedback_select_for_specialist"
    ON session_exercise_feedback FOR SELECT
    USING (
        workout_session_id IN (
            SELECT ws.id FROM workout_sessions ws
            JOIN patient_programs pp ON pp.id = ws.patient_program_id
            JOIN specialist_patients sp ON sp.patient_id = pp.patient_id
            WHERE sp.specialist_id = auth.uid()
        )
    );

-- =============================================================================
-- RPC: feedback por patient_program en una sola llamada (evita 2 round-trips REST)
-- =============================================================================
CREATE OR REPLACE FUNCTION public.list_session_exercise_feedback_for_patient_program(
    p_patient_program_id uuid
)
RETURNS TABLE (
    workout_session_id uuid,
    exercise_id uuid,
    effort integer,
    pain integer,
    comment text
)
LANGUAGE sql
STABLE
SECURITY INVOKER
SET search_path = public
AS $$
    SELECT
        f.workout_session_id,
        f.exercise_id,
        f.effort,
        f.pain,
        f.comment
    FROM session_exercise_feedback f
    INNER JOIN workout_sessions s ON s.id = f.workout_session_id
    WHERE s.patient_program_id = p_patient_program_id;
$$;

COMMENT ON FUNCTION public.list_session_exercise_feedback_for_patient_program(uuid) IS
    'Returns all session exercise feedback rows for sessions under the given patient_program; RLS applies via join to workout_sessions.';

GRANT EXECUTE ON FUNCTION public.list_session_exercise_feedback_for_patient_program(uuid) TO authenticated;
