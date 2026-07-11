# 0009 — Every command gets a station

- **Finding**: research.md F2 final round (supersedes the two-tier
  Workflow/Utilities framing)
- **Question**: Which commands enter the core loop, with the rest
  documented in a Utilities tier?
- **Recommendation**: loop adds stamp + promote; install-hooks via
  integrate; the rest tiered as Utilities with situational docs.
- **User's answer**: **Override** — "全部加上啊，我们没说要删除啊。我觉得
  你这是在偷懒." No second-class tier. The refactored workflow is a map
  of flows in which every one of the 25 subcommands has a declared
  station and trigger:
  - Adoption flow (once per repo): integrate → install-hooks (fixed to
    the docs/ home) → discover (brownfield cold start)
  - Goal lifecycle flow (per goal): init → research/grill → spec →
    lint/contract → plan → implement → lifecycle (parse/verify/matrix as
    its debugging internals) → guard → stamp at commit → finish →
    promote into docs/capabilities/
  - Review flow (per review): explain → matrix → contract/brief
  - Library governance flow (periodic): audit → graph
  - Probe & AI flow (when triggered): check-structure (structural
    boundaries), resolve-ai + measure-determinism (AI-assisted verdicts)
  Help output groups by these flows; skills document every station.
- **Overrides**: The two-tier recommendation. Situational commands get
  situational stations with declared triggers (the same demand-driven
  pattern as research), not a drawer.
