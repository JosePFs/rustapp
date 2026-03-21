-- Add video_url (YouTube) and soft delete support to exercises.
-- Logical delete: set deleted_at so patients keep seeing the exercise in their dashboard.

ALTER TABLE exercises ADD COLUMN IF NOT EXISTS video_url TEXT;
ALTER TABLE exercises ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ DEFAULT NULL;

COMMENT ON COLUMN exercises.video_url IS 'YouTube (or other) video URL for the patient to watch';
COMMENT ON COLUMN exercises.deleted_at IS 'When set, exercise is logically deleted; patients still see it for their program';

CREATE INDEX IF NOT EXISTS idx_exercises_deleted_at ON exercises(program_id, deleted_at) WHERE deleted_at IS NULL;
