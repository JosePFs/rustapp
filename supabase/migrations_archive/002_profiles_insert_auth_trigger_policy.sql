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
