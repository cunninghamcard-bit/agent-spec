spec: task
name: "Finish Command"
inherits: project
tags: [architecture, sdd, cli, lifecycle]
---

## Intent

Mechanize goal graduation. `finish` is the counterpart of `init`: once a
goal's contract is fully verified, it removes the consumable execution
artifacts (plan.md, tasks.md), keeps or retires the contract, and points at
durable rules worth promoting. Execution materials mistaken for long-term
facts are a primary cause of documentation rot; finish makes their disposal
a checked, one-command step instead of manual hygiene.

## Current State

`init --kind` creates goal packages and `guard` warns on unresolved
documentation impact, but nothing governs completion: plan.md and tasks.md
accumulate after goals are done, and graduation (promote / retain / delete)
exists only as prose in the README. The DeepChat adaptation contract
explicitly deferred completion governance to a future finish workflow.

## UX Shape

```text
$ agent-spec finish docs/architecture/doc-impact-guard/spec.md --code .
verifying contract... lifecycle passed (6/6)
removed docs/architecture/doc-impact-guard/plan.md
removed docs/architecture/doc-impact-guard/tasks.md
kept spec.md (maintained contract; use --retire to remove the goal)
next: agent-spec promote --help (1 rule eligible)
```

## Decisions

- `finish <spec> --code .` first runs the same lint + verify pipeline as `lifecycle`; any non-passing verdict aborts with the failing summary and deletes nothing
- On success, plan.md and tasks.md next to the goal spec are deleted; each removal is printed
- The contract is kept by default; `--retire` removes the whole goal directory when its criteria no longer define a maintained contract
- When the verified spec declares behavior Rules, finish prints a promote hint naming the eligible rule count
- A goal spec without plan/tasks artifacts finishes cleanly and reports that nothing needed cleanup
- History is git's job: finish never writes archives

## Boundaries

### Allowed Changes
- src/main.rs
- tests/**
- README.md
- CHANGELOG.md
- docs/architecture/finish-command/**
- docs/architecture/doc-impact-guard/**

### Forbidden
- Do not delete anything when verification is not fully passing
- Do not touch files other than plan.md and tasks.md unless --retire is given

## Out of Scope

- Automatic promote execution (finish only points at it)
- Doc-impact blocking (guard owns that signal)
- Archive directories or history files

## Completion Criteria

Scenario: finishing a green goal removes the consumables
  Test:
    Package: agent-spec
    Filter: test_cli_finish_removes_plan_and_tasks_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process, stdout, and filesystem
  Given a goal package whose contract verifies fully
  When finish runs
  Then plan.md and tasks.md are removed
  And spec.md remains
  And the exit status is zero

Scenario: finishing a failing goal deletes nothing
  Test:
    Package: agent-spec
    Filter: test_cli_finish_refuses_on_failing_contract_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI exit status, stderr, and filesystem
  Given a goal package whose contract has a failing scenario
  When finish runs
  Then the command returns a non-zero exit status with the failing summary
  And plan.md and tasks.md still exist

Scenario: retire removes the whole goal directory
  Test:
    Package: agent-spec
    Filter: test_cli_finish_retire_removes_goal_directory_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given a goal package whose contract verifies fully
  When finish runs with --retire
  Then the goal directory no longer exists

Scenario: a bare spec finishes with nothing to clean
  Test:
    Package: agent-spec
    Filter: test_cli_finish_bare_spec_reports_nothing_to_clean_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and stdout
  Given a verified spec with no plan.md or tasks.md beside it
  When finish runs
  Then the output reports that no goal artifacts needed cleanup
  And the exit status is zero

## Open Questions

None.
