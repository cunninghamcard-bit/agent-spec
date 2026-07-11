spec: task
name: "Research and Learn"
inherits: project
tags: [feature, sdd, research, skill]
---

## Intent

Close the two gaps before contract authoring: the agent's ignorance of
industry norms and the user's un-absorbed understanding of both norms and
codebase state. Part one, research: demand-driven investigation captured as
a cited research.md in the goal folder. Part two, learning: a grill session
that puts every researched decision to the user one at a time, so the cost
of understanding is genuinely paid by the human who will review the
contract. All five governing decisions below were confirmed through this
feature's own grill protocol, grounded in research.md (F1–F5).

## Current State

See research.md in this goal folder for the full survey (five industry
sources, per-claim citations). In this repo today: the needs-clarification
lint is error-level and source-scanning; goal packages exist via init
--kind with finish removing consumables; the plan command already produces
codebase context from Allowed Changes paths; managed-region markers are
established by integrate and doc-impact; the agent-spec-sdd skill owns the
workflow but has no research or learning step.

## Decisions

- Research is demand-driven, never universal (research.md F1): the triggers are an unresolvable bracketed NEEDS-CLARIFICATION marker or an unfamiliar domain
- New CLI subcommand `research <goal-spec> --code .`: creates the sibling research.md from a template on first run and refreshes its codebase-state managed region using the plan command's scanner on every run
- research.md lives in the goal folder (research.md F2) with sections Unknowns, Industry Norms & Prior Art, Current Codebase State (managed region), and Findings in Decision/Rationale/Alternatives format with per-claim sources
- Learning is a grill session plus learning records (research.md F3): after research completes, the agent puts each Finding's decision to the user one at a time with a recommended answer; each round is captured as a learning record file under the goal folder's learning-records/ directory; spec authoring must not start before the grill completes
- Research methodology ships as a standalone `agent-spec-research` skill (research.md F4): primary sources first, never trust parametric knowledge, per-claim citation, the gh toolkit (topic search, tree scan, file read, in-repo code search, issues for rationale), compare multiple implementations, then run the grill and write learning records
- Enforcement is hard at the trigger (research.md F5, user override): a new error-level lint `research-required` fires when a spec contains an unresolved bracketed NEEDS-CLARIFICATION marker and no sibling research.md exists, directing the author to the research command
- When research.md exists, two warning-level lints keep it honest: `research-uncited` when the spec's Current State and Decisions never mention research.md, and `research-unfilled` when research.md still contains template placeholders
- `integrate` embeds and installs the new skill alongside the existing three
- `finish` treats research.md and learning-records/ as consumables, removed at graduation like plan.md and tasks.md
- The agent-spec-sdd skill's workflow gains the research and grill steps, delegating methodology to agent-spec-research
- Skills are linked, not siloed (the Pocock composition model): agent-spec-sdd invokes agent-spec-research by name at its research step, agent-spec-authoring points grilled decisions into the Decisions section, agent-spec-tool-first documents the research command and defers methodology to agent-spec-research
- Learning records are written the moment each grill round is confirmed, not batched at the end

## Boundaries

### Allowed Changes
- src/main.rs
- src/spec_lint/**
- src/spec_report/integrations.rs
- src/spec_gateway/plan.rs
- skills/**
- .claude/skills/**
- tests/**
- README.md
- CHANGELOG.md
- install-skills.sh
- docs/features/research-and-learn/**

### Forbidden
- Do not make research universal or trigger it without the declared conditions
- Do not let the grill step be skippable by the skill workflow when research.md exists
- Do not add new dependencies

## Out of Scope

- Multi-researcher fan-out orchestration (gsd model; single-researcher first)
- Refreshing Industry Norms content mechanically (only the codebase region is machine-owned)
- Verifying grill completion by lint (human interaction is skill-owned)
- Learning-record spaced-repetition features (Pocock teach's storage-strength tooling)

## Completion Criteria

Scenario: research command scaffolds and refreshes
  Test:
    Package: agent-spec
    Filter: test_cli_research_scaffolds_and_refreshes_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given a goal package whose spec.md declares Allowed Changes covering source files
  When research runs twice with hand-written prose added between runs
  Then research.md exists with all four sections
  And the codebase managed region is refreshed
  And the hand-written prose outside the region is preserved

Scenario: unresolved markers without research are an error
  Test:
    Package: agent-spec
    Filter: test_lint_research_required_fires_on_unresolved_marker
  Given a spec containing a bracketed NEEDS-CLARIFICATION marker and no sibling research.md
  When lint runs
  Then an error-level "research-required" diagnostic is reported
  And it names the research command as the resolution path

Scenario: research presence downgrades nothing else
  Test:
    Package: agent-spec
    Filter: test_lint_research_required_quiet_with_research_present
  Given the same spec with a sibling research.md
  When lint runs
  Then no "research-required" diagnostic is reported
  And the pre-existing needs-clarification error still fires

Scenario: uncited research warns
  Test:
    Package: agent-spec
    Filter: test_lint_research_uncited_warns
  Given a sibling research.md and a spec whose Current State and Decisions never mention it
  When lint runs
  Then a warning-level "research-uncited" diagnostic is reported

Scenario: unfilled research warns
  Test:
    Package: agent-spec
    Filter: test_lint_research_unfilled_warns
  Given a sibling research.md still containing template placeholders
  When lint runs
  Then a warning-level "research-unfilled" diagnostic is reported

Scenario: integrate ships the research skill
  Test:
    Package: agent-spec
    Filter: test_cli_integrate_installs_research_skill_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given an empty target project
  When integrate runs
  Then agent-spec-research SKILL.md exists in both skills trees

Scenario: finish removes research consumables
  Test:
    Package: agent-spec
    Filter: test_cli_finish_removes_research_consumables_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given a verified goal package containing research.md and learning-records
  When finish runs
  Then research.md and the learning-records directory are removed
  And spec.md remains

## Open Questions

None.
