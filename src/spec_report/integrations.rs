//! Single-source multi-tool integration generation (Phase 6).
//!
//! One canonical [`integration_body`] is rendered into per-tool formats so the
//! agent instructions never drift across `AGENTS.md`, `.cursorrules`, and the
//! Claude skill.

/// The canonical tool-first integration instructions — the single source.
pub fn integration_body() -> String {
    "\
agent-spec is an AI-native BDD/spec verification tool. Use it tool-first:\n\
\n\
1. For substantial new work without a contract, create the goal folder and its\n\
   contract skeleton: `agent-spec init --kind feature|issue|architecture --name <goal>`.\n\
2. Read the Task Contract: `agent-spec contract <spec>`.\n\
3. Generate plan context (births plan.md and tasks.md beside the contract):\n\
   `agent-spec plan <spec> --code . --out <goal>/plan.md`.\n\
4. Implement within the contract's Boundaries.\n\
5. Verify: `agent-spec lifecycle <spec> --code . --format json` — fix until all\n\
   scenarios pass (failed/skipped/uncertain all 0). Do not edit the spec to pass.\n\
6. Repo-level gate before committing: `agent-spec guard --spec-dir docs --code .`.\n\
7. Commit trailers come from the machine: `agent-spec stamp <spec> --code . --dry-run`.\n\
8. Graduate: `agent-spec finish <spec> --code .`; lift durable Rules with\n\
   `agent-spec promote` into `docs/capabilities/`.\n\
9. Render the PR acceptance summary: `agent-spec explain <spec> --code . --format markdown`.\n\
\n\
The machine verifies whether the code satisfies the contract; you implement\n\
against it, and a human reviews the contract."
        .to_string()
}

/// Integration output targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationTarget {
    Agents,
    Cursor,
    Claude,
}

impl IntegrationTarget {
    /// Parse a target name. `Err` for unknown targets (never panics).
    pub fn parse(name: &str) -> Result<Self, String> {
        match name {
            "agents" => Ok(Self::Agents),
            "cursor" => Ok(Self::Cursor),
            "claude" => Ok(Self::Claude),
            other => Err(format!(
                "unknown integration target `{other}` (expected agents/cursor/claude)"
            )),
        }
    }

    /// Conventional output filename for this target.
    pub fn filename(self) -> &'static str {
        match self {
            Self::Agents => "AGENTS.md",
            Self::Cursor => ".cursorrules",
            Self::Claude => "agent-spec-tool-first.md",
        }
    }
}

/// Render the canonical body into the given target's format.
pub fn render_target(target: IntegrationTarget, body: &str) -> String {
    match target {
        IntegrationTarget::Agents => {
            format!("# agent-spec — Agent Instructions\n\n{body}\n")
        }
        IntegrationTarget::Cursor => {
            format!("# agent-spec rules (Cursor)\n\n{body}\n")
        }
        IntegrationTarget::Claude => {
            format!(
                "---\nname: agent-spec-tool-first\ndescription: Use agent-spec as a CLI tool to verify code against Task Contracts.\n---\n\n{body}\n"
            )
        }
    }
}

/// Render a target by name (single entry point used by the CLI).
pub fn render_named(target: &str) -> Result<String, String> {
    let t = IntegrationTarget::parse(target)?;
    Ok(render_target(t, &integration_body()))
}

/// Whether existing content has drifted from the freshly-rendered output.
pub fn has_drifted(existing: &str, rendered: &str) -> bool {
    existing.trim() != rendered.trim()
}

// ── Target-project integration (the `integrate` command) ───────────────

pub const INTEGRATION_START: &str = "<!-- agent-spec:integration:start -->";
pub const INTEGRATION_END: &str = "<!-- agent-spec:integration:end -->";

/// The managed policy block written into a target project's AGENTS.md and
/// CLAUDE.md. Policy only: what applies, where artifacts live, what is
/// authoritative, and which skills own the workflow. No fenced code blocks,
/// no CLI invocations — operations are exposed through the CLI and skills.
pub const POLICY_BLOCK: &str = "\
## Spec-Driven Development

Use SDD before substantial changes to code, tests, configuration, or
structure when the work needs shared context or a durable decision record.
Skip SDD for trivial or tightly localized work unless explicitly asked.

Each substantial goal lives in one kebab-case folder: docs/features/<goal>
for new capabilities, docs/issues/<goal> for complex bugs, and
docs/architecture/<goal> for refactors and cross-module design. The goal's
spec.md is the authoritative contract — human-readable and mechanically
verified by agent-spec. plan.md and tasks.md are execution materials and
never override it.

Resolve every bracketed NEEDS-CLARIFICATION marker before implementation;
the lint gate enforces this. Verified goals graduate: consumable artifacts
are removed, durable rules are promoted, and history stays in git.

The agent-spec-sdd skill owns goal classification and the workflow. Command
usage lives in the agent-spec-tool-first skill; contract authoring guidance
lives in the agent-spec-authoring skill.";

