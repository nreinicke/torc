-- Revert to the original single-column index
DROP INDEX IF EXISTS idx_ro_crate_entity_workflow_entity;
CREATE INDEX idx_ro_crate_entity_workflow_id ON ro_crate_entity(workflow_id);
