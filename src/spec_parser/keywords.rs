use crate::spec_core::StepKind;

/// English keyword recognition for BDD steps. Structural keywords are
/// English-only; CJK aliases are detected separately and rejected with an
/// actionable error (see `detect_cjk_structural` / `detect_cjk_step_keyword`).
pub fn match_step_keyword(line: &str) -> Option<(StepKind, &str)> {
    let trimmed = line.trim();

    let en_mappings: &[(&str, StepKind)] = &[
        ("given ", StepKind::Given),
        ("when ", StepKind::When),
        ("then ", StepKind::Then),
        ("and ", StepKind::And),
        ("but ", StepKind::But),
    ];

    let lower = trimmed.to_lowercase();
    for &(kw, kind) in en_mappings {
        if lower.starts_with(kw) {
            let rest = trimmed[kw.len()..].trim();
            return Some((kind, rest));
        }
    }

    None
}

/// English section header recognition.
pub fn match_section_header(line: &str) -> Option<SectionKind> {
    let trimmed = line.trim().trim_start_matches('#').trim();
    let lower = trimmed.to_lowercase();

    if lower.starts_with("intent") {
        Some(SectionKind::Intent)
    } else if lower.starts_with("constraint") {
        Some(SectionKind::Constraints)
    } else if lower.starts_with("decision") {
        Some(SectionKind::Decisions)
    } else if lower.starts_with("boundaries") || lower.starts_with("boundary") {
        Some(SectionKind::Boundaries)
    } else if lower.starts_with("acceptance criter") || lower.starts_with("completion criter") {
        Some(SectionKind::AcceptanceCriteria)
    } else if lower.starts_with("out of scope") {
        Some(SectionKind::OutOfScope)
    } else if lower.starts_with("questions") || lower.starts_with("open questions") {
        Some(SectionKind::Questions)
    } else if lower.starts_with("current state") {
        Some(SectionKind::CurrentState)
    } else if lower.starts_with("ux shape") {
        Some(SectionKind::UxShape)
    } else {
        None
    }
}

/// Scenario header recognition. `Example:` is accepted as an alias of
/// `Scenario:` (Cucumber treats Example and Scenario as synonyms); the parser
/// stores both as `Scenario`.
pub fn match_scenario_header(line: &str) -> Option<&str> {
    let trimmed = line.trim().trim_start_matches('#').trim();

    // English keywords, accepting both ASCII `:` and full-width `：`
    // (common when authoring with a CJK IME on).
    let lower = trimmed.to_lowercase();
    for word in ["scenario", "example"] {
        for colon in [":", "："] {
            let prefix = format!("{word}{colon}");
            if lower.starts_with(&prefix) {
                return Some(trimmed[prefix.len()..].trim());
            }
        }
    }

    None
}

/// Behavior rule header recognition: `Rule:`.
/// Returns the raw content after the colon (id and optional display name);
/// the parser is responsible for splitting and validating the kebab-case id.
pub fn match_rule_header(line: &str) -> Option<&str> {
    let trimmed = line.trim().trim_start_matches('#').trim();

    let lower = trimmed.to_lowercase();
    for colon in [":", "："] {
        let prefix = format!("rule{colon}");
        if lower.starts_with(&prefix) {
            return Some(trimmed[prefix.len()..].trim());
        }
    }
    None
}

/// Scenario-level test selector binding.
pub fn match_test_selector(line: &str) -> Option<&str> {
    let trimmed = line.trim().trim_start_matches('#').trim();

    let lower = trimmed.to_lowercase();
    if lower.starts_with("test:") {
        Some(trimmed["test:".len()..].trim())
    } else {
        None
    }
}

