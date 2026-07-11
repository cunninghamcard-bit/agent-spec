---
artifact: research
goal: "Research and Learn"
derived_into: spec.md
---

# Research and Learn — Research

> Every claim below cites the source that owns it. Primary sources were
> examined directly (repo trees, file contents via `gh api`); star counts
> read 2026-07-11.

## Unknowns

1. Must every spec be preceded by research, or is research demand-driven?
2. What artifact shape and finding format does the industry converge on?
3. How is the *human's* understanding (not just the agent's) established?
4. What research methodology is teachable and enforceable?

## Industry Norms & Prior Art

### Survey set

| Project | Scale | Form |
|---|---|---|
| mattpocock/skills | 165k★ | pure prompt pack: 101 md files, no product code |
| gsd-build/get-shit-done | 64.7k★ | prompt-orchestration system: 786 md + installer/SDK (`bin/install.js`, claude-agent-sdk dep) |
| Fission-AI/OpenSpec | 60k★ | SDD toolkit (not deep-read this round) |
| github/spec-kit | GitHub official | CLI + templates; plan Phase 0 generates research.md |
| DeepChat | reference corpus | 148 goals examined locally |
| rust-lang/rfcs | mature governance | RFC template with embedded Prior art section |

### Findings per source

- **DeepChat corpus** (local scan): research.md exists in **1 of 148** goal folders (`docs/features/agent-memory/research.md`) — research is exceptional, reserved for the least-familiar domain. Source: local find over DeepChat@18820651.
- **spec-kit**: plan's Phase 0 turns each `[NEEDS CLARIFICATION]` into a research task; findings consolidate into research.md as **Decision / Rationale / Alternatives considered**. Research exists to resolve unknowns, not as ritual. Source: [spec-kit plan command template](https://github.com/github/spec-kit/blob/main/templates/commands/plan.md).
- **Rust RFC**: Prior art is an embedded required *section* (not a separate document); "If there is no prior art, that is fine." Source: [RFC 2333](https://rust-lang.github.io/rfcs/2333-prior-art.html), [RFC template](https://github.com/rust-lang/rfcs/blob/master/0000-template.md).
- **mattpocock/skills `research`**: ~12 lines; one hard rule — investigate against **primary sources**, "follow every claim back to the source that owns it"; output one Markdown file with per-claim citations; run as a background agent. Source: skills/engineering/research/SKILL.md (read in full).
- **mattpocock/skills `grilling`**: "Interview me relentlessly ... one at a time ... For each question, provide your recommended answer. If a fact can be found by exploring the codebase, look it up rather than asking me. The decisions, though, are mine." No enactment before shared understanding. Source: skills/productivity/grilling/SKILL.md (read in full).
- **mattpocock/skills `teach`**: "Never trust your parametric knowledge"; distinguishes fluency strength from storage strength; retrieval practice over passive reading. Source: skills/productivity/teach/SKILL.md (read in full).
- **gsd**: research is a first-class multi-agent phase — six specialized researchers (domain/project/phase/ui/ai/advisor) plus a research-synthesizer; researcher prompts carry concrete search-query templates and a structured output format where every finding requires a **Source** field. Source: agents/gsd-domain-researcher.md (read in full), repo tree.

## Current Codebase State

<!-- agent-spec:generated:start -->
Files (27):
- .claude/skills/agent-spec-authoring/SKILL.md
- .claude/skills/agent-spec-authoring/references/patterns.md
- .claude/skills/agent-spec-research/SKILL.md
- .claude/skills/agent-spec-sdd/SKILL.md
- .claude/skills/agent-spec-tool-first/SKILL.md
- .claude/skills/agent-spec-tool-first/references/commands.md
- docs/features/research-and-learn/plan.md
- docs/features/research-and-learn/research.md
- docs/features/research-and-learn/spec.md
- docs/features/research-and-learn/tasks.md
- skills/agent-spec-authoring/SKILL.md
- skills/agent-spec-authoring/references/patterns.md
- skills/agent-spec-estimate/SKILL.md
- skills/agent-spec-estimate/references/examples.md
- skills/agent-spec-research/SKILL.md
- skills/agent-spec-sdd/SKILL.md
- skills/agent-spec-tool-first/SKILL.md
- skills/agent-spec-tool-first/references/commands.md
- src/spec_lint/linters.rs — pub struct VagueVerbLinter;
- src/spec_lint/mod.rs
- src/spec_lint/pipeline.rs — pub trait SpecLinter: Send + Sync {
- tests/doc_impact_guard_e2e.rs — Black-box E2E for the Doc Impact Guard: warnings surface on stderr and
- tests/finish_cli_e2e.rs — Black-box E2E for `finish`: graduation removes consumables only after the
- tests/integrate_cli_e2e.rs — Black-box E2E for `integrate`: skills for both agent systems, managed
- tests/repo_specs_english_only.rs — Regression guard: the repo's own spec corpus must use English structural
- tests/research_cli_e2e.rs — Black-box E2E for `research`: scaffold on first run, refresh only the
- tests/sdd_init_cli.rs

Test functions (8):
- src/spec_lint/linters.rs: test_lint_research_required_fires_on_unresolved_marker, test_lint_research_required_quiet_with_research_present, test_lint_research_uncited_warns, test_lint_research_unfilled_warns, test_lint_needs_clarification_marker_is_error, test_lint_needs_clarification_scans_comments_and_front_matter, test_lint_needs_clarification_ignores_documented_marker_examples, test_lint_open_questions_items_warn, test_lint_resolved_questions_none_is_quiet, test_vague_verb_linter, test_unquantified_linter, test_testability_linter, test_determinism_linter, test_full_pipeline, test_explicit_test_binding_linter_requires_task_scenario_selectors, test_sycophancy_linter_flags_bug_finding_bias, test_quality_report_scores_testability_and_smells, test_cross_check_reports_boundary_and_decision_conflicts, test_explicit_test_binding_linter_accepts_explicit_selector, test_scenario_presence_linter_requires_acceptance_criteria, test_decision_coverage_warns_on_uncovered_decision, test_decision_coverage_passes_when_all_covered, test_observable_decision_coverage_warns_when_behavioral_decisions_lack_scenarios, test_output_mode_coverage_warns_when_json_or_output_flags_are_uncovered, test_precedence_fallback_coverage_warns_when_ordered_behavior_has_no_scenario, test_external_io_error_strength_warns_on_weak_mock_only_http_scenarios, test_behavior_completeness_linters_do_not_flag_plain_implementation_choices, test_lint_suggests_verification_metadata_for_external_io_scenarios, test_error_path_warns_on_all_happy_paths, test_error_path_passes_with_error_scenario, test_error_path_detects_english_error_indicators, test_universal_claim_warns_single_scenario_for_all_entry_points, test_universal_claim_passes_with_multiple_scenarios, test_universal_claim_ignores_non_universal_decisions, test_boundary_entry_point_warns_uncovered_entry, test_boundary_entry_point_passes_all_covered, test_boundary_entry_point_ignores_single_entry, test_flag_combination_warns_when_multiple_flags_but_no_combo_scenario, test_flag_combination_passes_when_combo_scenario_exists, test_flag_combination_ignores_single_flag_specs, test_platform_tag_warns_on_untagged_npm_reference, test_platform_tag_passes_when_tagged, test_platform_tag_ignores_non_platform_decisions, test_scenario_presence_linter_rejects_empty_acceptance_criteria, test_lint_detects_circular_dependency, test_lint_no_circular_dependency_for_linear_chain, test_freeform_rule_emits_warning_and_does_not_group_scenarios, test_bdd_rule_grouping_suggests_when_three_or_more_scenarios_uncategorized, test_bdd_rule_grouping_warns_on_empty_rule, test_bdd_scenario_shape_flags_missing_when_or_then, test_bdd_scenario_shape_flags_leading_and_or_but, test_bdd_implementation_detail_flags_ui_verbs_en_and_zh, test_open_question_warns, test_resolved_question_not_warned, test_open_question_is_non_gating, test_new_bdd_lints_emit_self_correction_guidance
- src/spec_lint/pipeline.rs: test_lint_ack_moves_warning_to_acknowledged, test_lint_ack_leaves_other_diagnostics, test_lint_ack_cannot_suppress_error, test_ack_does_not_change_gating
- tests/doc_impact_guard_e2e.rs: test_cli_guard_doc_impact_warns_without_failing_e2e
- tests/finish_cli_e2e.rs: test_cli_finish_removes_plan_and_tasks_e2e, test_cli_finish_removes_research_consumables_e2e, test_cli_finish_refuses_on_failing_contract_e2e, test_cli_finish_retire_removes_goal_directory_e2e, test_cli_finish_bare_spec_reports_nothing_to_clean_e2e
- tests/integrate_cli_e2e.rs: test_cli_integrate_installs_skills_e2e, test_cli_integrate_installs_research_skill_e2e, test_cli_integrate_writes_policy_blocks_e2e, test_cli_integrate_preserves_and_refreshes_e2e, test_cli_init_prefills_vitest_binding_for_node_e2e, test_cli_init_omits_binding_for_cargo_e2e
- tests/repo_specs_english_only.rs: test_repo_specs_use_english_structural_keywords
- tests/research_cli_e2e.rs: test_cli_research_scaffolds_and_refreshes_e2e
- tests/sdd_init_cli.rs: test_cli_init_feature_creates_sdd_package_e2e, test_cli_init_architecture_creates_sdd_package_e2e, test_cli_init_issue_creates_single_spec_e2e, test_cli_init_sdd_package_refuses_partial_overwrite_e2e, test_cli_init_rejects_invalid_sdd_kind_e2e, test_cli_init_requires_kind_and_name_e2e, test_agent_spec_sdd_skill_routes_cli_workflow
<!-- agent-spec:generated:end -->

## Findings

### F1: Research trigger

- **Decision**: demand-driven, never universal. Triggers: (a) a
  `[NEEDS CLARIFICATION]` marker that cannot be resolved from the codebase
  alone; (b) the goal's domain is unfamiliar to the team.
- **Rationale**: unanimous across all five surveyed sources (DeepChat 1/148,
  spec-kit unknown-driven Phase 0, RFC "no prior art is fine", Pocock
  on-demand delegation, gsd orchestrator-dispatched).
- **Alternatives considered**: mandatory for all goals (rejected: no
  industry precedent, ritualizes the artifact); mandatory for
  feature/architecture only (rejected: still flat-rate, not need-based).

### F2: Artifact & format

- **Decision**: research.md in the goal folder; findings as
  Decision/Rationale/Alternatives with per-claim Source; codebase-state
  section machine-refreshed inside managed markers.
- **Rationale**: spec-kit owns the D/R/A precedent; Pocock/gsd own the
  citation discipline; our own integrate/doc-impact own the marker
  mechanics; plan's scanner already produces the codebase half.
- **Alternatives considered**: embedded spec.md section like Rust RFC
  (rejected: research is consumable, spec is durable — mixing lifecycles
  is the rot pattern we just eliminated); standalone docs/research/ dir
  like gsd (rejected: goal-scoped materials belong in the goal folder).

### F3: The learning half

- **Decision**: a grill session grounded in research.md — the agent puts
  each Finding's Decision to the user one at a time with a recommended
  answer; confirmed decisions flow into spec.md Decisions; spec authoring
  does not start before the grill completes.
- **Rationale**: Pocock's grilling ("decisions are mine") + teach
  (retrieval practice beats passive reading; fluency vs storage). A
  read-this-file checkbox is fake-able; an interview transcript is not.
- **Alternatives considered**: passive review gate checkbox (rejected:
  self-reported, the DeepChat drift pattern); quiz on facts (rejected:
  the user needs to own decisions, not recite trivia).

### F4: Methodology content

- **Decision**: a skill reference (research-methods.md) teaching: primary
  sources first; never trust parametric knowledge; per-claim citation; the
  gh toolkit (topic search → tree scan → file read → in-repo code search →
  issues/discussions for rationale); compare N implementations before
  concluding.
- **Rationale**: gsd encodes query recipes in researcher prompts; Pocock
  encodes the source discipline; our own ecosystem blind spots this round
  (missed 165k★ and 64.7k★ repos twice) prove parametric knowledge fails
  for fast-moving ecosystems.
- **Alternatives considered**: leave methodology implicit (rejected: the
  gap this feature exists to close).

### F5: Enforcement

- **Decision**: warning-level lint only when research.md exists
  (spec-does-not-cite-research; research-has-unfilled-placeholders).
  Grill completion is skill-owned, not lint-checkable.
- **Rationale**: absence is legitimate (RFC); escalate-after-noise-data is
  this repo's established gating policy; human interaction cannot be
  mechanically verified without theater.
- **Alternatives considered**: error-level (rejected: would force ritual
  research); no lint (rejected: cited-but-stale research would rot
  silently).
