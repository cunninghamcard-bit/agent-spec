# 0002 — Rename depth

- **Finding**: blast-radius survey (research.md F3) surfaced a real
  choice: rename only the product-facing identity, or also the internal
  namespace (skill directory names, `.agent-spec/` state directory,
  `agent-spec:*` HTML-comment marker prefixes).
- **Question**: How deep does the rename go?
- **Recommendation**: all the way — crate/binary/CLI text, skill
  directory names, `.agent-spec/` → `.docwright/`, and the marker prefix.
  We are the only user right now; zero external compatibility debt to
  protect, so there is no reason to leave an internal/external name
  mismatch for a future adopter to trip over.
- **User's answer**: Confirmed — 全改到底.
- **Scope this locks in**: skills/* and .claude/skills/* directories
  rename (agent-spec-sdd → docwright-sdd, etc.); `.agent-spec/` state dir
  → `.docwright/`; `RESEARCH_GENERATED_START/END`, `GOVERNS_MARKER`,
  `INTEGRATION_START/END` marker constants get the `docwright:` prefix;
  every CLI-facing string in src/main.rs; install-skills.sh;
  integrations.rs single source (regenerates AGENTS.md/.cursorrules/the
  Claude integration file, itself renamed from
  `agent-spec-tool-first.md`); tests updated to match (including
  `CARGO_BIN_EXE_agent-spec` → `CARGO_BIN_EXE_docwright`, a
  compile-required change once Cargo.toml's name changes).
