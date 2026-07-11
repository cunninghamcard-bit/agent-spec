=== Contract ===

# Task Contract: Rename And Publish

## Intent
`agent-spec` (the crate name) is unavailable: it is upstream
ZhangHanDong's own actively-published crate (0.3.0, published
2026-06-06), not a squattable name, per crates.io's own policy. The fork
has also diverged far enough — Doc Impact Guard, the single docs/
household, Research and Learn, Learning Archive, the capabilities library
— that its differentiation is documentation governance and learning
capture, not spec-checking. Rename to `docwright`, chosen for a validated
`-wright` craftsman-suffix pattern (the sibling fork
`BUNotesAI/specwright` solved the same name collision the same way) and
because it maps directly onto the docs/ household this project just
built. Prepare the crate for crates.io publish; the actual `cargo login`
+ `cargo publish` step is the user's to run (API tokens are never handled
by the agent). Grounded in research.md (F1–F5) and learning records
0001–0002.

## Current State
See research.md: the crates.io policy findings, the sibling-fork naming
precedent, and the full blast-radius survey (source strings, skill
directories, the `.agent-spec/` state directory, marker prefixes,
generated integration files, and tests referencing
`CARGO_BIN_EXE_agent-spec`). Cargo.toml's `repository`/`homepage` also
currently point at upstream's repo, not this fork's — a pre-existing bug
independent of the rename.

## Must
- 任务合约以 goal 包形式存放在 `docs/features|issues|architecture/<goal>/`（单一户口）
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
- 耐久能力规则经 `promote` 累积于 `docs/capabilities/`（真相层）；历史从 git 恢复，不设归档目录
- Task Contract 应区分 `Must`、`Must Not` 与 `Decisions`
- 默认文本 `contract` 输出应保留结构化 Completion Criteria 细节

## Must NOT
- 不要把 `skip` 记为 `pass`
- 不要要求普通磁盘用例手工提供继承搜索路径
- 不要丢弃 BDD 步骤里的结构化输入

## Decisions
- New identity: `docwright` (research.md F4, record 0001)
- Full rename depth (research.md F5, record 0002): crate/binary/CLI-facing text, skill directory names (`skills/agent-spec-*` -> `skills/docwright-*`, mirrored in `.claude/skills/`), the `.agent-spec/` state directory -> `.docwright/`, and the `agent-spec:*` marker prefix (`governs`, `generated`, `integration`) -> `docwright:*`
- Cargo.toml gets `repository`/`homepage` pointing at this fork's actual GitHub URL (fixing the pre-existing bug), plus `readme`, `keywords`, and `categories` fields (upstream's crate has none set; ours should for crates.io discoverability)
- Generated integration files (`AGENTS.md`, `.cursorrules`, the Claude skill-content file — renamed from `agent-spec-tool-first.md` to `docwright-tool-first.md`) are regenerated via `gen-integrations` from the renamed single source, never hand-edited
- CHANGELOG.md: only the evergreen header line renames; every dated historical entry is left untouched (it documents what shipped under the old name at the time); a new entry documents the rename itself
- Historical/frozen documents are out of scope for the rename sweep: `docs/phase-*-retrospective.md`, `docs/bdd-spine-end-state.md`, `docs/comparison-openspec-speckit.md`, `docs/behavior-contract-improvement-proposal.md`, `docs/jj-vcs-integration-guide.md`, `docs/superpowers/**`, `docs/learning/**` (archived), and already-graduated maintained goal contracts under `docs/architecture/{doc-impact-guard,finish-command,workflow-refactor}/`, `docs/features/{integrate-command,research-and-learn,learning-archive}/`, `docs/issues/parser-fence-blindness/` — their inline `agent-spec <cmd>` prose is historical narrative, not a live citation
- `docs/project.spec.md` (the living constitution) is in scope like any other currently-maintained document
- Renaming the GitHub repository itself is explicitly out of this contract's automated Boundaries — it is a shared/visible action requiring separate confirmation outside the agent's normal write path
- `cargo publish` itself is out of scope for automated execution: this goal prepares and validates the package (`cargo publish --dry-run`); the user runs `cargo login` and `cargo publish` themselves

