# agent-spec CLI Command Reference

## All Commands

```
agent-spec <COMMAND>

Commands:
  parse               Parse .spec/.spec.md files and show AST
  lint                Analyze spec quality (detect smells)
  verify              Verify code against specs
  init                Create a starter .spec.md file
  lifecycle           Run full lifecycle: lint -> verify -> report
  brief               Compatibility alias for the contract view
  contract            Render an explicit Task Contract for agent execution
  guard               Git guard: lint all specs + verify against change scope
  graph               Generate dependency graph from spec files (DOT/SVG)
  explain             Generate a human-readable contract review summary
  stamp               Preview git trailers for a verified contract
  checkpoint          Preview or create a VCS checkpoint
  resolve-ai          Merge external AI decisions into a verification report
  measure-determinism [Experimental] Measure contract verification determinism
  install-hooks       Install git hooks for automatic spec checking
```

## Core Flow

```bash
# 1. Read the contract
agent-spec contract specs/task.spec

# 2. Implement code...

# 3. Verify
agent-spec lifecycle specs/task.spec --code . --format json

# 4. Repo-wide guard
agent-spec guard --spec-dir specs --code .
```

## contract

```bash
agent-spec contract <spec> [--format text|json]
```

Renders the Task Contract with: Intent, Must/Must NOT, Decisions, Boundaries, Completion Criteria.

## lifecycle

```bash
agent-spec lifecycle <spec> --code <dir> \
  [--change <path>]... \
  [--change-scope none|staged|worktree|jj] \
  [--ai-mode off|stub] \
  [--min-score 0.6] \
  [--format text|json|md|compact|diagnostic] \
  [--run-log-dir <dir>] \
  [--adversarial] \
  [--layers lint,boundary,test,ai,complexity] \
  [--resume[=conservative]] \
  [--review-mode auto|strict]
```

Full pipeline: lint -> verify -> report. Default format is `json`.

New flags:
- `--format compact` — single-line per scenario, human-readable: `[PASS] 场景名 [FAIL] 场景名`
- `--format diagnostic` — JSON with full stdout/stderr from test runs
- `--resume` — skip already-passed scenarios (incremental mode)
- `--resume=conservative` — rerun all but detect regressions
- `--review-mode auto` (default) — treat `pending_review` as pass
- `--review-mode strict` — treat `pending_review` as non-passing

## guard

```bash
agent-spec guard \
  [--spec-dir specs] \
  [--code .] \
  [--change <path>]... \
  [--change-scope staged|worktree] \
  [--min-score 0.6]
```

Scans all `*.spec` and `*.spec.md` files in `--spec-dir`, runs lint + verify on each. Default change scope is `staged`.

## verify

```bash
agent-spec verify <spec> --code <dir> \
  [--change <path>]... \
  [--change-scope none|staged|worktree] \
  [--ai-mode off|stub] \
  [--format text|json|md]
```

Raw verification without lint quality gate. Default change scope is `none`.

## explain

```bash
agent-spec explain <spec> \
  [--code .] \
  [--format text|markdown] \
  [--history]
```

Human-readable contract review summary. Use `--format markdown` for PR descriptions. Use `--history` to include run log history. In jj repos, `--history` also shows file-level diffs between adjacent runs via operation IDs.

## stamp

```bash
agent-spec stamp <spec> [--code .] [--dry-run]
```

Preview git trailers (`Spec-Name`, `Spec-Passing`, `Spec-Summary`). Currently only `--dry-run` is supported.

In jj repositories, also outputs `Spec-Change:` trailer with the current jj change ID.

## lint

```bash
agent-spec lint <files>... [--format text|json|md] [--min-score 0.0]
```

Built-in linters: VagueVerb, Unquantified, Testability, Coverage, Determinism, ImplicitDep, ExplicitTestBinding, Sycophancy.

## init

```bash
agent-spec init --kind feature|issue|architecture [--root docs] --name <goal>
```

Both `--kind` and `--name` are required. Init creates a DeepChat-style goal
package: feature and architecture receive `spec.md`, `plan.md`, and
`tasks.md`; issue receives `spec.md`. Generated structural keywords are
English-only.

## Change Set Defaults

| Command | `--change-scope` default |
|---------|-------------------------|
| verify | `none` |
| lifecycle | `none` |
| guard | `staged` |

## resolve-ai

