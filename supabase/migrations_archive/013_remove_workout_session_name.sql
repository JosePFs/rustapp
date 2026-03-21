-- Session name removed: the "name" of a session is the workout name (edited by specialist in workout library).
-- Patients cannot rename sessions.
ALTER TABLE workout_sessions DROP COLUMN IF EXISTS name;
