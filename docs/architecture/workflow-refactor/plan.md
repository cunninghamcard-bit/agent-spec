=== Contract ===

# Task Contract: Workflow Refactor

## Intent
Refactor the workflow itself, not the code layout. This session's feature
velocity left contradictions: init births artifacts the workflow forbids
touching, task contracts have two households, sixteen inherited commands
sat unaudited, and a known invariant lived only in memory. The refactored
workflow is a map of five flows in which **every one of the 25
subcommands has a declared station and trigger** — no second-class tier,
nothing deleted, nothing hidden. All governing decisions were grilled
(learning records 0001–0009, including one full reopen after the command
inventory was taught) and are grounded in research.md (F1–F4).

## Current State
See research.md in this goal folder: the workflow state and the full
24-subcommand inventory live under its Current Codebase State section
(researching our own codebase), and the surveyed industry practice under
Industry Norms & Prior Art (researching others' best practice).

## Must
- 任务级规格文件存放在 `specs/`
- 公开 CLI 与 gateway 行为必须有回归测试
- DSL 语法变更必须同时更新 AST、解析输出和回归测试
- 验证结果必须区分 `pass`、`fail`、`skip`、`uncertain`
- 任务级完成条件中的每个场景应显式声明 `测试:` selector
- 任务级边界应支持对显式 change set 的机械验证
- 测试选择器应支持结构化字段，而不仅是裸字符串过滤器
- guard 应支持可选择的 git change scope，而不局限于 staged index
- verify 与 lifecycle 应支持可选的 git change scope，同时保持默认行为稳定
- AI verifier 的 `uncertain` 结果应附带结构化 `AiAnalysis` 证据
- AI verifier 应通过可插拔 backend 接口产生结构化分析结果
- agent-spec 应保持 provider-agnostic，由宿主 agent 注入 AI backend
- 项目应提供 Claude Code 的 project-local skills，且主路径是 tool-first
- 长期路线图 task spec 应暂存于 `specs/roadmap/`，只有提升到顶层 `specs/` 后才进入默认 guard
- Task Contract 应区分 `Must`、`Must Not` 与 `Decisions`
- 默认文本 `contract` 输出应保留结构化 Completion Criteria 细节

## Must NOT
- 不要把 `skip` 记为 `pass`
- 不要要求普通磁盘用例手工提供继承搜索路径
- 不要丢弃 BDD 步骤里的结构化输入

## Decisions
- Staged artifact birth (research.md F1, record 0001): `init --kind` creates the goal folder and spec.md only; plan.md and tasks.md are born by `plan --out` when the planning step arrives, giving plan.md a single owner
- Every command stationed across five flows (research.md F2, records 0002/0009, user override of two-tier help): Adoption (once per repo: integrate → install-hooks → discover), Goal lifecycle (per goal: init → research → spec → lint/contract → plan → implement → lifecycle with parse/verify/matrix as debugging internals → guard → stamp at commit → finish → promote), Review (explain → matrix → contract/brief), Library governance (periodic: audit → graph), Probe & AI (on trigger: check-structure, resolve-ai, measure-determinism)
- Default `--help` groups subcommands under the five flow headings instead of one flat list; no command is hidden
- Single household with the upstream loop re-homed (research.md F3, records 0007/0008): docs/*/ goal packages are proposals; docs/capabilities/ is the truth layer; `promote` re-points its target from specs/capabilities/ to docs/capabilities/
- Wholesale retirement, informed (record 0008): the specs/ task corpus and roadmap staging directory are removed now; the library grows from future goals only; history lives in git
- The constitution survives relocation: specs/project.spec.md moves to docs/project.spec.md; goal specs' `inherits: project` resolves against the new location
- stamp replaces hand-written commit trailers: the sdd skill's commit step runs `stamp --dry-run` and pastes its machine-verified output
- install-hooks is fixed to the docs/ household (its generated hook currently hardcodes `--spec-dir specs`) and documented in the adoption flow
- gen-integrations is documented as integrate's ancestor, kept functional; its operational-steps output style is superseded by integrate's prose-only policy blocks
- Declared invariant with mechanical warning (research.md F4, record 0005): the agent-spec-sdd skill and README state "one active goal per worktree"; lifecycle prints a non-blocking warning when more than one goal folder carries uncommitted changes
- The agent-spec-sdd and agent-spec-tool-first skills document all five flows and every station, and are re-embedded for integrate

## Boundaries
Allowed changes:
- src/main.rs
- src/spec_gateway/**
- src/spec_parser/resolver.rs
- src/spec_report/integrations.rs
- skills/**
- .claude/skills/**
- tests/**
- README.md
- CHANGELOG.md
- specs/**
- docs/project.spec.md
- docs/capabilities/**
- docs/architecture/workflow-refactor/**
Forbidden:
- Do not delete any CLI subcommand
- Do not weaken guard: the docs/ contracts must remain verified
- Do not add new dependencies
Out of scope:
- Mining the 43 historical contracts into the capability library (rejected in record 0008; history lives in git)
- Hard-blocking multi-goal worktrees (rejected in grill round 4)
- Building new review-flow features beyond documenting the existing stations (explain/matrix ship as-is)
- Splitting main.rs or linters.rs (code-layout refactor, separate goal)

## Completion Criteria

Rule: staged-birth — artifacts are born by their workflow step
Scenario: init births only the contract
  Test:
    Package: agent-spec
    Filter: test_cli_init_feature_creates_spec_only_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given an empty project
  When init --kind feature runs
  Then the goal folder contains spec.md
  And plan.md and tasks.md do not exist

Scenario: plan births the planning artifacts
  Test:
    Package: agent-spec
    Filter: test_cli_plan_out_births_plan_and_tasks_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given a goal folder containing only spec.md
  When plan --out runs
  Then plan.md and tasks.md are created beside the spec


Rule: stationed-surface — every subcommand has a declared station
Scenario: help is grouped by flow
  Test:
    Package: agent-spec
    Filter: test_cli_help_groups_commands_by_flow_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process
  Given the installed CLI
  When --help runs
  Then the output shows the five flow headings
  And every subcommand appears under exactly one heading

Scenario: every command is documented in the audit artifact
  Test:
    Package: agent-spec
    Filter: test_tool_first_reference_covers_all_subcommands
  Given the agent-spec-tool-first commands reference
  When the documented names are compared against the CLI's subcommand list
  Then every subcommand appears in the reference

Scenario: the sdd skill stations every flow
  Test:
    Package: agent-spec
    Filter: test_sdd_skill_documents_five_flows
  Given the embedded agent-spec-sdd skill
  When its content is inspected
  Then all five flow names are documented
  And the commit step names stamp and the graduation step names promote


Rule: single-household — docs/ is the one home for contracts and capabilities
Scenario: promote targets the docs household
  Test:
    Package: agent-spec
    Filter: test_cli_promote_targets_docs_capabilities_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given a goal spec with a passing task Rule
  When promote runs with a capability name
  Then the capability spec is created under docs/capabilities/

Scenario: install-hooks guards the docs household
  Test:
    Package: agent-spec
    Filter: test_cli_install_hooks_targets_docs_home_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given a git repository
  When install-hooks runs
  Then the generated pre-commit hook invokes guard against the docs/ household
  And it does not reference the retired specs/ directory

Scenario: inheritance resolves from the relocated constitution
  Test:
    Package: agent-spec
    Filter: test_cli_goal_spec_inherits_relocated_project_constitution_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given a goal spec declaring inherits: project and a docs/project.spec.md
  When parse runs on the goal spec
  Then the resolved document carries the project constitution's constraints

Scenario: guard collects only the docs home
  Test:
    Package: agent-spec
    Filter: test_cli_guard_collects_docs_contracts_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given a repo with docs/ goal contracts and no specs/ directory
  When guard runs
  Then all docs/ contracts are collected and verified


Rule: declared-invariants — workflow invariants are stated and mechanically warned
Scenario: lifecycle warns on parallel dirty goals
  Test:
    Package: agent-spec
    Filter: test_cli_lifecycle_warns_on_multiple_dirty_goals_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given two goal folders with uncommitted changes in one worktree
  When lifecycle runs on one of them
  Then a warning naming the one-active-goal invariant is printed
  And the run is not blocked

=== Codebase Context ===

Files (102):
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
  - docs/architecture/workflow-refactor/learning-records/0006-decisions-reopened.md
  - docs/architecture/workflow-refactor/learning-records/0007-capability-library-in-docs.md
  - docs/architecture/workflow-refactor/learning-records/0008-corpus-retirement-reaffirmed.md
  - docs/architecture/workflow-refactor/learning-records/0009-all-commands-stationed.md
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
  src/spec_gateway/brief.rs:
    - test_contract_renders_current_state_and_ux_shape
    - test_contract_renders_scenarios_grouped_by_rule
    - test_legacy_contract_without_rule_stays_flat
  src/spec_gateway/lifecycle.rs:
    - test_full_lifecycle
    - test_plan_returns_task_contract
    - test_quality_gate
    - test_quality_gate_fails_on_error_lint_issue
    - test_contract_prompt_format
    - test_skip_is_not_passing
    - test_load_resolves_inherited_constraints_from_spec_directory
    - test_load_resolves_full_project_contract_from_spec_directory
    - test_task_contract_keeps_must_must_not_and_decisions_distinct
    - test_legacy_brief_stays_derived_from_contract
    - test_pass_plus_skip_is_not_passing
    - test_new_bdd_lints_do_not_affect_lifecycle_verdict
    - test_existing_specs_pass_lifecycle_after_v1_changes
    - test_verify_with_ai_mode_stub_marks_uncovered_scenarios_uncertain
    - test_matrix_does_not_change_is_passing
    - test_provenance_ai_stub_is_inferential
    - test_verify_default_keeps_uncovered_scenarios_skipped
    - test_load_resolves_org_project_task_chain
    - test_verify_with_injected_ai_backend_uses_host_backend
    - test_critical_scenario_fail_sets_gate_blocked
    - test_critical_scenario_pass_no_gate_block
    - test_no_critical_tag_preserves_existing_behavior
    - test_critical_suffix_in_scenario_name
    - test_critical_fail_exit_code_is_2
    - test_human_review_scenario_produces_pending_review
    - test_auto_review_mode_treats_pending_as_pass
    - test_strict_review_mode_treats_pending_as_not_pass
    - test_optimize_scenario_pass_listed_as_candidate
    - test_optimize_scenario_fail_blocks_pass
    - test_dependency_skip_on_prerequisite_fail
    - test_topological_sort_execution_order
    - test_no_dependency_preserves_original_order
  src/spec_gateway/plan.rs:
    - test_plan_includes_contract_section
    - test_plan_includes_codebase_context
    - test_plan_includes_task_sketch
    - test_plan_respects_gitignore
    - test_plan_json_format_is_valid
    - test_plan_prompt_format_is_self_contained
    - test_plan_prompt_includes_rule_grouping
    - test_plan_full_depth_includes_pub_signatures
    - test_plan_warns_on_missing_boundary_path
    - test_plan_lists_existing_test_functions
    - test_extract_summary_doc_comment
    - test_extract_summary_pub_item
    - test_extract_test_functions
    - test_extract_pub_signatures
    - test_is_gitignored
    - test_extract_base_dir
  tests/doc_impact_guard_e2e.rs:
    - test_cli_guard_doc_impact_warns_without_failing_e2e
  tests/finish_cli_e2e.rs:
    - test_cli_finish_removes_plan_and_tasks_e2e
    - test_cli_finish_removes_research_consumables_e2e
    - test_cli_finish_refuses_on_failing_contract_e2e
    - test_cli_finish_retire_removes_goal_directory_e2e
    - test_cli_finish_bare_spec_reports_nothing_to_clean_e2e
  tests/integrate_cli_e2e.rs:
    - test_cli_integrate_installs_skills_e2e
    - test_cli_integrate_installs_research_skill_e2e
    - test_cli_integrate_writes_policy_blocks_e2e
    - test_cli_integrate_preserves_and_refreshes_e2e
    - test_cli_init_prefills_vitest_binding_for_node_e2e
    - test_cli_init_omits_binding_for_cargo_e2e
  tests/repo_specs_english_only.rs:
    - test_repo_specs_use_english_structural_keywords
  tests/research_cli_e2e.rs:
    - test_cli_research_scaffolds_and_refreshes_e2e
  tests/sdd_init_cli.rs:
    - test_cli_init_feature_creates_sdd_package_e2e
    - test_cli_init_architecture_creates_sdd_package_e2e
    - test_cli_init_issue_creates_single_spec_e2e
    - test_cli_init_sdd_package_refuses_partial_overwrite_e2e
    - test_cli_init_rejects_invalid_sdd_kind_e2e
    - test_cli_init_requires_kind_and_name_e2e
    - test_agent_spec_sdd_skill_routes_cli_workflow

=== Task Sketch ===

Group 1 (order 1):
  Scenarios:
    - init births only the contract
    - plan births the planning artifacts
    - help is grouped by flow
    - every command is documented in the audit artifact
    - the sdd skill stations every flow
    - promote targets the docs household
    - install-hooks guards the docs household
    - inheritance resolves from the relocated constitution
    - guard collects only the docs home
    - lifecycle warns on parallel dirty goals
  Boundary paths:
    - docs/capabilities/**
    - specs/**
    - docs/project.spec.md
  Test selectors:
    - test_cli_init_feature_creates_spec_only_e2e
    - test_cli_plan_out_births_plan_and_tasks_e2e
    - test_cli_help_groups_commands_by_flow_e2e
    - test_tool_first_reference_covers_all_subcommands
    - test_sdd_skill_documents_five_flows
    - test_cli_promote_targets_docs_capabilities_e2e
    - test_cli_install_hooks_targets_docs_home_e2e
    - test_cli_goal_spec_inherits_relocated_project_constitution_e2e
    - test_cli_guard_collects_docs_contracts_e2e
    - test_cli_lifecycle_warns_on_multiple_dirty_goals_e2e

=== Warnings ===

  - Allowed Changes path not found: docs/project.spec.md (resolved to ./docs/project.spec.md)
  - Allowed Changes path not found: docs/capabilities/** (resolved to ./docs/capabilities)
