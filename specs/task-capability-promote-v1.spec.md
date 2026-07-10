spec: task
name: "Capability 层 + promote v1：长寿命真相库"
inherits: project
tags: [bdd, capability, promote, phase3]
depends: [task-coverage-matrix-v1]
estimate: 3d
---

## Intent

引入 BDD-spine 的累积真相层:capability spec(`specs/capabilities/<name>.spec.md`)持有
长寿命的 Rule(系统当前行为的承诺),task spec 用 Example 证明它们。提供 `promote`:
当一条 task-scope Rule 的所有 Example 都机械通过后,把它提升进 capability spec(scope
从 Task 变为 Capability,id 不变)。这是 OpenSpec 活规格库的 BDD-native 形态:
promote 的前置门禁是真实跑过的测试,而不是文档完整性。

## Decisions

- 新增 `SpecLevel::Capability`;`spec: capability` 解析为该级别。
- capability spec 的 `## 完成条件` 下的 `Rule:` 行,scope 解析为 `RuleScope::Capability(<spec-name>)`(不是 Task)。capability spec 的 Rule 通常没有 Example(Example 住在 task),因此 `bdd-rule-grouping` 的"空 Rule" warning 对 capability spec 不适用(降级为 info 或不触发)。
- task spec 新增可选 frontmatter `capability: <name>`(additive,`SpecMeta.capability: Option<String>`)。
- parser 按 spec 级别决定 Rule scope:Task 级 → `Task(file-stem)`,Capability 级 → `Capability(name)`。
- `BehaviorRule` 新增 additive `events: Vec<RuleEvent>`(provenance event log);`RuleEvent { kind: created|promoted|affirmed|deprecated, note }`。serde default skip-if-empty。
- 新命令 `agent-spec promote <task-spec> --rule <id> --to <capability-name> --code .`:
  - 门禁:对 task 跑验证,该 Rule 名下所有 Example 的 verdict 必须全为 `pass`,否则拒绝提升(非零退出)。
  - 合并:把该 Rule 以 `Capability(<name>)` scope 追加进 `specs/capabilities/<name>.spec.md`(不存在则创建一个 capability spec)。
  - 幂等:capability spec 已有同 id 的 Rule 时不重复追加。
  - 事件:被提升的 Rule 在 capability spec 中带一条 `promoted` 事件。
- observability/治理动作,**不改变 `is_passing` 语义**;promote 失败只阻止提升,不改 task 验证结果。

## Boundaries

### Allowed Changes

