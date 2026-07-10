---
artifact: tasks
goal: "Doc Impact Guard"
status: done
derived_from:
  - spec.md
  - plan.md
---

# Doc Impact Guard Tasks

> Link each implementation task to a Scenario name or Test selector from `spec.md`.

## Review Gate

- [x] Review `spec.md` and resolve every open question.
- [x] Review `plan.md` against the authoritative contract.

## Implementation

- [x] Add `src/doc_impact.rs`: marker parsing. Covers: `test_doc_impact_parses_governs_marker`
- [x] Add discovery + warning join. Covers: `test_doc_impact_warns_when_governed_path_changes`, `test_doc_impact_quiet_when_doc_updated_together`, `test_doc_impact_quiet_without_overlap`
- [x] Hook into `cmd_guard` (stderr only, exit status untouched). Covers: `test_cli_guard_doc_impact_warns_without_failing_e2e`
- [x] Add governs marker + docs to README.md (self-application)

## Tests

- [x] Unit tests for scenarios 1-4 in `src/doc_impact.rs`
- [x] Black-box E2E `tests/doc_impact_guard_e2e.rs` for scenario 5

## Documentation Impact

- [x] README.md updated (marker syntax section + dogfood marker); CHANGELOG entry

## Quality Gates

- [x] `agent-spec lint docs/architecture/doc-impact-guard/spec.md --min-score 0.7`
- [x] `agent-spec lifecycle docs/architecture/doc-impact-guard/spec.md --code . --change-scope worktree`
- [x] `agent-spec guard --spec-dir specs --spec-dir docs/architecture/doc-impact-guard --code .`
