-- Remove the standalone idx_job_status index on job(status).
-- It is redundant: every query that filters on job.status also filters on
-- workflow_id (covered by idx_job_workflow_status) or on unblocking_processed
-- (covered by the partial indexes idx_job_unblocking_pending and
-- idx_job_workflow_unblocking).
DROP INDEX IF EXISTS idx_job_status;
