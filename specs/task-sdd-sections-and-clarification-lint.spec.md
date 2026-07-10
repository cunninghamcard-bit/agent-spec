spec: task
name: "SDD sections (Current State / UX Shape / Open Questions) and needs-clarification lint"
inherits: project
tags: [dsl, parser, lint, sdd]
---

## Intent

Adopt the highest-value pieces of DeepChat's spec anatomy into the Task
Contract DSL: a `Current State` section that spares the agent code
archaeology, a `UX Shape` section for ASCII interface sketches, and an
`Open Questions` alias for the existing Questions section. Unresolved
NEEDS-CLARIFICATION markers become a lint error so a contract cannot
pass the gate before every ambiguity is resolved.

## Decisions

- New prose sections `Current State` and `UX Shape` are recognized section headers; both are informational and never affect verification verdicts
- `Open Questions` is accepted as a header alias for the existing Questions section
- The `contract` rendering includes `Current State` and `UX Shape` content when present
- New linter `needs-clarification`: any occurrence of the bracketed NEEDS-CLARIFICATION marker in intent, items, scenario names, or step text is an error-level diagnostic, which gates `lifecycle` via the existing quality gate
- A Questions section with unresolved bullet items yields a warning-level diagnostic; prose such as "None." produces no items and stays quiet
- All new headers are English-only, consistent with the 0.4.0 DSL

## Boundaries

### Allowed Changes
- src/main.rs
- src/spec_core/**
- src/spec_parser/**
- src/spec_lint/**
- src/spec_gateway/**
- src/spec_report/**
- specs/task-sdd-sections-and-clarification-lint.spec.md

### Forbidden
- Do not let the new sections change any verification verdict
- Do not make a non-empty Questions section an error by itself

## Out of Scope

- Directory-layout support for SDD folders (guard multi-dir, plan --out)
- Template and documentation updates

## Completion Criteria

Scenario: Current State section parses as prose
  Test:
    Package: agent-spec
    Filter: test_parser_accepts_current_state_section
  Given a spec containing a "## Current State" section with prose
  When the spec is parsed
  Then parsing succeeds
  And the section content is preserved

Scenario: UX Shape section parses and keeps ASCII art
  Test:
    Package: agent-spec
    Filter: test_parser_accepts_ux_shape_section
  Given a spec containing a "## UX Shape" section with an ASCII layout block
  When the spec is parsed
  Then parsing succeeds
  And the ASCII block is preserved verbatim

Scenario: Open Questions header maps to the Questions section
  Test:
    Package: agent-spec
    Filter: test_parser_accepts_open_questions_header_alias
  Given a spec containing a "## Open Questions" section with one bullet item
  When the spec is parsed
  Then the section parses as the Questions section kind
  And the item is preserved

Scenario: contract rendering includes the new sections
  Test:
    Package: agent-spec
    Filter: test_contract_renders_current_state_and_ux_shape
  Given a spec with Current State and UX Shape sections
  When the Task Contract is rendered
  Then the output contains the Current State content
  And the output contains the UX Shape content

Scenario: NEEDS CLARIFICATION marker is a lint error
  Test:
    Package: agent-spec
    Filter: test_lint_needs_clarification_marker_is_error
  Given a spec whose intent contains a bracketed NEEDS-CLARIFICATION marker with a question
  When lint runs
  Then an error-level "needs-clarification" diagnostic is reported
  And the quality gate fails

Scenario: unresolved Open Questions items warn
  Test:
    Package: agent-spec
    Filter: test_lint_open_questions_items_warn
  Given a spec whose Questions section has one bullet item
  When lint runs
  Then a warning-level "open-questions" diagnostic is reported

Scenario: resolved Questions section stays quiet
  Test:
    Package: agent-spec
    Filter: test_lint_resolved_questions_none_is_quiet
  Given a spec whose Questions section contains only the prose "None."
  When lint runs
  Then no "open-questions" diagnostic is reported
  And no "needs-clarification" diagnostic is reported
