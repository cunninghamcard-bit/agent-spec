spec: task
name: "DeepChat-style SDD documentation package"
inherits: project
tags: [cli, init, sdd, documentation]
---

## Intent

Make `agent-spec` create the first complete DeepChat-style documentation
package for a substantial goal. The CLI, rather than copied project-specific
AGENTS instructions, owns the goal classification, directory layout, and
starter Markdown. Feature and architecture goals receive readable `spec.md`,
`plan.md`, and `tasks.md` files; issue goals keep DeepChat's lightweight
single-file form. The generated `spec.md` remains an agent-spec Task Contract,
so documentation and mechanical verification share one source of truth.

## Current State

Before this task, `agent-spec init` created one `<name>.spec.md` file in the current directory.
The README describes DeepChat-style goal folders and `plan --out`, but the CLI
does not create a goal folder, classify work, scaffold `plan.md` or `tasks.md`,
or recognize the fixed goal filename `spec.md` during guard collection.

### DeepChat reference baseline

This task is a generic adaptation of DeepChat commit
`18820651410e3d4407c3e5be4e693f19de6cc030`, specifically:

- `.agents/skills/deepchat-sdd/SKILL.md` for SDD applicability, goal classification, required artifacts, workflow, and documentation hygiene
- `.agents/skills/deepchat-sdd-cleanup/SKILL.md` for the later completion-governance model
- `AGENTS.md` and `docs/spec-driven-dev.md` for repository-level policy and the feature/issue/architecture layouts
- Representative `docs/features/**`, `docs/issues/**`, and `docs/architecture/**` goal packages for the actual `spec.md` / `plan.md` / `tasks.md` shapes

The relationship is adaptation rather than vendoring: DeepChat supplies the
workflow model, while agent-spec makes the reusable parts CLI-driven,
provider-agnostic, BDD-native, and mechanically verified. DeepChat-specific
Presenter, Vue, i18n, pnpm, release, and GitHub-label rules do not become
generic agent-spec policy.

## UX Shape

```plantuml
@startuml
actor Developer
participant "agent-spec init" as CLI
folder "docs/features/plugins-hub" as Goal

Developer -> CLI: --kind feature --name "Plugins Hub"
CLI -> Goal: create spec.md
CLI -> Goal: create plan.md
CLI -> Goal: create tasks.md
CLI --> Developer: print created package and next command
@enduml
```

```text
docs/
├── features/<goal>/
│   ├── spec.md
│   ├── plan.md
│   └── tasks.md
├── issues/<goal>/
│   └── spec.md
└── architecture/<goal>/
    ├── spec.md
    ├── plan.md
    └── tasks.md
```

## Decisions

