---
artifact: tasks
goal: "Integrate Command"
status: active
derived_from:
  - spec.md
  - plan.md
---

# Integrate Command Tasks

> Link each implementation task to a Scenario name or Test selector from `spec.md`.

## Review Gate

- [x] Review `spec.md` and resolve every open question.
- [x] Review `plan.md` against the authoritative contract.

## Implementation

- [x] Policy block + upsert + embedded skills in integrations.rs. Covers: `test_integration_policy_block_is_prose_only`
- [x] Integrate subcommand. Covers: `test_cli_integrate_installs_skills_e2e`, `test_cli_integrate_writes_policy_blocks_e2e`, `test_cli_integrate_preserves_and_refreshes_e2e`
- [x] init test-binding prefill. Covers: `test_cli_init_prefills_vitest_binding_for_node_e2e`, `test_cli_init_omits_binding_for_cargo_e2e`

## Tests

- [x] Unit tests beside the constants; five E2E in tests/integrate_cli_e2e.rs

## Documentation Impact

- [x] README (integrate step in SDD workflow) + CHANGELOG

## Quality Gates

- [x] `agent-spec lifecycle docs/features/integrate-command/spec.md --code . --change-scope worktree`
- [x] `agent-spec guard --spec-dir specs --code . --change-scope worktree`
- [x] Real-project dogfood: run integrate against a scratch TS project

## Notes

- Two concurrent goals in one worktree make each other's lifecycle
  boundary check fail (observed with parser-fence-blindness); keep one
  active goal per worktree or pass explicit --change.
