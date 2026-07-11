---
artifact: tasks
goal: "Rename And Publish"
status: active
derived_from:
  - spec.md
  - plan.md
---

# Rename And Publish Tasks

> Link each implementation task to a Scenario name or Test selector from `spec.md`.

## Review Gate

- [x] Review `spec.md` and resolve every open question.
- [x] Review `plan.md` against the authoritative contract.

## Implementation

- [x] Add the smallest implementation slice. Covers: `<Scenario or Test selector>`

## Tests

- [x] Add public E2E evidence for every Scenario.
- [x] Run focused tests for changed behavior.

## Documentation Impact

- [x] Update affected maintained documentation or record why no update is required.

## Quality Gates

- [x] `agent-spec lint spec.md --min-score 0.7`
- [x] `agent-spec lifecycle spec.md --code .`
- [x] `agent-spec guard --spec-dir . --code .`
