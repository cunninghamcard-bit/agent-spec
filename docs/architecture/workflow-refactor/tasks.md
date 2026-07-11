# Tasks — Workflow Refactor

Linked contract: spec.md (Rules: staged-birth, stationed-surface,
single-household, declared-invariants)

## Rule: staged-birth
- [x] init --kind: stop creating plan.md/tasks.md; spec.md only (Scenario: init births only the contract)
- [x] plan --out: create tasks.md skeleton beside plan.md when absent (Scenario: plan births the planning artifacts)
- [x] update existing sdd_init_cli tests to the new birth order

## Rule: stationed-surface
- [x] --help groups all subcommands under five flow headings, none hidden from the map (Scenario: help is grouped by flow)
- [x] tool-first references/commands.md documents all subcommands with station + trigger (Scenario: every command is documented in the audit artifact)
- [x] agent-spec-sdd skill rewritten around the five flows; commit step = stamp --dry-run; graduation step = finish then promote (Scenario: the sdd skill stations every flow)
- [x] gen-integrations documented as integrate's ancestor in commands.md

## Rule: single-household
- [x] promote target: specs/capabilities/ -> docs/capabilities/ (Scenario: promote targets the docs household)
- [x] install-hooks: generated hook guards the docs household, no specs/ reference (Scenario: install-hooks guards the docs household)
- [x] constitution move: specs/project.spec.md -> docs/project.spec.md; amend its Must items that mandate the specs/ household (任务级规格文件存放在 specs/; roadmap 暂存于 specs/roadmap/) to the docs/ household rules
- [x] resolver: inherits: project resolves docs/project.spec.md (keep specs/ lookup as fallback for downstream adopters) (Scenario: inheritance resolves from the relocated constitution)
- [x] guard: docs-only collection works with no specs/ dir present (Scenario: guard collects only the docs home)
- [x] retire specs/ task corpus + roadmap (git rm; history in git)
- [x] sweep repo references to the retired household (README, .cursorrules, tests scanning specs/)

## Rule: declared-invariants
- [x] lifecycle: non-blocking warning when >1 goal folder has uncommitted changes (Scenario: lifecycle warns on parallel dirty goals)
- [x] declare one-active-goal invariant in agent-spec-sdd skill + README

## Cross-cutting
- [x] re-embed skills for integrate (integrations.rs include_str targets)
- [x] agent-spec-research skill: add the process lesson — teaching completes before any grill round (records 0006)
- [x] README: five-flow workflow section, single-household story, capability library
- [x] CHANGELOG entry (0.5.0 breaking: household + init behavior)
- [x] full gates: cargo fmt/clippy/test, lifecycle 10/10, guard, reinstall CLI, stamp trailers on commit
