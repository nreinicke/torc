#!/usr/bin/env bash
# Register workflow scripts as SoftwareApplication entities in the RO-Crate.
# Reads TORC_WORKFLOW_ID from the environment (set automatically by torc).
set -euo pipefail

SCRIPTS=(
  "examples/scripts/ro_crate_preprocess.py"
  "examples/scripts/ro_crate_analyze.py"
  "examples/scripts/ro_crate_transform.py"
  "examples/scripts/ro_crate_report.py"
)

for script in "${SCRIPTS[@]}"; do
  script_name=$(basename "$script")
  torc ro-crate create "$TORC_WORKFLOW_ID" \
    --entity-id "$script" \
    --entity-type SoftwareApplication \
    --metadata "{\"name\": \"$script_name\", \"programmingLanguage\": \"Python\"}"
  echo "Registered $script_name"
done
