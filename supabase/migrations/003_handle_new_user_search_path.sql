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
