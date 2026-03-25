-- Eixe full schema. Apply once to an empty DB (requires auth.users). History: ../migrations_archive/

-- =============================================================================
-- ENUMS
-- =============================================================================

CREATE TYPE app_role AS ENUM ('specialist', 'patient');

CREATE TYPE program_status AS ENUM ('active', 'completed', 'cancelled');

-- =============================================================================
-- TABLES
-- =============================================================================

-- Extends Supabase auth.users; id matches auth.uid()
CREATE TABLE profiles (
    id UUID PRIMARY KEY REFERENCES auth.users(id) ON DELETE CASCADE,
    email TEXT NOT NULL,
    full_name TEXT NOT NULL,
    role app_role NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX idx_profiles_email ON profiles(email);

COMMENT ON TABLE profiles IS 'User profiles; id is auth.uid()';

-- Specialist–patient relationship (a specialist has many patients)
CREATE TABLE specialist_patients (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    specialist_id UUID NOT NULL DEFAULT auth.uid() REFERENCES profiles(id) ON DELETE CASCADE,
    patient_id UUID NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_specialist_patient UNIQUE (specialist_id, patient_id),
    CONSTRAINT chk_no_self_assign CHECK (specialist_id != patient_id)
);

CREATE INDEX idx_specialist_patients_specialist ON specialist_patients(specialist_id);
CREATE INDEX idx_specialist_patients_patient ON specialist_patients(patient_id);

-- Programs (templates) created by specialists. duration_days defines the agenda length for the patient.
CREATE TABLE programs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    specialist_id UUID NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    duration_days INT NOT NULL DEFAULT 30 CHECK (duration_days > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_programs_specialist ON programs(specialist_id);

-- Exercises belong to a program (ordered). Soft delete via deleted_at so patients keep access.
CREATE TABLE exercises (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    program_id UUID NOT NULL REFERENCES programs(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    order_index INT NOT NULL DEFAULT 0,
    video_url TEXT,
    deleted_at TIMESTAMPTZ DEFAULT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT chk_order_non_negative CHECK (order_index >= 0)
);

CREATE INDEX idx_exercises_program ON exercises(program_id);
CREATE INDEX idx_exercises_program_order ON exercises(program_id, order_index);

-- Assignment of a program to a patient
CREATE TABLE patient_programs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_id UUID NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
    program_id UUID NOT NULL REFERENCES programs(id) ON DELETE CASCADE,
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    status program_status NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_patient_programs_patient_program UNIQUE (patient_id, program_id)
);

CREATE INDEX idx_patient_programs_patient ON patient_programs(patient_id);
CREATE INDEX idx_patient_programs_program ON patient_programs(program_id);
CREATE INDEX idx_patient_programs_patient_status ON patient_programs(patient_id, status);

-- Daily session per patient_program (one row per day per assignment)
CREATE TABLE workout_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    patient_program_id UUID NOT NULL REFERENCES patient_programs(id) ON DELETE CASCADE,
    session_date DATE NOT NULL,
    completed_at TIMESTAMPTZ,
    effort INT,
    pain INT,
    comment TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_patient_program_session_date UNIQUE (patient_program_id, session_date),
    CONSTRAINT chk_effort_range CHECK (effort IS NULL OR (effort >= 1 AND effort <= 10)),
    CONSTRAINT chk_pain_range CHECK (pain IS NULL OR (pain >= 0 AND pain <= 10))
);

CREATE INDEX idx_workout_sessions_patient_program ON workout_sessions(patient_program_id);
CREATE INDEX idx_workout_sessions_date ON workout_sessions(session_date);

-- =============================================================================
-- HELPER FUNCTIONS (SECURITY DEFINER to break RLS recursion)
-- =============================================================================

-- Returns the program_ids that the current patient has an assignment for.
-- SECURITY DEFINER bypasses RLS so it can be safely called from within RLS policies
-- of other tables without causing infinite recursion through patient_programs.
CREATE OR REPLACE FUNCTION public.patient_assigned_program_ids()
RETURNS SETOF UUID
LANGUAGE sql
STABLE
SECURITY DEFINER
SET search_path = public
AS $$
    SELECT program_id FROM patient_programs WHERE patient_id = auth.uid();
$$;

-- Returns the patient_program ids owned by the current patient.
CREATE OR REPLACE FUNCTION public.patient_own_patient_program_ids()
RETURNS SETOF UUID
LANGUAGE sql
STABLE
SECURITY DEFINER
SET search_path = public
AS $$
    SELECT id FROM patient_programs WHERE patient_id = auth.uid();
$$;

-- =============================================================================
-- UPDATED_AT TRIGGER
-- =============================================================================

CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER tr_profiles_updated_at
    BEFORE UPDATE ON profiles
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE TRIGGER tr_programs_updated_at
    BEFORE UPDATE ON programs
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE TRIGGER tr_patient_programs_updated_at
    BEFORE UPDATE ON patient_programs
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

CREATE TRIGGER tr_workout_sessions_updated_at
    BEFORE UPDATE ON workout_sessions
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- =============================================================================
-- ROW LEVEL SECURITY (RLS)
-- =============================================================================

ALTER TABLE profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE specialist_patients ENABLE ROW LEVEL SECURITY;
ALTER TABLE programs ENABLE ROW LEVEL SECURITY;
ALTER TABLE exercises ENABLE ROW LEVEL SECURITY;
ALTER TABLE patient_programs ENABLE ROW LEVEL SECURITY;
ALTER TABLE workout_sessions ENABLE ROW LEVEL SECURITY;

-- Helper: current user's profile id
-- In Supabase, auth.uid() returns the authenticated user's UUID.

-- ---------- profiles ----------
-- Users can read and update their own profile
CREATE POLICY "profiles_select_own"
    ON profiles FOR SELECT
    USING (id = auth.uid());

CREATE POLICY "profiles_update_own"
    ON profiles FOR UPDATE
    USING (id = auth.uid())
    WITH CHECK (id = auth.uid());

-- Specialists can read profiles of their patients
CREATE POLICY "profiles_select_patients"
    ON profiles FOR SELECT
    USING (
        id IN (
            SELECT patient_id FROM specialist_patients
            WHERE specialist_id = auth.uid()
        )
    );

-- Insert: own profile (client signup) or from trigger on auth.users
CREATE POLICY "profiles_insert_own"
    ON profiles FOR INSERT
    WITH CHECK (id = auth.uid());

-- Allow Auth trigger to create profile (trigger runs as service/backend role, not the new user)
CREATE POLICY "profiles_insert_auth_trigger"
    ON profiles FOR INSERT
    WITH CHECK (
        auth.role() = 'service_role'
        OR auth.role() = 'postgres'
        OR auth.role() = 'supabase_auth_admin'
    );

-- ---------- specialist_patients ----------
-- Specialists can do everything on their own links
CREATE POLICY "specialist_patients_all_for_specialist"
    ON specialist_patients FOR ALL
    USING (specialist_id = auth.uid())
    WITH CHECK (specialist_id = auth.uid());

-- Patients can only read their own links (see who is their specialist)
CREATE POLICY "specialist_patients_select_for_patient"
    ON specialist_patients FOR SELECT
    USING (patient_id = auth.uid());

-- ---------- programs ----------
-- Specialists can CRUD their own programs
CREATE POLICY "programs_all_for_specialist"
    ON programs FOR ALL
    USING (specialist_id = auth.uid())
    WITH CHECK (specialist_id = auth.uid());

-- Patients can read programs that are assigned to them.
-- Uses SECURITY DEFINER function to avoid RLS recursion:
-- (INSERT into patient_programs → checks programs → checks patient_programs → cycle).
CREATE POLICY "programs_select_for_patient"
    ON programs FOR SELECT
    USING (id IN (SELECT patient_assigned_program_ids()));

-- ---------- exercises ----------
-- Read via program: same as programs (specialist owns program, or patient has assignment)
CREATE POLICY "exercises_select_for_specialist_program"
    ON exercises FOR SELECT
    USING (
        program_id IN (SELECT id FROM programs WHERE specialist_id = auth.uid())
    );

CREATE POLICY "exercises_select_for_patient_program"
    ON exercises FOR SELECT
    USING (program_id IN (SELECT patient_assigned_program_ids()));

-- Only specialist can modify exercises (via program ownership)
CREATE POLICY "exercises_insert_for_specialist"
    ON exercises FOR INSERT
    WITH CHECK (
        program_id IN (SELECT id FROM programs WHERE specialist_id = auth.uid())
    );

CREATE POLICY "exercises_update_for_specialist"
    ON exercises FOR UPDATE
    USING (
        program_id IN (SELECT id FROM programs WHERE specialist_id = auth.uid())
    )
    WITH CHECK (
        program_id IN (SELECT id FROM programs WHERE specialist_id = auth.uid())
    );

CREATE POLICY "exercises_delete_for_specialist"
    ON exercises FOR DELETE
    USING (
        program_id IN (SELECT id FROM programs WHERE specialist_id = auth.uid())
    );

-- ---------- patient_programs ----------
-- Specialist can insert/select/update for their patients and their programs
CREATE POLICY "patient_programs_insert_for_specialist"
    ON patient_programs FOR INSERT
    WITH CHECK (
        patient_id IN (SELECT patient_id FROM specialist_patients WHERE specialist_id = auth.uid())
        AND program_id IN (SELECT id FROM programs WHERE specialist_id = auth.uid())
    );

CREATE POLICY "patient_programs_select_for_specialist"
    ON patient_programs FOR SELECT
    USING (
        patient_id IN (SELECT patient_id FROM specialist_patients WHERE specialist_id = auth.uid())
        OR patient_id = auth.uid()
    );

CREATE POLICY "patient_programs_update_for_specialist"
    ON patient_programs FOR UPDATE
    USING (
        patient_id IN (SELECT patient_id FROM specialist_patients WHERE specialist_id = auth.uid())
    )
    WITH CHECK (
        patient_id IN (SELECT patient_id FROM specialist_patients WHERE specialist_id = auth.uid())
    );

-- Patient can only read their own assignments
CREATE POLICY "patient_programs_select_for_patient"
    ON patient_programs FOR SELECT
    USING (patient_id = auth.uid());

-- ---------- workout_sessions ----------
-- Patient can select/insert/update their own sessions (via patient_programs they own).
-- Uses SECURITY DEFINER function to avoid potential RLS recursion through patient_programs.
CREATE POLICY "workout_sessions_select_for_patient"
    ON workout_sessions FOR SELECT
    USING (patient_program_id IN (SELECT patient_own_patient_program_ids()));

CREATE POLICY "workout_sessions_insert_for_patient"
    ON workout_sessions FOR INSERT
    WITH CHECK (patient_program_id IN (SELECT patient_own_patient_program_ids()));

CREATE POLICY "workout_sessions_update_for_patient"
    ON workout_sessions FOR UPDATE
    USING (patient_program_id IN (SELECT patient_own_patient_program_ids()))
    WITH CHECK (patient_program_id IN (SELECT patient_own_patient_program_ids()));

-- Specialist can select workout_sessions for their patients
CREATE POLICY "workout_sessions_select_for_specialist"
    ON workout_sessions FOR SELECT
    USING (
        patient_program_id IN (
            SELECT pp.id FROM patient_programs pp
            JOIN specialist_patients sp ON sp.patient_id = pp.patient_id
            WHERE sp.specialist_id = auth.uid()
        )
    );

-- =============================================================================
-- PROFILE CREATION ON SIGNUP
-- =============================================================================
-- Creates a profile when a new user signs up. Role can be set via
-- raw_user_meta_data.role ('specialist' | 'patient'); defaults to 'patient'.

CREATE OR REPLACE FUNCTION public.handle_new_user()
RETURNS TRIGGER
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
DECLARE
    user_role public.app_role;
BEGIN
    user_role := COALESCE(
        (NEW.raw_user_meta_data->>'role')::public.app_role,
        'patient'::public.app_role
    );
    INSERT INTO public.profiles (id, email, full_name, role)
    VALUES (
        NEW.id,
        COALESCE(NEW.raw_user_meta_data->>'email', NEW.email),
        COALESCE(NEW.raw_user_meta_data->>'full_name', 'User'),
        user_role
    );
    RETURN NEW;
EXCEPTION
    WHEN invalid_text_representation THEN
        user_role := 'patient'::public.app_role;
        INSERT INTO public.profiles (id, email, full_name, role)
        VALUES (
            NEW.id,
            COALESCE(NEW.raw_user_meta_data->>'email', NEW.email),
            COALESCE(NEW.raw_user_meta_data->>'full_name', 'User'),
            user_role
        );
        RETURN NEW;
END;
$$;

CREATE TRIGGER on_auth_user_created
    AFTER INSERT ON auth.users
    FOR EACH ROW EXECUTE FUNCTION public.handle_new_user();

-- =============================================================================
-- RPC: Specialist can resolve patient id by email (to add patient)
-- =============================================================================
CREATE OR REPLACE FUNCTION public.get_patient_id_by_email(p_email TEXT)
RETURNS UUID
LANGUAGE sql
SECURITY DEFINER
SET search_path = public
AS $$
    SELECT id FROM profiles WHERE role = 'patient' AND email = p_email LIMIT 1;
$$;

-- Only authenticated users can call (RLS on specialist_patients INSERT will enforce specialist)
GRANT EXECUTE ON FUNCTION public.get_patient_id_by_email(TEXT) TO authenticated;
GRANT EXECUTE ON FUNCTION public.get_patient_id_by_email(TEXT) TO service_role;

-- Fix: allow trigger handle_new_user() to insert into profiles when Auth creates a user.
-- Run this if you get "Database error creating new user" on signup.
-- The trigger runs as a backend role (e.g. service_role), so we need a policy that allows that role to insert.

DO $$
BEGIN
    CREATE POLICY "profiles_insert_auth_trigger"
        ON profiles FOR INSERT
        WITH CHECK (
            auth.role() = 'service_role'
            OR auth.role() = 'postgres'
            OR auth.role() = 'supabase_auth_admin'
        );
EXCEPTION
    WHEN duplicate_object THEN NULL;  -- policy already exists
END $$;

-- Fix: trigger "app_role does not exist" — set search_path and qualify enum as public.app_role

CREATE OR REPLACE FUNCTION public.handle_new_user()
RETURNS TRIGGER
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
DECLARE
    user_role public.app_role;
BEGIN
    user_role := COALESCE(
        (NEW.raw_user_meta_data->>'role')::public.app_role,
        'patient'::public.app_role
    );
    INSERT INTO public.profiles (id, email, full_name, role)
    VALUES (
        NEW.id,
        COALESCE(NEW.raw_user_meta_data->>'email', NEW.email),
        COALESCE(NEW.raw_user_meta_data->>'full_name', 'User'),
        user_role
    );
    RETURN NEW;
EXCEPTION
    WHEN invalid_text_representation THEN
        user_role := 'patient'::public.app_role;
        INSERT INTO public.profiles (id, email, full_name, role)
        VALUES (
            NEW.id,
            COALESCE(NEW.raw_user_meta_data->>'email', NEW.email),
            COALESCE(NEW.raw_user_meta_data->>'full_name', 'User'),
            user_role
        );
        RETURN NEW;
END;
$$;

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

-- Ensure a patient can only be assigned once to the same program.
-- If duplicates exist, keep the row with the earliest assigned_at (first assignment).

-- 1. Remove duplicate assignments, keeping one per (patient_id, program_id)
DELETE FROM patient_programs a
USING patient_programs b
WHERE a.patient_id = b.patient_id
  AND a.program_id = b.program_id
  AND a.id <> b.id
  AND a.assigned_at > b.assigned_at;

-- Add video_url (YouTube) and soft delete support to exercises.
-- Logical delete: set deleted_at so patients keep seeing the exercise in their dashboard.

ALTER TABLE exercises ADD COLUMN IF NOT EXISTS video_url TEXT;
ALTER TABLE exercises ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ DEFAULT NULL;

COMMENT ON COLUMN exercises.video_url IS 'YouTube (or other) video URL for the patient to watch';
COMMENT ON COLUMN exercises.deleted_at IS 'When set, exercise is logically deleted; patients still see it for their program';

CREATE INDEX IF NOT EXISTS idx_exercises_deleted_at ON exercises(program_id, deleted_at) WHERE deleted_at IS NULL;

-- Programs have a duration in days for the patient agenda (date range of the program).
ALTER TABLE programs
ADD COLUMN IF NOT EXISTS duration_days INT NOT NULL DEFAULT 30;

ALTER TABLE programs
ADD CONSTRAINT chk_programs_duration_positive CHECK (duration_days > 0);

COMMENT ON COLUMN programs.duration_days IS 'Number of days the program runs; defines the agenda range from assigned_at';

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
ALTER TABLE exercises ADD COLUMN IF NOT EXISTS specialist_id UUID DEFAULT auth.uid() REFERENCES profiles(id) ON DELETE CASCADE;

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

ALTER TABLE workout_sessions DROP CONSTRAINT IF EXISTS chk_effort_range;
ALTER TABLE workout_sessions DROP CONSTRAINT IF EXISTS chk_pain_range;
ALTER TABLE workout_sessions DROP COLUMN IF EXISTS effort;
ALTER TABLE workout_sessions DROP COLUMN IF EXISTS pain;
ALTER TABLE workout_sessions DROP COLUMN IF EXISTS comment;

ALTER TABLE workout_exercises ADD COLUMN IF NOT EXISTS sets INT NOT NULL DEFAULT 3;
ALTER TABLE workout_exercises ADD COLUMN IF NOT EXISTS reps INT NOT NULL DEFAULT 10;
COMMENT ON COLUMN workout_exercises.sets IS 'Number of sets for this exercise in the workout.';
COMMENT ON COLUMN workout_exercises.reps IS 'Number of repetitions per set.';

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

-- RPC: all session_exercise_feedback rows for one patient_program (fewer REST round-trips).
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

-- RPC: get_workout_with_exercises
CREATE OR REPLACE FUNCTION public.get_workout_with_exercises(p_workout_id uuid)
RETURNS jsonb
LANGUAGE sql
STABLE
SECURITY INVOKER
SET search_path = public
AS $$
    SELECT jsonb_build_object(
        'workout', jsonb_build_object(
            'id', w.id,
            'specialist_id', w.specialist_id,
            'name', w.name,
            'description', w.description,
            'order_index', w.order_index,
            'created_at', w.created_at,
            'updated_at', w.updated_at
        ),
        'exercises', COALESCE((
            SELECT jsonb_agg(
                jsonb_build_object(
                    'order_index', we.order_index,
                    'sets', we.sets,
                    'reps', we.reps,
                    'exercise', jsonb_build_object(
                        'id', e.id,
                        'specialist_id', e.specialist_id,
                        'name', e.name,
                        'description', e.description,
                        'order_index', e.order_index,
                        'video_url', e.video_url,
                        'deleted_at', e.deleted_at,
                        'created_at', e.created_at
                    )
                ) ORDER BY we.order_index
            )
            FROM workout_exercises we
            JOIN exercises e ON e.id = we.exercise_id
            WHERE we.workout_id = w.id
        ), '[]'::jsonb)
    )
    FROM workouts w
    WHERE w.id = p_workout_id;
$$;

COMMENT ON FUNCTION public.get_workout_with_exercises(uuid) IS
    'Returns workout with all exercises embedded in a single JSONB response.';

GRANT EXECUTE ON FUNCTION public.get_workout_with_exercises(uuid) TO authenticated;

-- RPC: get_program_with_agenda
CREATE OR REPLACE FUNCTION public.get_program_with_agenda(p_program_id uuid)
RETURNS jsonb
LANGUAGE sql
STABLE
SECURITY INVOKER
SET search_path = public
AS $$
    SELECT jsonb_build_object(
        'program', jsonb_build_object(
            'id', p.id,
            'specialist_id', p.specialist_id,
            'name', p.name,
            'description', p.description
        ),
        'schedule', COALESCE((
            SELECT jsonb_agg(
                jsonb_build_object(
                    'id', ps.id,
                    'program_id', ps.program_id,
                    'order_index', ps.order_index,
                    'workout_id', ps.workout_id,
                    'days_count', ps.days_count,
                    'created_at', ps.created_at
                ) ORDER BY ps.order_index
            )
            FROM program_schedule ps
            WHERE ps.program_id = p.id
        ), '[]'::jsonb),
        'workouts', COALESCE((
            SELECT jsonb_agg(DISTINCT
                jsonb_build_object(
                    'workout', jsonb_build_object(
                        'id', w.id,
                        'specialist_id', w.specialist_id,
                        'name', w.name,
                        'description', w.description,
                        'order_index', w.order_index,
                        'created_at', w.created_at,
                        'updated_at', w.updated_at
                    ),
                    'exercises', COALESCE((
                        SELECT jsonb_agg(
                            jsonb_build_object(
                                'order_index', we.order_index,
                                'sets', we.sets,
                                'reps', we.reps,
                                'exercise', jsonb_build_object(
                                    'id', e.id,
                                    'specialist_id', e.specialist_id,
                                    'name', e.name,
                                    'description', e.description,
                                    'order_index', e.order_index,
                                    'video_url', e.video_url,
                                    'deleted_at', e.deleted_at,
                                    'created_at', e.created_at
                                )
                            ) ORDER BY we.order_index
                        )
                        FROM workout_exercises we
                        JOIN exercises e ON e.id = we.exercise_id
                        WHERE we.workout_id = w.id
                    ), '[]'::jsonb)
                )
            )
            FROM program_schedule ps2
            JOIN workouts w ON w.id = ps2.workout_id
            WHERE ps2.program_id = p.id AND ps2.workout_id IS NOT NULL
        ), '[]'::jsonb)
    )
    FROM programs p
    WHERE p.id = p_program_id;
$$;

COMMENT ON FUNCTION public.get_program_with_agenda(uuid) IS
    'Returns program with schedule and all workouts (with exercises) in a single JSONB response.';

GRANT EXECUTE ON FUNCTION public.get_program_with_agenda(uuid) TO authenticated;

-- RPC: get_patient_program_full
CREATE OR REPLACE FUNCTION public.get_patient_program_full(p_patient_program_id uuid)
RETURNS jsonb
LANGUAGE sql
STABLE
SECURITY INVOKER
SET search_path = public
AS $$
    SELECT jsonb_build_object(
        'patient_program', jsonb_build_object(
            'id', pp.id,
            'patient_id', pp.patient_id,
            'program_id', pp.program_id,
            'status', pp.status
        ),
        'program', jsonb_build_object(
            'id', prog.id,
            'specialist_id', prog.specialist_id,
            'name', prog.name,
            'description', prog.description
        ),
        'schedule', COALESCE((
            SELECT jsonb_agg(
                jsonb_build_object(
                    'id', ps.id,
                    'program_id', ps.program_id,
                    'order_index', ps.order_index,
                    'workout_id', ps.workout_id,
                    'days_count', ps.days_count,
                    'created_at', ps.created_at
                ) ORDER BY ps.order_index
            )
            FROM program_schedule ps
            WHERE ps.program_id = pp.program_id
        ), '[]'::jsonb),
        'workouts', COALESCE((
            SELECT jsonb_agg(DISTINCT
                jsonb_build_object(
                    'workout', jsonb_build_object(
                        'id', w.id,
                        'specialist_id', w.specialist_id,
                        'name', w.name,
                        'description', w.description,
                        'order_index', w.order_index,
                        'created_at', w.created_at,
                        'updated_at', w.updated_at
                    ),
                    'exercises', COALESCE((
                        SELECT jsonb_agg(
                            jsonb_build_object(
                                'order_index', we.order_index,
                                'sets', we.sets,
                                'reps', we.reps,
                                'exercise', jsonb_build_object(
                                    'id', e.id,
                                    'specialist_id', e.specialist_id,
                                    'name', e.name,
                                    'description', e.description,
                                    'order_index', e.order_index,
                                    'video_url', e.video_url,
                                    'deleted_at', e.deleted_at,
                                    'created_at', e.created_at
                                )
                            ) ORDER BY we.order_index
                        )
                        FROM workout_exercises we
                        JOIN exercises e ON e.id = we.exercise_id
                        WHERE we.workout_id = w.id
                    ), '[]'::jsonb)
                )
            )
            FROM program_schedule ps2
            JOIN workouts w ON w.id = ps2.workout_id
            WHERE ps2.program_id = pp.program_id AND ps2.workout_id IS NOT NULL
        ), '[]'::jsonb),
        'sessions', COALESCE((
            SELECT jsonb_agg(
                jsonb_build_object(
                    'id', ws.id,
                    'patient_program_id', ws.patient_program_id,
                    'day_index', ws.day_index,
                    'session_date', ws.session_date,
                    'completed_at', ws.completed_at,
                    'created_at', ws.created_at,
                    'updated_at', ws.updated_at
                ) ORDER BY ws.day_index
            )
            FROM workout_sessions ws
            WHERE ws.patient_program_id = pp.id
        ), '[]'::jsonb),
        'feedback', COALESCE((
            SELECT jsonb_agg(
                jsonb_build_object(
                    'workout_session_id', sef.workout_session_id,
                    'exercise_id', sef.exercise_id,
                    'effort', sef.effort,
                    'pain', sef.pain,
                    'comment', sef.comment
                )
            )
            FROM session_exercise_feedback sef
            JOIN workout_sessions ws2 ON ws2.id = sef.workout_session_id
            WHERE ws2.patient_program_id = pp.id
        ), '[]'::jsonb)
    )
    FROM patient_programs pp
    JOIN programs prog ON prog.id = pp.program_id
    WHERE pp.id = p_patient_program_id;
$$;

COMMENT ON FUNCTION public.get_patient_program_full(uuid) IS
    'Returns complete patient program data including program, schedule, workouts, sessions, and feedback.';

GRANT EXECUTE ON FUNCTION public.get_patient_program_full(uuid) TO authenticated;

-- RPC: get_specialist_dashboard
CREATE OR REPLACE FUNCTION public.get_specialist_dashboard()
RETURNS jsonb
LANGUAGE sql
STABLE
SECURITY INVOKER
SET search_path = public
AS $$
    SELECT jsonb_build_object(
        'links', COALESCE((
            SELECT jsonb_agg(
                jsonb_build_object(
                    'id', sp.id,
                    'specialist_id', sp.specialist_id,
                    'patient_id', sp.patient_id,
                    'created_at', sp.created_at
                )
            )
            FROM specialist_patients sp
            WHERE sp.specialist_id = auth.uid()
        ), '[]'::jsonb),
        'profiles', COALESCE((
            SELECT jsonb_agg(
                jsonb_build_object(
                    'id', prof.id,
                    'email', prof.email,
                    'full_name', prof.full_name,
                    'role', prof.role,
                    'created_at', prof.created_at,
                    'updated_at', prof.updated_at
                )
            )
            FROM profiles prof
            WHERE prof.id IN (
                SELECT sp2.patient_id FROM specialist_patients sp2
                WHERE sp2.specialist_id = auth.uid()
            )
        ), '[]'::jsonb),
        'programs', COALESCE((
            SELECT jsonb_agg(
                jsonb_build_object(
                    'id', prog.id,
                    'specialist_id', prog.specialist_id,
                    'name', prog.name,
                    'description', prog.description
                ) ORDER BY prog.created_at DESC
            )
            FROM programs prog
            WHERE prog.specialist_id = auth.uid()
        ), '[]'::jsonb),
        'assignments', COALESCE((
            SELECT jsonb_agg(
                jsonb_build_object(
                    'id', pp.id,
                    'patient_id', pp.patient_id,
                    'program_id', pp.program_id,
                    'status', pp.status
                ) ORDER BY pp.assigned_at DESC
            )
            FROM patient_programs pp
            WHERE pp.patient_id IN (
                SELECT sp3.patient_id FROM specialist_patients sp3
                WHERE sp3.specialist_id = auth.uid()
            )
        ), '[]'::jsonb)
    );
$$;

COMMENT ON FUNCTION public.get_specialist_dashboard(uuid) IS
    'Returns specialist dashboard data: linked patients with profiles, programs, and assignments.';

GRANT EXECUTE ON FUNCTION public.get_specialist_dashboard(uuid) TO authenticated;
