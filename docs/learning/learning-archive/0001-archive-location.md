# 0001 — Archive location

- **Question**: Where do research.md and learning-records/ go at
  graduation, now that deletion feels wrong?
- **Recommendation (first attempt, rejected)**: `.agents/learning/` — the
  user pointed out `.agents/` is Codex's tool-specific directory, distinct
  from `.claude/`; learning records belong to no single tool.
- **Recommendation (reframed)**: `docs/learning/<goal>/` — the single
  household: proposals in docs/*/, current-truth in docs/capabilities/,
  decision archaeology in docs/learning/. finish migrates instead of
  deleting; plan.md/tasks.md stay consumable.
- **User's answer**: Confirmed — docs/learning/<goal>/, research.md
  included, and backfill the already-graduated research-and-learn goal's
  records from git.
- **Overrides**: The research-and-learn contract's "removed at
  graduation" decision for research.md and learning-records/ (that
  maintained contract is amended by this goal).
