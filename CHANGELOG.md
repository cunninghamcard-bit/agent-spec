# Changelog

All notable changes to `agent-spec` are documented here. Format follows
[Keep a Changelog](https://keepachangelog.com/), and this project adheres to
[Semantic Versioning](https://semver.org/).

## [Unreleased]

### Changed (BREAKING) — Workflow Refactor (0.5.0)

- **Staged artifact birth.** `init --kind` creates the goal folder and
  `spec.md` only; `plan --out` births `plan.md` and `tasks.md` when the
  planning step arrives. plan.md has a single owner.
- **Single household.** The `specs/` task corpus and roadmap staging
  directory are retired (history lives in git). Contracts live in
  `docs/features|issues|architecture/<goal>/`; the project constitution
  moved to `docs/project.spec.md` (`inherits: project` resolves `docs/`
  before `specs/`); `promote` writes to `docs/capabilities/`.
- **Defaults re-homed.** `guard`, `audit`, `graph`, and the
  `install-hooks` pre-commit hook default to the `docs/` household; guard
  collection is recursive (goal `spec.md` files included; `roadmap/`
  directories skipped).
- **Flow-grouped help.** `--help` stations all 25 subcommands under five
  workflow flows (Adoption / Goal lifecycle / Review / Library governance
  / Probe & AI); nothing hidden, nothing deleted.
- **Declared invariant.** `lifecycle` prints a non-blocking warning when
  more than one goal folder carries uncommitted changes (one active goal
  per worktree).
- **Skills rewritten around the flows.** `agent-spec-sdd` owns the flow
  map; the commit step uses `stamp --dry-run` (never hand-written
  trailers); graduation is `finish` then `promote`. The tool-first
  commands reference documents every subcommand's station.


### Changed (BREAKING)

- **`init` is goal-oriented and English-only.** `--kind` and `--name` are now
  required. The legacy `--level`, `--lang`, and `--template` options and
  single-file scaffold were removed; init always creates a classified SDD
  goal package.

- **English-only structural keywords (0.4.0).** Adopting the specwright
  fork's judgment: the parser hard-rejects CJK structural keywords (section
  headers, `场景:`/`测试:`/`包:`/`过滤:`, step keywords 假设/当/那么/并且/但是,
  and all other aliases) with an actionable error naming the English
  replacement, e.g. `keywords must be English; '场景:' is not recognized —
  use 'Scenario:'`. Descriptive free text (scenario titles, step prose,
  quoted parameters) may remain in any language. The repo's own spec corpus
  is migrated, and a regression test keeps it English-only.

### Added

- **Research and Learn.** Demand-driven pre-contract research: the
  `research` command scaffolds a goal's research.md and machine-refreshes
  its codebase-state region; the `agent-spec-research` skill owns the
  methodology (primary sources, per-claim citations, gh toolkit,
  Decision/Rationale/Alternatives findings) and the grill protocol (each
  decision put to the user one at a time, learning records written per
  confirmed round). Lints: `research-required` (error — unresolved
  clarification markers with no research.md), `research-uncited` and
  `research-unfilled` (warnings). integrate ships the new skill; finish
  removes research.md and learning-records/ as consumables. Skills are
  now cross-linked in the Pocock composition model.

- **`integrate` — one-command project adoption.** Installs the embedded
  workflow skills into the target project's `.agents/skills/` (Codex) and
  `.claude/skills/` (Claude Code) and writes a managed, prose-only policy
  block into AGENTS.md and CLAUDE.md (created, appended, or refreshed
  between markers; surrounding content preserved). Declaration files carry
  policy only — operations live in the CLI and skills. `init --kind` now
  prefills a vitest JUnit test binding for Node projects.

- **`finish` — goal graduation.** The counterpart of `init`: re-verifies
  the goal's contract (aborting with the failing summary and deleting
  nothing otherwise), removes the consumable plan.md/tasks.md, keeps the
  contract by default or retires the whole goal with `--retire`, and hints
  at `promote` when durable Rules exist.

- **Doc Impact Guard.** Maintained Markdown documents can declare governed
  code paths with an invisible `<!-- agent-spec:governs: <globs> -->`
  marker; `guard` warns (`documentation impact unresolved`) when governed
  paths change without the document. Warning-level in this phase — exit
  status is never affected. This repo dogfoods it: README.md governs
  `src/main.rs`.

- **DeepChat-style SDD goal packages.** `agent-spec init --kind
  feature|issue|architecture --name <goal>` creates readable Markdown under
  `docs/`: feature and architecture receive `spec.md`, `plan.md`, and
  `tasks.md`; issues receive a comprehensive `spec.md`. Fixed-name `spec.md`
  participates in guard collection, existing artifacts are never overwritten,
  and the new `agent-spec-sdd` skill keeps the workflow CLI-driven and
  BDD/E2E-first.

- **SDD integration (DeepChat-inspired).** New contract sections
  `## Current State` and `## UX Shape` (informational, rendered in the
  contract view) and an `## Open Questions` alias for Questions. New lints:
  `needs-clarification` (error — an unresolved bracketed marker blocks the
  gate) and `open-questions` (warning while items remain). CLI:
  `guard --spec-dir` is repeatable for goal-folder layouts, `plan --out`
  writes the rendered plan as a plan.md draft, and `init` task templates
  scaffold the new sections.