/// Scenario-level tags line recognition (e.g., `Tags: [critical]`).
pub fn match_scenario_tags(line: &str) -> Option<Vec<String>> {
    let trimmed = line.trim().trim_start_matches('#').trim();

    let value = {
        let lower = trimmed.to_lowercase();
        if lower.starts_with("tags:") {
            Some(trimmed["tags:".len()..].trim())
        } else {
            None
        }
    };

    value.map(|v| {
        let v = v.trim_start_matches('[').trim_end_matches(']');
        v.split(',')
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect()
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestSelectorField {
    Package,
    Filter,
    Level,
    TestDouble,
    Targets,
}

/// Structured fields under a `Test:` selector block.
pub fn match_test_selector_field(line: &str) -> Option<(TestSelectorField, &str)> {
    let trimmed = line.trim().trim_start_matches('#').trim();

    let lower = trimmed.to_lowercase();
    if lower.starts_with("package:") {
        return Some((
            TestSelectorField::Package,
            trimmed["package:".len()..].trim(),
        ));
    }
    if lower.starts_with("filter:") {
        return Some((TestSelectorField::Filter, trimmed["filter:".len()..].trim()));
    }
    if lower.starts_with("level:") {
        return Some((TestSelectorField::Level, trimmed["level:".len()..].trim()));
    }
    if lower.starts_with("test double:") {
        return Some((
            TestSelectorField::TestDouble,
            trimmed["test double:".len()..].trim(),
        ));
    }
    if lower.starts_with("targets:") {
        return Some((
            TestSelectorField::Targets,
            trimmed["targets:".len()..].trim(),
        ));
    }

    None
}

/// Review field recognition: `Review: human`.
/// Returns Some("human") or Some("auto"), or None if not a review line.
pub fn match_review_field(line: &str) -> Option<&str> {
    let trimmed = line.trim().trim_start_matches('#').trim();

    let lower = trimmed.to_lowercase();
    if lower.starts_with("review:") {
        return Some(trimmed["review:".len()..].trim());
    }

    None
}

/// Mode field recognition: `Mode: optimize`.
/// Returns Some("optimize") or Some("standard"), or None if not a mode line.
pub fn match_mode_field(line: &str) -> Option<&str> {
    let trimmed = line.trim().trim_start_matches('#').trim();

    let lower = trimmed.to_lowercase();
    if lower.starts_with("mode:") {
        return Some(trimmed["mode:".len()..].trim());
    }

    None
}

/// Depends field recognition: `Depends: A, B`.
/// Returns Some("A, B") or None if not a depends line.
pub fn match_depends_field(line: &str) -> Option<&str> {
    let trimmed = line.trim().trim_start_matches('#').trim();

    let lower = trimmed.to_lowercase();
    if lower.starts_with("depends:") {
        return Some(trimmed["depends:".len()..].trim());
    }

    None
}

/// Every CJK structural keyword the DSL previously accepted, mapped to its
/// English replacement. Keywords must be English; these tables exist only so
/// the parser can reject a legacy spec with an actionable message instead of
/// silently mis-parsing it.
const CJK_SECTION_HEADERS: &[(&str, &str)] = &[
    ("已定决策", "Decisions"),
    ("验收标准", "Acceptance Criteria"),
    ("完成条件", "Completion Criteria"),
    ("排除范围", "Out of Scope"),
    ("允许修改", "Allowed Changes"),
    ("禁止做", "Forbidden"),
    ("待澄清", "Questions"),
    ("意图", "Intent"),
    ("约束", "Constraints"),
    ("决策", "Decisions"),
    ("边界", "Boundaries"),
    ("问题", "Questions"),
];

const CJK_COLON_KEYWORDS: &[(&str, &str)] = &[
    ("场景", "Scenario:"),
    ("示例", "Example:"),
    ("例子", "Example:"),
    ("规则", "Rule:"),
    ("测试", "Test:"),
    ("过滤", "Filter:"),
    ("层级", "Level:"),
    ("替身", "Test Double:"),
    ("命中", "Targets:"),
    ("审核", "Review:"),
    ("模式", "Mode:"),
    ("标签", "Tags:"),
    ("前置", "Depends:"),
    ("包", "Package:"),
];

const CJK_STEP_KEYWORDS: &[(&str, &str)] = &[
    ("假设", "Given"),
    ("那么", "Then"),
    ("并且", "And"),
    ("但是", "But"),
    ("当", "When"),
];

/// Detect a CJK `keyword:` line (scenario, selector, rule, tags, ...).
/// Returns `(found, english_replacement)` so the parser can fail with an
/// actionable message.
pub fn detect_cjk_structural(line: &str) -> Option<(&'static str, &'static str)> {
    let trimmed = line.trim().trim_start_matches('#').trim();

    for &(cjk, en) in CJK_COLON_KEYWORDS {
        for colon in [":", "："] {
            let mut prefix = String::with_capacity(cjk.len() + colon.len());
            prefix.push_str(cjk);
            prefix.push_str(colon);
            if trimmed.starts_with(&prefix) {
                return Some((cjk, en));
            }
        }
    }

    None
}

/// Detect a CJK section header on a markdown heading line.
pub fn detect_cjk_section_header(line: &str) -> Option<(&'static str, &'static str)> {
    let trimmed = line.trim().trim_start_matches('#').trim();
    CJK_SECTION_HEADERS
        .iter()
        .find(|(cjk, _)| trimmed.starts_with(cjk))
        .copied()
}

/// Detect a CJK step keyword. Only meaningful inside a scenario body, where
/// these prefixes were previously parsed as structural steps.
pub fn detect_cjk_step_keyword(line: &str) -> Option<(&'static str, &'static str)> {
    let trimmed = line.trim();
    CJK_STEP_KEYWORDS
        .iter()
        .find(|(cjk, _)| trimmed.starts_with(cjk))
        .copied()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionKind {
    Intent,
    Constraints,
    Decisions,
    Boundaries,
    AcceptanceCriteria,
    OutOfScope,
    Questions,
    CurrentState,
    UxShape,
}

/// Extract quoted parameters from step text.
/// e.g., `存在一笔金额为 "100.00" 元的交易 "TXN-001"` → ["100.00", "TXN-001"]
pub fn extract_params(text: &str) -> Vec<String> {
    let mut params = Vec::new();
    let mut chars = text.chars();
    while let Some(ch) = chars.next() {
        if ch == '"' || ch == '\u{201C}' || ch == '\u{201D}' {
            // collect until closing quote
            let mut param = String::new();
            for inner in chars.by_ref() {
                if inner == '"' || inner == '\u{201C}' || inner == '\u{201D}' {
                    break;
                }
                param.push(inner);
            }
            if !param.is_empty() {
                params.push(param);
            }
        }
    }
    params
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_match_step_chinese_rejected() {
        assert!(match_step_keyword("  假设 数据库中存在用户").is_none());
        assert_eq!(
            detect_cjk_step_keyword("  假设 数据库中存在用户"),
            Some(("假设", "Given"))
        );
        assert!(match_step_keyword("  并且 用户已登录").is_none());
        assert_eq!(
            detect_cjk_step_keyword("  并且 用户已登录"),
            Some(("并且", "And"))
        );
        assert_eq!(detect_cjk_step_keyword("当 用户点击"), Some(("当", "When")));
        assert_eq!(detect_cjk_step_keyword("那么 成功"), Some(("那么", "Then")));
        assert_eq!(detect_cjk_step_keyword("但是 失败"), Some(("但是", "But")));
    }

    #[test]
    fn test_match_step_english() {
        let (kind, rest) = match_step_keyword("  Given a user exists").unwrap();
        assert_eq!(kind, StepKind::Given);
        assert_eq!(rest, "a user exists");
    }

    #[test]
    fn test_scenario_header_chinese_rejected() {
        assert!(match_scenario_header("场景: 全额退款").is_none());
        assert!(match_scenario_header("场景：全额退款").is_none());
        assert_eq!(
            detect_cjk_structural("场景: 全额退款"),
            Some(("场景", "Scenario:"))
        );
        assert_eq!(
            detect_cjk_structural("场景：全额退款"),
            Some(("场景", "Scenario:"))
        );
    }

    #[test]
    fn test_scenario_header_english() {
        assert_eq!(
            match_scenario_header("Scenario: Full refund"),
            Some("Full refund")
        );
    }

    #[test]
    fn test_scenario_header_example_alias_english_only() {
        assert_eq!(
            match_scenario_header("Example: Full refund"),
            Some("Full refund")
        );
        assert_eq!(
            match_scenario_header("### Example: Happy path"),
            Some("Happy path")
        );
        // CJK aliases are rejected and reported for replacement.
        assert!(match_scenario_header("示例: 余额不足").is_none());
        assert!(match_scenario_header("例子: 余额充足").is_none());
        assert_eq!(
            detect_cjk_structural("示例: 余额不足"),
            Some(("示例", "Example:"))
        );
        assert_eq!(
            detect_cjk_structural("例子: 余额充足"),
            Some(("例子", "Example:"))
        );
    }

    #[test]
    fn test_match_rule_header() {
        assert_eq!(
            match_rule_header("Rule: auth-must-not-leak — 鉴权失败不得泄漏内部错误"),
            Some("auth-must-not-leak — 鉴权失败不得泄漏内部错误")
        );
        assert_eq!(
            match_rule_header("### Rule: refund-idempotent"),
            Some("refund-idempotent")
        );
        assert_eq!(match_rule_header("规则: vip-discount-priority"), None);
        assert_eq!(
            detect_cjk_structural("规则: vip-discount-priority"),
            Some(("规则", "Rule:"))
        );
        assert_eq!(match_rule_header("场景: 普通场景"), None);
        assert_eq!(match_rule_header("- 普通条目"), None);
    }

    #[test]
    fn test_scenario_header_accepts_markdown_heading() {
        assert_eq!(
            match_scenario_header("### Scenario: Full refund"),
            Some("Full refund")
        );
        assert!(match_scenario_header("### 场景: 全额退款").is_none());
        assert_eq!(
            detect_cjk_structural("### 场景: 全额退款"),
            Some(("场景", "Scenario:"))
        );
    }

    #[test]
    fn test_extract_params() {
        let params = extract_params(r#"金额为 "100.00" 元的交易 "TXN-001""#);
        assert_eq!(params, vec!["100.00", "TXN-001"]);
    }

    #[test]
    fn test_extract_params_chinese_quotes() {
        let params = extract_params("金额为\u{201C}100.00\u{201D}元");
        assert_eq!(params, vec!["100.00"]);
    }

    #[test]
    fn test_match_test_selector_chinese_rejected() {
        assert!(match_test_selector("  测试: test_parse_contract").is_none());
        assert!(match_test_selector("  测试：test_parse_contract").is_none());
        assert_eq!(
            detect_cjk_structural("  测试: test_parse_contract"),
            Some(("测试", "Test:"))
        );
        assert_eq!(
            detect_cjk_structural("  测试：test_parse_contract"),
            Some(("测试", "Test:"))
        );
    }

    #[test]
    fn test_match_test_selector_fields_support_verification_metadata() {
        assert!(match_test_selector_field("  层级: integration").is_none());
        assert_eq!(
            detect_cjk_structural("  层级: integration"),
            Some(("层级", "Level:"))
        );
        assert!(match_test_selector_field("  替身: local_http_stub").is_none());
        assert_eq!(
            detect_cjk_structural("  替身: local_http_stub"),
            Some(("替身", "Test Double:"))
        );
        assert!(match_test_selector_field("  命中: commands/update").is_none());
        assert_eq!(
            detect_cjk_structural("  命中: commands/update"),
            Some(("命中", "Targets:"))
        );
        assert_eq!(
            match_test_selector_field("  Level: integration"),
            Some((TestSelectorField::Level, "integration"))
        );
        assert_eq!(
            match_test_selector_field("  Test Double: local_http_stub"),
            Some((TestSelectorField::TestDouble, "local_http_stub"))
        );
        assert_eq!(
            match_test_selector_field("  Targets: commands/update"),
            Some((TestSelectorField::Targets, "commands/update"))
        );
    }

    #[test]
    fn test_match_test_selector_english() {
        assert_eq!(
            match_test_selector("  Test: test_parse_contract"),
            Some("test_parse_contract")
        );
    }

    #[test]
    fn test_match_test_selector_accepts_markdown_heading() {
        assert_eq!(
            match_test_selector("### Test: test_parse_contract"),
            Some("test_parse_contract")
        );
        assert!(match_test_selector("### 测试: test_parse_contract").is_none());
        assert_eq!(
            detect_cjk_structural("### 测试: test_parse_contract"),
            Some(("测试", "Test:"))
        );
    }

    #[test]
    fn test_match_test_selector_field_chinese_rejected() {
        assert!(match_test_selector_field("  包: spec-parser").is_none());
        assert_eq!(
            detect_cjk_structural("  包: spec-parser"),
            Some(("包", "Package:"))
        );
        assert!(match_test_selector_field("  过滤: test_parse_contract").is_none());
        assert_eq!(
            detect_cjk_structural("  过滤: test_parse_contract"),
            Some(("过滤", "Filter:"))
        );
    }

    #[test]
    fn test_match_test_selector_field_english() {
        assert_eq!(
            match_test_selector_field("  Package: spec-parser"),
            Some((TestSelectorField::Package, "spec-parser"))
        );
        assert_eq!(
            match_test_selector_field("  Filter: test_parse_contract"),
            Some((TestSelectorField::Filter, "test_parse_contract"))
        );
    }

    #[test]
    fn test_match_test_selector_field_accepts_markdown_heading() {
        assert_eq!(
            match_test_selector_field("### Package: spec-parser"),
            Some((TestSelectorField::Package, "spec-parser"))
        );
        assert!(match_test_selector_field("### 过滤: test_parse_contract").is_none());
        assert_eq!(
            detect_cjk_structural("### 过滤: test_parse_contract"),
            Some(("过滤", "Filter:"))
        );
    }

    #[test]
    fn test_section_headers() {
        assert_eq!(match_section_header("## Intent"), Some(SectionKind::Intent));
        assert_eq!(
            match_section_header("## Constraints"),
            Some(SectionKind::Constraints)
        );
        assert_eq!(
            match_section_header("## Decisions"),
            Some(SectionKind::Decisions)
        );
        assert_eq!(
            match_section_header("## Boundaries"),
            Some(SectionKind::Boundaries)
        );
        assert_eq!(
            match_section_header("## Acceptance Criteria"),
            Some(SectionKind::AcceptanceCriteria)
        );
        assert_eq!(
            match_section_header("## Completion Criteria"),
            Some(SectionKind::AcceptanceCriteria)
        );
    }

    #[test]
    fn test_section_headers_chinese_rejected() {
        for (header, cjk, en) in [
            ("## 意图", "意图", "Intent"),
            ("## 约束", "约束", "Constraints"),
            ("## 决策", "决策", "Decisions"),
            ("## 边界", "边界", "Boundaries"),
            ("## 验收标准", "验收标准", "Acceptance Criteria"),
            ("## 完成条件", "完成条件", "Completion Criteria"),
            ("## 排除范围", "排除范围", "Out of Scope"),
        ] {
            assert!(match_section_header(header).is_none(), "{header}");
            assert_eq!(detect_cjk_section_header(header), Some((cjk, en)));
        }
    }

    #[test]
    fn test_not_a_step() {
        assert!(match_step_keyword("这是普通文字").is_none());
        assert!(match_step_keyword("- 约束条目").is_none());
    }
}
