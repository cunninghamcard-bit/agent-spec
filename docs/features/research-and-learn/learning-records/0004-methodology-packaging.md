# 0004 — How the methodology ships

- **Finding**: research.md F4 (Methodology content)
- **Question**: Does the research methodology (primary sources, gh
  toolkit, per-claim citation, never trust parametric knowledge) live as a
  reference file inside agent-spec-sdd, or as a standalone skill?
- **Recommendation**: Standalone `agent-spec-research` skill,
  cross-linked from the others — the Pocock composition model: primitives
  plus thin composers, skills invoke each other by name instead of
  duplicating content.
- **User's answer**: Confirmed — 独立 research skill，且必须联动
  ("可以的得联动起来啊，我们也需要联动"): sdd invokes it at the research
  step, authoring consumes grilled decisions, tool-first documents the
  command surface.
- **Overrides**: Nothing; the answer hardened the linkage requirement into
  a spec.md Decision ("Skills are linked, not siloed").
