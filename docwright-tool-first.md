---
name: docwright-tool-first
description: Use docwright as a CLI tool to verify code against Task Contracts.
---

docwright is an AI-native BDD/spec verification tool. Use it tool-first:

1. For substantial new work without a contract, create the goal folder and its
contract skeleton: `docwright init --kind feature|issue|architecture --name <goal>`.
2. Read the Task Contract: `docwright contract <spec>`.
3. Generate plan context (births plan.md and tasks.md beside the contract):
`docwright plan <spec> --code . --out <goal>/plan.md`.
4. Implement within the contract's Boundaries.
5. Verify: `docwright lifecycle <spec> --code . --format json` — fix until all
scenarios pass (failed/skipped/uncertain all 0). Do not edit the spec to pass.
6. Repo-level gate before committing: `docwright guard --spec-dir docs --code .`.
7. Commit trailers come from the machine: `docwright stamp <spec> --code . --dry-run`.
8. Graduate: `docwright finish <spec> --code .`; lift durable Rules with
`docwright promote` into `docs/capabilities/`.
9. Render the PR acceptance summary: `docwright explain <spec> --code . --format markdown`.

The machine verifies whether the code satisfies the contract; you implement
against it, and a human reviews the contract.
