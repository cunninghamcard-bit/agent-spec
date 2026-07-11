spec: task
name: "Workflow Refactor"
inherits: project
tags: [architecture, workflow, sdd]
---

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

### Allowed Changes
- Cargo.toml
- Cargo.lock
- AGENTS.md
- .cursorrules
- agent-spec-tool-first.md
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

### Forbidden
- Do not delete any CLI subcommand
- Do not weaken guard: the docs/ contracts must remain verified
- Do not add new dependencies

## Out of Scope

- Mining the 43 historical contracts into the capability library (rejected in record 0008; history lives in git)
- Hard-blocking multi-goal worktrees (rejected in grill round 4)
- Building new review-flow features beyond documenting the existing stations (explain/matrix ship as-is)
- Splitting main.rs or linters.rs (code-layout refactor, separate goal)

## Completion Criteria

Rule: staged-birth — artifacts are born by their workflow step

Scenario: init births only the contract
  Test:
    Package: docwright
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
    Package: docwright
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
    Package: docwright
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
    Package: docwright
    Filter: test_tool_first_reference_covers_all_subcommands
  Given the agent-spec-tool-first commands reference
  When the documented names are compared against the CLI's subcommand list
  Then every subcommand appears in the reference

Scenario: the sdd skill stations every flow
  Test:
    Package: docwright
    Filter: test_sdd_skill_documents_five_flows
  Given the embedded agent-spec-sdd skill
  When its content is inspected
  Then all five flow names are documented
  And the commit step names stamp and the graduation step names promote

Rule: single-household — docs/ is the one home for contracts and capabilities

Scenario: promote targets the docs household
  Test:
    Package: docwright
    Filter: test_cli_promote_targets_docs_capabilities_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given a goal spec with a passing task Rule
  When promote runs with a capability name
  Then the capability spec is created under docs/capabilities/

Scenario: install-hooks guards the docs household
  Test:
    Package: docwright
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
    Package: docwright
    Filter: test_cli_goal_spec_inherits_relocated_project_constitution_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given a goal spec declaring inherits: project and a docs/project.spec.md
  When parse runs on the goal spec
  Then the resolved document carries the project constitution's constraints

Scenario: guard collects only the docs home
  Test:
    Package: docwright
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
    Package: docwright
    Filter: test_cli_lifecycle_warns_on_multiple_dirty_goals_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given two goal folders with uncommitted changes in one worktree
  When lifecycle runs on one of them
  Then a warning naming the one-active-goal invariant is printed
  And the run is not blocked

## Open Questions

None.
