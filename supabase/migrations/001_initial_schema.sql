-- MVP Phase 1 — Database Schema
-- Supabase (PostgreSQL) with RLS
-- Run this migration in Supabase SQL Editor or via Supabase CLI

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
    specialist_id UUID NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
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
