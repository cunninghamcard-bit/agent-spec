---
name: agent-spec-tool-first
description: Use agent-spec as a CLI tool to verify code against Task Contracts.
---

agent-spec is an AI-native BDD/spec verification tool. Use it tool-first:

1. For substantial new work without a contract, create the goal folder and its
contract skeleton: `agent-spec init --kind feature|issue|architecture --name <goal>`.
2. Read the Task Contract: `agent-spec contract <spec>`.
3. Generate plan context (births plan.md and tasks.md beside the contract):
`agent-spec plan <spec> --code . --out <goal>/plan.md`.
4. Implement within the contract's Boundaries.
5. Verify: `agent-spec lifecycle <spec> --code . --format json` — fix until all
scenarios pass (failed/skipped/uncertain all 0). Do not edit the spec to pass.
6. Repo-level gate before committing: `agent-spec guard --spec-dir docs --code .`.
7. Commit trailers come from the machine: `agent-spec stamp <spec> --code . --dry-run`.
8. Graduate: `agent-spec finish <spec> --code .`; lift durable Rules with
`agent-spec promote` into `docs/capabilities/`.
9. Render the PR acceptance summary: `agent-spec explain <spec> --code . --format markdown`.

The machine verifies whether the code satisfies the contract; you implement
against it, and a human reviews the contract.
