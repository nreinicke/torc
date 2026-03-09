-- Re-add step_nodes to resource_requirements for rollback.
-- Backfill from num_nodes so multi-node jobs retain their per-step node count.
ALTER TABLE resource_requirements ADD COLUMN step_nodes INTEGER NOT NULL DEFAULT 1;
UPDATE resource_requirements SET step_nodes = num_nodes WHERE num_nodes > 1;
