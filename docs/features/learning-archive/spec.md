spec: task
name: "Learning Archive"
inherits: project
tags: [feature, sdd, learning]
---

## Intent

Graduation currently deletes research.md and learning-records/ — the
user's real learning trail dies with the goal. Preserve it: finish
migrates both into docs/learning/<goal>/, the decision-archaeology layer
of the single household, complementing docs/capabilities/ (capabilities
say what the rules are now; learning says why they were decided, and
which recommendations were overridden). Decided in learning record 0001
of this goal; tool-specific homes (.agents/, .claude/) were rejected
because learning belongs to no single agent tool.

## Current State

finish removes plan.md, tasks.md, research.md, and learning-records/
after full verification (finish-command and research-and-learn
contracts). The already-graduated research-and-learn goal's five records
survive only in git history (commit 911d5da).

## Decisions

- finish archives instead of deleting: research.md and learning-records/ move to docs/learning/<goal>/ at graduation (learning record 0001); plan.md and tasks.md remain deleted consumables
- The archive root is the nearest docs/ ancestor of the goal spec, mirroring promote's household resolution
- Collisions are refused: if docs/learning/<goal>/ already holds a file finish would write, abort before moving anything
- The research-and-learn and finish-command maintained contracts are amended to state the archive behavior
- The research-and-learn goal's five learning records and research.md are backfilled from git history into docs/learning/research-and-learn/
- The agent-spec-research and agent-spec-sdd skills state the archive destination

## Boundaries

### Allowed Changes
- src/main.rs
- skills/**
- .claude/skills/**
- tests/finish_cli_e2e.rs
- README.md
- CHANGELOG.md
- docs/learning/**
- docs/features/learning-archive/**
- docs/features/research-and-learn/spec.md
- docs/architecture/finish-command/spec.md

### Forbidden
- Do not keep plan.md or tasks.md at graduation
- Do not write the archive to tool-specific directories (.agents/, .claude/)

## Out of Scope

- Aggregating or indexing the archive (a flat per-goal folder is enough)
- Archiving goals retired via finish --retire (withdrawal is not graduation)

## Completion Criteria

Scenario: graduation archives the learning trail
  Test:
    Package: agent-spec
    Filter: test_cli_finish_archives_research_and_learning_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given a verified goal package containing research.md and learning-records
  When finish runs
  Then research.md and the learning records exist under docs/learning/<goal>/
  And they are gone from the goal folder
  And plan.md and tasks.md are deleted

Scenario: archive collisions abort the graduation
  Test:
    Package: agent-spec
    Filter: test_cli_finish_refuses_archive_collision_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given docs/learning/<goal>/ already contains a conflicting file
  When finish runs
  Then the command fails naming the collision
  And the goal folder keeps research.md and its learning records
