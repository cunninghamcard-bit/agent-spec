//! Report-driven verification (report mode).
//!
//! When a spec declares `test_command` in its front-matter, verification runs
//! that command once (the project's own way of running tests), parses the
//! JUnit XML report it produces at `test_report`, and judges every scenario
//! by matching its `Test:` selector against testcase names in the report.
//! Specs without `test_command` never enter this module.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};

use crate::spec_core::{
    Evidence, ReviewMode, Scenario, ScenarioResult, SpecError, SpecResult, StepVerdict,
    TestSelector, Verdict,
};

use super::VerificationContext;

const SELECTORS_PLACEHOLDER: &str = "{selectors}";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JunitCase {
    pub classname: String,
    pub name: String,
    pub status: CaseStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaseStatus {
    Passed,
    Failed { message: String },
    Skipped,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MatchOutcome {
    None,
    Unique(JunitCase),
    Ambiguous(Vec<String>),
}

pub fn run_report_mode(
    ctx: &VerificationContext,
    test_command: &str,
    test_report: &str,
) -> SpecResult<Vec<ScenarioResult>> {
    let root = resolve_code_root(&ctx.code_paths).ok_or_else(|| {
        SpecError::Verification("report mode: no usable directory among code paths".into())
    })?;
    let report_path = root.join(test_report);
    // A stale report from a previous run must never satisfy this run.
    let _ = fs::remove_file(&report_path);

    let selectors: Vec<String> = bound_scenarios(ctx)
        .map(|(_, selector)| selector.filter.clone())
        .collect();
    let command = substitute_selectors(test_command, &selectors);

    let started = Instant::now();
    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .current_dir(&root)
        .output()
        .map_err(|err| SpecError::Verification(format!("failed to run test_command: {err}")))?;
    let duration_ms = started.elapsed().as_millis() as u64;

    if !report_path.is_file() {
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        let reason = format!(
            "test_command completed but no report exists at `{}` (front-matter says `test_report: {test_report}`); make the command write a JUnit XML report to that path",
            report_path.display()
        );
        return Ok(bound_scenarios(ctx)
            .map(|(scenario, selector)| {
                build_result(
                    scenario,
                    selector,
                    Verdict::Fail,
                    reason.clone(),
                    selector.filter.clone(),
                    stderr.clone(),
                    duration_ms,
                )
            })
            .collect());
    }

    let xml = fs::read_to_string(&report_path)?;
    let cases = parse_junit(&xml)?;
    Ok(bound_scenarios(ctx)
        .map(|(scenario, selector)| {
            judge_scenario(scenario, selector, &cases, test_report, duration_ms)
        })
        .collect())
}

fn bound_scenarios(ctx: &VerificationContext) -> impl Iterator<Item = (&Scenario, &TestSelector)> {
    ctx.resolved_spec
        .all_scenarios
        .iter()
        .filter_map(|scenario| {
            scenario
                .test_selector
                .as_ref()
                .map(|selector| (scenario, selector))
        })
}

fn judge_scenario(
    scenario: &Scenario,
    selector: &TestSelector,
    cases: &[JunitCase],
    test_report: &str,
    duration_ms: u64,
) -> ScenarioResult {
    let (verdict, reason, test_name, detail) = match match_selector(cases, selector) {
        MatchOutcome::None => (
            Verdict::Fail,
            format!(
                "selector `{}` matched no testcase in report `{test_report}`",
                selector.filter
            ),
            selector.filter.clone(),
            String::new(),
        ),
        MatchOutcome::Ambiguous(names) => (
            Verdict::Fail,
            format!(
                "selector `{}` matched {} testcases: {}",
                selector.filter,
                names.len(),
                names.join(", ")
            ),
            selector.filter.clone(),
            String::new(),
        ),
        MatchOutcome::Unique(case) => match &case.status {
            CaseStatus::Passed => {
                let verdict = if scenario.review == ReviewMode::Human {
                    Verdict::PendingReview
                } else {
                    Verdict::Pass
                };
                (
                    verdict,
                    format!("report testcase `{}` passed", case.name),
                    case.name.clone(),
                    String::new(),
                )
            }
            CaseStatus::Failed { message } => (
                Verdict::Fail,
                format!("report testcase `{}` failed: {message}", case.name),
                case.name.clone(),
                message.clone(),
            ),
            CaseStatus::Skipped => (
                Verdict::Fail,
                format!(
                    "report testcase `{}` was skipped; a skipped test does not satisfy the contract",
                    case.name
                ),
                case.name.clone(),
                String::new(),
            ),
        },
    };

    build_result(
        scenario,
        selector,
        verdict,
        reason,
        test_name,
        detail,
        duration_ms,
    )
}

fn build_result(
    scenario: &Scenario,
    selector: &TestSelector,
    verdict: Verdict,
    reason: String,
    test_name: String,
    stdout: String,
    duration_ms: u64,
) -> ScenarioResult {
    let step_results = scenario
        .steps
        .iter()
        .map(|step| StepVerdict {
            step_text: step.text.clone(),
            verdict,
            reason: reason.clone(),
        })
        .collect();

    // The reason must stay observable even for scenarios without steps.
    let stdout = if stdout.is_empty() {
        reason.clone()
    } else {
        stdout
    };

    ScenarioResult {
        scenario_name: scenario.name.clone(),
        verdict,
        step_results,
        evidence: vec![Evidence::TestOutput {
            test_name,
            stdout,
            passed: matches!(verdict, Verdict::Pass | Verdict::PendingReview),
            package: selector.package.clone(),
            level: selector.level.clone(),
            test_double: selector.test_double.clone(),
            targets: selector.targets.clone(),
        }],
        duration_ms,
        provenance: None,
    }
}

fn match_selector(cases: &[JunitCase], selector: &TestSelector) -> MatchOutcome {
    let hits: Vec<&JunitCase> = cases
        .iter()
        .filter(|case| {
            selector
                .package
                .as_ref()
                .is_none_or(|prefix| case.classname.starts_with(prefix.as_str()))
        })
        .filter(|case| case.name == selector.filter || case.name.ends_with(&selector.filter))
        .collect();

    match hits.as_slice() {
        [] => MatchOutcome::None,
        [single] => MatchOutcome::Unique((*single).clone()),
        many => MatchOutcome::Ambiguous(many.iter().map(|case| case.name.clone()).collect()),
    }
}

fn parse_junit(xml: &str) -> SpecResult<Vec<JunitCase>> {
    let mut reader = Reader::from_str(xml);
    let mut cases = Vec::new();
    let mut open: Option<JunitCase> = None;

    loop {
        let event = reader
            .read_event()
            .map_err(|err| SpecError::Verification(format!("invalid JUnit XML: {err}")))?;
        match event {
            Event::Empty(e) => match e.name().as_ref() {
                b"testcase" => cases.push(new_case(&e)),
                b"failure" | b"error" => mark_failure(&mut open, &e),
                b"skipped" => mark_skipped(&mut open),
                _ => {}
            },
            Event::Start(e) => match e.name().as_ref() {
                b"testcase" => open = Some(new_case(&e)),
                b"failure" | b"error" => mark_failure(&mut open, &e),
                b"skipped" => mark_skipped(&mut open),
                _ => {}
            },
            Event::End(e) => {
                if e.name().as_ref() == b"testcase"
                    && let Some(case) = open.take()
                {
                    cases.push(case);
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }

    Ok(cases)
}

fn new_case(e: &BytesStart) -> JunitCase {
    JunitCase {
        classname: attr(e, "classname").unwrap_or_default(),
        name: attr(e, "name").unwrap_or_default(),
        status: CaseStatus::Passed,
    }
}

fn mark_failure(open: &mut Option<JunitCase>, e: &BytesStart) {
    if let Some(case) = open.as_mut() {
        case.status = CaseStatus::Failed {
            message: attr(e, "message").unwrap_or_default(),
        };
    }
}

fn mark_skipped(open: &mut Option<JunitCase>) {
    if let Some(case) = open.as_mut()
        && case.status == CaseStatus::Passed
    {
        case.status = CaseStatus::Skipped;
    }
}

fn attr(e: &BytesStart, key: &str) -> Option<String> {
    e.try_get_attribute(key)
        .ok()
        .flatten()
        .and_then(|attribute| {
            attribute
                .normalized_value(quick_xml::XmlVersion::Implicit1_0)
                .ok()
        })
        .map(|value| value.into_owned())
}

fn substitute_selectors(command: &str, selectors: &[String]) -> String {
    if !command.contains(SELECTORS_PLACEHOLDER) {
        return command.to_string();
    }
    let pattern = format!(
        "({})",
        selectors
            .iter()
            .map(|selector| escape_regex(selector))
            .collect::<Vec<_>>()
            .join("|")
    );
    command.replace(SELECTORS_PLACEHOLDER, &pattern)
}

fn escape_regex(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        if "\\.^$|?*+()[]{}".contains(ch) {
            out.push('\\');
        }
        out.push(ch);
    }
    out
}

fn resolve_code_root(code_paths: &[PathBuf]) -> Option<PathBuf> {
    code_paths.iter().find_map(|path| {
        if path.is_dir() {
            Some(path.clone())
        } else if path.is_file() {
            path.parent().map(Path::to_path_buf)
        } else {
            None
        }
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use crate::spec_core::{
        Evidence, ResolvedSpec, Scenario, Span, SpecDocument, SpecLevel, SpecMeta, TestSelector,
        Verdict,
    };
    use crate::spec_verify::{AiMode, VerificationContext, Verifier};

    use super::*;

    fn scenario_with_selector(name: &str, selector: TestSelector) -> Scenario {
        Scenario {
            name: name.into(),
            steps: Vec::new(),
            test_selector: Some(selector),
            tags: Vec::new(),
            review: Default::default(),
            mode: Default::default(),
            depends_on: vec![],
            rule: None,
            span: Span::default(),
        }
    }

    fn ctx_with(
        scenarios: Vec<Scenario>,
        code_root: PathBuf,
        test_command: Option<&str>,
        test_report: Option<&str>,
    ) -> VerificationContext {
        VerificationContext {
            code_paths: vec![code_root],
            change_paths: vec![],
            ai_mode: AiMode::Off,
            resolved_spec: ResolvedSpec {
                task: SpecDocument {
                    meta: SpecMeta {
                        level: SpecLevel::Task,
                        name: "report-mode-test".into(),
                        inherits: None,
                        lang: vec![],
                        tags: vec![],
                        depends: vec![],
                        estimate: None,
                        capability: None,
                        test_command: test_command.map(str::to_string),
                        test_report: test_report.map(str::to_string),
                    },
                    sections: vec![],
                    lint_acks: vec![],
                    source_path: PathBuf::new(),
                    source: String::new(),
                },
                inherited_constraints: vec![],
                inherited_decisions: vec![],
                all_scenarios: scenarios,
            },
        }
    }

    fn temp_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "agent-spec-report-mode-{tag}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn evidence_test_output(result: &crate::spec_core::ScenarioResult) -> (&str, &str) {
        match result.evidence.as_slice() {
            [
                Evidence::TestOutput {
                    test_name, stdout, ..
                },
            ] => (test_name.as_str(), stdout.as_str()),
            other => panic!("expected one TestOutput evidence, got {other:?}"),
        }
    }

    #[test]
    fn test_report_mode_passes_when_named_testcase_passes() {
        let dir = temp_dir("pass");
        let ctx = ctx_with(
            vec![scenario_with_selector(
                "成功注册",
                TestSelector::filter_only("register rejects duplicate"),
            )],
            dir.clone(),
            Some("command"),
            Some("report.xml"),
        );
        let command = r#"printf '%s' '<testsuites><testsuite><testcase classname="register.test.ts" name="register rejects duplicate"/></testsuite></testsuites>' > report.xml"#;

        let results = run_report_mode(&ctx, command, "report.xml").unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].verdict, Verdict::Pass);
        let (test_name, _) = evidence_test_output(&results[0]);
        assert_eq!(test_name, "register rejects duplicate");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_report_mode_fails_with_failure_evidence() {
        let xml = r#"<testsuites><testsuite>
            <testcase classname="t" name="register rejects duplicate">
                <failure message="expected 409 got 200">stack</failure>
            </testcase>
        </testsuite></testsuites>"#;
        let cases = parse_junit(xml).unwrap();
        let scenario = scenario_with_selector(
            "重复邮箱",
            TestSelector::filter_only("register rejects duplicate"),
        );
        let selector = scenario.test_selector.clone().unwrap();

        let result = judge_scenario(&scenario, &selector, &cases, "report.xml", 0);

        assert_eq!(result.verdict, Verdict::Fail);
        let (_, stdout) = evidence_test_output(&result);
        assert!(stdout.contains("expected 409 got 200"), "got: {stdout}");
    }

    #[test]
    fn test_report_mode_fails_when_selector_not_in_report() {
        let cases = parse_junit("<testsuites></testsuites>").unwrap();
        let scenario =
            scenario_with_selector("查无此名", TestSelector::filter_only("no_such_test"));
        let selector = scenario.test_selector.clone().unwrap();

        let result = judge_scenario(&scenario, &selector, &cases, "report.xml", 0);

        assert_eq!(result.verdict, Verdict::Fail);
        let (_, stdout) = evidence_test_output(&result);
        assert!(
            stdout.contains("matched no testcase"),
            "reason must state the selector missed, got: {stdout}"
        );
        match match_selector(&cases, &selector) {
            MatchOutcome::None => {}
            other => panic!("expected no match, got {other:?}"),
        }
    }

    #[test]
    fn test_report_mode_skipped_testcase_is_not_pass() {
        let xml = r#"<testsuites><testsuite>
            <testcase classname="t" name="flaky thing"><skipped/></testcase>
        </testsuite></testsuites>"#;
        let cases = parse_junit(xml).unwrap();
        let scenario = scenario_with_selector("被跳过", TestSelector::filter_only("flaky thing"));
        let selector = scenario.test_selector.clone().unwrap();

        let result = judge_scenario(&scenario, &selector, &cases, "report.xml", 0);

        assert_eq!(result.verdict, Verdict::Fail);
    }

    #[test]
    fn test_report_mode_ambiguous_match_fails_with_candidates() {
        let xml = r#"<testsuites><testsuite>
            <testcase classname="a" name="suite one register rejects duplicate"/>
            <testcase classname="b" name="suite two register rejects duplicate"/>
        </testsuite></testsuites>"#;
        let cases = parse_junit(xml).unwrap();
        let scenario = scenario_with_selector(
            "歧义",
            TestSelector::filter_only("register rejects duplicate"),
        );
        let selector = scenario.test_selector.clone().unwrap();

        let result = judge_scenario(&scenario, &selector, &cases, "report.xml", 0);

        assert_eq!(result.verdict, Verdict::Fail);
        match match_selector(&cases, &selector) {
            MatchOutcome::Ambiguous(names) => {
                assert!(names.contains(&"suite one register rejects duplicate".to_string()));
                assert!(names.contains(&"suite two register rejects duplicate".to_string()));
            }
            other => panic!("expected ambiguous match, got {other:?}"),
        }
    }

    #[test]
    fn test_report_mode_missing_report_fails_with_actionable_error() {
        let dir = temp_dir("missing");
        let ctx = ctx_with(
            vec![scenario_with_selector(
                "报告缺失",
                TestSelector::filter_only("anything"),
            )],
            dir.clone(),
            Some("true"),
            Some("report.xml"),
        );

        let results = run_report_mode(&ctx, "true", "report.xml").unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].verdict, Verdict::Fail);
        let (_, stdout) = evidence_test_output(&results[0]);
        assert!(
            stdout.contains("report.xml"),
            "reason must name the expected report path, got: {stdout}"
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_report_mode_package_filters_by_classname_prefix() {
        let xml = r#"<testsuites><testsuite>
            <testcase classname="web/admin/register.test.ts" name="register rejects duplicate"/>
            <testcase classname="web/site/register.test.ts" name="register rejects duplicate"/>
        </testsuite></testsuites>"#;
        let cases = parse_junit(xml).unwrap();
        let selector = TestSelector {
            package: Some("web/admin".into()),
            filter: "register rejects duplicate".into(),
            level: None,
            test_double: None,
            targets: None,
        };
        let scenario = scenario_with_selector("前缀过滤", selector.clone());

        let result = judge_scenario(&scenario, &selector, &cases, "report.xml", 0);

        assert_eq!(result.verdict, Verdict::Pass);
        match match_selector(&cases, &selector) {
            MatchOutcome::Unique(case) => assert_eq!(case.classname, "web/admin/register.test.ts"),
            other => panic!("expected unique match, got {other:?}"),
        }
    }

    #[test]
    fn test_report_mode_substitutes_selectors_placeholder() {
        let command = r#"pnpm vitest run -t "{selectors}" --reporter=junit"#;
        let selectors = vec!["selector_a".to_string(), "selector_b".to_string()];

        let substituted = substitute_selectors(command, &selectors);

        assert_eq!(
            substituted,
            r#"pnpm vitest run -t "(selector_a|selector_b)" --reporter=junit"#
        );
    }

    #[test]
    fn test_report_mode_escapes_regex_metacharacters_in_selectors() {
        let selectors = vec!["a.b".to_string()];

        let substituted = substitute_selectors("run -t {selectors}", &selectors);

        assert_eq!(substituted, r"run -t (a\.b)");
    }

    #[test]
    fn test_specs_without_test_command_keep_cargo_path() {
        // A spec without `test_command` must take the existing cargo path.
        // In a directory with no Cargo.toml that path returns no results at
        // all, while report mode would have produced fail results — so an
        // empty result set proves the routing is unchanged.
        let dir = temp_dir("cargo-path");
        let ctx = ctx_with(
            vec![scenario_with_selector(
                "老路径",
                TestSelector::filter_only("some_test"),
            )],
            dir.clone(),
            None,
            None,
        );

        let verifier = crate::spec_verify::TestVerifier;
        let results = verifier.verify(&ctx).unwrap();
        assert!(
            results.is_empty(),
            "cargo path without Cargo.toml yields no results"
        );

        // And the same spec WITH test_command but no test_report is a hard
        // configuration error, proving report mode routing is distinct.
        let ctx = ctx_with(
            vec![scenario_with_selector(
                "缺 report 配置",
                TestSelector::filter_only("some_test"),
            )],
            dir.clone(),
            Some("true"),
            None,
        );
        assert!(verifier.verify(&ctx).is_err());
        let _ = fs::remove_dir_all(&dir);
    }
}
