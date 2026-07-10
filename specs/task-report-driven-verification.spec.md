spec: task
name: "报告驱动验证：test_command + JUnit XML 报告解析"
inherits: project
tags: [verify, report, junit, polyglot, node]
---

## 意图

让非 Rust 项目也能接入机械验证：spec 在 frontmatter 声明项目自己的测试命令
`test_command` 与报告路径 `test_report`，验证时运行该命令一次，解析生成的
JUnit XML 报告，按每个场景的 `Test:` selector 名字对号入座给出判定。
现有 cargo 直跑路径保持不变，report mode 是新增的可选路径。

## 已定决策

- frontmatter 新增 `test_command`（字符串，项目自己的测试命令）与 `test_report`（JUnit XML 报告路径，相对 `--code` 根目录）
- 声明了 `test_command` 的 spec 走 report mode；未声明的 spec 走现有 cargo 直跑路径，行为不变
- report mode 每次验证只运行一次 `test_command`（`sh -c` 执行，工作目录为 `--code` 根），用同一份报告判定该 spec 的所有场景
- JUnit XML 解析使用 `quick-xml`
- selector 匹配规则：与 testcase 的 name 精确相等，或 name 以 selector 结尾；匹配到 0 个判 fail；匹配到多个判 fail 且 reason 列出候选名
- 结构化 selector 的 `Package:` 字段在 report mode 映射为 testcase classname 前缀过滤
- 报告中标记 skipped 的 testcase 不判 pass
- `test_command` 的退出码不直接决定场景判定；报告文件不存在时该 spec 的 report-mode 场景判 fail，错误信息包含期望的报告路径
- `test_command` 支持可选占位符 `{selectors}`：执行前替换为该 spec 全部场景 selector 的正则交替模式（形如 `(selector_a|selector_b)`），selector 中的正则元字符以反斜杠转义；命令中不含占位符时原样执行

## 边界

### 允许修改
- src/spec_parser/meta.rs
- src/spec_core/**
- src/spec_verify/**
- Cargo.toml
- Cargo.lock
- tests/**
- specs/task-report-driven-verification.spec.md
- README.md

### 禁止做
- 不要改变未声明 `test_command` 的 spec 的验证行为
- 不要把"selector 查无此名"判成 pass 或 skip
- 不要新增 `quick-xml` 以外的第三方依赖
- 不要改动 pass、fail、skip、uncertain 四种判定的既有语义

## 排除范围

- 混合路由（同一 spec 内不同场景走不同 runner）
- 按测试框架逐家适配 CLI 参数（vitest、jest 等的命令由项目写在 `test_command` 里）
- JUnit XML 以外的报告格式

## 完成条件

场景: 报告中的同名测试通过则场景判 pass
  测试:
    包: agent-spec
    过滤: test_report_mode_passes_when_named_testcase_passes
  假设 某 spec 声明了 `test_command` 且命令生成的 JUnit 报告包含名为 "register rejects duplicate" 的通过 testcase
  当 对绑定 selector "register rejects duplicate" 的场景执行验证
  那么 该场景判定为 pass
  并且 evidence 记录命中的 testcase 名

场景: 报告中的同名测试失败则场景判 fail 且证据包含失败信息
  测试:
    包: agent-spec
    过滤: test_report_mode_fails_with_failure_evidence
  假设 JUnit 报告中同名 testcase 带有 failure 节点
  当 对该场景执行验证
  那么 该场景判定为 fail
  并且 evidence 包含 failure 节点的 message 文本

场景: selector 在报告中查无此名判 fail
  测试:
    包: agent-spec
    过滤: test_report_mode_fails_when_selector_not_in_report
  假设 JUnit 报告中不存在与 selector 匹配的 testcase
  当 对该场景执行验证
  那么 该场景判定为 fail
  并且 reason 说明 selector 未命中任何 testcase

场景: 报告中被 skip 的测试不判 pass
  测试:
    包: agent-spec
    过滤: test_report_mode_skipped_testcase_is_not_pass
  假设 JUnit 报告中同名 testcase 带有 skipped 节点
  当 对该场景执行验证
  那么 该场景判定为 fail
  并且 reason 说明该 testcase 被跳过

场景: selector 匹配多个 testcase 判 fail 并列出候选
  测试:
    包: agent-spec
    过滤: test_report_mode_ambiguous_match_fails_with_candidates
  假设 JUnit 报告中存在两个 name 都以同一 selector 结尾的 testcase
  当 对该场景执行验证
  那么 该场景判定为 fail
  并且 reason 列出全部候选 testcase 名

场景: 报告文件缺失时判 fail 并给出可操作错误
  测试:
    包: agent-spec
    过滤: test_report_mode_missing_report_fails_with_actionable_error
  假设 某 spec 的 `test_command` 执行后未在 `test_report` 路径生成文件
  当 对该 spec 执行验证
  那么 该 spec 的 report-mode 场景判定为 fail
  并且 reason 包含期望的报告文件路径

场景: Package 字段按 classname 前缀过滤
  测试:
    包: agent-spec
    过滤: test_report_mode_package_filters_by_classname_prefix
  假设 JUnit 报告中两个同名 testcase 的 classname 前缀不同
  当 场景使用带 `Package:` 的结构化 selector 执行验证
  那么 只有 classname 前缀匹配的 testcase 参与判定
  并且 该场景判定为 pass

场景: test_command 的 selectors 占位符替换为全部 selector 交替模式
  测试:
    包: agent-spec
    过滤: test_report_mode_substitutes_selectors_placeholder
  假设 某 spec 的 `test_command` 包含 `{selectors}` 且合约有两个场景 selector
  当 report mode 组装待执行命令
  那么 占位符被替换为 "(selector_a|selector_b)" 形式的交替模式
  并且 命令其余部分保持原样

场景: selector 中的正则元字符在占位符替换时被转义
  测试:
    包: agent-spec
    过滤: test_report_mode_escapes_regex_metacharacters_in_selectors
  假设 某场景 selector 含有正则元字符 "."
  当 report mode 组装待执行命令
  那么 交替模式中的元字符以反斜杠转义
  并且 替换后的命令不因元字符改变匹配范围

场景: 未声明 test_command 的 spec 保持 cargo 直跑行为
  测试:
    包: agent-spec
    过滤: test_specs_without_test_command_keep_cargo_path
  假设 某 spec 的 frontmatter 未声明 `test_command`
  当 对该 spec 执行验证
  那么 验证走现有 cargo 直跑路径
  并且 判定结果与本次改动之前一致