/// Insert or refresh the managed block in a declaration file's content.
/// Missing markers: append (preserving existing content). Present markers:
/// replace only what is between them.
pub fn upsert_managed_block(existing: &str, block: &str) -> String {
    let framed = format!("{INTEGRATION_START}\n{block}\n{INTEGRATION_END}");
    if let (Some(start), Some(end)) = (
        existing.find(INTEGRATION_START),
        existing.find(INTEGRATION_END),
    ) && start < end
    {
        let after = end + INTEGRATION_END.len();
        return format!("{}{}{}", &existing[..start], framed, &existing[after..]);
    }
    if existing.trim().is_empty() {
        return format!("{framed}\n");
    }
    let sep = if existing.ends_with('\n') {
        "\n"
    } else {
        "\n\n"
    };
    format!("{existing}{sep}{framed}\n")
}

/// The workflow skills embedded at compile time so `integrate` can install
/// them into target projects without a network or source checkout.
pub const EMBEDDED_SKILLS: &[(&str, &[(&str, &str)])] = &[
    (
        "agent-spec-sdd",
        &[(
            "SKILL.md",
            include_str!("../../skills/agent-spec-sdd/SKILL.md"),
        )],
    ),
    (
        "agent-spec-tool-first",
        &[
            (
                "SKILL.md",
                include_str!("../../skills/agent-spec-tool-first/SKILL.md"),
            ),
            (
                "references/commands.md",
                include_str!("../../skills/agent-spec-tool-first/references/commands.md"),
            ),
        ],
    ),
    (
        "agent-spec-research",
        &[(
            "SKILL.md",
            include_str!("../../skills/agent-spec-research/SKILL.md"),
        )],
    ),
    (
        "agent-spec-authoring",
        &[
            (
                "SKILL.md",
                include_str!("../../skills/agent-spec-authoring/SKILL.md"),
            ),
            (
                "references/patterns.md",
                include_str!("../../skills/agent-spec-authoring/references/patterns.md"),
            ),
        ],
    ),
];

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_policy_block_is_prose_only() {
        assert!(
            !POLICY_BLOCK.contains("```"),
            "policy block must contain no fenced code"
        );
        for line in POLICY_BLOCK.lines() {
            let t = line.trim();
            assert!(
                !t.starts_with("agent-spec ") && !t.starts_with("$"),
                "policy block must not invoke the CLI: {t}"
            );
        }
        assert!(POLICY_BLOCK.contains("agent-spec-sdd skill"));
    }

    #[test]
    fn test_upsert_managed_block_appends_and_replaces() {
        let created = upsert_managed_block("", "policy v1");
        assert!(created.starts_with(INTEGRATION_START));
        assert!(created.contains("policy v1"));

        let appended = upsert_managed_block("# My Project\n\ncustom rules\n", "policy v1");
        assert!(appended.starts_with("# My Project"));
        assert!(appended.contains("custom rules"));
        assert!(appended.contains("policy v1"));

        let refreshed = upsert_managed_block(&appended, "policy v2");
        assert!(refreshed.contains("policy v2"));
        assert!(!refreshed.contains("policy v1"));
        assert!(refreshed.starts_with("# My Project"));
        assert_eq!(refreshed.matches(INTEGRATION_START).count(), 1);
    }

    #[test]
    fn test_all_targets_share_integration_body() {
        let body = integration_body();
        for t in [
            IntegrationTarget::Agents,
            IntegrationTarget::Cursor,
            IntegrationTarget::Claude,
        ] {
            let out = render_target(t, &body);
            assert!(
                out.contains("lifecycle"),
                "{t:?} must carry the shared body"
            );
            assert!(out.contains("guard"), "{t:?} must carry the shared body");
        }
    }

    #[test]
    fn test_integration_body_is_tool_first() {
        let b = integration_body();
        assert!(b.contains("contract"));
        assert!(b.contains("init --kind"));
        assert!(b.contains("lifecycle"));
        assert!(b.contains("guard"));
    }

    #[test]
    fn test_claude_target_has_frontmatter() {
        let out = render_target(IntegrationTarget::Claude, &integration_body());
        assert!(
            out.starts_with("---"),
            "claude target must start with frontmatter"
        );
        assert!(out.contains("name:"));
    }

    #[test]
    fn test_agents_target_is_plain_markdown() {
        let out = render_target(IntegrationTarget::Agents, &integration_body());
        assert!(
            !out.starts_with("---"),
            "agents target must not be frontmatter"
        );
        assert!(out.contains("# "));
    }

    #[test]
    fn test_unknown_target_errors() {
        assert!(IntegrationTarget::parse("vim").is_err());
        assert!(render_named("vim").is_err());
    }

    #[test]
    fn test_check_passes_when_content_matches() {
        let rendered = render_named("agents").unwrap();
        assert!(
            !has_drifted(&rendered, &rendered),
            "identical content must not drift"
        );
    }

    #[test]
    fn test_check_reports_drift_when_different() {
        let rendered = render_named("agents").unwrap();
        assert!(has_drifted("stale hand-edited content", &rendered));
    }
}
