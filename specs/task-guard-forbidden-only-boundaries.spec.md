spec: task
name: "guard 边界只执行 Forbidden 项"
inherits: project
tags: [guard, boundaries, verify]
---

## Intent

历史任务合约的 `Allowed Changes` 只描述当年那个任务允许碰什么；repo 级 `guard`
把当前变更集拿去和所有历史合约的允许清单求交集，导致任何真实变更都会全红。
修正语义：`guard` 下边界校验只执行 `Forbidden` 项（跨任务的持久禁区），
`Allowed Changes` 覆盖检查仅在单 spec 的 `verify`/`lifecycle` 中生效。

## Decisions

- `BoundariesVerifier` 增加 forbidden-only 模式，默认模式行为不变
- `guard` 对每份 spec 的验证使用 forbidden-only 模式的边界校验
- 单 spec 的 `verify` 与 `lifecycle` 继续使用完整模式（Forbidden + Allowed 覆盖检查）
- forbidden-only 模式下，变更不在 Allowed 清单内时该条记为 pass，reason 说明 guard 只执行 forbidden 边界

## Boundaries

### Allowed Changes
- src/spec_verify/boundaries.rs
- src/spec_gateway/**
- src/main.rs
- specs/task-guard-forbidden-only-boundaries.spec.md
- CHANGELOG.md
- README.md

### Forbidden
- 不要改变单 spec `verify`/`lifecycle` 的边界判定结果
- 不要让 forbidden-only 模式放过命中 `Forbidden` 边界的变更

## Completion Criteria

Scenario: guard 模式下变更不在 Allowed 清单内也判 pass
  Test:
    Package: agent-spec
    Filter: test_guard_boundaries_ignore_allowed_coverage
  Given 某 spec 的 Allowed Changes 只含 "crates/other/**" 且变更集包含 "src/lib.rs"
  When forbidden-only 模式的边界校验运行
  Then 边界场景判定为 pass

Scenario: guard 模式下命中 Forbidden 边界仍判 fail
  Test:
    Package: agent-spec
    Filter: test_guard_boundaries_still_enforce_forbidden
  Given 某 spec 的 Forbidden 含 "tests/golden/**" 且变更集包含 "tests/golden/a.txt"
  When forbidden-only 模式的边界校验运行
  Then 边界场景判定为 fail
  And reason 指出命中的 forbidden 边界

Scenario: 单 spec 模式保持 Allowed 覆盖检查
  Test:
    Package: agent-spec
    Filter: test_single_spec_boundaries_keep_allowed_coverage
  Given 某 spec 的 Allowed Changes 只含 "crates/other/**" 且变更集包含 "src/lib.rs"
  When 默认模式的边界校验运行
  Then 边界场景判定为 fail
  And reason 为该变更不在任何允许边界内
