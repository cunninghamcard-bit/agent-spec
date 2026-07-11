---
name: agent-spec-sdd
description: Use before substantial feature, issue, refactor, migration, or architecture work that needs a durable Spec-Driven Development record. Owns the five workflow flows, classifies the goal, creates the contract through the CLI, and keeps spec.md authoritative over plan.md and tasks.md.
---

# Agent Spec SDD

This workflow adapts DeepChat's `deepchat-sdd` skills into a
provider-agnostic, BDD-native CLI workflow. Every agent-spec subcommand
has a station in one of five flows; this skill owns the map.

## The Five Flows

1. **Adoption** (once per repo): `integrate` installs skills and policy
   blocks; `install-hooks` installs the pre-commit guard; `discover`
   reverse-engineers draft contracts from existing tests on brownfield
   projects. (`gen-integrations` is integrate's ancestor, kept for
   compatibility.)
2. **Goal lifecycle** (per goal): the numbered workflow below.
3. **Review** (per review): `explain` renders the reviewer-facing
   contract summary; `brief` is a compatibility alias of `contract`.
4. **Library governance** (periodic): `audit` health-checks the contract
   library; `graph` renders its dependency structure.
5. **Probe & AI** (on trigger): `check-structure` enforces structural
   boundaries declared by a contract; `resolve-ai` merges host-agent AI
   verdicts; `measure-determinism` measures verdict stability when AI
   participates.

## Invariants

- **One active goal per worktree.** Parallel goals trip each other's
  boundary checks; lifecycle warns when it detects multiple dirty goal
  folders. Finish or commit one goal before starting the next.
- **Single household.** Contracts live in
  `docs/features|issues|architecture/<goal>/`; durable capability rules
  accumulate in `docs/capabilities/` via promote. Long-term history is
  recovered from git, never from archive directories.

## Classify And Initialize

- Feature or user-visible capability: `agent-spec init --kind feature --name "<goal>"`
- Complex bug or regression: `agent-spec init --kind issue --name "<goal>"`
- Refactor, migration, or architecture boundary: `agent-spec init --kind architecture --name "<goal>"`

The CLI creates a kebab-case goal folder containing `spec.md` only —
staged birth: `plan.md` and `tasks.md` are born by `plan --out` when the
planning step arrives.

## Artifact Authority

1. `spec.md` is the authoritative, human-readable Task Contract. It defines intent, decisions, boundaries, BDD Scenarios, and Test selectors. It may contain PlantUML fenced blocks.
2. `plan.md` describes implementation. It must not redefine the contract.
3. `tasks.md` tracks execution. Link tasks to Scenario names or Test selectors.

Resolve every open question and clarification marker before implementation. Prefer public CLI or product E2E evidence for behavior; add lower-level tests only where they provide useful isolation.

## Goal Lifecycle Workflow

1. Inspect the current code and maintained documentation.
2. Decide whether SDD is warranted and classify the goal.
3. Run the matching `agent-spec init --kind` command.
4. Research and learn when triggered (see the `agent-spec-research`
   skill): if a clarification marker cannot be resolved from the codebase
   or the domain is unfamiliar, run
   `agent-spec research <goal>/spec.md --code .`, investigate against
   primary sources, teach the user what the research found, then grill
   decision-by-decision, writing a learning record per confirmed round.
   Do not author the contract before the grill completes.
5. Author and review `spec.md`; run `agent-spec parse`, `lint`, and
   `contract`. Debugging internals when a gate is unclear: `parse` for
   the AST, `verify` for verification alone, `matrix` for per-scenario
   coverage and dangling selectors.
6. Generate the plan: `agent-spec plan <goal>/spec.md --code . --out
   <goal>/plan.md` — this births plan.md and tasks.md. Curate both; keep
   tasks linked to the contract as work lands.
7. Implement within Boundaries without weakening the spec to pass.
8. Run `agent-spec lifecycle <goal>/spec.md --code .` until every
   Scenario passes.
9. Run the repo-level gate: `agent-spec guard --spec-dir docs --code .`.
10. Commit with machine-verified trailers: run
    `agent-spec stamp <goal>/spec.md --code . --dry-run` and paste its
    output into the commit message. Never hand-write Spec-* trailers.
11. Graduate: `agent-spec finish <goal>/spec.md --code .` removes the
    consumables and keeps the contract.
12. Promote durable Rules into the truth layer:
    `agent-spec promote <goal>/spec.md --rule <id> --to <capability>
    --code .` writes to `docs/capabilities/<capability>.spec.md`.
