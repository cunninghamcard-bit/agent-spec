spec: task
name: "Probe 抽象 v1：Example 的可验证绑定泛化"
inherits: project
tags: [bdd, probe, phase6_5]
depends: [task-gen-integrations-v1]
estimate: 1d
---

## Intent

把 scenario 的可验证绑定从"只有 Test selector"泛化为 `Probe`:Test 是 Probe 的一种,
未来可扩展 Static(结构检查)、Benchmark(性能)、External(外部探针)、Inferential(AI)。
v1 只引入抽象与从现有 `test_selector` 的派生(`Probe::from_scenario`),**不改 verification、
不改 DSL、不给 Scenario 加字段**——为 Phase 7 的多 runner 留出统一挂载点,同时零迁移成本。

## Decisions

- 新增 `Probe` 枚举:`Test(TestSelector)` + 预留变体 `Static(String)` / `Benchmark { runner, filter, threshold }` / `External { runner, args }` / `Inferential`。
- `Probe::from_scenario(&Scenario) -> Option<Probe>`:scenario 有 `test_selector` 时派生 `Probe::Test(selector)`,否则 `None`。这是向后兼容 shim:`Test:` 仍是 Probe::Test 的语法糖。
- `Probe::kind_label() -> &str`:返回 `test`/`static`/`benchmark`/`external`/`inferential`,供报告显示。
- v1 不给 `Scenario` 加 `probe` 字段(避免构造点改动);非 Test 探针的 DSL 解析与 verification 执行留待 Phase 7。
- 不改 `is_passing` / verification / lint。

## Boundaries

### Allowed Changes

- src/spec_core/ast.rs（Probe 枚举 + from_scenario + kind_label,additive,不改 Scenario 结构)
- src/spec_core/mod.rs（导出)
- README.md、examples/**

### Forbidden

- 不要给 `Scenario` 增加字段(本期零构造点改动)。
- 不要改 TestVerifier / verification / is_passing。
- 不要在本期解析或执行非 Test 探针(Phase 7)。

## Completion Criteria

### Rule: probe-derives-from-test-selector — Test 是 Probe 的一种,可从现有 selector 派生

Scenario: 带 test_selector 的 scenario 派生出 Probe::Test
  Test:
    Filter: test_probe_from_scenario_with_selector
  Given 一个带 `test_selector`(filter 为 `test_x`)的 scenario
  When 调用 `Probe::from_scenario`
  Then 返回 `Some(Probe::Test(selector))`,其 filter 为 `test_x`

Scenario: 无 test_selector 的 scenario 派生为 None
  Test:
    Filter: test_probe_from_scenario_without_selector
  Given 一个没有 `test_selector` 的 scenario
  When 调用 `Probe::from_scenario`
  Then 返回 `None`

### Rule: probe-kind-labels — 每个 Probe 变体有稳定标签

Scenario: Test 探针标签为 test
  Test:
    Filter: test_probe_kind_label_test
  Given 一个 `Probe::Test`
  When 调用 `kind_label`
  Then 返回 `"test"`

Scenario: 预留变体有各自标签
  Test:
    Filter: test_probe_kind_label_reserved_variants
  Given `Probe::Static`、`Probe::Benchmark`、`Probe::External`、`Probe::Inferential`
  When 分别调用 `kind_label`
  Then 分别返回 `static`/`benchmark`/`external`/`inferential`

### Rule: probe-is-additive-scaffolding — 抽象不改既有结构与验证

Scenario: Probe 序列化往返
  Test:
    Filter: test_probe_roundtrips
  Given 一个 `Probe::Benchmark`
  When 序列化再反序列化
  Then 结果与原值相等

Scenario: Scenario 结构未变(仍可无 probe 字段构造)
  Test:
    Filter: test_scenario_unchanged_no_probe_field
  Given 现有任一 scenario 构造路径
  When 解析现有 spec
  Then scenario 仍通过 `test_selector` 暴露绑定,Probe 仅作派生视图

## Out of Scope

- 非 Test 探针的 DSL 解析(`Probe: Static`/`Benchmark` 等)—— Phase 7
- 各 runner 的实际执行(criterion / 外部探针 / 导入分析)—— Phase 7
- 给 `Scenario` 增加持久化 `probe` 字段 —— Phase 7(届时一并迁移)
- StructuralRule / NFR 门禁 —— Phase 7
