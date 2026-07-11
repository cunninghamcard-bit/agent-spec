# 0004 — Historical specs disposal

- **Finding**: research.md F3 follow-up (disposal of the specs/ corpus)
- **Question**: With docs/ as the single home, what happens to the 43
  historical task contracts in specs/?
- **Recommendation**: retire-when-blocking — keep them in guard as a
  zero-maintenance regression net; delete individually only when one
  conflicts with a new goal's contract.
- **User's answer**: **Override** — 现在就整体退役. Remove the specs/
  task corpus wholesale; guard runs only the docs/*/ goal contracts;
  history lives in git.
- **Overrides**: The recommendation. Accepted consequence: the guard
  regression net shrinks from 44 contracts to the docs/ goal contracts;
  historical behavior remains covered by cargo tests only.
- **Implementation consequence to resolve in the contract**: docs/ goal
  specs declare `inherits: project`, whose root is specs/project.spec.md
  (the living constitution, not a historical task contract). Wholesale
  retirement must either relocate the constitution into the docs/ home or
  drop inheritance — to be decided in spec.md, not silently.
