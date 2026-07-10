spec: task
name: "guard 边界只执行 Forbidden 项"
inherits: project
tags: [guard, boundaries, verify]
---

## 意图

历史任务合约的 `Allowed Changes` 只描述当年那个任务允许碰什么；repo 级 `guard`
把当前变更集拿去和所有历史合约的允许清单求交集，导致任何真实变更都会全红。
修正语义：`guard` 下边界校验只执行 `Forbidden` 项（跨任务的持久禁区），
`Allowed Changes` 覆盖检查仅在单 spec 的 `verify`/`lifecycle` 中生效。

## 已定决策

- `BoundariesVerifier` 增加 forbidden-only 模式，默认模式行为不变
- `guard` 对每份 spec 的验证使用 forbidden-only 模式的边界校验
- 单 spec 的 `verify` 与 `lifecycle` 继续使用完整模式（Forbidden + Allowed 覆盖检查）
- forbidden-only 模式下，变更不在 Allowed 清单内时该条记为 pass，reason 说明 guard 只执行 forbidden 边界

## 边界

### 允许修改
- src/spec_verify/boundaries.rs
- src/spec_gateway/**
- src/main.rs
- specs/task-guard-forbidden-only-boundaries.spec.md
- CHANGELOG.md
- README.md

### 禁止做
- 不要改变单 spec `verify`/`lifecycle` 的边界判定结果
- 不要让 forbidden-only 模式放过命中 `Forbidden` 边界的变更

## 完成条件

场景: guard 模式下变更不在 Allowed 清单内也判 pass
  测试:
    包: agent-spec
    过滤: test_guard_boundaries_ignore_allowed_coverage
  假设 某 spec 的 Allowed Changes 只含 "crates/other/**" 且变更集包含 "src/lib.rs"
  当 forbidden-only 模式的边界校验运行
  那么 边界场景判定为 pass

场景: guard 模式下命中 Forbidden 边界仍判 fail
  测试:
    包: agent-spec
    过滤: test_guard_boundaries_still_enforce_forbidden
  假设 某 spec 的 Forbidden 含 "tests/golden/**" 且变更集包含 "tests/golden/a.txt"
  当 forbidden-only 模式的边界校验运行
  那么 边界场景判定为 fail
  并且 reason 指出命中的 forbidden 边界

场景: 单 spec 模式保持 Allowed 覆盖检查
  测试:
    包: agent-spec
    过滤: test_single_spec_boundaries_keep_allowed_coverage
  假设 某 spec 的 Allowed Changes 只含 "crates/other/**" 且变更集包含 "src/lib.rs"
  当 默认模式的边界校验运行
  那么 边界场景判定为 fail
  并且 reason 为该变更不在任何允许边界内
