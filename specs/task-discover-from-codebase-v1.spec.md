spec: task
name: "discover --from-codebase v1：从测试反向生成 spec 骨架"
inherits: project
tags: [discovery, cold-start, phase9]
depends: [task-audit-v1]
estimate: 2d
---

## Intent

缓解 §9.4 的冷启动代价:项目已有测试但没有 spec 时,机械地把每个测试函数反向生成为一个
绑定该测试的 scenario 草案,产出一份可被 agent-spec 验证的 task spec 骨架,并附 `## Questions`
提示这些是自动草拟、需人工细化(Discovery 种子)。AI 进一步充实 Given/When/Then 的语义部分
留待后续;本期是纯机械的"测试 → spec 骨架"。

## Decisions

- 纯函数 `draft_spec_from_tests(test_names, spec_name) -> String`:生成一份 `spec: task` 的 .spec.md,
  - 每个测试函数名 → 一个 scenario,带 `测试: <fn>` selector 与占位 `当/那么` 步骤;
  - 含一个 `## Questions` section,提示这些 scenario 由 discover 自动草拟、需人工细化意图与步骤。
- 复用 Phase 2 的 `collect_test_function_names` 从 `--code` 路径收集测试函数。
- 新命令 `agent-spec discover --from-codebase --code <dir> --name <n> [--out <file>]`:扫描测试,生成草案,写入或打印。
- 生成的草案必须能被 `agent-spec parse` 成功解析(可直接进入后续验证/细化)。
- 不改 `is_passing` / verification。

## Boundaries

### Allowed Changes

- src/spec_report/**（discover 模块)
- src/main.rs（discover 子命令)
- README.md、examples/**

### Forbidden

- 不要用 AI 充实 scenario 语义(本期纯机械,占位步骤)。
- 不要假装草案是完整契约(必须带 Questions 种子标注需人工细化)。
- 不要改 `is_passing` / verification。

## Completion Criteria

### Rule: draft-binds-each-test — 每个测试反向生成一个绑定 scenario

Scenario: 每个测试函数生成一个带 selector 的 scenario
  Test:
    Filter: test_draft_creates_scenario_per_test
  Given 测试函数名集合 `["test_a", "test_b"]`
  When 调用 `draft_spec_from_tests`
  Then 生成的文本含两个 scenario,分别带 `测试: test_a` 与 `测试: test_b`

Scenario: 生成的草案可被解析
  Test:
    Filter: test_draft_is_parseable
  Given 一组测试函数名
  When 生成草案并用 parser 解析
  Then 解析成功,scenario 数等于测试函数数

Scenario: 空测试集生成可解析的占位草案
  Test:
    Filter: test_draft_empty_tests_is_parseable
  Given 测试函数名集合为空
  When 生成草案并解析
  Then 解析成功且不 panic

### Rule: draft-seeds-discovery — 草案标注需人工细化

Scenario: 草案含 Questions 种子
  Test:
    Filter: test_draft_includes_questions_seed
  Given 一组测试函数名
  When 生成草案
  Then 文本含 `## Questions` section
  And 含提示这些 scenario 为自动草拟、需人工细化的问题

Scenario: scenario 名来源于测试名
  Test:
    Filter: test_draft_scenario_names_derive_from_tests
  Given 测试函数名 `test_register_returns_201`
  When 生成草案
  Then 某个 scenario 的名称或 selector 含 `test_register_returns_201`

## Out of Scope

- AI 充实 scenario 的 Given/When/Then 语义(本期占位步骤)
- 从非 Rust 测试框架收集(本期复用 Rust 测试扫描)
- 从代码结构推断 Rule 分组 / capability(本期只做扁平 scenario 骨架)
- 自动决定 Intent / Decisions / Boundaries(草案留占位 + Questions 提示)