- Replace the existing `init` surface instead of keeping parallel legacy and goal-package workflows
- Require `--kind feature|issue|architecture` and `--name <goal>`; missing or invalid values fail through clap before filesystem changes
- Remove the legacy `--level`, `--lang`, and `--template` init options; project/org scaffolding and rewrite-parity profiles may return later only through an explicitly designed goal-oriented surface
- Add `--root <path>` for SDD packages, defaulting to `docs`
- Derive a lowercase kebab-case goal directory from `--name`
- Generate feature packages under `<root>/features/<goal>/` and architecture packages under `<root>/architecture/<goal>/`, each with `spec.md`, `plan.md`, and `tasks.md`
- Generate issue packages under `<root>/issues/<goal>/` with one comprehensive `spec.md`, matching DeepChat's lightweight issue convention
- Treat a file whose basename is exactly `spec.md` as an agent-spec file when explicitly passed or collected from a goal directory; existing `.spec` and `.spec.md` support remains unchanged
- Keep `spec.md` human-readable and machine-verifiable: it uses the existing Task Contract frontmatter and supported sections, and its `UX Shape` starter contains a fenced PlantUML example
- `plan.md` contains implementation approach, affected interfaces, data flow, compatibility, test strategy, and risks, including a fenced PlantUML placeholder
- `tasks.md` contains review, implementation, test, documentation-impact, lifecycle, and finish checklists, with instructions to link tasks to Scenario names or test selectors
- Refuse to overwrite any existing goal artifact; package creation is all-or-nothing
- Print the created paths and the next `agent-spec lint <goal>/spec.md` command after success
- Treat black-box CLI E2E tests as the primary acceptance evidence: invoke the compiled `agent-spec` binary in a temporary workspace and assert exit status, user-visible output, filesystem results, and generated content
- Delete legacy single-file template generators and their tests rather than retain unreachable compatibility code
- Keep focused unit tests only for pure file-classification helpers that are not more meaningfully proven through the public CLI
- Add a dedicated `agent-spec-sdd` skill as the thin workflow orchestrator: decide whether SDD is warranted, classify the goal, invoke `init --kind`, preserve `spec.md` authority over plan/tasks, and drive implementation toward BDD scenarios proven by public CLI E2E tests
- Preserve these behaviors from DeepChat's `deepchat-sdd` skill: skip trivial work, classify substantial goals as feature/issue/architecture, use one kebab-case folder per goal, create the kind-specific artifact set, resolve ambiguity before implementation, update tasks while work lands, and keep active versus maintained documentation distinct
- Replace DeepChat project-specific quality commands and architecture rules with agent-spec `lint`, `contract`, `lifecycle`, and `guard` gates
- Map DeepChat's separate `deepchat-sdd-cleanup` responsibility to the future CLI `finish` workflow; do not create `agent-spec-sdd-cleanup` in this task
- Do not copy DeepChat's optional GitHub issue synchronization and label policy into the generic SDD skill in this task
- Keep `agent-spec-tool-first` focused on exact CLI execution, verdicts, and retry behavior; add only the new command syntax and a pointer to `agent-spec-sdd`
- Keep `agent-spec-authoring` focused on writing the readable, machine-verifiable `spec.md`; add only the fixed goal filename, PlantUML-in-Markdown guidance, and E2E-first BDD authoring rule
- Ship matching `skills/agent-spec-sdd/SKILL.md` and `.claude/skills/agent-spec-sdd/SKILL.md` files without adding another reference hierarchy or a cleanup skill
- Update the canonical integration body used by `gen-integrations`; generated AGENTS/Cursor/Claude guidance must point agents to the CLI workflow instead of duplicating the full SDD policy
- Upgrade the `needs-clarification` lint to source-level scanning: `SpecDocument` retains the original text so markers in comments and front-matter are caught, additional marker spellings are recognized, and inline-code spans are exempt so documentation may quote the marker
- Root-cure the boundary path heuristic: any whitespace-free token containing a dot is a path boundary (covers `.cursorrules`, `install-skills.sh`); extensionless bare names like `Makefile` must be written as `./Makefile`

## Boundaries

