# 0005 — Workflow invariants made explicit

- **Finding**: research.md F4 (Workflow invariants made explicit)
- **Question**: How is the "one active goal per worktree" invariant
  fixed — documentation, mechanical warning, or hard block?
- **Recommendation**: declaration plus mechanical warning — the
  agent-spec-sdd skill and README state the invariant; lifecycle prints a
  non-blocking warning when multiple goal folders carry uncommitted
  changes. Declaration serves readers; the warning serves executors who
  skip the docs.
- **User's answer**: Confirmed — 声明+机械警告.
- **Overrides**: Nothing. Hard blocking was rejected (false positives on
  legitimate states, e.g. a just-graduated goal awaiting commit).
