spec: task
name: "Integrate Command"
inherits: project
tags: [feature, sdd, cli, integration]
---

## Intent

Make adopting agent-spec in a target project a one-command step, with a
strict separation of concerns: AGENTS.md and CLAUDE.md carry only a short
policy block (when SDD applies, where artifacts live, what is authoritative),
while every concrete operation is exposed through the CLI and the installed
skills. Upgrading agent-spec and re-running integrate refreshes governance in
every project without hand-editing agent instruction files.

## Current State

Skills install only to `~/.claude/skills` via install-skills.sh; Codex reads
project-level `.agents/skills/` (as DeepChat ships) but nothing installs
there. There is no managed declaration block for target projects: adopting
agent-spec means hand-writing AGENTS.md/CLAUDE.md guidance, which drifts and
tends to accumulate command snippets that belong in skills. Generated goal
specs carry no test binding, so non-Rust projects must hand-author
`test_command`/`test_report`.

## UX Shape

```text
$ cd ~/projects/my-ts-app
$ agent-spec integrate
installed .agents/skills/agent-spec-sdd
installed .agents/skills/agent-spec-tool-first
installed .agents/skills/agent-spec-authoring
installed .claude/skills/agent-spec-sdd (+2 more)
updated AGENTS.md (managed policy block)
updated CLAUDE.md (managed policy block)

$ agent-spec init --kind feature --name "User Onboarding"
(generated spec.md frontmatter already contains the vitest test binding)
```

## Decisions

- New subcommand `integrate` with `--into <path>` defaulting to the current directory
- The three workflow skills (agent-spec-sdd, agent-spec-tool-first, agent-spec-authoring) are embedded in the binary at compile time and written to the target project's `.agents/skills/` and `.claude/skills/`, overwriting stale copies so re-running integrate upgrades them
- A managed policy block is written to both AGENTS.md and CLAUDE.md between `<!-- agent-spec:integration:start -->` and `<!-- agent-spec:integration:end -->` markers: missing file is created, file without markers gets the block appended, existing markers get the content between them replaced
- The policy block is prose-only: it must contain no fenced code blocks and no CLI invocations; classification, artifact layout, contract authority, the clarification rule, and graduation policy are stated as policy, and the skills are named as the owners of workflow and command usage
- The policy block lives as one canonical constant beside the other integration bodies in spec_report/integrations.rs
- `init --kind` detects the target project type: when `package.json` exists and no `Cargo.toml`, the generated goal `spec.md` frontmatter is prefilled with a vitest JUnit `test_command`/`test_report` binding; otherwise no binding lines are emitted
- Skill installation and block writing are idempotent: re-running integrate produces identical results

## Boundaries

### Allowed Changes
- src/main.rs
- src/spec_report/integrations.rs
- tests/**
- README.md
- CHANGELOG.md
- docs/features/integrate-command/**
- install-skills.sh

### Forbidden
- Do not put fenced code blocks or CLI invocations into the managed policy block
- Do not touch target-project content outside the managed markers, the skills directories, and file creation
- Do not add new dependencies

## Out of Scope

- Global Codex skill installation (project-level `.agents/skills/` only)
- Pre-commit hook installation from integrate (install-hooks already exists)
- Refreshing skills in `~/.claude/skills` (install-skills.sh owns the global copy)
- Windows path separators in printed output

## Completion Criteria

Scenario: integrate installs skills for both agent systems
  Test:
    Package: agent-spec
    Filter: test_cli_integrate_installs_skills_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given an empty target project directory
  When integrate runs
  Then `.agents/skills/agent-spec-sdd/SKILL.md` exists
  And `.claude/skills/agent-spec-sdd/SKILL.md` exists
  And both tool-first and authoring skills exist in both trees

Scenario: integrate writes the managed policy block to both declaration files
  Test:
    Package: agent-spec
    Filter: test_cli_integrate_writes_policy_blocks_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given a target project without AGENTS.md or CLAUDE.md
  When integrate runs
  Then both files exist and contain the managed markers
  And the block names the agent-spec-sdd skill as the workflow owner

Scenario: the policy block contains no operations
  Test:
    Package: agent-spec
    Filter: test_integration_policy_block_is_prose_only
  Given the canonical policy block
  When its content is inspected
  Then it contains no fenced code block
  And no line invokes the agent-spec CLI

Scenario: integrate preserves surrounding content and is idempotent
  Test:
    Package: agent-spec
    Filter: test_cli_integrate_preserves_and_refreshes_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given an AGENTS.md with custom content above and below an existing managed block
  When integrate runs twice
  Then the custom content is unchanged
  And exactly one managed block remains with refreshed content

Scenario: init prefills the vitest binding for Node projects
  Test:
    Package: agent-spec
    Filter: test_cli_init_prefills_vitest_binding_for_node_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given a target directory containing package.json and no Cargo.toml
  When init creates a feature goal
  Then the generated spec.md frontmatter contains a vitest test_command
  And a test_report path

Scenario: init emits no binding for Cargo projects
  Test:
    Package: agent-spec
    Filter: test_cli_init_omits_binding_for_cargo_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given a target directory containing Cargo.toml
  When init creates a feature goal
  Then the generated spec.md frontmatter contains no test_command line

## Open Questions

None.
