//! Cold-start reverse engineering (Phase 9): draft a task-spec skeleton from a
//! codebase's existing test functions. Mechanical — one bound scenario per
//! test, plus a Discovery (`## Questions`) seed flagging the draft for human
//! refinement. No AI.

/// Build a draft `.spec.md` skeleton: one scenario per test function (bound via
/// `Test:`), placeholder steps, and a `## Questions` seed.
pub fn draft_spec_from_tests(test_names: &[String], spec_name: &str) -> String {
    let mut out = format!(
        "spec: task\nname: \"{spec_name}\"\n---\n\n## Intent\n\n[由 discover 自动草拟] 为现有测试反向补齐 Task Contract;请人工细化意图。\n\n## Completion Criteria\n\n"
    );
    if test_names.is_empty() {
        out.push_str(
            "Scenario: 占位场景\n  When [待人工填写触发动作]\n  Then [待人工填写可观察结果]\n",
        );
    } else {
        for t in test_names {
            out.push_str(&format!(
                "Scenario: {t}\n  Test: {t}\n  When [待人工填写触发动作]\n  Then [待人工填写可观察结果]\n\n"
            ));
        }
    }
    out.push_str(
        "## Questions\n\n- 这些 scenario 由 discover 从测试名自动草拟,需人工细化每个 scenario 的意图、Given/When/Then 与可观察结果\n- 是否应把相关 scenario 归入 Rule 分组?\n",
    );
    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::spec_core::Section;
    use crate::spec_parser::parse_spec_from_str;

    fn scenario_count(content: &str) -> usize {
        let doc = parse_spec_from_str(content).unwrap();
        doc.sections
            .iter()
            .filter_map(|s| match s {
                Section::AcceptanceCriteria { scenarios, .. } => Some(scenarios.len()),
                _ => None,
            })
            .sum()
    }

    #[test]
    fn test_draft_creates_scenario_per_test() {
        let d = draft_spec_from_tests(&["test_a".into(), "test_b".into()], "drafted");
        assert!(d.contains("Test: test_a"));
        assert!(d.contains("Test: test_b"));
        assert_eq!(scenario_count(&d), 2);
    }

    #[test]
    fn test_draft_is_parseable() {
        let names = vec![
            "test_one".to_string(),
            "test_two".to_string(),
            "test_three".to_string(),
        ];
        let d = draft_spec_from_tests(&names, "drafted");
        assert_eq!(scenario_count(&d), 3);
    }

    #[test]
    fn test_draft_empty_tests_is_parseable() {
        let d = draft_spec_from_tests(&[], "drafted");
        // Placeholder scenario keeps the draft parseable.
        assert!(scenario_count(&d) >= 1);
    }

    #[test]
    fn test_draft_includes_questions_seed() {
        let d = draft_spec_from_tests(&["test_a".into()], "drafted");
        assert!(d.contains("## Questions"));
        assert!(d.contains("自动草拟"));
    }

    #[test]
    fn test_draft_scenario_names_derive_from_tests() {
        let d = draft_spec_from_tests(&["test_register_returns_201".into()], "drafted");
        assert!(d.contains("test_register_returns_201"));
    }
}
