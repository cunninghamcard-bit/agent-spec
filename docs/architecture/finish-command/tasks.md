---
artifact: tasks
goal: "Finish Command"
status: active
derived_from:
  - spec.md
  - plan.md
---

# Finish Command Tasks

> Link each implementation task to a Scenario name or Test selector from `spec.md`.

## Review Gate

- [x] Review `spec.md` and resolve every open question.
- [x] Review `plan.md` against the authoritative contract.

## Implementation

- [x] Add Finish subcommand + cmd_finish verification gate. Covers: `test_cli_finish_refuses_on_failing_contract_e2e`
- [x] Cleanup of plan.md/tasks.md + prints. Covers: `test_cli_finish_removes_plan_and_tasks_e2e`, `test_cli_finish_bare_spec_reports_nothing_to_clean_e2e`
- [x] --retire with spec.md-basename safety check. Covers: `test_cli_finish_retire_removes_goal_directory_e2e`
- [x] Promote hint when Rules present.

## Tests

- [x] Black-box E2E `tests/finish_cli_e2e.rs` for all four scenarios.

## Documentation Impact

- [x] README (SDD workflow section gains the finish step) + CHANGELOG.

## Quality Gates

- [x] `agent-spec lint docs/architecture/finish-command/spec.md --min-score 0.7`
- [x] `agent-spec lifecycle docs/architecture/finish-command/spec.md --code . --change-scope worktree`
- [x] `agent-spec guard --spec-dir specs --spec-dir docs/architecture/finish-command --code .`
- [x] Dogfood: `agent-spec finish docs/architecture/doc-impact-guard/spec.md --code .`