```bash
agent-spec resolve-ai <spec> \
  [--code .] \
  --decisions <decisions.json> \
  [--format text|json]
```

Merges external AI decisions into a verification report. Used as step 2 of the caller mode protocol:
1. `lifecycle --ai-mode caller` emits pending requests to `.agent-spec/pending-ai-requests.json`
2. Agent analyzes scenarios and writes `ScenarioAiDecision` JSON
3. `resolve-ai` merges decisions, replacing Skip verdicts with AI verdicts

The decisions file format:
```json
[
  {
    "scenario_name": "场景名称",
    "model": "claude-agent",
    "confidence": 0.92,
    "verdict": "pass",
    "reasoning": "All steps verified"
  }
]
```

Cleans up `pending-ai-requests.json` after successful merge.

## AI Mode

- `off` (default) - No AI verification layer
- `stub` - Returns `uncertain` for all scenarios (testing/scaffolding)
- `caller` - Agent-as-verifier: emits `AiRequest` JSON, resolved via `resolve-ai`
- `external` - Reserved for host-injected `AiBackend` trait implementations

## Verification Layers

Use `--layers` to select which verification layers to run:

```bash
# Only lint and boundary checking
agent-spec lifecycle specs/task.spec --code . --layers lint,boundary

# Skip lint, run structural + boundary + test
agent-spec lifecycle specs/task.spec --code . --layers boundary,test
```

Available layers: `lint`, `boundary`, `test`, `ai`, `complexity`

## graph

```bash
agent-spec graph \
  [--spec-dir specs] \
  [--format dot|svg]
```

Scans all spec files in `--spec-dir`, extracts `depends` and `estimate` from frontmatter, and generates a DOT dependency graph.

- Nodes use `box` shape (pending) or `doubleoctagon` (completed, tagged `done`/`completed`)
- Node labels include spec name + estimate (e.g., `"Goal Gate\n[0.5d]"`)
- Edges represent dependency relationships
- Critical path edges highlighted in red (`color=red, penwidth=2.0`)
- `--format svg` pipes DOT through system `dot` command (requires graphviz installed)

Example:

```bash
# Generate DOT and view
agent-spec graph --spec-dir specs/roadmap

# Generate SVG
agent-spec graph --spec-dir specs/roadmap --format svg > deps.svg
```

## Frontmatter: depends and estimate

Spec-level dependency and effort fields in frontmatter:

```yaml
spec: task
name: "检查点与增量重跑"
inherits: project
tags: [bootstrap, lifecycle, phase8]
depends: [task-goal-gate, task-context-fidelity]
estimate: 1d
---
```

- `depends`: list of spec file stems or spec names this spec depends on
- `estimate`: effort estimate string (`0.5d`, `1d`, `2d`, `1w`, `4h`)
- Both fields are optional; specs without them still work normally
- Used by `agent-spec graph` to generate dependency visualization and critical path

## Six Verdicts

| Verdict | Meaning | Action |
|---------|---------|--------|
| `pass` | Scenario verified | No action needed |
| `fail` | Scenario failed verification | Read evidence, fix code |
| `skip` | Test not found or not run | Check `Test:` selector matches a real test name |
| `uncertain` | AI stub / manual review needed | Review manually or enable AI backend |
| `pending_review` | Test passed but needs human review | Human reviews, or `--review-mode auto` treats as pass |

## Scenario DSL Extensions

### Critical tags (Goal Gate)

```spec
Scenario: 用户注册成功（critical）
  Tags: critical
```

- `critical` scenarios failing → `gate_blocked=true` in JSON, exit code 2
- Name suffix `（critical）`/`(critical)` also works as shorthand

### Review mode

```spec
Scenario: 安全审核
  Review: human
```

- `Review: human` → verdict becomes `pending_review` when test passes
- `--review-mode auto` (default): treats as pass; `--review-mode strict`: treats as non-pass

### Optimize mode

```spec
Scenario: 性能优化
  Mode: optimize
```

- `Mode: optimize` → scenario listed in `optimization_candidates` when pass
- Fail still blocks `passed: false` (optimize is a floor, not a ceiling)

### Scenario dependencies

```spec
Scenario: 用户登录
  Depends: 用户注册
```

- `Depends:` → lifecycle executes in topological order
- Prerequisite fail → dependent scenario auto-skipped with evidence
- Circular dependencies detected by lint
