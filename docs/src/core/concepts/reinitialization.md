# Workflow Reinitialization

When you modify input files or configuration after a workflow has run, you need a way to re-execute
only the affected jobs. Reinitialization handles this by detecting what changed and marking the
appropriate jobs for re-execution.

## When to Use Reinitialization

Use `torc workflows reinit` when:

- **Input files changed** — You modified an input file and want dependent jobs to rerun
- **Configuration updated** — You changed user_data parameters
- **Output files missing** — Output files were deleted and need regeneration
- **Job definition changed** — You modified a job's command or other attributes
- **Iterative development** — You're refining a workflow and need quick iteration

## Basic Usage

```bash
# Preview what would change (recommended first step)
torc workflows reinit <workflow_id> --dry-run

# Reinitialize the workflow
torc workflows reinit <workflow_id>

# Force reinitialization even with warnings
torc workflows reinit <workflow_id> --force
```

## How Change Detection Works

Reinitialization detects changes through three mechanisms:

### 1. File Modification Times

For files tracked in the workflow, Torc compares the current `st_mtime` (modification time) against
the stored value. If a file was modified since the last run, jobs that use it as input are marked
for re-execution.

```bash
# Modify an input file
echo "new data" > input.json

# Reinitialize detects the change
torc workflows reinit <workflow_id>
# Output: Reset 3 jobs due to changed inputs
```

### 2. Job Attribute and User Data Hashing

Torc computes SHA256 hashes of critical job attributes (such as the command) and `user_data` input
values. If any hash differs from the stored value, the job is marked for re-execution. This detects
changes like modified commands, updated scripts, or changed configuration parameters.

### 3. Missing Output Files

If a job's output file no longer exists on disk, the job is marked for re-execution regardless of
whether inputs changed.

## The Reinitialization Process

When you run `reinitialize`, Torc performs these steps:

1. **Bump run_id** — Increments the workflow's run counter for tracking
2. **Reset workflow status** — Clears the previous run's completion state
3. **Check file modifications** — Compares current `st_mtime` values to stored values
4. **Check missing outputs** — Identifies jobs whose output files no longer exist
5. **Check user_data changes** — Computes and compares input hashes
6. **Mark affected jobs** — Sets jobs needing re-execution to `uninitialized`
7. **Re-evaluate dependencies** — Runs `initialize_jobs` to set jobs to `ready` or `blocked`

## Dependency Propagation

When a job is marked for re-execution, all downstream jobs that depend on its outputs are also
marked. This ensures the entire dependency chain is re-executed:

```
preprocess (input changed) → marked for rerun
    ↓
process (depends on preprocess output) → also marked
    ↓
postprocess (depends on process output) → also marked
```

## Dry Run Mode

Always use `--dry-run` first to preview changes without modifying anything:

```bash
torc workflows reinit <workflow_id> --dry-run
```

Example output:

```
Dry run: 5 jobs would be reset due to changed inputs
  - preprocess
  - analyze_batch_1
  - analyze_batch_2
  - merge_results
  - generate_report
```

## Async Reinitialization

Reinitialization runs asynchronously on the server: even though `torc workflows reinit` feels
synchronous, it is implemented by queuing a task and waiting for it to finish. For large workflows
the dependency graph rebuild can take seconds to minutes, which is why the work is offloaded.

By default the CLI blocks on the task using the workflow SSE stream (with polling as a fallback) and
exits non-zero if it fails. Pass `--async` to skip the wait and get the task handle back for
scripting:

```bash
# Kick off a reinit and resume later
task_id=$(torc -f json workflows reinit <workflow_id> --async | jq -r .task_id)
# ... do other work ...
torc tasks wait --timeout 300 "$task_id"
```

A few properties worth knowing:

- **One active task per workflow.** At most one async task is in-flight for a given workflow at a
  time (different async operations would conflict on overlapping state, so they are serialized).
  Calling `reinit` while a previous reinit is still running is idempotent: you receive the existing
  task instead of a new one.
- **Crash-safe.** Tasks are persisted server-side. If the server restarts while a reinit is
  in-flight, the task is marked `failed` with an explanatory error on startup, so clients polling or
  waiting receive a terminal state rather than hanging.
- **Timeout leaves the task running.** If the CLI's wait times out (via `--wait-timeout`), the
  server-side task keeps going. Resume with `torc tasks wait <id>`.
- **Dry-run is synchronous.** `--dry-run` does not create a task; it returns the preview directly.

## Retrying Failed Jobs

**Important:** Reinitialization does not automatically retry failed jobs. To retry failed jobs, use
`reset-status`:

```bash
# Reset failed jobs to ready status, then reinitialize to check for other changes
torc workflows reset-status <workflow_id> --failed-only --reinitialize

# Or just reset failed jobs without reinitialization
torc workflows reset-status <workflow_id> --failed-only
```

## Comparison with Full Reset

| Scenario                 | Use `reinitialize`  | Use `reset-status`        |
| ------------------------ | ------------------- | ------------------------- |
| Input file changed       | Yes                 | No                        |
| Job command changed      | Yes                 | No                        |
| Want to rerun everything | No                  | Yes                       |
| Retry failed jobs only   | No                  | **Yes** (`--failed-only`) |
| Iterative development    | Yes                 | Depends                   |
| Changed workflow spec    | Create new workflow | Create new workflow       |
