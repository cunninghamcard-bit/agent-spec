# 0005 — Enforcement strength

- **Finding**: research.md F5 (Enforcement)
- **Question**: How hard does the machine enforce research — warnings
  only, or an error-level gate?
- **Recommendation**: Warning-level only (research-uncited,
  research-unfilled), because absence is legitimate (RFC: "no prior art is
  fine") and this repo's policy is escalate-after-noise-data.
- **User's answer**: **Override** — 触发条件命中即强制: when the trigger
  condition is met (an unresolved bracketed NEEDS-CLARIFICATION marker
  with no sibling research.md), enforcement is a hard error, not a
  warning. The two research-exists lints stay at warning level.
- **Overrides**: The recommendation and research.md F5's recorded
  decision. Shipped as the error-level `research-required` lint; spec.md
  Decision "Enforcement is hard at the trigger (research.md F5, user
  override)".
