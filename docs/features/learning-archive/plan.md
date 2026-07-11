=== Contract ===

# Task Contract: Learning Archive

## Intent
Graduation currently deletes research.md and learning-records/ — the
user's real learning trail dies with the goal. Preserve it: finish
migrates both into docs/learning/<goal>/, the decision-archaeology layer
of the single household, complementing docs/capabilities/ (capabilities
say what the rules are now; learning says why they were decided, and
which recommendations were overridden). Decided in learning record 0001
of this goal; tool-specific homes (.agents/, .claude/) were rejected
because learning belongs to no single agent tool.

## Current State
finish removes plan.md, tasks.md, research.md, and learning-records/
after full verification (finish-command and research-and-learn
contracts). The already-graduated research-and-learn goal's five records
survive only in git history (commit 911d5da).

## Must
- 任务合约以 goal 包形式存放在 `docs/features|issues|architecture/<goal>/`（单一户口）
- 公开 CLI 与 gateway 行为必须有回归测试
- DSL 语法变更必须同时更新 AST、解析输出和回归测试
- 验证结果必须区分 `pass`、`fail`、`skip`、`uncertain`
- 任务级完成条件中的每个场景应显式声明 `测试:` selector
- 任务级边界应支持对显式 change set 的机械验证
- 测试选择器应支持结构化字段，而不仅是裸字符串过滤器
- guard 应支持可选择的 git change scope，而不局限于 staged index
- verify 与 lifecycle 应支持可选的 git change scope，同时保持默认行为稳定
- AI verifier 的 `uncertain` 结果应附带结构化 `AiAnalysis` 证据
- AI verifier 应通过可插拔 backend 接口产生结构化分析结果
- agent-spec 应保持 provider-agnostic，由宿主 agent 注入 AI backend
- 项目应提供 Claude Code 的 project-local skills，且主路径是 tool-first
- 耐久能力规则经 `promote` 累积于 `docs/capabilities/`（真相层）；历史从 git 恢复，不设归档目录
- Task Contract 应区分 `Must`、`Must Not` 与 `Decisions`
- 默认文本 `contract` 输出应保留结构化 Completion Criteria 细节

## Must NOT
- 不要把 `skip` 记为 `pass`
- 不要要求普通磁盘用例手工提供继承搜索路径
- 不要丢弃 BDD 步骤里的结构化输入

## Decisions
- finish archives instead of deleting: research.md and learning-records/ move to docs/learning/<goal>/ at graduation (learning record 0001); plan.md and tasks.md remain deleted consumables
- The archive root is the nearest docs/ ancestor of the goal spec, mirroring promote's household resolution
- Collisions are refused: if docs/learning/<goal>/ already holds a file finish would write, abort before moving anything
- The research-and-learn and finish-command maintained contracts are amended to state the archive behavior
- The research-and-learn goal's five learning records and research.md are backfilled from git history into docs/learning/research-and-learn/
- The agent-spec-research and agent-spec-sdd skills state the archive destination

## Boundaries
Allowed changes:
- src/main.rs
- skills/**
- .claude/skills/**
- tests/finish_cli_e2e.rs
- README.md
- CHANGELOG.md
- docs/learning/**
- docs/features/learning-archive/**
- docs/features/research-and-learn/spec.md
- docs/architecture/finish-command/spec.md
Forbidden:
- Do not keep plan.md or tasks.md at graduation
- Do not write the archive to tool-specific directories (.agents/, .claude/)
Out of scope:
- Aggregating or indexing the archive (a flat per-goal folder is enough)
- Archiving goals retired via finish --retire (withdrawal is not graduation)

## Completion Criteria
Scenario: graduation archives the learning trail
  Test:
    Package: agent-spec
    Filter: test_cli_finish_archives_research_and_learning_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given a verified goal package containing research.md and learning-records
  When finish runs
  Then research.md and the learning records exist under docs/learning/<goal>/
  And they are gone from the goal folder
  And plan.md and tasks.md are deleted

Scenario: archive collisions abort the graduation
  Test:
    Package: agent-spec
    Filter: test_cli_finish_refuses_archive_collision_e2e
    Level: e2e
    Test Double: none
    Targets: public CLI process and filesystem
  Given docs/learning/<goal>/ already contains a conflicting file
  When finish runs
  Then the command fails naming the collision
  And the goal folder keeps research.md and its learning records

=== Codebase Context ===

Files (18):
  - .claude/skills/agent-spec-authoring/SKILL.md
  - .claude/skills/agent-spec-authoring/references/patterns.md
  - .claude/skills/agent-spec-estimate/SKILL.md
  - .claude/skills/agent-spec-estimate/references/examples.md
  - .claude/skills/agent-spec-research/SKILL.md
  - .claude/skills/agent-spec-sdd/SKILL.md
  - .claude/skills/agent-spec-tool-first/SKILL.md
  - .claude/skills/agent-spec-tool-first/references/commands.md
  - docs/features/learning-archive/learning-records/0001-archive-location.md
  - docs/features/learning-archive/spec.md
  - skills/agent-spec-authoring/SKILL.md
  - skills/agent-spec-authoring/references/patterns.md
  - skills/agent-spec-estimate/SKILL.md
  - skills/agent-spec-estimate/references/examples.md
  - skills/agent-spec-research/SKILL.md
  - skills/agent-spec-sdd/SKILL.md
  - skills/agent-spec-tool-first/SKILL.md
  - skills/agent-spec-tool-first/references/commands.md

=== Task Sketch ===

Group 1 (order 1):
  Scenarios:
    - graduation archives the learning trail
    - archive collisions abort the graduation
  Boundary paths:
    - docs/learning/**
  Test selectors:
    - test_cli_finish_archives_research_and_learning_e2e
    - test_cli_finish_refuses_archive_collision_e2e

=== Warnings ===

  - Allowed Changes path not found: docs/learning/** (resolved to ./docs/learning)