## Boundaries
Allowed changes:
- Cargo.toml
- Cargo.lock
- src/**
- skills/**
- .claude/skills/**
- install-skills.sh
- README.md
- CHANGELOG.md
- AGENTS.md
- .cursorrules
- agent-spec-tool-first.md
- docwright-tool-first.md
- docs/project.spec.md
- examples/**
- tests/**
- docs/architecture/rename-and-publish/**
Forbidden:
- Do not rewrite historical/frozen documents (retrospectives, comparison docs, docs/learning/**, already-graduated goal contracts) to the new name
- Do not rewrite past dated CHANGELOG entries
- Do not run `cargo login` or `cargo publish` (or handle any crates.io API token)
- Do not rename the GitHub repository
Out of scope:
- The actual `cargo publish` execution (user's action)
- Renaming the GitHub repository (separate, explicitly-confirmed action)
- Updating historical/frozen documents to the new name

## Completion Criteria

Rule: identity — the CLI and package present as docwright everywhere live
Scenario: the binary reports its new identity
  Test:
    Package: docwright
    Filter: test_cli_binary_reports_docwright_identity_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process
  Given the installed CLI
  When --version and --help run
  Then the binary name is docwright
  And no output contains the string agent-spec

Scenario: install-hooks guards under the new name
  Test:
    Package: docwright
    Filter: test_cli_install_hooks_uses_docwright_identity_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given a git repository
  When install-hooks runs
  Then the generated pre-commit hook invokes docwright, not agent-spec


Rule: skills — the skill package matches the new identity
Scenario: skill directories carry the new name
  Test:
    Package: docwright
    Filter: test_docwright_sdd_skill_routes_cli_workflow
    Level: unit
    Test Double: none
    Targets: filesystem
  Given the embedded and mirrored skill directories
  When their paths and content are inspected
  Then every skill directory is named docwright-<role>
  And the skills/ and .claude/skills/ copies are identical

Scenario: no leftover old identity in generated integration files
  Test:
    Package: docwright
    Filter: test_cli_integrate_generated_files_carry_new_identity_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given an empty target project
  When integrate runs
  Then AGENTS.md, .cursorrules, and docwright-tool-first.md all reference docwright
  And none of them contain the string agent-spec

=== Codebase Context ===

Files (66):
  - .claude/skills/agent-spec-authoring/SKILL.md
  - .claude/skills/agent-spec-authoring/references/patterns.md
  - .claude/skills/agent-spec-estimate/SKILL.md
  - .claude/skills/agent-spec-estimate/references/examples.md
  - .claude/skills/agent-spec-research/SKILL.md
  - .claude/skills/agent-spec-sdd/SKILL.md
  - .claude/skills/agent-spec-tool-first/SKILL.md
  - .claude/skills/agent-spec-tool-first/references/commands.md
  - docs/architecture/rename-and-publish/learning-records/0001-new-identity.md
  - docs/architecture/rename-and-publish/learning-records/0002-rename-depth.md
  - docs/architecture/rename-and-publish/research.md
  - docs/architecture/rename-and-publish/spec.md
  - examples/README.md
  - examples/no-unwrap.spec
  - examples/refactor-payment-service.spec
  - examples/refund.spec
  - examples/rewrite-parity-contract.spec
  - examples/user-registration-contract.spec
  - skills/agent-spec-authoring/SKILL.md
  - skills/agent-spec-authoring/references/patterns.md
  - skills/agent-spec-estimate/SKILL.md
  - skills/agent-spec-estimate/references/examples.md
  - skills/agent-spec-research/SKILL.md
  - skills/agent-spec-sdd/SKILL.md
  - skills/agent-spec-tool-first/SKILL.md
  - skills/agent-spec-tool-first/references/commands.md
  - src/doc_impact.rs — Doc Impact Guard: a mechanical freshness signal for maintained docs.
  - src/main.rs
  - src/spec_core/ast.rs — pub enum SpecLevel {
  - src/spec_core/error.rs — pub enum SpecError {
  - src/spec_core/lint.rs — pub enum Severity {
  - src/spec_core/mod.rs
  - src/spec_core/verify.rs — pub enum Verdict {
  - src/spec_gateway/brief.rs — pub struct TaskContract {
  - src/spec_gateway/lifecycle.rs — pub struct SpecGateway {
  - src/spec_gateway/mod.rs
  - src/spec_gateway/plan.rs — pub struct PlanContext {
  - src/spec_lint/linters.rs — pub struct VagueVerbLinter;
  - src/spec_lint/mod.rs
  - src/spec_lint/pipeline.rs — pub trait SpecLinter: Send + Sync {
  - src/spec_parser/keywords.rs — pub fn match_step_keyword(line: &str) -> Option<(StepKind, &str)> {
  - src/spec_parser/meta.rs — pub fn parse_meta(lines: &[&str]) -> Result<SpecMeta, String> {
  - src/spec_parser/mod.rs
  - src/spec_parser/parser.rs — pub fn parse_spec(path: &Path) -> SpecResult<SpecDocument> {
  - src/spec_parser/resolver.rs — pub fn resolve_spec(doc: SpecDocument, search_dirs: &[&Path]) -> SpecResult<ResolvedSpec> {
  - src/spec_report/audit.rs — Spec-library health audit (Phase 8): mechanical aggregation across a whole
  - src/spec_report/coverage.rs — Coverage matrix (Phase 2): Rule × Scenario × Test × Verdict × Provenance.
  - src/spec_report/discover.rs — Cold-start reverse engineering (Phase 9): draft a task-spec skeleton from a
  - src/spec_report/integrations.rs — Single-source multi-tool integration generation (Phase 6).
  - src/spec_report/mod.rs — pub enum OutputFormat {
  - src/spec_report/structural.rs — Mechanical structural checks (Phase 7): dependency-cruiser-lite layering /
  - src/spec_verify/ai_verifier.rs — pub struct AiVerifier {
  - src/spec_verify/boundaries.rs — pub struct BoundariesVerifier {
  - src/spec_verify/complexity.rs — pub struct ComplexityVerifier;
  - src/spec_verify/mod.rs
  - src/spec_verify/report_mode.rs — Report-driven verification (report mode).
  - src/spec_verify/structural.rs — pub struct StructuralVerifier;
  - src/spec_verify/test_verifier.rs — pub struct TestVerifier;
  - src/vcs.rs — VCS context detection and retrieval.
  - tests/doc_impact_guard_e2e.rs — Black-box E2E for the Doc Impact Guard: warnings surface on stderr and
  - tests/finish_cli_e2e.rs — Black-box E2E for `finish`: graduation removes consumables only after the
  - tests/integrate_cli_e2e.rs — Black-box E2E for `integrate`: skills for both agent systems, managed
  - tests/repo_specs_english_only.rs — Regression guard: the repo's own spec corpus must use English structural
  - tests/research_cli_e2e.rs — Black-box E2E for `research`: scaffold on first run, refresh only the
  - tests/sdd_init_cli.rs
  - tests/workflow_refactor_e2e.rs — Black-box E2E for the workflow refactor: staged artifact birth, the

Test functions (35):
  src/doc_impact.rs:
    - test_doc_impact_parses_governs_marker
    - test_doc_impact_warns_when_governed_path_changes
    - test_doc_impact_quiet_when_doc_updated_together
    - test_doc_impact_quiet_without_overlap
    - test_doc_impact_discovery_scans_root_and_docs
  src/main.rs:
    - test_promote_refuses_rule_with_no_examples
    - test_promote_rule_name_excludes_provenance_comment
    - test_promote_rejects_unsafe_capability_name
    - test_rule_id_of_matches_parser_on_double_space_before_em_dash
    - test_promote_appends_under_completion_criteria
    - test_promote_unknown_rule_id_errors
    - test_promote_refuses_when_an_example_fails
    - test_promote_appends_rule_when_examples_pass
    - test_promote_is_idempotent_for_same_rule
    - test_promote_does_not_change_is_passing
    - test_matrix_command_runs_verification_in_default_mode
    - test_explain_markdown_embeds_coverage_matrix
    - test_merge_ai_decisions_only_replaces_skip
    - test_provenance_resolve_ai_is_inferential
    - test_brief_output_matches_contract_output
    - test_resolve_guard_change_paths_prefers_explicit_changes
    - test_resolve_guard_change_paths_reads_staged_git_changes
    - test_repo_root_discovery_ignores_process_cwd
    - test_resolve_guard_change_paths_returns_empty_outside_git_repo
    - test_resolve_guard_change_paths_reads_worktree_git_changes
    - test_resolve_guard_change_paths_ignores_unstaged_changes_in_default_staged_scope
    - test_parse_ai_mode_accepts_stub
    - test_resolve_command_change_paths_prefers_explicit_changes
    - test_resolve_command_change_paths_returns_empty_for_none_scope
    - test_resolve_command_change_paths_reads_worktree_git_changes
    - test_claude_code_tool_first_skill_exists_and_mentions_contract_lifecycle_guard
    - test_claude_code_authoring_skill_exists_and_mentions_task_contract_sections
    - test_authoring_skill_includes_behavior_surface_checklist
    - test_tool_first_skill_mentions_unbound_observable_behavior_review_step
    - test_rewrite_parity_example_spec_exists_and_covers_behavior_matrix
    - test_guard_collects_specs_from_multiple_dirs
    - test_plan_out_writes_rendered_output_to_file
    - test_readme_documents_claude_code_tool_first_skills
    - test_readme_documents_rewrite_parity_contract_authoring_guidance
    - test_contract_output_preserves_step_tables_and_test_selectors
    - test_contract_and_json_output_preserve_verification_metadata
    - test_explain_command_renders_contract_review_summary
    - test_explain_markdown_output_is_suitable_for_pr_description
    - test_stamp_dry_run_outputs_trailers_without_rewriting_history
    - test_lifecycle_writes_structured_run_log_summary
    - test_explain_history_reads_run_log_summary
    - test_resolve_command_change_paths_reads_jj_changes
    - test_adversarial_verification_is_disabled_by_default
    - test_additional_agent_integration_templates_exist
    - test_checkpoint_commands_are_optional_and_vcs_aware
    - test_lifecycle_layers_flag_selects_verification_stack
    - test_measure_determinism_is_explicitly_experimental
    - test_stamp_trailers_include_jj_change_id
    - test_stamp_trailers_omit_change_id_for_git
    - test_run_log_entry_serialises_vcs_context
    - test_run_log_entry_without_vcs_is_backward_compatible
    - test_explain_history_shows_jj_diff_between_runs
    - test_explain_history_degrades_without_jj
    - test_parse_ai_mode_accepts_caller
    - test_resolve_ai_command_parses_correctly
    - test_scenario_ai_decision_serialization_roundtrip
    - test_guard_discovers_spec_md_files
    - test_guard_discovers_both_spec_and_spec_md
    - test_boundary_checker_recognizes_spec_md
    - test_spec_md_not_matched_by_extension_alone
    - test_plain_md_files_not_matched_as_spec
    - test_guard_collects_fixed_goal_spec_filename
    - test_lint_warns_on_duplicate_spec_extensions
    - test_resume_incremental_skips_passed_scenarios
    - test_resume_conservative_detects_regression
    - test_resume_without_run_log_dir_errors
    - test_checkpoint_roundtrip_serialization
    - test_load_checkpoint_returns_none_when_missing
    - test_graph_generates_dot_output
    - test_graph_nodes_include_estimate
    - test_graph_independent_specs_are_isolated_nodes
    - test_graph_critical_path_highlighted
    - test_tool_first_reference_covers_all_subcommands
    - test_sdd_skill_documents_five_flows
  src/spec_core/ast.rs:
    - test_probe_kind_label_test
    - test_probe_kind_label_reserved_variants
    - test_probe_roundtrips
  src/spec_core/lint.rs:
    - test_dimension_of_known_rules
    - test_dimension_of_unknown_falls_back
  src/spec_core/verify.rs:
    - test_json_provenance_additive_only
    - test_rule_events_additive_empty_by_default
    - test_rule_event_roundtrips
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
  src/spec_lint/linters.rs:
    - test_lint_research_required_fires_on_unresolved_marker
    - test_lint_research_required_quiet_with_research_present
    - test_lint_research_uncited_warns
    - test_lint_research_unfilled_warns
    - test_lint_needs_clarification_marker_is_error
    - test_lint_needs_clarification_scans_comments_and_front_matter
    - test_lint_needs_clarification_ignores_documented_marker_examples
    - test_lint_open_questions_items_warn
    - test_lint_resolved_questions_none_is_quiet
    - test_vague_verb_linter
    - test_unquantified_linter
    - test_testability_linter
    - test_determinism_linter
    - test_full_pipeline
    - test_explicit_test_binding_linter_requires_task_scenario_selectors
    - test_sycophancy_linter_flags_bug_finding_bias
    - test_quality_report_scores_testability_and_smells
    - test_cross_check_reports_boundary_and_decision_conflicts
    - test_explicit_test_binding_linter_accepts_explicit_selector
    - test_scenario_presence_linter_requires_acceptance_criteria
    - test_decision_coverage_warns_on_uncovered_decision
    - test_decision_coverage_passes_when_all_covered
    - test_observable_decision_coverage_warns_when_behavioral_decisions_lack_scenarios
    - test_output_mode_coverage_warns_when_json_or_output_flags_are_uncovered
    - test_precedence_fallback_coverage_warns_when_ordered_behavior_has_no_scenario
    - test_external_io_error_strength_warns_on_weak_mock_only_http_scenarios
    - test_behavior_completeness_linters_do_not_flag_plain_implementation_choices
    - test_lint_suggests_verification_metadata_for_external_io_scenarios
    - test_error_path_warns_on_all_happy_paths
    - test_error_path_passes_with_error_scenario
    - test_error_path_detects_english_error_indicators
    - test_universal_claim_warns_single_scenario_for_all_entry_points
    - test_universal_claim_passes_with_multiple_scenarios
    - test_universal_claim_ignores_non_universal_decisions
    - test_boundary_entry_point_warns_uncovered_entry
    - test_boundary_entry_point_passes_all_covered
    - test_boundary_entry_point_ignores_single_entry
    - test_flag_combination_warns_when_multiple_flags_but_no_combo_scenario
    - test_flag_combination_passes_when_combo_scenario_exists
    - test_flag_combination_ignores_single_flag_specs
    - test_platform_tag_warns_on_untagged_npm_reference
    - test_platform_tag_passes_when_tagged
    - test_platform_tag_ignores_non_platform_decisions
    - test_scenario_presence_linter_rejects_empty_acceptance_criteria
    - test_lint_detects_circular_dependency
    - test_lint_no_circular_dependency_for_linear_chain
    - test_freeform_rule_emits_warning_and_does_not_group_scenarios
    - test_bdd_rule_grouping_suggests_when_three_or_more_scenarios_uncategorized
    - test_bdd_rule_grouping_warns_on_empty_rule
    - test_bdd_scenario_shape_flags_missing_when_or_then
    - test_bdd_scenario_shape_flags_leading_and_or_but
    - test_bdd_implementation_detail_flags_ui_verbs_en_and_zh
    - test_open_question_warns
    - test_resolved_question_not_warned
    - test_open_question_is_non_gating
    - test_new_bdd_lints_emit_self_correction_guidance
  src/spec_lint/pipeline.rs:
    - test_lint_ack_moves_warning_to_acknowledged
    - test_lint_ack_leaves_other_diagnostics
    - test_lint_ack_cannot_suppress_error
    - test_ack_does_not_change_gating
  src/spec_parser/keywords.rs:
    - test_match_step_chinese_rejected
    - test_match_step_english
    - test_scenario_header_chinese_rejected
    - test_scenario_header_english
    - test_scenario_header_example_alias_english_only
    - test_match_rule_header
    - test_scenario_header_accepts_markdown_heading
    - test_extract_params
    - test_extract_params_chinese_quotes
    - test_match_test_selector_chinese_rejected
    - test_match_test_selector_fields_support_verification_metadata
    - test_match_test_selector_english
    - test_match_test_selector_accepts_markdown_heading
    - test_match_test_selector_field_chinese_rejected
    - test_match_test_selector_field_english
    - test_match_test_selector_field_accepts_markdown_heading
    - test_section_headers
    - test_section_headers_chinese_rejected
    - test_not_a_step
  src/spec_parser/meta.rs:
    - test_parse_basic_meta
    - test_parse_minimal_meta
    - test_parse_spec_depends_and_estimate_fields
    - test_parse_meta_multiple_depends
    - test_parse_meta_no_depends_no_estimate
    - test_parse_capability_spec_level
    - test_unknown_spec_level_rejected
    - test_parse_task_capability_field
  src/spec_parser/parser.rs:
    - test_parse_full_spec
    - test_parse_english_spec
    - test_parse_mixed_lang_keywords_rejected
    - test_parse_step_table_and_preserve_json_output
    - test_parse_scenario_without_table_stays_unchanged
    - test_parse_task_contract_sections
    - test_parse_scenario_with_explicit_test_selector
    - test_parse_structured_test_selector_block
    - test_parse_scenario_verification_metadata_fields
    - test_parse_english_verification_metadata_fields
    - test_existing_specs_without_verification_metadata_remain_valid
    - test_parse_shorthand_test_selector_as_filter_only
    - test_missing_front_matter
    - test_unknown_top_level_section_header_is_rejected
    - test_markdown_heading_scenarios_and_test_selectors_are_accepted
    - test_serialization_roundtrip
    - test_parse_mode_field_in_scenario
    - test_parse_mode_field_english
    - test_parse_mode_field_standard_is_default
    - test_parse_depends_field_in_scenario
    - test_parse_depends_field_multiple
    - test_parse_rule_header_creates_behavior_rule
    - test_parse_rule_header_without_display_name
    - test_chinese_rule_alias_rejected
    - test_parse_example_alias_as_scenario
    - test_legacy_spec_without_rule_compat
    - test_rule_scope_serializes_to_json
    - test_capability_scope_is_reserved_in_v1
    - test_json_output_additive_only
    - test_rule_double_space_separator_with_em_dash_in_display
    - test_rule_em_dash_separator_with_double_space_in_display
    - test_stray_step_after_rule_header_does_not_leak_into_prior_scenario
    - test_fullwidth_colon_english_scenario_header_is_recognized
    - test_fullwidth_colon_english_rule_header_is_recognized
    - test_probe_from_scenario_with_selector
    - test_probe_from_scenario_without_selector
    - test_scenario_unchanged_no_probe_field
    - test_parse_lint_ack_marker
    - test_lint_acks_additive_empty
    - test_parse_questions_section
    - test_chinese_questions_header_rejected
    - test_spec_without_questions_unaffected
    - test_questions_do_not_affect_verification
    - test_capability_spec_rule_has_capability_scope
    - test_task_without_capability_is_none_additive
    - test_parse_review_field_in_scenario
    - test_parser_ignores_headers_inside_fences
    - test_parser_ignores_scenario_keywords_inside_fences
    - test_parser_accepts_current_state_section
    - test_parser_accepts_ux_shape_section
    - test_parser_accepts_open_questions_header_alias
    - test_parser_rejects_cjk_section_header_with_suggestion
    - test_parser_rejects_cjk_scenario_keyword
    - test_parser_rejects_cjk_step_keyword
    - test_parser_rejects_cjk_test_selector_keys
    - test_parser_keeps_chinese_free_text
  src/spec_parser/resolver.rs:
    - resolves_parent_from_source_directory_when_no_search_dirs_are_provided
    - resolves_parent_from_nested_spec_directory_via_ancestor_specs_dir
    - test_resolver_prefers_spec_md_over_spec
    - test_resolver_falls_back_to_spec_when_no_spec_md
    - test_resolver_errors_when_no_spec_or_spec_md_found
  src/spec_report/audit.rs:
    - test_audit_counts_specs_rules_scenarios
    - test_audit_counts_unproven_rules
    - test_audit_counts_ungrouped_scenarios
    - test_audit_counts_open_questions
    - test_audit_counts_malformed_rules
    - test_audit_empty_library
    - test_audit_json_serializes
  src/spec_report/coverage.rs:
    - test_matrix_has_one_row_per_scenario
    - test_matrix_flags_dangling_selector_as_missing
    - test_matrix_test_found_requires_exact_function_name
    - test_matrix_marks_scenario_without_selector_as_none
    - test_matrix_ungrouped_scenario_rule_column_is_dash
    - test_matrix_derives_inferential_from_ai_evidence
    - test_matrix_markdown_renders_table
    - test_matrix_json_is_machine_parseable
    - test_to_markdown_escapes_pipe_in_cells
    - test_scanner_ignores_tokio_test_in_comment_and_string
    - test_scanner_collects_single_line_test_fn
    - test_scanner_ignores_block_commented_test
    - test_matrix_includes_orphan_report_rows
  src/spec_report/discover.rs:
    - test_draft_creates_scenario_per_test
    - test_draft_is_parseable
    - test_draft_empty_tests_is_parseable
    - test_draft_includes_questions_seed
    - test_draft_scenario_names_derive_from_tests
  src/spec_report/integrations.rs:
    - test_integration_policy_block_is_prose_only
    - test_upsert_managed_block_appends_and_replaces
    - test_all_targets_share_integration_body
    - test_integration_body_is_tool_first
    - test_claude_target_has_frontmatter
    - test_agents_target_is_plain_markdown
    - test_unknown_target_errors
    - test_check_passes_when_content_matches
    - test_check_reports_drift_when_different
  src/spec_report/mod.rs:
    - test_format_verification_text
    - test_format_verification_text_includes_ai_analysis_evidence
    - test_format_verification_text_includes_test_binding_metadata
    - test_report_json_exposes_contract_and_verification_summary_for_orchestrators
    - test_cost_report_breaks_down_tokens_time_and_layers
    - test_status_file_writes_success_on_all_pass
    - test_status_file_writes_partial_success_on_mixed
    - test_status_file_outcome_reflects_gate_blocked
    - test_compact_format_outputs_single_line_summary
    - test_diagnostic_format_includes_raw_test_output
  src/spec_report/structural.rs:
    - test_structural_flags_forbidden_reference
    - test_structural_no_violation_returns_empty
    - test_structural_respects_glob_scope
    - test_structural_skips_target_dir
    - test_check_structure_reports_violations
  src/spec_verify/ai_verifier.rs:
    - test_stub_ai_backend_returns_uncertain_decision
    - test_ai_verifier_with_custom_backend_uses_backend_response
    - test_build_ai_request_includes_scenario_and_code_paths
    - test_build_ai_request_includes_contract_change_set_and_evidence_context
  src/spec_verify/boundaries.rs:
    - matches_double_star_path_patterns
    - test_boundaries_verifier_accepts_changes_within_allowed_paths
    - test_boundaries_verifier_rejects_change_outside_allowed_paths
    - test_boundaries_verifier_rejects_change_matching_forbidden_boundary
    - test_guard_boundaries_ignore_allowed_coverage
    - test_guard_boundaries_still_enforce_forbidden
    - test_single_spec_boundaries_keep_allowed_coverage
    - test_boundary_bare_manifest_filenames_are_path_boundaries
    - test_boundary_bare_dotted_filenames_are_path_boundaries
    - test_boundary_absolute_change_paths_relativized_against_workspace_root
    - verifier_skips_when_no_explicit_change_paths_are_provided
  src/spec_verify/complexity.rs:
    - test_complexity_verifier_fails_on_line_ratio_exceeded
    - test_complexity_verifier_silent_without_constraints
    - test_complexity_verifier_passes_on_acceptable_ratio
    - test_complexity_verifier_uses_git_diff_stats
    - test_parse_line_ratio_chinese
    - test_parse_line_ratio_english
    - test_parse_line_ratio_no_match
    - test_no_constraints_no_change_paths
  src/spec_verify/mod.rs:
    - run_verification_keeps_first_result_for_same_scenario
    - test_provenance_test_verifier_is_computational
    - test_provenance_ai_verifier_is_inferential
  src/spec_verify/report_mode.rs:
    - test_report_mode_passes_when_named_testcase_passes
    - test_report_mode_fails_with_failure_evidence
    - test_report_mode_fails_when_selector_not_in_report
    - test_report_mode_skipped_testcase_is_not_pass
    - test_report_mode_ambiguous_match_fails_with_candidates
    - test_report_mode_missing_report_fails_with_actionable_error
    - test_report_mode_package_filters_by_classname_prefix
    - test_report_mode_substitutes_selectors_placeholder
    - test_report_mode_escapes_regex_metacharacters_in_selectors
    - test_specs_without_test_command_keep_cargo_path
  src/spec_verify/structural.rs:
    - keeps_code_like_backtick_patterns
    - ignores_plain_language_backtick_words
    - only_checks_explicit_structural_must_not_rules
  src/spec_verify/test_verifier.rs:
    - extracts_spec_bindings_from_test_comments
    - test_example
    - ignores_comments_not_followed_by_a_test
    - test_explicit_scenario_selector_takes_precedence_over_legacy_comment_binding
    - test_legacy_comment_binding_is_used_when_no_explicit_selector_exists
    - test_build_cargo_test_command_with_package_selector
  src/vcs.rs:
    - test_vcs_detect_prefers_jj_when_colocated
    - test_vcs_detect_returns_git_when_only_git
    - test_vcs_detect_returns_none_outside_repo
    - test_vcs_context_returns_jj_ids
    - test_vcs_context_returns_git_hash
  tests/doc_impact_guard_e2e.rs:
    - test_cli_guard_doc_impact_warns_without_failing_e2e
  tests/finish_cli_e2e.rs:
    - test_cli_finish_removes_plan_and_tasks_e2e
    - test_cli_finish_archives_research_and_learning_e2e
    - test_cli_finish_refuses_archive_collision_e2e
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
    - test_cli_init_feature_creates_spec_only_e2e
    - test_cli_init_architecture_creates_sdd_package_e2e
    - test_cli_init_issue_creates_single_spec_e2e
    - test_cli_init_sdd_package_refuses_partial_overwrite_e2e
    - test_cli_init_rejects_invalid_sdd_kind_e2e
    - test_cli_init_requires_kind_and_name_e2e
    - test_agent_spec_sdd_skill_routes_cli_workflow
  tests/workflow_refactor_e2e.rs:
    - test_cli_plan_out_births_plan_and_tasks_e2e
    - test_cli_help_groups_commands_by_flow_e2e
    - test_cli_promote_targets_docs_capabilities_e2e
    - test_cli_install_hooks_targets_docs_home_e2e
    - test_cli_goal_spec_inherits_relocated_project_constitution_e2e
    - test_cli_guard_collects_docs_contracts_e2e
    - test_cli_lifecycle_warns_on_multiple_dirty_goals_e2e
    - test_cli_guard_skips_capability_specs_e2e

=== Task Sketch ===

Group 1 (order 1):
  Scenarios:
    - the binary reports its new identity
    - install-hooks guards under the new name
    - skill directories carry the new name
    - no leftover old identity in generated integration files
  Boundary paths:
    - skills/**
    - .claude/skills/**
  Test selectors:
    - test_cli_binary_reports_docwright_identity_e2e
    - test_cli_install_hooks_uses_docwright_identity_e2e
    - test_docwright_sdd_skill_routes_cli_workflow
    - test_cli_integrate_generated_files_carry_new_identity_e2e

