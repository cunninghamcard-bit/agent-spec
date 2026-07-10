---
name: agent-spec-sdd
description: Use before substantial feature, issue, refactor, migration, or architecture work that needs a durable Spec-Driven Development record. Classify the goal, create the agent-spec documentation package through the CLI, and keep spec.md authoritative over plan.md and tasks.md.
---

# Agent Spec SDD

This workflow adapts DeepChat's `deepchat-sdd` and `deepchat-sdd-cleanup` skills into a provider-agnostic, BDD-native CLI workflow. It preserves goal classification, goal folders, artifact roles, ambiguity resolution, and documentation hygiene. It replaces DeepChat-specific Presenter, Vue, pnpm, i18n, GitHub-label, and release rules with agent-spec contracts and gates.

## When To Use

Use SDD for substantial features, complex issues, migrations, cross-module refactors, and architecture decisions that benefit from shared context or a durable record. Skip visual/copy-only changes, obvious localized fixes, and routine documentation edits unless the developer explicitly requests SDD.

## Classify And Initialize

- Feature or user-visible capability: `agent-spec init --kind feature --name "<goal>"`
- Complex bug or regression: `agent-spec init --kind issue --name "<goal>"`
- Refactor, migration, or architecture boundary: `agent-spec init --kind architecture --name "<goal>"`

The CLI creates a kebab-case goal folder under `docs/features`, `docs/issues`, or `docs/architecture`. Feature and architecture goals receive `spec.md`, `plan.md`, and `tasks.md`; issues receive a comprehensive `spec.md`.

## Artifact Authority

1. `spec.md` is the authoritative, human-readable Task Contract. It defines intent, decisions, boundaries, BDD Scenarios, and Test selectors. It may contain PlantUML fenced blocks.
2. `plan.md` describes implementation. It must not redefine the contract.
3. `tasks.md` tracks execution. Link tasks to Scenario names or Test selectors.

Resolve every open question and clarification marker before implementation. Prefer public CLI or product E2E evidence for behavior; add lower-level tests only where they provide useful isolation.

## Workflow

1. Inspect the current code and maintained documentation.
2. Decide whether SDD is warranted and classify the goal.
3. Run the matching `agent-spec init --kind` command.
4. Author and review `spec.md`; run `agent-spec parse`, `lint`, and `contract`.
5. Refine `plan.md`, then keep `tasks.md` linked to the contract as work lands.
6. Implement within Boundaries without weakening the spec to pass.
7. Run `agent-spec lifecycle <goal>/spec.md --code .` until every Scenario passes.
8. Run the relevant repo-level `agent-spec guard` before handoff.

Completion governance will be handled by the future `agent-spec finish` CLI workflow; do not create a separate cleanup skill or invent manual graduation rules here.
