-- Fix infinite RLS recursion on patient_programs.
--
-- Root cause:
--   INSERT into patient_programs
--     → evaluates INSERT policy → subquery on `programs`
--     → evaluates programs SELECT policy for patient
--       → subquery on `patient_programs`
--         → evaluates patient_programs SELECT policies  ← cycle
--
-- Fix: wrap the patient_programs subqueries that are used inside
-- policies of OTHER tables inside SECURITY DEFINER functions.
-- Those functions run as the DB owner and bypass RLS, breaking the cycle.

-- 1. Helper: program_ids the current patient has an assignment for
CREATE OR REPLACE FUNCTION public.patient_assigned_program_ids()
RETURNS SETOF UUID
LANGUAGE sql
STABLE
SECURITY DEFINER
SET search_path = public
AS $$
    SELECT program_id FROM patient_programs WHERE patient_id = auth.uid();
$$;

-- 2. Helper: patient_program ids the current patient owns
CREATE OR REPLACE FUNCTION public.patient_own_patient_program_ids()
RETURNS SETOF UUID
LANGUAGE sql
STABLE
SECURITY DEFINER
SET search_path = public
AS $$
    SELECT id FROM patient_programs WHERE patient_id = auth.uid();
$$;

-- 3. Rebuild programs_select_for_patient using the helper
DROP POLICY IF EXISTS "programs_select_for_patient" ON programs;
CREATE POLICY "programs_select_for_patient"
    ON programs FOR SELECT
    USING (id IN (SELECT patient_assigned_program_ids()));

-- 4. Rebuild exercises_select_for_patient_program using the helper
DROP POLICY IF EXISTS "exercises_select_for_patient_program" ON exercises;
CREATE POLICY "exercises_select_for_patient_program"
    ON exercises FOR SELECT
    USING (program_id IN (SELECT patient_assigned_program_ids()));

-- 5. Rebuild workout_sessions policies using the helper
--    (these also subquery patient_programs and can cause recursion during session ops)
DROP POLICY IF EXISTS "workout_sessions_select_for_patient" ON workout_sessions;
CREATE POLICY "workout_sessions_select_for_patient"
    ON workout_sessions FOR SELECT
    USING (patient_program_id IN (SELECT patient_own_patient_program_ids()));

DROP POLICY IF EXISTS "workout_sessions_insert_for_patient" ON workout_sessions;
CREATE POLICY "workout_sessions_insert_for_patient"
    ON workout_sessions FOR INSERT
    WITH CHECK (patient_program_id IN (SELECT patient_own_patient_program_ids()));

DROP POLICY IF EXISTS "workout_sessions_update_for_patient" ON workout_sessions;
CREATE POLICY "workout_sessions_update_for_patient"
    ON workout_sessions FOR UPDATE
    USING (patient_program_id IN (SELECT patient_own_patient_program_ids()))
    WITH CHECK (patient_program_id IN (SELECT patient_own_patient_program_ids()));
