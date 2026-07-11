---
artifact: tasks
goal: "Learning Archive"
status: active
derived_from:
  - spec.md
  - plan.md
---

# Learning Archive Tasks

> Link each implementation task to a Scenario name or Test selector from `spec.md`.

## Review Gate

- [ ] Review `spec.md` and resolve every open question.
- [ ] Review `plan.md` against the authoritative contract.

## Implementation

- [ ] Add the smallest implementation slice. Covers: `<Scenario or Test selector>`

## Tests

- [ ] Add public E2E evidence for every Scenario.
- [ ] Run focused tests for changed behavior.

## Documentation Impact

- [ ] Update affected maintained documentation or record why no update is required.

## Quality Gates

- [ ] `agent-spec lint spec.md --min-score 0.7`
- [ ] `agent-spec lifecycle spec.md --code .`
- [ ] `agent-spec guard --spec-dir . --code .`

## Execution

- [x] finish archives research.md + learning-records/ to docs/learning/<goal>/ (Scenario: graduation archives the learning trail)
- [x] collision refusal before any move (Scenario: archive collisions abort the graduation)
- [x] amend research-and-learn + finish-command maintained contracts
- [x] backfill docs/learning/research-and-learn/ from git (911d5da / 599ab00)
- [x] skills (sdd/research) + README + CHANGELOG state the archive