- src/spec_core/ast.rs（SpecLevel::Capability、RuleEvent/RuleEventKind、BehaviorRule.events、SpecMeta.capability —— 均 additive）
- src/spec_parser/meta.rs（解析 `spec: capability` + `capability:` 字段）
- src/spec_parser/parser.rs（按级别决定 Rule scope;capability 级 Rule 解析）
- src/spec_parser/resolver.rs（capability spec 解析支持）
- src/spec_lint/linters.rs（bdd-rule-grouping 对 capability 级不报"空 Rule" warning）
- src/spec_gateway/**
- src/spec_report/**
- src/main.rs（新增 `promote` 子命令 + 合并写回逻辑）
- README.md
- examples/**
- specs/capabilities/**（promote 产物目录）

### Forbidden

- 不要修改 `is_passing` / verdict 判定逻辑。
- 不要在本期实现 capability Rule 反向继承进 task 验证(读回),也不要做 capability 依赖图——留待 Phase 3.5/后续(见排除范围)。
- 不要让 promote 在门禁未通过时仍写回。
- additive 字段导致的构造点机械补全沿用既有 carve-out(仅补字段,不改逻辑)。

## Completion Criteria

### Rule: capability-spec-level — capability spec 是独立级别且 Rule 带 Capability scope

Scenario: spec capability 级别被解析
  Test:
    Filter: test_parse_capability_spec_level
  Given 一份 frontmatter 为 `spec: capability` 的 spec
  When parser 解析
  Then `meta.level` 为 `SpecLevel::Capability`

Scenario: capability spec 的 Rule 带 Capability scope
  Test:
    Filter: test_capability_spec_rule_has_capability_scope
  Given 一份名为 `ecosystem-import` 的 capability spec,`## 完成条件` 下有 `Rule: import-preserves-traceability`
  When parser 解析
  Then 该 BehaviorRule 的 `key.scope` 为 `Capability("ecosystem-import")`

Scenario: 非法 spec 级别被拒绝
  Test:
    Filter: test_unknown_spec_level_rejected
  Given frontmatter 为 `spec: nonsense`
  When parser 解析
  Then 返回错误,指出未知 spec 级别

### Rule: task-declares-capability — task 可声明所属 capability

Scenario: task 的 capability frontmatter 字段被解析
  Test:
    Filter: test_parse_task_capability_field
  Given 一份 task spec frontmatter 含 `capability: ecosystem-import`
  When parser 解析
  Then `meta.capability` 为 `Some("ecosystem-import")`

Scenario: 无 capability 字段时为 None 且 JSON 不输出该键
  Test:
    Filter: test_task_without_capability_is_none_additive
  Given 一份不含 `capability:` 的旧 task spec
  When parser 解析并序列化为 JSON
  Then `meta.capability` 为 `None`
  And JSON 不出现 `capability` 键

### Rule: promote-gates-on-passing-examples — 提升前所有 Example 必须机械通过

Scenario: 所有 Example 通过时 Rule 被提升进 capability spec
  Test:
    Filter: test_promote_appends_rule_when_examples_pass
  Given 一份 task spec 的 Rule `r-ok` 名下所有 Example 的 verdict 为 `pass`
  When 运行 `agent-spec promote <task> --rule r-ok --to billing --code .`
  Then `specs/capabilities/billing.spec.md` 中出现 id 为 `r-ok`、scope 为 Capability 的 Rule
  And 该 Rule 带一条 `promoted` 事件

Scenario: 有 Example 未通过时拒绝提升
  Test:
    Filter: test_promote_refuses_when_an_example_fails
  Given task spec 的 Rule `r-bad` 名下有 Example 的 verdict 不是 `pass`
  When 运行 `agent-spec promote <task> --rule r-bad --to billing --code .`
  Then 命令以非零状态失败,提示门禁未通过
  And `specs/capabilities/billing.spec.md` 不包含 `r-bad`

Scenario: 重复提升同一 Rule 幂等
  Test:
    Filter: test_promote_is_idempotent_for_same_rule
  Given capability spec `billing` 已含 id 为 `r-ok` 的 Rule
  When 再次提升 `r-ok` 到 `billing`
  Then capability spec 中 `r-ok` 只出现一次

Scenario: 提升不存在的 Rule id 报错
  Test:
    Filter: test_promote_unknown_rule_id_errors
  Given task spec 中没有 id 为 `r-missing` 的 Rule
  When 运行 promote `--rule r-missing`
  Then 命令失败,提示找不到该 Rule id

### Rule: rule-event-log-is-additive — Rule 事件日志只增不减

Scenario: 新建 Rule 默认无事件且 JSON 不输出该键
  Test:
    Filter: test_rule_events_additive_empty_by_default
  Given 一份普通 task spec 的 Rule
  When 序列化为 JSON
  Then 不出现 `events` 键

Scenario: 事件日志可序列化与反序列化
  Test:
    Filter: test_rule_event_roundtrips
  Given 一个带 `promoted` 事件的 `RuleEvent`
  When 序列化再反序列化
  Then 结果与原值相等

### Rule: promote-does-not-change-verdict-semantics — promote 是治理动作不改门禁

Scenario: promote 不改变 is_passing
  Test:
    Filter: test_promote_does_not_change_is_passing
  Given 一份全部场景 pass 的验证报告
  When 执行 promote 的合并逻辑后再调用 `is_passing`
  Then `is_passing` 仍为 true
  And summary 计数未变

## Out of Scope

- capability Rule 反向继承进 task 验证(读回 capability 真相到 task resolution)—— Phase 3.5
- capability 依赖图与 promote 跨能力前置校验 —— 后续
- `affirmed` / `deprecated` 事件的命令入口(本期只产出 `created` / `promoted`)
- capability spec 的多层继承(org → capability → task 的完整链)
- 把 capability 覆盖纳入 `is_passing` 或 guard 门禁
