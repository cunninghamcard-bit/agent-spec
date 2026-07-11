# 0003 — Where research.md lives

- **Finding**: research.md F2 (Artifact & format)
- **Question**: Does research live as a section inside spec.md (Rust RFC
  style), in a repo-wide docs/research/ directory (gsd style), or as
  research.md inside the goal folder (spec-kit style)?
- **Recommendation**: Goal-folder research.md. Research is a consumable
  with the goal's lifecycle; spec.md is durable — mixing lifecycles is the
  doc-rot pattern the finish command exists to eliminate.
- **User's answer**: Confirmed — goal 内 research.md.
- **Overrides**: Nothing. Adopted as spec.md Decision "research.md lives
  in the goal folder", with finish treating it as a consumable.
