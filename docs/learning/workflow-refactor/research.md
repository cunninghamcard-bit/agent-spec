---
artifact: research
goal: "Workflow Refactor"
derived_into: spec.md
---

# Workflow Refactor — Research

> Follow every claim back to the source that owns it. Primary sources
> first; never trust parametric knowledge. See the agent-spec-research
> skill for the methodology.

## Unknowns

1. Artifact birth order: do SDD tools scaffold spec/plan/tasks in one
   shot, or does each workflow step create its own artifact?
2. Command-surface tiering: how do mature CLIs keep a large command
   surface navigable — what is the precedent for core vs. legacy tiers?
3. Two spec homes: what relationship do other tools define between a
   durable spec library and per-goal change folders?

## Industry Norms & Prior Art

- **spec-kit (GitHub official)**: one command per phase, each birthing its
  own artifact — `specify` creates the spec, `plan` creates plan.md (and
  research.md at Phase 0), `tasks` creates tasks.md; `clarify`, `analyze`,
  `implement` round out the set. There is no one-shot scaffold of the
  trio. Source: [templates/commands/](https://github.com/github/spec-kit/tree/main/templates/commands)
  (specify.md read in full; directory listing read 2026-07-11).
- **DeepChat**: no scaffold command at all. The workflow is
  Classification → Specification ("Write the required artifact set") →
  Implementation; artifacts are authored by hand when their step arrives.
  Completed goals graduate into maintained docs; "Long-term history should
  be recovered from git history, not accumulated under docs/archives/".
  Source: DeepChat docs/spec-driven-dev.md (read in full, local checkout).
- **OpenSpec (Fission-AI, 60k★)**: counterpoint on ordering — "fluid not
  rigid ... you can create artifacts in any order that makes sense".
  Structural answer on spec homes: `specs/` is the source of truth for
  current behavior, `changes/` holds proposal folders, and archiving a
  change merges its deltas back into `specs/`. Source:
  [docs/concepts.md](https://github.com/Fission-AI/OpenSpec/blob/main/docs/concepts.md)
  (read in full).
- **git**: the canonical command-tiering precedent — "We divide Git into
  high level ('porcelain') commands and low level ('plumbing') commands"
  and "We separate the porcelain commands into the main commands and some
  ancillary user utilities"; default `git help` lists only the common
  subset. Source: [Documentation/git.adoc](https://github.com/git/git/blob/master/Documentation/git.adoc)
  lines 237–252 (read 2026-07-11).
- **This repo, dogfood evidence (this session)**: `init --kind` births
  spec.md + plan.md + tasks.md before research/grill are permitted to
  start, contradicting the grill-before-spec rule the research-and-learn
  contract just established; plan.md has two owners (init template vs
  `plan --out`); the CLI exposes 25 subcommands while the exercised SDD
  loop uses 9 (init, research, lint, contract, plan, lifecycle, guard,
  finish, integrate); `brief` is a self-declared alias, `gen-integrations`
  name-collides with `integrate`; parallel goals in one worktree trip each
  other's boundary checks (known limitation, undeclared in the workflow).
  Source: session history; `agent-spec --help` output 2026-07-11.

## Current Codebase State

### Workflow state (hand-surveyed)

`init --kind` creates spec.md, plan.md, and tasks.md in one shot while the
research-and-learn contract forbids spec authoring before the grill
completes; plan.md has two owners (init template and `plan --out`). Task
contracts live in both specs/ (43 historical task specs, the project
constitution, and a roadmap staging dir) and docs/*/ goal packages;
specs/capabilities/ was never populated. The one-active-goal-per-worktree
limitation is undeclared anywhere.

### Command inventory (all 24 subcommands, surveyed 2026-07-11)

Sources: `agent-spec <cmd> --help` for every command; the upstream task
spec's Intent section for each command's design rationale (cited inline).

**The exercised SDD loop (9):** init, research, lint, contract, plan,
lifecycle, guard, finish, integrate.

**Verification internals** — the pieces lifecycle composes; runnable
standalone as plumbing:
- `parse` — show the AST of a spec file
- `verify` — run verification only (no lint/report wrap)
- `matrix` — coverage matrix Rule × Scenario × Test × Verdict ×
  Provenance; also detects dangling selectors (task-coverage-matrix-v1)
- `explain` — human-readable contract review summary; Phase 1's "contract
  replaces the code diff as the review entry"
  (task-phase1-contract-review-loop)
- `brief` — self-declared compatibility alias of `contract`

**Review/VCS flow** — Phase 1–2's review-loop design
(task-phase1-contract-review-loop, task-phase2-run-history-and-vcs-context):
- `stamp` — git trailers for a verified contract. Fills a felt hole: this
  session hand-wrote Spec-Name/Spec-Passing trailers on every commit.
- `checkpoint` — VCS-aware checkpoint preview/create
- `install-hooks` — git hooks for automatic spec checking
- `resolve-ai` — merge an external AI's decisions into a verification
  report; the provider-agnostic seam (task-host-injected-ai-backend)

**Library governance** — the BDD-spine phases:
- `audit` — spec-library health check: unproven rules, orphan scenarios,
  open questions (task-audit-v1)
- `promote` — lift a proven task Rule into specs/capabilities/; upstream's
  BDD-native OpenSpec-style living library (task-capability-promote-v1).
  Not adopted (grill record 0003); specs/capabilities/ was never created.
- `discover` — reverse-engineer a draft spec from existing tests; the
  brownfield cold-start entry (task-discover-from-codebase-v1)
- `graph` — spec dependency graph (DOT/SVG)

**Structural probe:**
- `check-structure` — forbid a reference within a file glob; the first
  non-Test mechanical probe, dependency-cruiser-lite
  (task-structural-check-v1)

**Superseded (overlap discovered by this audit):**
- `gen-integrations` — upstream's single-source generator for
  AGENTS.md/.cursorrules/skills with a `--check` drift guard
  (task-gen-integrations-v1). We unknowingly reinvented this as
  `integrate`; ours adds managed policy blocks and embedded skills, theirs
  adds drift-checking. The name collision is real ancestry, not accident.

### Machine-scanned file list

<!-- agent-spec:generated:start -->
Files (98):
- .claude/skills/agent-spec-authoring/SKILL.md
- .claude/skills/agent-spec-authoring/references/patterns.md
- .claude/skills/agent-spec-research/SKILL.md
- .claude/skills/agent-spec-sdd/SKILL.md
- .claude/skills/agent-spec-tool-first/SKILL.md
- .claude/skills/agent-spec-tool-first/references/commands.md
- docs/architecture/workflow-refactor/learning-records/0001-artifact-birth-order.md
- docs/architecture/workflow-refactor/learning-records/0002-inherited-command-surface.md
- docs/architecture/workflow-refactor/learning-records/0003-single-spec-home.md
- docs/architecture/workflow-refactor/learning-records/0004-historical-specs-disposal.md
- docs/architecture/workflow-refactor/learning-records/0005-workflow-invariants.md
- docs/architecture/workflow-refactor/plan.md
- docs/architecture/workflow-refactor/research.md
- docs/architecture/workflow-refactor/spec.md
- docs/architecture/workflow-refactor/tasks.md
- skills/agent-spec-authoring/SKILL.md
- skills/agent-spec-authoring/references/patterns.md
- skills/agent-spec-estimate/SKILL.md
- skills/agent-spec-estimate/references/examples.md
- skills/agent-spec-research/SKILL.md
- skills/agent-spec-sdd/SKILL.md
- skills/agent-spec-tool-first/SKILL.md
- skills/agent-spec-tool-first/references/commands.md
- specs/project.spec.md
- specs/roadmap/README.md
- specs/roadmap/task-checkpoint-resume.spec.md
- specs/roadmap/task-complexity-gate.spec.md
- specs/roadmap/task-context-fidelity.spec.md
- specs/roadmap/task-goal-gate.spec.md
- specs/roadmap/task-history-summary.spec.md
- specs/roadmap/task-human-review.spec.md
- specs/roadmap/task-optimize-scenario-mode.spec.md
- specs/roadmap/task-phase0-contract-fidelity.spec.md
- specs/roadmap/task-phase1-contract-review-loop.spec.md
- specs/roadmap/task-phase2-run-history-and-vcs-context.spec.md
- specs/roadmap/task-phase3-spec-governance.spec.md
- specs/roadmap/task-phase4-ai-verification-expansion.spec.md
- specs/roadmap/task-phase5-ecosystem-integrations.spec.md
- specs/roadmap/task-phase6-advanced-verification.spec.md
- specs/roadmap/task-plan-command.spec.md
- specs/roadmap/task-scenario-dependencies.spec.md
- specs/roadmap/task-spec-dependency-graph.spec.md
- specs/roadmap/task-status-file-contract.spec.md
- specs/roadmap/task-strengthen-rewrite-contract-authoring.spec.md
- specs/roadmap/task-support-scenario-verification-metadata.spec.md
- specs/task-add-ai-verifier-skeleton.spec.md
- specs/task-add-behavior-completeness-linters.spec.md
- specs/task-audit-v1.spec.md
- specs/task-bdd-semantics-v1.spec.md
- specs/task-capability-promote-v1.spec.md
- specs/task-coverage-matrix-v1.spec.md
- specs/task-deepchat-style-sdd-docs.spec.md
- specs/task-derive-change-set-from-staged-git-index.spec.md
- specs/task-discover-from-codebase-v1.spec.md
- specs/task-discovery-questions-v1.spec.md
- specs/task-enforce-boundaries-with-explicit-change-set.spec.md
- specs/task-english-only-dsl.spec.md
- specs/task-fail-on-skipped.spec.md
- specs/task-fix-contract-fidelity.spec.md
- specs/task-fix-inheritance.spec.md
- specs/task-formalize-test-binding.spec.md
- specs/task-gen-integrations-v1.spec.md
- specs/task-guard-forbidden-only-boundaries.spec.md
- specs/task-host-injected-ai-backend.spec.md
- specs/task-jj-vcs-integration.spec.md
- specs/task-lint-ack-dimensions-v1.spec.md
- specs/task-make-contract-default.spec.md
- specs/task-phase1-contract-review-loop.spec.md
- specs/task-phase2-run-history-and-vcs-context.spec.md
- specs/task-phase3-spec-governance.spec.md
- specs/task-phase4-ai-verification-expansion.spec.md
- specs/task-phase5-ecosystem-integrations.spec.md
- specs/task-phase6-advanced-verification.spec.md
- specs/task-pluggable-ai-backend-interface.spec.md
- specs/task-probe-abstraction-v1.spec.md
- specs/task-remove-cwd-repo-fallback.spec.md
- specs/task-report-driven-verification.spec.md
- specs/task-require-explicit-test-selectors.spec.md
- specs/task-sdd-sections-and-clarification-lint.spec.md
- specs/task-sdd-workflow-cli.spec.md
- specs/task-ship-claude-code-tool-first-skills.spec.md
- specs/task-stage-roadmap-specs.spec.md
- specs/task-structural-check-v1.spec.md
- specs/task-structure-test-selectors.spec.md
- specs/task-support-change-scope-in-verify-and-lifecycle.spec.md
- specs/task-support-git-worktree-change-scope.spec.md
- specs/task-support-spec-md-extension.spec.md
- specs/task-support-step-tables.spec.md
- src/spec_gateway/brief.rs — pub struct TaskContract {
- src/spec_gateway/lifecycle.rs — pub struct SpecGateway {
- src/spec_gateway/mod.rs
- src/spec_gateway/plan.rs — pub struct PlanContext {
- tests/doc_impact_guard_e2e.rs — Black-box E2E for the Doc Impact Guard: warnings surface on stderr and
- tests/finish_cli_e2e.rs — Black-box E2E for `finish`: graduation removes consumables only after the
- tests/integrate_cli_e2e.rs — Black-box E2E for `integrate`: skills for both agent systems, managed
- tests/repo_specs_english_only.rs — Regression guard: the repo's own spec corpus must use English structural
- tests/research_cli_e2e.rs — Black-box E2E for `research`: scaffold on first run, refresh only the
- tests/sdd_init_cli.rs

Test functions (9):
- src/spec_gateway/brief.rs: test_contract_renders_current_state_and_ux_shape, test_contract_renders_scenarios_grouped_by_rule, test_legacy_contract_without_rule_stays_flat
- src/spec_gateway/lifecycle.rs: test_full_lifecycle, test_plan_returns_task_contract, test_quality_gate, test_quality_gate_fails_on_error_lint_issue, test_contract_prompt_format, test_skip_is_not_passing, test_load_resolves_inherited_constraints_from_spec_directory, test_load_resolves_full_project_contract_from_spec_directory, test_task_contract_keeps_must_must_not_and_decisions_distinct, test_legacy_brief_stays_derived_from_contract, test_pass_plus_skip_is_not_passing, test_new_bdd_lints_do_not_affect_lifecycle_verdict, test_existing_specs_pass_lifecycle_after_v1_changes, test_verify_with_ai_mode_stub_marks_uncovered_scenarios_uncertain, test_matrix_does_not_change_is_passing, test_provenance_ai_stub_is_inferential, test_verify_default_keeps_uncovered_scenarios_skipped, test_load_resolves_org_project_task_chain, test_verify_with_injected_ai_backend_uses_host_backend, test_critical_scenario_fail_sets_gate_blocked, test_critical_scenario_pass_no_gate_block, test_no_critical_tag_preserves_existing_behavior, test_critical_suffix_in_scenario_name, test_critical_fail_exit_code_is_2, test_human_review_scenario_produces_pending_review, test_auto_review_mode_treats_pending_as_pass, test_strict_review_mode_treats_pending_as_not_pass, test_optimize_scenario_pass_listed_as_candidate, test_optimize_scenario_fail_blocks_pass, test_dependency_skip_on_prerequisite_fail, test_topological_sort_execution_order, test_no_dependency_preserves_original_order
- src/spec_gateway/plan.rs: test_plan_includes_contract_section, test_plan_includes_codebase_context, test_plan_includes_task_sketch, test_plan_respects_gitignore, test_plan_json_format_is_valid, test_plan_prompt_format_is_self_contained, test_plan_prompt_includes_rule_grouping, test_plan_full_depth_includes_pub_signatures, test_plan_warns_on_missing_boundary_path, test_plan_lists_existing_test_functions, test_extract_summary_doc_comment, test_extract_summary_pub_item, test_extract_test_functions, test_extract_pub_signatures, test_is_gitignored, test_extract_base_dir
- tests/doc_impact_guard_e2e.rs: test_cli_guard_doc_impact_warns_without_failing_e2e
- tests/finish_cli_e2e.rs: test_cli_finish_removes_plan_and_tasks_e2e, test_cli_finish_removes_research_consumables_e2e, test_cli_finish_refuses_on_failing_contract_e2e, test_cli_finish_retire_removes_goal_directory_e2e, test_cli_finish_bare_spec_reports_nothing_to_clean_e2e
- tests/integrate_cli_e2e.rs: test_cli_integrate_installs_skills_e2e, test_cli_integrate_installs_research_skill_e2e, test_cli_integrate_writes_policy_blocks_e2e, test_cli_integrate_preserves_and_refreshes_e2e, test_cli_init_prefills_vitest_binding_for_node_e2e, test_cli_init_omits_binding_for_cargo_e2e
- tests/repo_specs_english_only.rs: test_repo_specs_use_english_structural_keywords
- tests/research_cli_e2e.rs: test_cli_research_scaffolds_and_refreshes_e2e
- tests/sdd_init_cli.rs: test_cli_init_feature_creates_sdd_package_e2e, test_cli_init_architecture_creates_sdd_package_e2e, test_cli_init_issue_creates_single_spec_e2e, test_cli_init_sdd_package_refuses_partial_overwrite_e2e, test_cli_init_rejects_invalid_sdd_kind_e2e, test_cli_init_requires_kind_and_name_e2e, test_agent_spec_sdd_skill_routes_cli_workflow
<!-- agent-spec:generated:end -->

## Findings

### F1: Artifact birth order

- **Decision**: staged birth. `init --kind` creates the goal folder and a
  spec.md skeleton only. plan.md is born by `plan --out` when the planning
  step arrives; tasks.md is born with the plan (same step). research.md is
  already staged (born by `research` on trigger).
- **Rationale**: spec-kit births one artifact per phase command; DeepChat
  authors each artifact when its step arrives; our own contract forbids
  spec authoring before the grill completes, so pre-born plan/tasks
  templates are contradictions lying on disk. Single ownership: today
  plan.md has two owners (init template, plan --out).
- **Alternatives considered**: keep the one-shot trio (rejected: it is the
  contradiction, and no surveyed source does it); fully fluid ordering per
  OpenSpec (rejected for the default path: our grill gate is deliberately
  rigid — fluidity is preserved by commands remaining runnable
  individually).

### F2: The inherited command surface

- **Correction (user grill round 2)**: all 16 "non-core" commands are
  upstream inheritance, never evaluated by us — we added only research,
  finish, integrate (`git diff upstream/main dev` on the Commands enum).
  "Redundant" was the wrong frame; "unaudited assets" is the fact.
- **Decision (final, records 0002/0009)**: inventory first — done, see
  the Command inventory above — then **every command gets a station**: no
  second-class tier. The workflow is a map of five flows, each command
  with a declared station and trigger: Adoption (once per repo: integrate
  → install-hooks → discover), Goal lifecycle (per goal: init → research
  → spec → lint/contract → plan → implement → lifecycle with
  parse/verify/matrix as debugging internals → guard → stamp → finish →
  promote), Review (explain → matrix → contract/brief), Library
  governance (periodic: audit → graph), Probe & AI (on trigger:
  check-structure, resolve-ai, measure-determinism). Help groups by flow;
  skills document every station. Nothing deleted, nothing hidden.
- **Rationale**: user override of the two-tier framing ("全部加上啊,我们
  没说要删除啊"): situational commands get situational stations with
  declared triggers — the same demand-driven pattern as research — not a
  drawer. The audit found real assets (stamp fills the hand-written
  trailer hole; promote is F3's channel; gen-integrations is integrate's
  ancestor; install-hooks points at the retiring household and needs the
  fix).
- **Alternatives considered**: two-tier Workflow/Utilities help with a
  core loop (rejected by user: reads as demotion, hides upstream value);
  demote-by-usage-this-session (rejected earlier: usage-to-date is not an
  evaluation); delete legacy (rejected: breaking, irreversible).

### F3: The two spec homes

- **Decision (final, records 0007/0008 — re-grilled after the command
  inventory was taught; records 0003/0004 superseded via 0006)**: single
  household docs/, **containing the upstream closed loop re-homed**:
  docs/*/ goal packages are proposals (mortal); docs/capabilities/ is the
  truth layer, grown via `promote` (target path re-pointed from specs/ —
  "把specs/ 换成docs/ 就行了"); `audit` is its periodic health check.
  Disposal reaffirmed informed (record 0008): the specs/ task corpus and
  roadmap retire wholesale now; the library grows from future goals only;
  history lives in git. The constitution relocates to
  docs/project.spec.md; `inherits: project` keeps resolving.
- **Rationale**: one household satisfies "where does a contract live";
  the capability library satisfies "what are the rules now" (ends the
  historical-spec-all-red pattern for future work); OpenSpec semantics
  via upstream's own promote machinery, just re-homed.
- **Alternatives considered**: library at upstream's specs/capabilities/
  (rejected: perpetuates the second household); no library, docs/-only
  (the interim decision 0003 — revoked once promote/audit/graph were
  understood); gradual promote-then-retire of the 43 historical contracts
  (my recommendation — overridden: wholesale retirement, informed this
  time).

### F4: Workflow invariants made explicit

- **Decision**: declare "one active goal per worktree" as a workflow
  invariant in the agent-spec-sdd skill and README; lifecycle prints a
  warning when it detects multiple goal folders with uncommitted changes.
- **Rationale**: the boundary-crossfire limitation was discovered
  empirically this session and cost a debugging round; undeclared
  invariants are how workflows rot.
- **Alternatives considered**: full multi-goal isolation support
  (rejected for now: real need unproven, cost high); document-only with
  no mechanical warning (weaker but acceptable fallback — grill decides).
