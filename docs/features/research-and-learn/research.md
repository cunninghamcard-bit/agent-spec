---
artifact: research
goal: "Research and Learn"
derived_into: spec.md
---

# Research and Learn — Research

> Every claim below cites the source that owns it. Primary sources were
> examined directly (repo trees, file contents via `gh api`); star counts
> read 2026-07-11.

## Unknowns

1. Must every spec be preceded by research, or is research demand-driven?
2. What artifact shape and finding format does the industry converge on?
3. How is the *human's* understanding (not just the agent's) established?
4. What research methodology is teachable and enforceable?

## Industry Norms & Prior Art

### Survey set

| Project | Scale | Form |
|---|---|---|
| mattpocock/skills | 165k★ | pure prompt pack: 101 md files, no product code |
| gsd-build/get-shit-done | 64.7k★ | prompt-orchestration system: 786 md + installer/SDK (`bin/install.js`, claude-agent-sdk dep) |
| Fission-AI/OpenSpec | 60k★ | SDD toolkit (not deep-read this round) |
| github/spec-kit | GitHub official | CLI + templates; plan Phase 0 generates research.md |
| DeepChat | reference corpus | 148 goals examined locally |
| rust-lang/rfcs | mature governance | RFC template with embedded Prior art section |

### Findings per source

- **DeepChat corpus** (local scan): research.md exists in **1 of 148** goal folders (`docs/features/agent-memory/research.md`) — research is exceptional, reserved for the least-familiar domain. Source: local find over DeepChat@18820651.
- **spec-kit**: plan's Phase 0 turns each `[NEEDS CLARIFICATION]` into a research task; findings consolidate into research.md as **Decision / Rationale / Alternatives considered**. Research exists to resolve unknowns, not as ritual. Source: [spec-kit plan command template](https://github.com/github/spec-kit/blob/main/templates/commands/plan.md).
- **Rust RFC**: Prior art is an embedded required *section* (not a separate document); "If there is no prior art, that is fine." Source: [RFC 2333](https://rust-lang.github.io/rfcs/2333-prior-art.html), [RFC template](https://github.com/rust-lang/rfcs/blob/master/0000-template.md).
- **mattpocock/skills `research`**: ~12 lines; one hard rule — investigate against **primary sources**, "follow every claim back to the source that owns it"; output one Markdown file with per-claim citations; run as a background agent. Source: skills/engineering/research/SKILL.md (read in full).
- **mattpocock/skills `grilling`**: "Interview me relentlessly ... one at a time ... For each question, provide your recommended answer. If a fact can be found by exploring the codebase, look it up rather than asking me. The decisions, though, are mine." No enactment before shared understanding. Source: skills/productivity/grilling/SKILL.md (read in full).
- **mattpocock/skills `teach`**: "Never trust your parametric knowledge"; distinguishes fluency strength from storage strength; retrieval practice over passive reading. Source: skills/productivity/teach/SKILL.md (read in full).
- **gsd**: research is a first-class multi-agent phase — six specialized researchers (domain/project/phase/ui/ai/advisor) plus a research-synthesizer; researcher prompts carry concrete search-query templates and a structured output format where every finding requires a **Source** field. Source: agents/gsd-domain-researcher.md (read in full), repo tree.

## Current Codebase State

<!-- agent-spec:generated:start -->
Manually surveyed this round (the `research` command that will own this
region does not exist yet):

- `[NEEDS CLARIFICATION]` gate: needs-clarification lint is error-level and
  source-scanning (src/spec_lint/linters.rs); adopted from spec-kit per
  docs/comparison-openspec-speckit.md line 554.
- Goal packages: init --kind scaffolds spec/plan/tasks (src/main.rs,
  generate_sdd_* fns); finish removes consumables; guard multi-dir.
- Codebase scanning already exists: plan command's build_plan_context
  (src/spec_gateway/plan.rs) produces file summaries, pub APIs, existing
  tests from Allowed Changes paths.
- Managed-region precedent: integrate's AGENTS.md/CLAUDE.md markers;
  doc_impact's governs markers ignore fenced examples.
- agent-spec-sdd skill owns the workflow; no research or grill step today.
<!-- agent-spec:generated:end -->

## Findings

### F1: Research trigger

- **Decision**: demand-driven, never universal. Triggers: (a) a
  `[NEEDS CLARIFICATION]` marker that cannot be resolved from the codebase
  alone; (b) the goal's domain is unfamiliar to the team.
- **Rationale**: unanimous across all five surveyed sources (DeepChat 1/148,
  spec-kit unknown-driven Phase 0, RFC "no prior art is fine", Pocock
  on-demand delegation, gsd orchestrator-dispatched).
- **Alternatives considered**: mandatory for all goals (rejected: no
  industry precedent, ritualizes the artifact); mandatory for
  feature/architecture only (rejected: still flat-rate, not need-based).

### F2: Artifact & format

- **Decision**: research.md in the goal folder; findings as
  Decision/Rationale/Alternatives with per-claim Source; codebase-state
  section machine-refreshed inside managed markers.
- **Rationale**: spec-kit owns the D/R/A precedent; Pocock/gsd own the
  citation discipline; our own integrate/doc-impact own the marker
  mechanics; plan's scanner already produces the codebase half.
- **Alternatives considered**: embedded spec.md section like Rust RFC
  (rejected: research is consumable, spec is durable — mixing lifecycles
  is the rot pattern we just eliminated); standalone docs/research/ dir
  like gsd (rejected: goal-scoped materials belong in the goal folder).

### F3: The learning half

- **Decision**: a grill session grounded in research.md — the agent puts
  each Finding's Decision to the user one at a time with a recommended
  answer; confirmed decisions flow into spec.md Decisions; spec authoring
  does not start before the grill completes.
- **Rationale**: Pocock's grilling ("decisions are mine") + teach
  (retrieval practice beats passive reading; fluency vs storage). A
  read-this-file checkbox is fake-able; an interview transcript is not.
- **Alternatives considered**: passive review gate checkbox (rejected:
  self-reported, the DeepChat drift pattern); quiz on facts (rejected:
  the user needs to own decisions, not recite trivia).

### F4: Methodology content

- **Decision**: a skill reference (research-methods.md) teaching: primary
  sources first; never trust parametric knowledge; per-claim citation; the
  gh toolkit (topic search → tree scan → file read → in-repo code search →
  issues/discussions for rationale); compare N implementations before
  concluding.
- **Rationale**: gsd encodes query recipes in researcher prompts; Pocock
  encodes the source discipline; our own ecosystem blind spots this round
  (missed 165k★ and 64.7k★ repos twice) prove parametric knowledge fails
  for fast-moving ecosystems.
- **Alternatives considered**: leave methodology implicit (rejected: the
  gap this feature exists to close).

### F5: Enforcement

- **Decision**: warning-level lint only when research.md exists
  (spec-does-not-cite-research; research-has-unfilled-placeholders).
  Grill completion is skill-owned, not lint-checkable.
- **Rationale**: absence is legitimate (RFC); escalate-after-noise-data is
  this repo's established gating policy; human interaction cannot be
  mechanically verified without theater.
- **Alternatives considered**: error-level (rejected: would force ritual
  research); no lint (rejected: cited-but-stale research would rot
  silently).
