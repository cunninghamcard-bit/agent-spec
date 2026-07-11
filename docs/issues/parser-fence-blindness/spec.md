spec: task
name: "Parser Fence Blindness"
inherits: project
tags: [issue, parser]
---

## Intent

Fix the parser treating `#`-prefixed lines inside fenced code blocks as
top-level section headers, which rejects valid specs whose UX Shape or other
prose sections contain fenced examples with comment lines.

## Current State

`parse_body` scans every line for markdown headings without tracking fence
state, so a spec containing a fenced shell example with a `# comment` line
fails with `unknown top-level section header`. Discovered live while
authoring the integrate-command goal; the same class of bug was already
fixed once in doc_impact marker parsing (fences are documentation, not
structure).

## Decisions

- `parse_body` tracks fenced code blocks (``` and ~~~) and treats every line inside a fence as section prose, never as a header, scenario, or step
- Fence tracking applies uniformly, so scenario keywords quoted inside fenced examples are also inert

## Boundaries

### Allowed Changes
- src/spec_parser/**
- docs/issues/parser-fence-blindness/**

### Forbidden
- Do not change how fences render in Intent/UX Shape content (verbatim preservation stays)

## Out of Scope

- Inline code spans (single backticks) — already inert
- Markdown quote blocks

## Completion Criteria

Scenario: hash comments inside fences are prose
  Test:
    Package: docwright
    Filter: test_parser_ignores_headers_inside_fences
  Given a spec whose UX Shape fence contains a line starting with "#"
  When the spec is parsed
  Then parsing succeeds
  And the fence content is preserved verbatim

Scenario: scenario keywords inside fences are inert
  Test:
    Package: docwright
    Filter: test_parser_ignores_scenario_keywords_inside_fences
  Given a completion criteria section whose fenced example contains a "Scenario:" line
  When the spec is parsed
  Then the fenced line does not start a new scenario

## Open Questions

None.