- **Report-driven verification (report mode).** Specs may declare
  `test_command` + `test_report` in the frontmatter; verification runs the
  project's own test command once via `sh -c` and judges every scenario by
  matching its `Test:` selector against testcase names in the JUnit XML
  report. Opens mechanical verification to any stack that emits JUnit XML
  (vitest/jest, Maven/Gradle, pytest, cargo-nextest) with no per-framework
  CLI adapter. Strict verdicts: zero-match, ambiguous match, skipped
  testcase, and missing report are all `fail`. Optional `{selectors}`
  placeholder expands to an escaped regex alternation for targeted runs;
  a structured selector's `Package:` filters by `classname` prefix.

### Fixed

- **Guard boundary semantics.** `guard` now runs boundary verification in
  forbidden-only mode: `Forbidden` entries are enforced repo-wide, while
  `Allowed Changes` coverage (task-scoped by nature) only gates single-spec
  `verify`/`lifecycle`. Previously any real change set failed every
  historical spec whose allow-list didn't cover it.
- Git repo-root discovery no longer falls back to the process cwd when
  neither `--code` nor the spec path is inside a repository — the change
  set is empty instead of borrowed from an unrelated repo.
- Boundary checks now treat bare manifest filenames (`Cargo.toml`,
  `Cargo.lock`, `*.md`) as path boundaries, and relativize absolute change
  paths against the canonicalized workspace root before matching — both
  previously made `--change-scope` boundary verification fail spuriously.

## [0.3.0] - 2026-06-04

The **BDD-spine** release. agent-spec absorbs living-spec-library (OpenSpec) and
scaffolding/governance (Spec Kit) capabilities under one model — Discovery →
Formulation → Automation — while staying BDD-core and single-binary. Verdict
semantics are unchanged: the six verdicts (`pass`/`fail`/`skip`/`uncertain`/
`pending_review` + gate) and `is_passing` are untouched; every new check is a
sensor (lint / report / audit), never a silent change to pass/fail.

### Added

- **Rule → Example BDD semantics (Phase 1).** First-class `Rule:` / `规则:`
  grouping over scenarios, with a stable keystone identity `RuleKey { scope, id }`
  — `id` is stable kebab-case, the display `name` is mutable. `Example` is
  recognized as a synonym for `Scenario` (incl. `示例` / `例子`, fullwidth colon).
  Four `bdd-*` linters with agent-readable self-correction guidance.
- **Coverage matrix (Phase 2).** `agent-spec matrix` renders
  Rule × Scenario × Test × Verdict × Provenance (text / json / markdown).
  New `EvidenceProvenance` (Computational vs Inferential) stamped per result.
- **Capability level + promote (Phase 3).** `spec: capability` specs and
  `agent-spec promote`, lifting a passing task Rule into a capability spec
  **without changing its `id`**. Promotion gate requires ≥1 proving Example;
  capability names are path-traversal-checked.
- **Discovery questions (Phase 4).** `## Questions` / `问题` / `待澄清` sections
  parse to a structured `Questions` section; `open-question` lint (non-blocking).
- **lint-ack + dimensions (Phase 5).** `<!-- lint-ack: CODE reason -->` lets
  authors acknowledge a Warning/Info **with a mandatory reason** (never an Error);
  acknowledged counts stay visible. Lint codes classify into dimensions.
- **Single-source integrations (Phase 6).** `agent-spec gen-integrations`
  renders agents / cursor / claude integration files from one source, with a
  `--check` drift gate (write and check share the same renderer).
- **Probe abstraction (Phase 6.5).** Unifies evidence sources as
  `Test / Static / Benchmark / External / Inferential` (scaffolding; runner
  execution for Benchmark/External is deferred).
- **Structural check (Phase 7).** `agent-spec check-structure --forbid X --in glob`
  — mechanical layering guard (dependency-cruiser-lite); non-zero exit on violation.
- **Library health audit (Phase 8).** `agent-spec audit` aggregates spec/rule/
  scenario counts, unproven rules, ungrouped scenarios, open questions, malformed
  rules (text / json). Observability only — never gates.
- **Cold-start reverse spec (Phase 9).** `agent-spec discover --from-codebase`
  drafts a parseable task-spec skeleton from existing test functions (one bound
  scenario per test) plus a `## Questions` seed flagging it for human refinement.

### Changed

- README command table now documents the full BDD-spine command surface.
- `ReviewMode` now derives `Default` (`Auto`); no behavior change.

### Fixed

- Cleared `cargo clippy --all-targets --all-features -- -D warnings` (the CI gate):
  derivable impl, `slice::from_ref` over `&[x.clone()]` in tests, and missing
  `unwrap_used`/`expect_used` allows on two test modules.

### Notes

- 342 tests pass; `guard` 37/37 specs; clippy CI gate clean.
- Verification semantics and `is_passing` are unchanged from 0.2.x.

## [0.2.7] and earlier

See git history. 0.2.x established the core contract pipeline: `parse`, `lint`,
`verify`, `lifecycle`, `guard`, `explain`, `stamp`, `contract`, `plan`, `graph`,
the four verifier layers, run logging, and VCS-aware checkpoints.

[0.3.0]: https://github.com/ZhangHanDong/agent-spec/releases/tag/v0.3.0
