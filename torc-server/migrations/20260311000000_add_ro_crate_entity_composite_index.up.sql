-- Replace the single-column workflow_id index with a composite index
-- on (workflow_id, entity_id) to cover duplicate-check queries.
-- The composite index also serves workflow_id-only queries.

-- Deduplicate existing rows before creating the unique index.
-- Keep only the most recent record (highest id) per (workflow_id, entity_id).
DELETE FROM ro_crate_entity
WHERE id NOT IN (
    SELECT MAX(id) FROM ro_crate_entity GROUP BY workflow_id, entity_id
);

DROP INDEX IF EXISTS idx_ro_crate_entity_workflow_id;
CREATE UNIQUE INDEX idx_ro_crate_entity_workflow_entity ON ro_crate_entity(workflow_id, entity_id);
