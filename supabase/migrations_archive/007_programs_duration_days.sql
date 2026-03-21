-- Programs have a duration in days for the patient agenda (date range of the program).
ALTER TABLE programs
ADD COLUMN IF NOT EXISTS duration_days INT NOT NULL DEFAULT 30;

ALTER TABLE programs
ADD CONSTRAINT chk_programs_duration_positive CHECK (duration_days > 0);

COMMENT ON COLUMN programs.duration_days IS 'Number of days the program runs; defines the agenda range from assigned_at';
