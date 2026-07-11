---
artifact: tasks
goal: "Research and Learn"
status: active
derived_from:
  - spec.md
  - plan.md
---

# Research and Learn Tasks

> Link each implementation task to a Scenario name or Test selector from `spec.md`.

## Review Gate

- [x] research.md written with per-claim sources (five industry samples).
- [x] Grill session completed: 5/5 decisions confirmed by the user (F5 overridden to hard enforcement).
- [x] Review `plan.md` against the authoritative contract (plan --out draft + curation).

## Implementation

- [x] plan.rs codebase-only formatter. Covers: `test_cli_research_scaffolds_and_refreshes_e2e`
- [x] CLI research subcommand + template. Covers: `test_cli_research_scaffolds_and_refreshes_e2e`
- [x] research-required error lint. Covers: `test_lint_research_required_fires_on_unresolved_marker`, `test_lint_research_required_quiet_with_research_present`
- [x] research-uncited + research-unfilled warnings. Covers: `test_lint_research_uncited_warns`, `test_lint_research_unfilled_warns`
- [x] agent-spec-research skill + sdd/authoring/tool-first cross-links (Pocock composition). Covers: `test_cli_integrate_installs_research_skill_e2e`
- [x] integrate EMBEDDED_SKILLS + install-skills.sh. Covers: `test_cli_integrate_installs_research_skill_e2e`
- [x] finish consumables += research.md, learning-records/. Covers: `test_cli_finish_removes_research_consumables_e2e`

## Tests

- [x] tests/research_cli_e2e.rs; appended cases in integrate/finish suites; lint units beside needs-clarification tests.

## Documentation Impact

- [x] README (research step in SDD workflow) + CHANGELOG.

## Quality Gates

- [x] `agent-spec lint docs/features/research-and-learn/spec.md --min-score 0.7`
- [x] `agent-spec lifecycle docs/features/research-and-learn/spec.md --code . --change-scope worktree`
- [x] `agent-spec guard --spec-dir specs --code . --change-scope worktree`
