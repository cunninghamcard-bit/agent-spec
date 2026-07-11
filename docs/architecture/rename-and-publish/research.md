---
artifact: research
goal: "Rename And Publish"
derived_into: spec.md
---

# Rename And Publish — Research

> Follow every claim back to the source that owns it. Primary sources
> first; never trust parametric knowledge. See the agent-spec-research
> skill for the methodology.

## Unknowns

1. Is the `agent-spec` crate name actually taken, by whom, and is it a
   legitimate active project or an abandoned/squatted name we could
   request?
2. What does crates.io's own policy say about name collisions and
   transfers?
3. What naming convention fits a fork whose identity has diverged this
   far from upstream — is there real prior art, including from a sibling
   fork of the same upstream project?
4. What else changes when the crate is renamed (binary name, GitHub repo,
   embedded skill strings, Cargo.toml metadata) — is the blast radius
   bigger than "one string in Cargo.toml"?

## Industry Norms & Prior Art

- **crates.io is first-come-first-served, and `agent-spec` is legitimately
  owned, not squatted.** The sparse index
  (`https://index.crates.io/ag/en/agent-spec`, queried 2026-07-11) lists
  9 published versions, newest `0.3.0`, `pubtime: 2026-06-06`. The full
  metadata API confirms: description "AI-native BDD/Spec verification
  tool for contract-driven agent coding", homepage/repository
  `https://github.com/ZhangHanDong/agent-spec`, 626 downloads, 466 in the
  last 90 days. This is upstream's own crate, actively published as
  recently as five weeks ago — not abandoned. Source: crates.io sparse
  index + `GET /api/v1/crates/agent-spec`.
- **crates.io's own policy rules out requesting a transfer.** "crates.io
  has a first-come, first-serve policy on crate names... If you want to
  take over a package, we recommend you try and contact the current
  owner directly... For security reasons, the crates.io team will not
  transfer ownership of existing crates without the explicit approval of
  the current owner." Squatting-reclaim only applies to "a prolonged
  period of time... without any genuine functionality" — the opposite of
  what we found. Source:
  [crates.io Usage Policy](https://github.com/rust-lang/crates.io/blob/main/svelte/src/routes/policies/+page.svelte)
  (`+page.svelte`, read in full, "Package Ownership" section).
- **A sibling fork of the same upstream already solved this exact
  problem by renaming, not by asking for the name back.**
  `BUNotesAI/specwright` (explored earlier this session) forked
  `ZhangHanDong/agent-spec` and ships under a distinct identity,
  "-wright" suffix (craftsman metaphor: playwright, wheelwright,
  shipwright). Checked the crates.io sparse index for `specwright`
  itself: not yet published (`404`) — the repo-level rename didn't
  require a crates.io claim, or they haven't published yet either way.
  Source: session history (BUNotesAI/specwright exploration) +
  `https://index.crates.io/sp/ec/specwright` (2026-07-11).
- **The closest thing in scale on crates.io, `xchecker`
  (EffortlessMetrics), does not glue "spec" onto anything** — its
  description is "Rust CLI for spec pipelines (requirements→design→tasks)
  with lockfiles, safe fixups, and versioned receipts," but the name is
  abstract. `specdown` (32★, "test markdown files, drive development from
  documentation") does use a literal spec+suffix compound. Both patterns
  have precedent; neither is exclusively "the norm." Source:
  `gh search repos --topic bdd --language rust --sort stars` (2026-07-11).
- **Renaming a Rust CLI touches more than Cargo.toml's `name` field.**
  `Cargo.toml` also carries `repository`/`homepage` still pointing at
  `ZhangHanDong/agent-spec` (upstream, not our fork) — a pre-existing
  bug independent of the rename. The binary name (what users type),
  every embedded skill string (`skills/*/SKILL.md` reference `agent-spec
  <command>` throughout), README badges/title, and CHANGELOG all carry
  the current name. Source: `grep -n "^name\|^repository\|^homepage"
  Cargo.toml` (2026-07-11).

## Current Codebase State

<!-- agent-spec:generated:start -->
(no files matched the contract's Allowed Changes)
<!-- agent-spec:generated:end -->

## Findings

### F1: Is a rename actually required?

- **Decision**: Yes. `agent-spec` is unavailable — it is upstream's own
  actively-published crate, not squatted, so no transfer path exists
  under crates.io's stated policy.
- **Rationale**: Direct primary-source confirmation (sparse index +
  metadata API + policy page), not assumption.
- **Alternatives considered**: request the name from upstream (rejected:
  they are the legitimate active owner of their own project, asking to
  take their name would be an odd request, not a squatting-reclaim case);
  publish under a namespace/scope (rejected: crates.io has no
  npm-style `@scope/name` namespacing — flat global names only, per the
  same policy page).

### F2: Naming convention space

- **Decision**: two validated strategies, both with real prior art in
  this exact neighborhood — (a) craftsman-suffix metaphor
  (`-wright`/`-forge`), validated by the sibling fork's own choice; (b)
  literal contract/spec compound, validated by `specdown`. An abstract
  name (`xchecker`-style) is a third legitimate option but has no direct
  precedent tied to *this* upstream's naming lineage.
- **Rationale**: the sibling-fork precedent is the strongest signal
  because it is the same rename problem, already solved once by someone
  else forking the same project.
- **Alternatives considered**: reuse `specwright` itself (rejected:
  another fork's evident chosen identity — taking it would be
  confusing/bad-faith even though it is technically unclaimed on
  crates.io); n/a for others, this finding is presenting the space for
  the grill, not closing it.

### F3: Blast radius beyond Cargo.toml

- **Decision**: a rename must also cover the binary name, `repository`/
  `homepage` (currently wrong regardless — points at upstream, not our
  fork), every embedded skill's CLI-invocation strings, README title and
  examples, and CHANGELOG. This is comparable in shape to the workflow
  household rename already done for `specs/` → `docs/`.
- **Rationale**: grepped the actual surface area rather than assuming
  "just change one line."
- **Alternatives considered**: n/a — this is a scope finding, not a
  decision with alternatives.

### F4: New identity

- **Decision**: `docwright`. Chosen after the user redirected the
  candidate search away from spec/contract vocabulary toward what the
  project actually differentiates on now: documentation governance and
  learning capture (Doc Impact Guard, single docs/ household, Research
  and Learn, Learning Archive, docs/capabilities/). `docwright` maps
  directly onto the on-disk household the workflow-refactor goal just
  built.
- **Rationale**: `-wright` is a validated pattern (the sibling fork's own
  choice for the identical naming collision); `docwright` was available
  on crates.io.
- **Alternatives considered**: see the candidate table in this goal's
  learning record 0001 (contractwright, knowledgewright, docsteward,
  learnwright, and the spec/contract-anchored first round).

### F5: Rename depth

- **Decision**: full depth — crate/binary/CLI text, skill directory
  names, `.agent-spec/` state directory, and the `agent-spec:*` marker
  prefix all rename together (record 0002).
- **Rationale**: zero external adopters yet; no compatibility debt to
  protect by leaving an internal/external name mismatch.
- **Alternatives considered**: product-identity-only (rejected: leaves a
  mismatch for no compatibility benefit); Cargo.toml-only (rejected:
  ships a crate whose own CLI output still says the old name).
