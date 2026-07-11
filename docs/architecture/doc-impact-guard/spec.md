spec: task
name: "Doc Impact Guard"
inherits: project
tags: [architecture, sdd, guard, documentation]
---

## Intent

Give long-lived documentation a mechanical freshness signal. A maintained
document declares which code paths it governs; when `guard` sees changes
under those paths without a matching update to the document, it reports an
unresolved documentation impact. Dates tell you a document is old — only a
causal link between code and documentation can tell you it is wrong.

## Current State

Contracts already bind specs to code (Boundaries) and to tests (Test
selectors). Documentation has no such edge: ARCHITECTURE-style documents and
README sections rot silently when the code they describe changes. The
completion-governance model from the DeepChat adaptation defers cleanup to a
future `finish` workflow; this goal adds the missing document-to-code edge
that both `guard` and `finish` need.

## UX Shape

```text
README.md:
  <!-- agent-spec:governs: src/main.rs, skills/** -->

$ agent-spec guard --spec-dir specs --code . --change-scope worktree
warning: documentation impact unresolved:
  src/main.rs changed; governed by README.md (not updated in this change set)
agent-spec guard: 44 spec(s) passed
```

## Decisions

- A maintained document declares governance with a single-line HTML comment marker: `<!-- agent-spec:governs: <glob>[, <glob>...] -->`; the marker is invisible in rendered Markdown and requires no YAML front-matter
- Multiple marker lines in one document accumulate; documents without the marker are ignored
- Markers inside fenced code blocks are documentation examples and are ignored
- Discovery scans `*.md` at the code root and `docs/**/*.md` recursively
- The check runs inside `guard` only: for each governed document, if any change path matches a governs glob and the document itself is not in the change set, `guard` prints a `documentation impact unresolved` warning naming the document and the changed path
- Doc-impact findings are warning-level in this phase: they never change guard's exit status; escalation and explicit acknowledgment mechanics are deferred until real-world noise rates are known
- Glob matching reuses the existing boundary pattern matcher
- Self-application: README.md declares governance over `src/main.rs` so this repo dogfoods the check

## Boundaries

### Allowed Changes
- src/main.rs
- src/doc_impact.rs
- src/spec_verify/**
- tests/**
- README.md
- CHANGELOG.md
- docs/architecture/doc-impact-guard/**

### Forbidden
- Do not change guard's exit status based on doc-impact findings in this phase
- Do not add a YAML or template dependency

## Out of Scope

- Acknowledgment / exemption mechanics (`no-doc-impact` decisions)
- Escalation of doc-impact findings to errors
- An explicit persisted artifact graph
- `finish` command integration (separate goal)

## Completion Criteria

Scenario: governed path changes without the document
  Test:
    Package: docwright
    Filter: test_doc_impact_warns_when_governed_path_changes
  Given a document governing "src/**" and a change set containing "src/lib.rs"
  When doc impact is checked
  Then a warning names the document and the changed path

Scenario: document updated together with the code stays quiet
  Test:
    Package: docwright
    Filter: test_doc_impact_quiet_when_doc_updated_together
  Given a governed document that is itself part of the change set
  When doc impact is checked
  Then no warning is reported for that document

Scenario: changes outside governed paths stay quiet
  Test:
    Package: docwright
    Filter: test_doc_impact_quiet_without_overlap
  Given a document governing "src/agent/**" and a change set touching only "docs/notes.md"
  When doc impact is checked
  Then no warning is reported

Scenario: governs markers parse from HTML comments
  Test:
    Package: docwright
    Filter: test_doc_impact_parses_governs_marker
  Given a Markdown file containing two governs marker lines and prose
  When markers are parsed
  Then all globs from both lines are extracted
  And a file without markers yields no governed document

Scenario: doc-impact warnings do not gate guard
  Test:
    Package: docwright
    Filter: test_cli_guard_doc_impact_warns_without_failing_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process, stderr, and exit status
  Given a temporary workspace with a governed document, a passing spec, and an explicit change under the governed path
  When guard runs
  Then stderr contains "documentation impact unresolved"
  And the exit status is zero

## Open Questions

None.