### Allowed Changes
- src/main.rs
- src/spec_report/integrations.rs
- src/spec_core/ast.rs
- src/spec_parser/parser.rs
- src/spec_lint/linters.rs
- src/spec_gateway/plan.rs
- src/spec_verify/**
- tests/**
- README.md
- CHANGELOG.md
- AGENTS.md
- .cursorrules
- install-skills.sh
- skills/**
- .claude/skills/**
- specs/task-deepchat-style-sdd-docs.spec.md

### Forbidden
- Do not implement `finish`, graduation, document impact graphs, PlantUML rendering, or docu.md integration in this task
- Do not make `plan.md` or `tasks.md` additional contract authorities
- Do not overwrite or partially update an existing goal package
- Do not add a template engine or new dependency
- Do not vendor DeepChat skills or retain DeepChat-specific application architecture rules in generated templates

## Out of Scope

- Automatic CLI classification of a natural-language request; the `agent-spec-sdd` skill applies the documented classification rules before invoking the CLI
- Updating generated `plan.md` and `tasks.md` after their initial creation
- Artifact Graph construction and code-to-document impact analysis
- `docs sync`, `docs check`, `tasks`, `start`, or `finish` commands
- Rendering or exporting PlantUML, PDF, DOCX, HTML, or images
- Migrating existing specs into goal folders
- GitHub issue synchronization, labels, or PR-closing behavior from DeepChat
- Replacement commands for org/project scaffolding or rewrite-parity templates

## Completion Criteria

Scenario: Feature init creates the readable three-file package
  Test:
    Package: agent-spec
    Filter: test_cli_init_feature_creates_sdd_package_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process, stdout, and filesystem
  Given an empty temporary root and the name "Plugins Hub"
  When init runs with kind feature
  Then `docs/features/plugins-hub/spec.md` exists as a parseable Task Contract
  And `docs/features/plugins-hub/plan.md` exists with implementation and PlantUML sections
  And `docs/features/plugins-hub/tasks.md` exists with contract-linked task guidance and quality gates

Scenario: Architecture init creates the readable three-file package
  Test:
    Package: agent-spec
    Filter: test_cli_init_architecture_creates_sdd_package_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given an empty temporary root and the name "Agent Runtime Split"
  When init runs with kind architecture
  Then `docs/architecture/agent-runtime-split/` contains `spec.md`, `plan.md`, and `tasks.md`
  And the generated spec identifies the goal as architecture work

Scenario: Issue init uses the lightweight single-file package
  Test:
    Package: agent-spec
    Filter: test_cli_init_issue_creates_single_spec_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given an empty temporary root and the name "Session Restore Jitter"
  When init runs with kind issue
  Then `docs/issues/session-restore-jitter/spec.md` exists
  And no `plan.md` or `tasks.md` is created
  And the issue spec includes impact, suspected root cause, fix plan, validation, and Open Questions sections

Scenario: Existing goal artifacts block package creation without partial writes
  Test:
    Package: agent-spec
    Filter: test_cli_init_sdd_package_refuses_partial_overwrite_e2e
    Level: e2e
    Test Double: pre-existing local file only
    Targets: public CLI exit status, stderr, and filesystem atomicity
  Given the target goal directory already contains `plan.md`
  When init attempts to create the feature package
  Then the command returns an error naming the existing path
  And neither `spec.md` nor `tasks.md` is created
  And the existing `plan.md` content is unchanged

Scenario: Invalid SDD kind fails before filesystem changes
  Test:
    Package: agent-spec
    Filter: test_cli_init_rejects_invalid_sdd_kind_e2e
    Level: e2e
    Test Double: none
    Targets: clap validation, stderr, exit status, and filesystem
  Given an empty temporary root
  When init receives an unsupported kind
  Then the command returns an actionable feature, issue, or architecture error
  And no goal directory is created

Scenario: Fixed goal spec filename participates in guard collection
  Test:
    Package: agent-spec
    Filter: test_guard_collects_fixed_goal_spec_filename
  Given a goal directory containing `spec.md`, `plan.md`, and `tasks.md`
  When guard collects spec files from that directory
  Then only `spec.md` is collected as an agent-spec contract

Scenario: Missing goal classification is rejected
  Test:
    Package: agent-spec
    Filter: test_cli_init_requires_kind_and_name_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI argument validation, stderr, exit status, and filesystem
  Given an empty temporary working directory
  When init runs without `--kind` or without `--name`
  Then each invocation returns a non-zero exit status naming the missing required argument
  And neither invocation creates a spec file or `docs/` goal directory

Scenario: Dedicated SDD skill orchestrates the package workflow
  Test:
    Package: agent-spec
    Filter: test_agent_spec_sdd_skill_routes_cli_workflow
  Given the dedicated SDD skill, focused tool-first and authoring skills, and generated integration guidance
  When their responsibilities and initialization instructions are checked
  Then `agent-spec-sdd` classifies substantial work and invokes `agent-spec init --kind`
  And its documentation identifies both referenced DeepChat skills as the workflow and completion-governance model
  And it states which DeepChat behaviors are preserved and which project-specific behaviors are replaced or deferred
  And `agent-spec-sdd` distinguishes authoritative `spec.md` from implementation `plan.md` and execution `tasks.md`
  And `agent-spec-authoring` prioritizes BDD scenarios with public CLI E2E evidence
  And `agent-spec-tool-first` retains CLI execution, verdict, and retry responsibilities without duplicating SDD governance
  And generated AGENTS guidance delegates workflow policy to agent-spec CLI

Scenario: markers in comments and front-matter are caught
  Test:
    Package: agent-spec
    Filter: test_lint_needs_clarification_scans_comments_and_front_matter
  Given a spec whose only unresolved marker sits in a comment or front-matter
  When lint runs
  Then an error-level "needs-clarification" diagnostic is reported

Scenario: documented marker examples in inline code stay quiet
  Test:
    Package: agent-spec
    Filter: test_lint_needs_clarification_ignores_documented_marker_examples
  Given a spec that quotes the marker inside inline code as documentation
  When lint runs
  Then no "needs-clarification" diagnostic is reported

Scenario: dotted bare filenames count as path boundaries
  Test:
    Package: agent-spec
    Filter: test_boundary_bare_dotted_filenames_are_path_boundaries
  Given Allowed Changes listing ".cursorrules" and "install-skills.sh"
  When the change set contains exactly those files
  Then the boundary check judges the change set allowed

## Open Questions

None.
