-- Restore the standalone job status index
CREATE INDEX idx_job_status ON job(status);
