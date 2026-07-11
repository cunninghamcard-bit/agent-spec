spec: task
name: "Rename And Publish"
inherits: project
tags: [architecture, sdd, identity]
---

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

## Decisions

- New identity: `docwright` (research.md F4, record 0001)
- Full rename depth (research.md F5, record 0002): crate/binary/CLI-facing text, skill directory names (`skills/agent-spec-*` -> `skills/docwright-*`, mirrored in `.claude/skills/`), the `.agent-spec/` state directory -> `.docwright/`, and the `agent-spec:*` marker prefix (`governs`, `generated`, `integration`) -> `docwright:*`
- Cargo.toml gets `repository`/`homepage` pointing at this fork's actual GitHub URL (fixing the pre-existing bug), plus `readme`, `keywords`, and `categories` fields (upstream's crate has none set; ours should for crates.io discoverability)
- Generated integration files (`AGENTS.md`, `.cursorrules`, the Claude skill-content file — renamed from `agent-spec-tool-first.md` to `docwright-tool-first.md`) are regenerated via `gen-integrations` from the renamed single source, never hand-edited
- CHANGELOG.md: only the evergreen header line renames; every dated historical entry is left untouched (it documents what shipped under the old name at the time); a new entry documents the rename itself
- Historical/frozen documents are out of scope for the rename sweep: `docs/phase-*-retrospective.md`, `docs/bdd-spine-end-state.md`, `docs/comparison-openspec-speckit.md`, `docs/behavior-contract-improvement-proposal.md`, `docs/jj-vcs-integration-guide.md`, `docs/superpowers/**`, `docs/learning/**` (archived) — their inline `agent-spec <cmd>` prose is historical narrative, not a live citation
- Already-graduated maintained goal contracts (`docs/architecture/{doc-impact-guard,finish-command,workflow-refactor}/spec.md`, `docs/features/{integrate-command,research-and-learn,learning-archive}/spec.md`, `docs/issues/parser-fence-blindness/spec.md`) keep their Decision-prose frozen, but their `Test: Package:` selector fields are not narrative — they are machine-executed (`cargo test --package agent-spec` literally fails once the crate is renamed) — discovered by running `guard`, and corrected surgically: `Package: agent-spec` -> `Package: docwright` only, prose untouched
- `docs/project.spec.md` (the living constitution) is in scope like any other currently-maintained document
- Renaming the GitHub repository itself is explicitly out of this contract's automated Boundaries — it is a shared/visible action requiring separate confirmation outside the agent's normal write path
- `cargo publish` itself is out of scope for automated execution: this goal prepares and validates the package (`cargo publish --dry-run`); the user runs `cargo login` and `cargo publish` themselves

## Boundaries

### Allowed Changes
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
- .aider.conf.yml
- .gitignore
- .github/workflows/**
- agent-spec-tool-first.md
- docwright-tool-first.md
- docs/project.spec.md
- docs/architecture/doc-impact-guard/spec.md
- docs/architecture/finish-command/spec.md
- docs/architecture/workflow-refactor/spec.md
- docs/features/integrate-command/spec.md
- docs/features/learning-archive/spec.md
- docs/features/research-and-learn/spec.md
- docs/issues/parser-fence-blindness/spec.md
- examples/**
- tests/**
- docs/architecture/rename-and-publish/**

### Forbidden
- Do not rewrite historical/frozen documents (retrospectives, comparison docs, docs/learning/**, already-graduated goal contracts) to the new name
- Do not rewrite past dated CHANGELOG entries
- Do not run `cargo login` or `cargo publish` (or handle any crates.io API token)
- Do not rename the GitHub repository

## Out of Scope

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

## Open Questions

None.
