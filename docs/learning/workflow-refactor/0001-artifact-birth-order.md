# 0001 — Artifact birth order

- **Finding**: research.md F1 (Artifact birth order)
- **Question**: Should `init --kind` keep scaffolding spec.md + plan.md +
  tasks.md in one shot?
- **Recommendation**: Staged birth — init creates the goal folder and a
  spec.md skeleton only; plan.md and tasks.md are born by `plan --out`
  when the planning step arrives. No surveyed source (spec-kit, DeepChat,
  OpenSpec) does a one-shot trio; plan.md loses its dual ownership.
- **User's answer**: Confirmed — 分步出生.
- **Overrides**: The current init behavior (a mechanization choice made in
  the DeepChat-fusion round, not a DeepChat mandate).
