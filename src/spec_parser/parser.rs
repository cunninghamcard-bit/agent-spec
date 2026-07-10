use crate::spec_core::{
    BehaviorRule, Boundary, BoundaryCategory, Constraint, ConstraintCategory, MalformedRule,
    ReviewMode, RuleKey, RuleScope, Scenario, ScenarioMode, Section, Span, SpecDocument, SpecError,
    SpecResult, Step, TestSelector,
};
use std::path::{Path, PathBuf};

use super::keywords::{
    SectionKind, TestSelectorField, detect_cjk_section_header, detect_cjk_step_keyword,
    detect_cjk_structural, extract_params, match_depends_field, match_mode_field,
    match_review_field, match_rule_header, match_scenario_header, match_scenario_tags,
    match_section_header, match_step_keyword, match_test_selector, match_test_selector_field,
};
use super::meta::parse_meta;

/// Parse a .spec/.spec.md file from disk.
pub fn parse_spec(path: &Path) -> SpecResult<SpecDocument> {
    let content = std::fs::read_to_string(path)?;
    let stem = task_stem_from_path(path);
    let mut doc = parse_spec_from_str_with_stem(&content, &stem)?;
    doc.source_path = path.to_path_buf();
    Ok(doc)
}

/// Derive the task scope namespace from a spec path: the file stem with any
/// `.spec.md` / `.spec` suffix removed (e.g. `task-foo.spec.md` -> `task-foo`).
pub fn task_stem_from_path(path: &Path) -> String {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();
    name.strip_suffix(".spec.md")
        .or_else(|| name.strip_suffix(".spec"))
        .unwrap_or(name)
        .to_string()
}

/// Parse a .spec/.spec.md string into a SpecDocument.
///
/// Task-scope behavior rules are namespaced by an empty stem; use
/// [`parse_spec`] (from a path) or [`parse_spec_from_str_with_stem`] when the
/// task stem matters.
pub fn parse_spec_from_str(input: &str) -> SpecResult<SpecDocument> {
    parse_spec_from_str_with_stem(input, "")
}

/// Parse a .spec/.spec.md string with an explicit task-scope namespace
/// (the spec file stem), used to build `RuleScope::Task(stem)`.
pub fn parse_spec_from_str_with_stem(input: &str, task_stem: &str) -> SpecResult<SpecDocument> {
    let lines: Vec<&str> = input.lines().collect();

    // Split on front-matter separator `---`
    let separator_pos = lines.iter().position(|l| l.trim() == "---");
    let (meta_lines, body_lines, body_offset) = match separator_pos {
        Some(pos) => (&lines[..pos], &lines[pos + 1..], pos + 1),
        None => {
            // No front-matter: try to parse entire content as body
            // with a minimal default meta
            return Err(SpecError::FrontMatter(
                "missing front-matter separator '---'".into(),
            ));
        }
    };

    let meta = parse_meta(meta_lines).map_err(SpecError::FrontMatter)?;

    // Rules parsed from a capability spec take Capability scope (the spec's
    // name); everything else takes Task scope (the file stem).
    let rule_scope = match meta.level {
        crate::spec_core::SpecLevel::Capability => RuleScope::Capability(meta.name.clone()),
        _ => RuleScope::Task(task_stem.to_string()),
    };

    let sections = parse_body(body_lines, body_offset, &rule_scope)?;
    let lint_acks = scan_lint_acks(body_lines);

    Ok(SpecDocument {
        meta,
        sections,
        lint_acks,
        source_path: PathBuf::new(),
        source: input.to_string(),
    })
}

/// Scan body lines for `<!-- lint-ack: <code> — <reason> -->` markers (Phase 5).
fn scan_lint_acks(lines: &[&str]) -> Vec<crate::spec_core::LintAck> {
    let mut acks = Vec::new();
    for line in lines {
        let t = line.trim();
        let Some(rest) = t.strip_prefix("<!-- lint-ack:") else {
            continue;
        };
        let Some(body) = rest.strip_suffix("-->") else {
            continue;
        };
        let body = body.trim();
        // Separator: em dash or colon.
        let (code, reason) = if let Some(idx) = body.find('—') {
            (body[..idx].trim(), body[idx + '—'.len_utf8()..].trim())
        } else if let Some(idx) = body.find(':') {
            (body[..idx].trim(), body[idx + 1..].trim())
        } else {
            (body, "")
        };
        if !code.is_empty() {
            acks.push(crate::spec_core::LintAck {
                code: code.to_string(),
                reason: reason.to_string(),
            });
        }
    }
    acks
}

/// Parse the body of a spec (after `---`) into sections.
fn parse_body(lines: &[&str], offset: usize, rule_scope: &RuleScope) -> SpecResult<Vec<Section>> {
    let mut sections = Vec::new();
    let mut current_section: Option<(SectionKind, usize)> = None; // (kind, start_line)
    let mut section_lines: Vec<(usize, &str)> = Vec::new(); // (absolute_line, text)

    for (i, &line) in lines.iter().enumerate() {
        let abs_line = offset + i + 1; // 1-indexed

        if let Some(kind) = match_section_header(line) {
            // Flush previous section
            if let Some((prev_kind, start)) = current_section.take() {
                let section = build_section(prev_kind, &section_lines, start, rule_scope)?;
                sections.push(section);
                section_lines.clear();
            }
            current_section = Some((kind, abs_line));
        } else if matches!(markdown_heading_level(line), Some(1 | 2)) {
            if let Some((cjk, en)) = detect_cjk_section_header(line) {
                return Err(cjk_keyword_error(cjk, en, abs_line));
            }
            let header = line.trim().trim_start_matches('#').trim();
            return Err(SpecError::Parse {
                message: format!(
                    "unknown top-level section header '{header}' - use only Intent/Constraints/Decisions/Boundaries/Acceptance Criteria/Out of Scope"
                ),
                span: Span::line(abs_line),
            });
        } else if current_section.is_some() {
            section_lines.push((abs_line, line));
        }
    }

    // Flush last section
    if let Some((kind, start)) = current_section {
        let section = build_section(kind, &section_lines, start, rule_scope)?;
        sections.push(section);
    }

    Ok(sections)
}

fn markdown_heading_level(line: &str) -> Option<usize> {
    let trimmed = line.trim_start();
    let level = trimmed.chars().take_while(|&ch| ch == '#').count();
    if level == 0 || level == trimmed.len() {
        return None;
    }
    Some(level)
}

fn build_section(
    kind: SectionKind,
    lines: &[(usize, &str)],
    start_line: usize,
    rule_scope: &RuleScope,
) -> SpecResult<Section> {
    let end_line = lines.last().map_or(start_line, |(ln, _)| *ln);
    let span = Span::new(start_line, 0, end_line, 0);

    match kind {
        SectionKind::Intent => {
            let content: String = lines
                .iter()
                .map(|(_, l)| *l)
                .collect::<Vec<_>>()
                .join("\n")
                .trim()
                .to_string();
            Ok(Section::Intent { content, span })
        }
        SectionKind::Constraints => {
            let items = parse_constraints(lines)?;
            Ok(Section::Constraints { items, span })
        }
        SectionKind::Decisions => {
            let items = parse_string_list(lines);
            Ok(Section::Decisions { items, span })
        }
        SectionKind::Boundaries => {
            let items = parse_boundaries(lines)?;
            Ok(Section::Boundaries { items, span })
        }
        SectionKind::AcceptanceCriteria => {
            let (scenarios, rules, malformed_rules) = parse_scenarios(lines, rule_scope)?;
            Ok(Section::AcceptanceCriteria {
                scenarios,
                rules,
                malformed_rules,
                span,
            })
        }
        SectionKind::OutOfScope => {
            let items = lines
                .iter()
                .filter_map(|(_, l)| {
                    let trimmed = l.trim().strip_prefix('-').map(str::trim);
                    trimmed.filter(|s| !s.is_empty()).map(String::from)
                })
                .collect();
            Ok(Section::OutOfScope { items, span })
        }
        SectionKind::Questions => {
            let items = lines
                .iter()
                .filter_map(|(_, l)| {
                    let trimmed = l.trim().strip_prefix('-').map(str::trim);
                    trimmed.filter(|s| !s.is_empty()).map(String::from)
                })
                .collect();
            Ok(Section::Questions { items, span })
        }
        SectionKind::CurrentState => {
            let content: String = lines
                .iter()
                .map(|(_, l)| *l)
                .collect::<Vec<_>>()
                .join("\n")
                .trim()
                .to_string();
            Ok(Section::CurrentState { content, span })
        }
        SectionKind::UxShape => {
            let content: String = lines
                .iter()
                .map(|(_, l)| *l)
                .collect::<Vec<_>>()
                .join("\n")
                .trim()
                .to_string();
            Ok(Section::UxShape { content, span })
        }
    }
}

fn parse_constraints(lines: &[(usize, &str)]) -> SpecResult<Vec<Constraint>> {
    let mut constraints = Vec::new();
    let mut category = ConstraintCategory::General;

    for &(line_num, line) in lines {
        let trimmed = line.trim();

        // Sub-section headers for constraint categories
        if trimmed.starts_with("###") || trimmed.starts_with("### ") {
            let header = trimmed.trim_start_matches('#').trim().to_lowercase();
            if header.contains("必须做") {
                return Err(cjk_keyword_error("必须做", "Must", line_num));
            }
            if header.contains("禁止") {
                return Err(cjk_keyword_error("禁止", "Must Not", line_num));
            }
            if header.contains("已定") {
                return Err(cjk_keyword_error("已定", "Decided", line_num));
            }
            if header.contains("must") && !header.contains("not") {
                category = ConstraintCategory::Must;
            } else if header.contains("must not") {
                category = ConstraintCategory::MustNot;
            } else if header.contains("decided") {
                category = ConstraintCategory::Decided;
            }
            continue;
        }

        // Bullet items
        if let Some(text) = trimmed.strip_prefix('-') {
            let text = text.trim();
            if !text.is_empty() {
                constraints.push(Constraint {
                    text: text.to_string(),
                    category,
                    span: Span::line(line_num),
                });
            }
        }
    }

    Ok(constraints)
}

fn parse_string_list(lines: &[(usize, &str)]) -> Vec<String> {
    lines
        .iter()
        .filter_map(|(_, line)| line.trim().strip_prefix('-').map(str::trim))
        .filter(|text| !text.is_empty())
        .map(String::from)
        .collect()
}

fn parse_boundaries(lines: &[(usize, &str)]) -> SpecResult<Vec<Boundary>> {
    let mut items = Vec::new();
    let mut category = BoundaryCategory::General;

    for &(line_num, line) in lines {
        let trimmed = line.trim();

        if trimmed.starts_with("###") || trimmed.starts_with("### ") {
            let header = trimmed.trim_start_matches('#').trim().to_lowercase();
            if header.contains("允许修改") {
                return Err(cjk_keyword_error("允许修改", "Allowed Changes", line_num));
            }
            if header.contains("禁止") {
                return Err(cjk_keyword_error("禁止做", "Forbidden", line_num));
            }
            if header.contains("allowed") || header.contains("allow") {
                category = BoundaryCategory::Allow;
            } else if header.contains("forbidden")
                || header.contains("must not")
                || header.contains("disallow")
            {
                category = BoundaryCategory::Deny;
            }
            continue;
        }

        if let Some(text) = trimmed.strip_prefix('-') {
            let text = text.trim();
            if !text.is_empty() {
                items.push(Boundary {
                    text: text.to_string(),
                    category,
                    span: Span::line(line_num),
                });
            }
        }
    }

    Ok(items)
}

/// Uniform error for a rejected CJK structural keyword.
fn cjk_keyword_error(found: &str, replacement: &str, line: usize) -> SpecError {
    SpecError::Parse {
        message: format!(
            "keywords must be English; '{found}' is not recognized — use '{replacement}'"
        ),
        span: Span::line(line),
    }
}

type ParsedScenarios = (Vec<Scenario>, Vec<BehaviorRule>, Vec<MalformedRule>);

fn parse_scenarios(lines: &[(usize, &str)], rule_scope: &RuleScope) -> SpecResult<ParsedScenarios> {
    let mut scenarios = Vec::new();
    let mut rules: Vec<BehaviorRule> = Vec::new();
    let mut malformed_rules: Vec<MalformedRule> = Vec::new();
    let mut current_name: Option<(String, usize)> = None;
    let mut current_steps: Vec<Step> = Vec::new();
    let mut current_test_selector: Option<TestSelectorDraft> = None;
    let mut current_tags: Vec<String> = Vec::new();
    let mut current_review: ReviewMode = ReviewMode::default();
    let mut current_mode: ScenarioMode = ScenarioMode::Standard;
    let mut current_depends_on: Vec<String> = Vec::new();
    let mut reading_test_selector_block = false;
    // The active behavior rule id for scenarios that follow a `Rule:` header.
    let mut current_rule_id: Option<String> = None;
    // The rule the in-progress scenario belongs to, captured at its start so
    // a later `Rule:` header does not retroactively re-group it.
    let mut current_scenario_rule: Option<String> = None;

    // Flush helper assigns the scenario's rule and links it back to the rule.
    macro_rules! push_scenario {
        ($name:expr, $start:expr, $end:expr, $steps:expr, $selector:expr,
         $tags:expr, $review:expr, $mode:expr, $depends:expr, $rule:expr) => {{
            let rule_id: Option<String> = $rule;
            if let Some(id) = &rule_id
                && let Some(r) = rules.iter_mut().find(|r| &r.key.id == id)
            {
                r.scenario_names.push($name.clone());
            }
            scenarios.push(Scenario {
                name: $name,
                steps: $steps,
                test_selector: $selector,
                tags: $tags,
                review: $review,
                mode: $mode,
                depends_on: $depends,
                rule: rule_id,
                span: Span::new($start, 0, $end, 0),
            });
        }};
    }

    for &(line_num, line) in lines {
        if let Some(raw) = match_rule_header(line) {
            // A `Rule:` header closes any in-progress scenario (so stray steps
            // after the header do not leak into the prior scenario across the
            // rule boundary) and sets the active rule for subsequent scenarios.
            if let Some((prev_name, start)) = current_name.take() {
                let end = current_steps.last().map_or(start, |s| s.span.end_line);
                push_scenario!(
                    prev_name,
                    start,
                    end,
                    std::mem::take(&mut current_steps),
                    finalize_test_selector(current_test_selector.take(), end)?,
                    std::mem::take(&mut current_tags),
                    std::mem::take(&mut current_review),
                    std::mem::take(&mut current_mode),
                    std::mem::take(&mut current_depends_on),
                    current_scenario_rule.take()
                );
            }
            // Invalid (non-kebab) ids are dropped here; the `bdd-rule-id` lint
            // reports them from the raw source.
            let (id, name) = parse_rule_header_content(raw);
            match id {
                Some(id) => {
                    rules.push(BehaviorRule {
                        key: RuleKey {
                            scope: rule_scope.clone(),
                            id: id.clone(),
                        },
                        name,
                        scenario_names: Vec::new(),
                        events: Vec::new(),
                        span: Span::line(line_num),
                    });
                    current_rule_id = Some(id);
                }
                None => {
                    current_rule_id = None;
                    malformed_rules.push(MalformedRule {
                        raw: raw.to_string(),
                        span: Span::line(line_num),
                    });
                }
            }
            continue;
        }
        if let Some(name) = match_scenario_header(line) {
            // Flush previous scenario
            if let Some((prev_name, start)) = current_name.take() {
                let end = current_steps.last().map_or(start, |s| s.span.end_line);
                push_scenario!(
                    prev_name,
                    start,
                    end,
                    std::mem::take(&mut current_steps),
                    finalize_test_selector(current_test_selector.take(), end)?,
                    std::mem::take(&mut current_tags),
                    std::mem::take(&mut current_review),
                    std::mem::take(&mut current_mode),
                    std::mem::take(&mut current_depends_on),
                    current_scenario_rule.take()
                );
            }
            current_name = Some((name.to_string(), line_num));
            current_tags = Vec::new();
            current_review = ReviewMode::default();
            current_mode = ScenarioMode::Standard;
            current_depends_on = Vec::new();
            current_scenario_rule = current_rule_id.clone();
            reading_test_selector_block = false;
        } else if let Some(tags) = match_scenario_tags(line) {
            if current_name.is_some() {
                current_tags = tags;
            }
        } else if let Some(review_value) = match_review_field(line) {
            if current_name.is_some() {
                let lower = review_value.to_lowercase();
                if lower == "human" {
                    current_review = ReviewMode::Human;
                } else {
                    current_review = ReviewMode::Auto;
                }
            }
        } else if let Some(mode_value) = match_mode_field(line) {
            if current_name.is_some() {
                let lower = mode_value.to_lowercase();
                if lower == "optimize" {
                    current_mode = ScenarioMode::Optimize;
                } else {
                    current_mode = ScenarioMode::Standard;
                }
            }
        } else if let Some(depends_value) = match_depends_field(line) {
            if current_name.is_some() {
                current_depends_on = depends_value
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
        } else if let Some(selector) = match_test_selector(line) {
            if current_name.is_some() {
                let draft = current_test_selector.get_or_insert_with(TestSelectorDraft::default);
                if selector.is_empty() {
                    reading_test_selector_block = true;
                } else {
                    draft.filter = Some(selector.to_string());
                    reading_test_selector_block = false;
                }
            }
        } else if reading_test_selector_block {
            if let Some((field, value)) = match_test_selector_field(line) {
                let draft = current_test_selector.get_or_insert_with(TestSelectorDraft::default);
                match field {
                    TestSelectorField::Package => draft.package = Some(value.to_string()),
                    TestSelectorField::Filter => draft.filter = Some(value.to_string()),
                    TestSelectorField::Level => draft.level = Some(value.to_string()),
                    TestSelectorField::TestDouble => draft.test_double = Some(value.to_string()),
                    TestSelectorField::Targets => draft.targets = Some(value.to_string()),
                }
                continue;
            }
            if line.trim().is_empty() {
                continue;
            }
            reading_test_selector_block = false;
        }

        // Keywords must be English: reject legacy CJK structural lines with
        // an actionable message instead of silently treating them as prose.
        if let Some((cjk, en)) = detect_cjk_structural(line) {
            return Err(cjk_keyword_error(cjk, en, line_num));
        }

        // Only collect steps while inside a scenario, so steps under an
        // unrecognized header do not bleed into the next scenario.
        if current_name.is_none() {
            continue;
        }

        if let Some((cjk, en)) = detect_cjk_step_keyword(line) {
            return Err(cjk_keyword_error(cjk, en, line_num));
        }

        if let Some((kind, text)) = match_step_keyword(line) {
            let params = extract_params(text);
            current_steps.push(Step {
                kind,
                text: text.to_string(),
                params,
                table: Vec::new(),
                span: Span::line(line_num),
            });
        } else if let Some(row) = parse_table_row(line)
            && let Some(step) = current_steps.last_mut()
        {
            step.table.push(row);
            step.span.end_line = line_num;
        }
        // Ignore blank lines and non-step text inside scenarios
    }

    // Flush last scenario
    if let Some((name, start)) = current_name {
        let end = current_steps.last().map_or(start, |s| s.span.end_line);
        push_scenario!(
            name,
            start,
            end,
            current_steps,
            finalize_test_selector(current_test_selector, end)?,
            current_tags,
            current_review,
            current_mode,
            current_depends_on,
            current_scenario_rule.take()
        );
    }

    Ok((scenarios, rules, malformed_rules))
}

/// Split a `Rule:` header's raw content into `(id, display_name)`.
///
/// The id must be an explicit kebab-case identifier (`^[a-z][a-z0-9-]*$`).
/// Separator between id and display name is an em dash (`—`) or two-or-more
/// spaces. No auto-slugify: if the leading token is not a valid id, returns
/// `(None, raw)` so the `bdd-rule-id` lint can flag it and no rule is created.
fn parse_rule_header_content(raw: &str) -> (Option<String>, String) {
    // Strip any trailing HTML comment (e.g. promotion provenance) so it never
    // leaks into the rule id or display name.
    let raw = raw.split("<!--").next().unwrap_or(raw).trim();
    // Leftmost separator wins, so an em dash inside the display name does not
    // hijack a double-space-delimited id (and vice versa).
    let em = raw.find('—');
    let dsp = raw.find("  ");
    let (id_part, name_part) = match (em, dsp) {
        (Some(e), Some(d)) if d < e => (raw[..d].trim(), raw[d..].trim()),
        (Some(e), _) => (raw[..e].trim(), raw[e + '—'.len_utf8()..].trim()),
        (None, Some(d)) => (raw[..d].trim(), raw[d..].trim()),
        (None, None) => (raw, ""),
    };

    if is_valid_rule_id(id_part) {
        let name = if name_part.is_empty() {
            id_part.to_string()
        } else {
            name_part.to_string()
        };
        (Some(id_part.to_string()), name)
    } else {
        (None, raw.to_string())
    }
}

/// Validates a behavior rule id against `^[a-z][a-z0-9-]*$`.
fn is_valid_rule_id(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

#[derive(Default)]
struct TestSelectorDraft {
    package: Option<String>,
    filter: Option<String>,
    level: Option<String>,
    test_double: Option<String>,
    targets: Option<String>,
}

fn finalize_test_selector(
    draft: Option<TestSelectorDraft>,
    line_num: usize,
) -> SpecResult<Option<TestSelector>> {
    let Some(draft) = draft else {
        return Ok(None);
    };

    let Some(filter) = draft.filter else {
        return Err(SpecError::Parse {
            message: "test selector is missing required `Filter:` field".into(),
            span: Span::line(line_num),
        });
    };

    Ok(Some(TestSelector {
        filter,
        package: draft.package,
        level: draft.level,
        test_double: draft.test_double,
        targets: draft.targets,
    }))
}

fn parse_table_row(line: &str) -> Option<Vec<String>> {
    let trimmed = line.trim();
    if !trimmed.starts_with('|') {
        return None;
    }

    let row: Vec<String> = trimmed
        .trim_matches('|')
        .split('|')
        .map(str::trim)
        .map(String::from)
        .collect();

    if row.is_empty() || row.iter().all(|cell| cell.is_empty()) {
        None
    } else {
        Some(row)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::spec_core::StepKind;

    const SAMPLE_SPEC: &str = r#"spec: task
name: "Refund feature"
inherits: project
tags: [payment, refund]
---

## Intent

Add refund support to the payment gateway, covering full and partial refunds.

## Constraints

- Refund amount must not exceed the original transaction amount
- Refund operations require administrator privileges
- Refunds must be initiated within 90 days of the original transaction

## Acceptance Criteria

Scenario: Full refund
  Given a completed transaction with amount "100.00" and id "TXN-001"
  And the current user has administrator privileges
  When the user initiates a full refund for "TXN-001"
  Then the refund status becomes "processing"
  And the original transaction status becomes "refunding"

Scenario: Refund rejected - expired
  Given a transaction "TXN-003" completed 91 days ago
  When the user initiates a refund for "TXN-003"
  Then the system rejects the refund
  And the error message contains "refund window exceeded"

## Out of Scope

- Login feature
- Password reset
"#;

    #[test]
    fn test_parse_full_spec() {
        let doc = parse_spec_from_str(SAMPLE_SPEC).unwrap();

        assert_eq!(doc.meta.name, "Refund feature");
        assert_eq!(doc.meta.level, crate::spec_core::SpecLevel::Task);
        assert_eq!(doc.meta.inherits, Some("project".into()));
        assert_eq!(doc.meta.tags, vec!["payment", "refund"]);

        // Should have 4 sections: intent, constraints, acceptance, out-of-scope
        assert_eq!(doc.sections.len(), 4);

        // Intent
        match &doc.sections[0] {
            Section::Intent { content, .. } => {
                assert!(content.contains("refund support"));
            }
            other => panic!("expected Intent, got {other:?}"),
        }

        // Constraints
        match &doc.sections[1] {
            Section::Constraints { items, .. } => {
                assert_eq!(items.len(), 3);
                assert!(items[0].text.contains("Refund amount"));
            }
            other => panic!("expected Constraints, got {other:?}"),
        }

        // Scenarios
        match &doc.sections[2] {
            Section::AcceptanceCriteria { scenarios, .. } => {
                assert_eq!(scenarios.len(), 2);

                let s1 = &scenarios[0];
                assert_eq!(s1.name, "Full refund");
                assert_eq!(s1.steps.len(), 5);
                assert_eq!(s1.steps[0].kind, StepKind::Given);
                assert_eq!(s1.steps[0].params, vec!["100.00", "TXN-001"]);
                assert_eq!(s1.steps[1].kind, StepKind::And);
                assert_eq!(s1.steps[2].kind, StepKind::When);
                assert_eq!(s1.steps[2].params, vec!["TXN-001"]);
                assert_eq!(s1.steps[3].kind, StepKind::Then);
                assert_eq!(s1.steps[4].kind, StepKind::And);

                let s2 = &scenarios[1];
                assert_eq!(s2.name, "Refund rejected - expired");
                assert_eq!(s2.steps.len(), 4);
            }
            other => panic!("expected AcceptanceCriteria, got {other:?}"),
        }

        // Out of scope
        match &doc.sections[3] {
            Section::OutOfScope { items, .. } => {
                assert_eq!(items.len(), 2);
                assert_eq!(items[0], "Login feature");
            }
            other => panic!("expected OutOfScope, got {other:?}"),
        }
    }

    #[test]
    fn test_parse_english_spec() {
        let input = r#"spec: task
name: "User Registration"
---

## Intent

Implement user registration API.

## Constraints

- Passwords must be hashed with bcrypt
- Email must be unique

## Acceptance Criteria

Scenario: Successful registration
  Given no user with email "alice@example.com" exists
  When POST /api/v1/auth/register with email "alice@example.com"
  Then response status should be 201
  And response body should contain "id"
"#;
        let doc = parse_spec_from_str(input).unwrap();
        assert_eq!(doc.meta.name, "User Registration");
        assert_eq!(doc.sections.len(), 3);

        match &doc.sections[2] {
            Section::AcceptanceCriteria { scenarios, .. } => {
                assert_eq!(scenarios.len(), 1);
                assert_eq!(scenarios[0].name, "Successful registration");
                assert_eq!(scenarios[0].steps.len(), 4);
                assert_eq!(scenarios[0].steps[0].params, vec!["alice@example.com"]);
            }
            other => panic!("expected AcceptanceCriteria, got {other:?}"),
        }
    }

    #[test]
    fn test_parse_mixed_lang_keywords_rejected() {
        // deliberate Chinese fixture: the CJK step keyword `当` is the test subject.
        // Mixing CJK step keywords into an otherwise-English scenario used to
        // be accepted; keywords are now English-only.
        let input = r#"spec: task
name: "混合语言测试"
---

## Acceptance Criteria

Scenario: 混合场景
  Given 用户已登录
  当 用户点击 "submit" 按钮
  Then 页面应显示成功消息
"#;
        let err = parse_spec_from_str(input).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("keywords must be English"), "{msg}");
        assert!(msg.contains("When"), "{msg}");
    }

    #[test]
    fn test_parse_step_table_and_preserve_json_output() {
        let input = r#"spec: task
name: "Table test"
---

## Acceptance Criteria

Scenario: Registration request
  When a POST /api/v1/auth/register request is sent:
    | field    | value             |
    | email    | alice@example.com |
    | password | Str0ng!Pass#2024  |
  Then the response status code should be 201
"#;

        let doc = parse_spec_from_str(input).unwrap();
        match &doc.sections[0] {
            Section::AcceptanceCriteria { scenarios, .. } => {
                let when_step = &scenarios[0].steps[0];
                assert_eq!(when_step.kind, StepKind::When);
                assert_eq!(when_step.table.len(), 3);
                assert_eq!(when_step.table[0], vec!["field", "value"]);
                assert_eq!(when_step.table[1], vec!["email", "alice@example.com"]);
            }
            other => panic!("expected AcceptanceCriteria, got {other:?}"),
        }

        let json = serde_json::to_string_pretty(&doc).unwrap();
        assert!(json.contains("\"table\""));
        assert!(json.contains("alice@example.com"));
        assert!(json.contains("Str0ng!Pass#2024"));
    }

    #[test]
    fn test_parse_scenario_without_table_stays_unchanged() {
        let input = r#"spec: task
name: "Plain scenario"
---

## Acceptance Criteria

Scenario: No table
  Given the user is logged in
  When the user clicks submit
  Then the page shows success
"#;

        let doc = parse_spec_from_str(input).unwrap();
        match &doc.sections[0] {
            Section::AcceptanceCriteria { scenarios, .. } => {
                let scenario = &scenarios[0];
                assert_eq!(scenario.steps.len(), 3);
                assert!(scenario.steps.iter().all(|step| step.table.is_empty()));
                assert_eq!(scenario.steps[1].text, "the user clicks submit");
            }
            other => panic!("expected AcceptanceCriteria, got {other:?}"),
        }
    }

    #[test]
    fn test_parse_task_contract_sections() {
        let input = r#"spec: task
name: "Contract"
---

## Intent

Implement the task safely.

## Decisions

- Use existing parser module

## Boundaries

### Allowed Changes
- crates/spec-parser/**

### Forbidden
- Do not modify crates/spec-verify/**

## Completion Criteria

Scenario: Parse succeeds
  Given a valid contract
  When the parser reads it
  Then the parser should succeed
"#;

        let doc = parse_spec_from_str(input).unwrap();
        assert_eq!(doc.sections.len(), 4);

        match &doc.sections[1] {
            Section::Decisions { items, .. } => {
                assert_eq!(items, &vec!["Use existing parser module".to_string()]);
            }
            other => panic!("expected Decisions, got {other:?}"),
        }

        match &doc.sections[2] {
            Section::Boundaries { items, .. } => {
                assert_eq!(items.len(), 2);
                assert_eq!(items[0].category, BoundaryCategory::Allow);
                assert_eq!(items[1].category, BoundaryCategory::Deny);
            }
            other => panic!("expected Boundaries, got {other:?}"),
        }

        match &doc.sections[3] {
            Section::AcceptanceCriteria { scenarios, .. } => {
                assert_eq!(scenarios.len(), 1);
            }
            other => panic!("expected AcceptanceCriteria, got {other:?}"),
        }
    }

    #[test]
    fn test_parse_scenario_with_explicit_test_selector() {
        let input = r#"spec: task
name: "Binding test"
---

## Completion Criteria

Scenario: Explicit binding
  Test: test_parse_scenario_with_explicit_test_selector
  Given a scenario declares a test selector
  When the parser reads the scenario
  Then the AST keeps the selector
"#;

        let doc = parse_spec_from_str(input).unwrap();
        match &doc.sections[0] {
            Section::AcceptanceCriteria { scenarios, .. } => {
                assert_eq!(scenarios.len(), 1);
                assert_eq!(
                    scenarios[0]
                        .test_selector
                        .as_ref()
                        .map(|selector| selector.filter.as_str()),
                    Some("test_parse_scenario_with_explicit_test_selector")
                );
                assert_eq!(scenarios[0].steps.len(), 3);
            }
            other => panic!("expected AcceptanceCriteria, got {other:?}"),
        }

        let json = serde_json::to_string_pretty(&doc).unwrap();
        assert!(json.contains("\"test_selector\""));
        assert!(json.contains("\"filter\""));
        assert!(json.contains("test_parse_scenario_with_explicit_test_selector"));
    }

    #[test]
    fn test_parse_structured_test_selector_block() {
        let input = r#"spec: task
name: "Structured binding"
---

## Completion Criteria

Scenario: Structured binding
  Test:
    Package: spec-parser
    Filter: test_parse_structured_test_selector_block
  Given a scenario declares a structured test selector
  When the parser reads the scenario
  Then the AST keeps the structured fields
"#;

        let doc = parse_spec_from_str(input).unwrap();
        match &doc.sections[0] {
            Section::AcceptanceCriteria { scenarios, .. } => {
                assert_eq!(scenarios.len(), 1);
                let selector = scenarios[0].test_selector.as_ref().unwrap();
                assert_eq!(selector.package.as_deref(), Some("spec-parser"));
                assert_eq!(selector.filter, "test_parse_structured_test_selector_block");
            }
            other => panic!("expected AcceptanceCriteria, got {other:?}"),
        }

        let json = serde_json::to_string_pretty(&doc).unwrap();
        assert!(json.contains("\"package\""));
        assert!(json.contains("\"spec-parser\""));
        assert!(json.contains("\"filter\""));
        assert!(json.contains("test_parse_structured_test_selector_block"));
    }

    #[test]
    fn test_parse_scenario_verification_metadata_fields() {
        let input = r#"spec: task
name: "Verification metadata"
---

## Completion Criteria

Scenario: Structured verification strength
  Test:
    Package: agent-spec
    Filter: test_parse_scenario_verification_metadata_fields
    Level: integration
    Test Double: local_http_stub
    Targets: commands/update
  Given a scenario declares verification metadata
  When the parser reads the scenario
  Then the AST keeps these fields
"#;

        let doc = parse_spec_from_str(input).unwrap();
        match &doc.sections[0] {
            Section::AcceptanceCriteria { scenarios, .. } => {
                let selector = scenarios[0].test_selector.as_ref().unwrap();
                assert_eq!(selector.package.as_deref(), Some("agent-spec"));
                assert_eq!(
                    selector.filter,
                    "test_parse_scenario_verification_metadata_fields"
                );
                assert_eq!(selector.level.as_deref(), Some("integration"));
                assert_eq!(selector.test_double.as_deref(), Some("local_http_stub"));
                assert_eq!(selector.targets.as_deref(), Some("commands/update"));
            }
            other => panic!("expected AcceptanceCriteria, got {other:?}"),
        }

        let json = serde_json::to_string_pretty(&doc).unwrap();
        assert!(json.contains("\"level\""));
        assert!(json.contains("\"test_double\""));
        assert!(json.contains("\"targets\""));
    }

    #[test]
    fn test_parse_english_verification_metadata_fields() {
        let input = r#"spec: task
name: "verification metadata"
---

## Completion Criteria

Scenario: verification metadata
  Test:
    Package: agent-spec
    Filter: test_parse_english_verification_metadata_fields
    Level: integration
    Test Double: local_http_stub
    Targets: commands/update
  Given a scenario declares verification metadata
  When the parser reads it
  Then the AST keeps the metadata
"#;

        let doc = parse_spec_from_str(input).unwrap();
        match &doc.sections[0] {
            Section::AcceptanceCriteria { scenarios, .. } => {
                let selector = scenarios[0].test_selector.as_ref().unwrap();
                assert_eq!(selector.level.as_deref(), Some("integration"));
                assert_eq!(selector.test_double.as_deref(), Some("local_http_stub"));
                assert_eq!(selector.targets.as_deref(), Some("commands/update"));
            }
            other => panic!("expected AcceptanceCriteria, got {other:?}"),
        }
    }

    #[test]
    fn test_existing_specs_without_verification_metadata_remain_valid() {
        let input = r#"spec: task
name: "legacy selector"
---

## Completion Criteria

Scenario: legacy selector
  Test:
    Package: agent-spec
    Filter: test_existing_specs_without_verification_metadata_remain_valid
  Given a legacy spec
  When the parser reads it
  Then the selector remains valid
"#;

        let doc = parse_spec_from_str(input).unwrap();
        match &doc.sections[0] {
            Section::AcceptanceCriteria { scenarios, .. } => {
                let selector = scenarios[0].test_selector.as_ref().unwrap();
                assert_eq!(selector.package.as_deref(), Some("agent-spec"));
                assert_eq!(
                    selector.filter,
                    "test_existing_specs_without_verification_metadata_remain_valid"
                );
                assert_eq!(selector.level, None);
                assert_eq!(selector.test_double, None);
                assert_eq!(selector.targets, None);
            }
            other => panic!("expected AcceptanceCriteria, got {other:?}"),
        }
    }

    #[test]
    fn test_parse_shorthand_test_selector_as_filter_only() {
        let input = r#"spec: task
name: "Single-line binding"
---

## Completion Criteria

Scenario: Single-line binding
  Test: test_parse_shorthand_test_selector_as_filter_only
  Given a scenario keeps using a single-line test binding
  When the parser reads the scenario
  Then the filter field is preserved
"#;

        let doc = parse_spec_from_str(input).unwrap();
        match &doc.sections[0] {
            Section::AcceptanceCriteria { scenarios, .. } => {
                let selector = scenarios[0].test_selector.as_ref().unwrap();
                assert_eq!(
                    selector.filter,
                    "test_parse_shorthand_test_selector_as_filter_only"
                );
                assert_eq!(selector.package, None);
            }
            other => panic!("expected AcceptanceCriteria, got {other:?}"),
        }
    }

    #[test]
    fn test_missing_front_matter() {
        let input = "## Intent\nSome content\n";
        let result = parse_spec_from_str(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_unknown_top_level_section_header_is_rejected() {
        let input = r#"spec: task
name: "Unknown section"
---

## Intent

Describe the task.

## Milestones

- phase 1
"#;

        let err = parse_spec_from_str(input).unwrap_err();
        match err {
            SpecError::Parse { message, span } => {
                assert!(message.contains("unknown top-level section header"));
                assert_eq!(span.start_line, 9);
            }
            other => panic!("expected parse error, got {other:?}"),
        }
    }

    #[test]
    fn test_markdown_heading_scenarios_and_test_selectors_are_accepted() {
        let input = r#"spec: task
name: "Markdown Scenario"
---

## Completion Criteria

### Scenario: Happy path
  ### Test: test_markdown_heading_scenarios_and_test_selectors_are_accepted
  Given valid input
  When parser reads the scenario
  Then the scenario is preserved
"#;

        let doc = parse_spec_from_str(input).unwrap();
        match &doc.sections[0] {
            Section::AcceptanceCriteria { scenarios, .. } => {
                assert_eq!(scenarios.len(), 1);
                assert_eq!(scenarios[0].name, "Happy path");
                assert_eq!(
                    scenarios[0]
                        .test_selector
                        .as_ref()
                        .map(|selector| selector.filter.as_str()),
                    Some("test_markdown_heading_scenarios_and_test_selectors_are_accepted")
                );
            }
            other => panic!("expected AcceptanceCriteria, got {other:?}"),
        }
    }

    #[test]
    fn test_serialization_roundtrip() {
        let doc = parse_spec_from_str(SAMPLE_SPEC).unwrap();
        let json = serde_json::to_string_pretty(&doc).unwrap();
        let _: SpecDocument = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_parse_mode_field_in_scenario() {
        let input = r#"spec: task
name: "Mode test"
---

## Completion Criteria

Scenario: Optimize scenario
  Mode: optimize
  Test: test_parse_mode_field_in_scenario
  Given a scenario declares optimize mode
  When the parser reads the scenario
  Then the mode field in the AST is Optimize
"#;
        let doc = parse_spec_from_str(input).unwrap();
        match &doc.sections[0] {
            Section::AcceptanceCriteria { scenarios, .. } => {
                assert_eq!(scenarios.len(), 1);
                assert_eq!(scenarios[0].mode, crate::spec_core::ScenarioMode::Optimize);
            }
            other => panic!("expected AcceptanceCriteria, got {other:?}"),
        }
    }

    #[test]
    fn test_parse_mode_field_english() {
        let input = r#"spec: task
name: "mode test"
---

## Completion Criteria

Scenario: optimize scenario
  Mode: optimize
  Given an optimize-mode scenario
  When parser reads it
  Then mode is Optimize
"#;
        let doc = parse_spec_from_str(input).unwrap();
        match &doc.sections[0] {
            Section::AcceptanceCriteria { scenarios, .. } => {
                assert_eq!(scenarios[0].mode, crate::spec_core::ScenarioMode::Optimize);
            }
            other => panic!("expected AcceptanceCriteria, got {other:?}"),
        }
    }

    #[test]
    fn test_parse_mode_field_standard_is_default() {
        let input = r#"spec: task
name: "default mode"
---

## Completion Criteria

Scenario: standard scenario
  Mode: standard
  Given a standard scenario
  When parser reads it
  Then mode is Standard

Scenario: no mode declared
  Given no mode field
  When parser reads it
  Then mode defaults to Standard
"#;
        let doc = parse_spec_from_str(input).unwrap();
        match &doc.sections[0] {
            Section::AcceptanceCriteria { scenarios, .. } => {
                assert_eq!(scenarios[0].mode, crate::spec_core::ScenarioMode::Standard);
                assert_eq!(scenarios[1].mode, crate::spec_core::ScenarioMode::Standard);
            }
            other => panic!("expected AcceptanceCriteria, got {other:?}"),
        }
    }

    #[test]
    fn test_parse_depends_field_in_scenario() {
        let input = r#"spec: task
name: "Dependency test"
---

## Completion Criteria

Scenario: User registration
  Given the registration form is open
  When the user submits registration
  Then registration succeeds

Scenario: User login
  Depends: User registration
  Given a registered user exists
  When the user logs in
  Then login succeeds
"#;
        let doc = parse_spec_from_str(input).unwrap();
        match &doc.sections[0] {
            Section::AcceptanceCriteria { scenarios, .. } => {
                assert_eq!(scenarios.len(), 2);
                assert!(scenarios[0].depends_on.is_empty());
                assert_eq!(scenarios[1].depends_on, vec!["User registration"]);
            }
            other => panic!("expected AcceptanceCriteria, got {other:?}"),
        }
    }

    #[test]
    fn test_parse_depends_field_multiple() {
        let input = r#"spec: task
name: "multi depends"
---

## Completion Criteria

Scenario: A
  Given A
  When A
  Then A

Scenario: B
  Given B
  When B
  Then B

Scenario: C
  Depends: A, B
  Given C depends on A and B
  When parser reads it
  Then depends_on contains both
"#;
        let doc = parse_spec_from_str(input).unwrap();
        match &doc.sections[0] {
            Section::AcceptanceCriteria { scenarios, .. } => {
                assert_eq!(scenarios[2].depends_on, vec!["A", "B"]);
            }
            other => panic!("expected AcceptanceCriteria, got {other:?}"),
        }
    }

    // ---- BDD semantics v1: Rule / Example parsing ----

    use crate::spec_core::{BehaviorRule, RuleScope, Section as Sec};

    fn rules_of(doc: &SpecDocument) -> Vec<BehaviorRule> {
        doc.sections
            .iter()
            .find_map(|s| match s {
                Sec::AcceptanceCriteria { rules, .. } => Some(rules.clone()),
                _ => None,
            })
            .unwrap_or_default()
    }

    fn scenarios_of(doc: &SpecDocument) -> Vec<Scenario> {
        doc.sections
            .iter()
            .find_map(|s| match s {
                Sec::AcceptanceCriteria { scenarios, .. } => Some(scenarios.clone()),
                _ => None,
            })
            .unwrap_or_default()
    }

    #[test]
    fn test_parse_rule_header_creates_behavior_rule() {
        let input = r#"spec: task
name: "Auth"
---

## Completion Criteria

### Rule: auth-must-not-leak — Auth failures must not leak internal errors
Scenario: Failure returns a stable error
  Test: test_auth_stable_error
  Given authentication fails
  When the response is returned
  Then it contains no internal stack trace
"#;
        let doc = parse_spec_from_str_with_stem(input, "task-auth").unwrap();
        let rules = rules_of(&doc);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].key.id, "auth-must-not-leak");
        assert_eq!(rules[0].key.scope, RuleScope::Task("task-auth".into()));
        assert_eq!(rules[0].name, "Auth failures must not leak internal errors");
        assert_eq!(
            rules[0].scenario_names,
            vec!["Failure returns a stable error".to_string()]
        );

        let scenarios = scenarios_of(&doc);
        assert_eq!(scenarios[0].rule.as_deref(), Some("auth-must-not-leak"));
    }

    #[test]
    fn test_parse_rule_header_without_display_name() {
        let input = r#"spec: task
name: "Refund"
---

## Completion Criteria

### Rule: refund-must-be-idempotent
Scenario: Repeated refunds take effect only once
  Test: test_refund_idempotent
  Given a refund was already made
  When a refund is attempted again
  Then no duplicate deduction happens
"#;
        let doc = parse_spec_from_str_with_stem(input, "task-refund").unwrap();
        let rules = rules_of(&doc);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].key.id, "refund-must-be-idempotent");
        assert_eq!(rules[0].name, "refund-must-be-idempotent");
    }

    #[test]
    fn test_chinese_rule_alias_rejected() {
        // deliberate Chinese fixture: the CJK keyword `规则:` is the test subject.
        // `规则:` used to be an accepted alias of `Rule:`; keywords are now
        // English-only and the parser must point at the replacement.
        let input = r#"spec: task
name: "促销"
---

## Completion Criteria

规则: vip-discount-priority — VIP 折扣优先级高于促销
Example: VIP 用户折扣优先
  Test: test_vip_priority
  Given 用户是 VIP
  When 同时存在促销
  Then 应用 VIP 折扣
"#;
        let err = parse_spec_from_str_with_stem(input, "task-promo").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("keywords must be English"), "{msg}");
        assert!(msg.contains("Rule:"), "{msg}");
    }

    #[test]
    fn test_parse_example_alias_as_scenario() {
        let input = r#"spec: task
name: "Withdrawal"
---

## Completion Criteria

Example: Withdrawal succeeds with sufficient balance
  Test: test_withdraw_ok
  Given a balance of "200"
  When withdrawing "100"
  Then it succeeds

Example: Withdrawal fails with insufficient balance
  Test: test_withdraw_insufficient
  Given a balance of "50"
  When withdrawing "100"
  Then it is rejected
"#;
        let doc = parse_spec_from_str_with_stem(input, "task-withdraw").unwrap();
        let scenarios = scenarios_of(&doc);
        assert_eq!(scenarios.len(), 2);
        assert_eq!(
            scenarios[0].name,
            "Withdrawal succeeds with sufficient balance"
        );
        assert_eq!(
            scenarios[1].name,
            "Withdrawal fails with insufficient balance"
        );
        // No new "Example" AST node — both are Scenario, serialized as such.
        let json = serde_json::to_string(&doc).unwrap();
        assert!(!json.contains("\"Example\""));
        assert!(!json.contains("\"example\""));
    }

    #[test]
    fn test_legacy_spec_without_rule_compat() {
        // A spec with no Rule line: scenarios carry rule == None, no rules emitted.
        let doc = parse_spec_from_str(SAMPLE_SPEC).unwrap();
        let scenarios = scenarios_of(&doc);
        assert!(!scenarios.is_empty());
        assert!(scenarios.iter().all(|s| s.rule.is_none()));
        assert!(rules_of(&doc).is_empty());
    }

    #[test]
    fn test_rule_scope_serializes_to_json() {
        let input = r#"spec: task
name: "Auth"
---

## Completion Criteria

### Rule: auth-must-not-leak — Auth failures must not leak internal errors
Scenario: Failure returns a stable error
  Test: test_auth_stable_error
  Given authentication fails
  When the response is returned
  Then it contains no internal stack trace
"#;
        let doc = parse_spec_from_str_with_stem(input, "task-auth").unwrap();
        let json = serde_json::to_string(&doc).unwrap();
        // RuleScope::Task(stem) serializes as { "task": "<stem>" }
        assert!(json.contains(r#""scope":{"task":"task-auth"}"#));
        assert!(json.contains(r#""id":"auth-must-not-leak""#));
        assert!(json.contains(r#""rule":"auth-must-not-leak""#));
    }

    #[test]
    fn test_capability_scope_is_reserved_in_v1() {
        // Capability / Project scope are declared and (de)serialize cleanly,
        // but the v1 parser never produces them.
        let doc = parse_spec_from_str_with_stem(
            r#"spec: task
name: "Auth"
---

## Completion Criteria

### Rule: auth-ok
Scenario: Pass
  Test: test_ok
  Given a
  When b
  Then c
"#,
            "task-auth",
        )
        .unwrap();
        assert!(
            rules_of(&doc)
                .iter()
                .all(|r| matches!(r.key.scope, RuleScope::Task(_)))
        );

        // Reserved variants round-trip through serde.
        let cap = RuleScope::Capability("ecosystem-import".into());
        let cap_json = serde_json::to_string(&cap).unwrap();
        assert_eq!(cap_json, r#"{"capability":"ecosystem-import"}"#);
        let back: RuleScope = serde_json::from_str(&cap_json).unwrap();
        assert_eq!(back, cap);
        let proj_json = serde_json::to_string(&RuleScope::Project).unwrap();
        assert_eq!(proj_json, r#""project""#);
        let proj_back: RuleScope = serde_json::from_str(&proj_json).unwrap();
        assert_eq!(proj_back, RuleScope::Project);
    }

    #[test]
    fn test_json_output_additive_only() {
        // A legacy spec (no Rule) must not emit `rules` or scenario `rule`
        // keys, so v0.2.7 consumers see an unchanged shape.
        let doc = parse_spec_from_str(SAMPLE_SPEC).unwrap();
        let json = serde_json::to_string(&doc).unwrap();
        assert!(!json.contains("\"rules\""));
        assert!(!json.contains("\"rule\""));
    }

    // ---- Adversarial hunt regressions (Phase 1 parser) ----

    #[test]
    fn test_rule_double_space_separator_with_em_dash_in_display() {
        // Bug 1/2: leftmost separator wins. Double-space is the intended
        // id/name separator even when the display name contains an em dash.
        let input = "spec: task\nname: \"x\"\n---\n\n## Completion Criteria\n\n### Rule: rate-limit  Throttling — protect upstream\nScenario: Over threshold\n  Test: t\n  When a\n  Then b\n";
        let doc = parse_spec_from_str_with_stem(input, "task-x").unwrap();
        let rules = rules_of(&doc);
        assert_eq!(rules.len(), 1, "double-space separator must yield one rule");
        assert_eq!(rules[0].key.id, "rate-limit");
        assert_eq!(rules[0].name, "Throttling — protect upstream");
        assert_eq!(scenarios_of(&doc)[0].rule.as_deref(), Some("rate-limit"));
    }

    #[test]
    fn test_rule_em_dash_separator_with_double_space_in_display() {
        // Mirror: em-dash is leftmost -> it is the separator; trailing double
        // spaces inside the display name are preserved (trimmed at ends only).
        let input = "spec: task\nname: \"x\"\n---\n\n## Completion Criteria\n\n### Rule: auth-leak — fails  must not leak\nScenario: s\n  Test: t\n  When a\n  Then b\n";
        let doc = parse_spec_from_str_with_stem(input, "task-x").unwrap();
        let rules = rules_of(&doc);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].key.id, "auth-leak");
        assert_eq!(rules[0].name, "fails  must not leak");
    }

    #[test]
    fn test_stray_step_after_rule_header_does_not_leak_into_prior_scenario() {
        // Bug 3: a step-like line between a Rule header and the next scenario
        // must not attach to the previous scenario across the rule boundary.
        let input = "spec: task\nname: \"x\"\n---\n\n## Completion Criteria\n\n### Rule: rule-one — first\nScenario: A\n  When b\n  Then c\n\n### Rule: rule-two — second\n  Then stray\nScenario: B\n  When e\n  Then f\n";
        let doc = parse_spec_from_str_with_stem(input, "task-x").unwrap();
        let scenarios = scenarios_of(&doc);
        let a = scenarios.iter().find(|s| s.name == "A").unwrap();
        assert_eq!(
            a.steps.len(),
            2,
            "scenario A must keep exactly its own steps"
        );
        assert_eq!(a.rule.as_deref(), Some("rule-one"));
        let b = scenarios.iter().find(|s| s.name == "B").unwrap();
        assert_eq!(b.rule.as_deref(), Some("rule-two"));
    }

    #[test]
    fn test_fullwidth_colon_english_scenario_header_is_recognized() {
        // Bug 4: `Scenario：` (English word + full-width colon) must parse as a
        // header, so steps do not bleed into the next scenario.
        let input = "spec: task\nname: \"x\"\n---\n\n## Completion Criteria\n\nScenario：A\n  Given a1\n  When a2\n  Then a3\n\nScenario: B\n  Given b1\n  When b2\n  Then b3\n";
        let doc = parse_spec_from_str(input).unwrap();
        let scenarios = scenarios_of(&doc);
        assert_eq!(scenarios.len(), 2, "both scenarios must be recognized");
        assert_eq!(scenarios[0].name, "A");
        assert_eq!(
            scenarios[0].steps.len(),
            3,
            "A's steps must not bleed into B"
        );
        assert_eq!(scenarios[1].steps.len(), 3);
    }

    #[test]
    fn test_fullwidth_colon_english_rule_header_is_recognized() {
        // Bug 5: `Rule：` (English word + full-width colon) must create a rule.
        let input = "spec: task\nname: \"x\"\n---\n\n## Completion Criteria\n\nRule：auth-must-not-leak\nScenario: stable error\n  When a\n  Then b\n";
        let doc = parse_spec_from_str_with_stem(input, "task-x").unwrap();
        let rules = rules_of(&doc);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].key.id, "auth-must-not-leak");
        assert_eq!(
            scenarios_of(&doc)[0].rule.as_deref(),
            Some("auth-must-not-leak")
        );
    }

    #[test]
    fn test_probe_from_scenario_with_selector() {
        use crate::spec_core::Probe;
        let input = r#"spec: task
name: "x"
---

## Completion Criteria

Scenario: Binding
  Test: test_x
  When a
  Then b
"#;
        let doc = parse_spec_from_str(input).unwrap();
        let sc = &scenarios_of(&doc)[0];
        match Probe::from_scenario(sc) {
            Some(Probe::Test(sel)) => assert_eq!(sel.filter, "test_x"),
            other => panic!("expected Probe::Test, got {other:?}"),
        }
    }

    #[test]
    fn test_probe_from_scenario_without_selector() {
        use crate::spec_core::Probe;
        let input = r#"spec: task
name: "x"
---

## Completion Criteria

Scenario: No binding
  When a
  Then b
"#;
        let doc = parse_spec_from_str(input).unwrap();
        let sc = &scenarios_of(&doc)[0];
        assert!(Probe::from_scenario(sc).is_none());
    }

    #[test]
    fn test_scenario_unchanged_no_probe_field() {
        use crate::spec_core::Probe;
        // Scenario still exposes binding via test_selector; Probe is a derived view.
        let doc = parse_spec_from_str(SAMPLE_SPEC).unwrap();
        for sc in scenarios_of(&doc) {
            // No panic; derivation is consistent with test_selector presence.
            assert_eq!(
                Probe::from_scenario(&sc).is_some(),
                sc.test_selector.is_some()
            );
        }
    }

    #[test]
    fn test_parse_lint_ack_marker() {
        let input = r#"spec: task
name: "x"
---

## Intent

Do something.
<!-- lint-ack: bdd-rule-id — deliberately kept as an example -->

## Completion Criteria

Scenario: s
  Test: t
  When a
  Then b
"#;
        let doc = parse_spec_from_str(input).unwrap();
        assert_eq!(doc.lint_acks.len(), 1);
        assert_eq!(doc.lint_acks[0].code, "bdd-rule-id");
        assert!(
            doc.lint_acks[0]
                .reason
                .contains("deliberately kept as an example")
        );
    }

    #[test]
    fn test_lint_acks_additive_empty() {
        let doc = parse_spec_from_str(SAMPLE_SPEC).unwrap();
        assert!(doc.lint_acks.is_empty());
        let json = serde_json::to_string(&doc).unwrap();
        assert!(!json.contains("lint_acks"));
    }

    fn questions_of(doc: &SpecDocument) -> Option<Vec<String>> {
        doc.sections.iter().find_map(|s| match s {
            Sec::Questions { items, .. } => Some(items.clone()),
            _ => None,
        })
    }

    #[test]
    fn test_parse_questions_section() {
        let input = r#"spec: task
name: "x"
---

## Questions

- Can discounts stack?
- Are refunds based on the original or the discounted price?
"#;
        let doc = parse_spec_from_str(input).unwrap();
        let q = questions_of(&doc).expect("Questions section present");
        assert_eq!(q.len(), 2);
        assert!(q[0].contains("discounts"));
    }

    #[test]
    fn test_chinese_questions_header_rejected() {
        // deliberate Chinese fixture: the CJK section header `## 问题` is the test subject.
        let input = r#"spec: task
name: "x"
---

## 问题

- VIP 等级怎么定义?
"#;
        let err = parse_spec_from_str(input).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("keywords must be English"), "{msg}");
        assert!(msg.contains("Questions"), "{msg}");
    }

    #[test]
    fn test_spec_without_questions_unaffected() {
        let doc = parse_spec_from_str(SAMPLE_SPEC).unwrap();
        assert!(questions_of(&doc).is_none());
    }

    #[test]
    fn test_questions_do_not_affect_verification() {
        // Questions are not scenarios: all_scenarios excludes them.
        let input = r#"spec: task
name: "x"
---

## Completion Criteria

Scenario: Only scenario
  Test: t
  When a
  Then b

## Questions

- A question not yet thought through
"#;
        let doc = parse_spec_from_str(input).unwrap();
        let resolved = crate::spec_parser::resolve_spec(doc, &[]).unwrap();
        assert_eq!(
            resolved.all_scenarios.len(),
            1,
            "questions must not count as scenarios"
        );
    }

    #[test]
    fn test_capability_spec_rule_has_capability_scope() {
        let input = r#"spec: capability
name: "ecosystem-import"
---

## Completion Criteria

### Rule: import-preserves-traceability — Import preserves source IDs
"#;
        let doc = parse_spec_from_str_with_stem(input, "ignored-stem").unwrap();
        let rules = rules_of(&doc);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].key.id, "import-preserves-traceability");
        assert_eq!(
            rules[0].key.scope,
            RuleScope::Capability("ecosystem-import".into())
        );
    }

    #[test]
    fn test_task_without_capability_is_none_additive() {
        let doc = parse_spec_from_str(SAMPLE_SPEC).unwrap();
        assert_eq!(doc.meta.capability, None);
        let json = serde_json::to_string(&doc).unwrap();
        assert!(!json.contains("\"capability\""));
    }

    #[test]
    fn test_parse_review_field_in_scenario() {
        let input = r#"spec: task
name: "Review test"
---

## Completion Criteria

Scenario: Requires human review
  Review: human
  Test: test_parse_review_field_in_scenario
  Given a scenario declares review as human
  When the parser reads the scenario
  Then the review field in the AST is Human

Scenario: Default auto review
  Test: test_default_auto_review
  Given a scenario declares no review field
  When the parser reads the scenario
  Then the review field in the AST is Auto
"#;

        let doc = parse_spec_from_str(input).unwrap();
        match &doc.sections[0] {
            Section::AcceptanceCriteria { scenarios, .. } => {
                assert_eq!(scenarios.len(), 2);
                assert_eq!(
                    scenarios[0].review,
                    crate::spec_core::ReviewMode::Human,
                    "scenario with 'Review: human' should have ReviewMode::Human"
                );
                assert_eq!(
                    scenarios[1].review,
                    crate::spec_core::ReviewMode::Auto,
                    "scenario without review field should default to ReviewMode::Auto"
                );
            }
            other => panic!("expected AcceptanceCriteria, got {other:?}"),
        }
    }

    // ---- English-only keywords: parser-level rejection contract ----

    fn parse_err_msg(input: &str) -> String {
        parse_spec_from_str(input).unwrap_err().to_string()
    }

    #[test]
    fn test_parser_accepts_current_state_section() {
        let spec = r#"spec: task
name: "sdd"
---

## Intent

Add a thing.

## Current State

The verifier only runs cargo test today.

## Completion Criteria

Scenario: ok
  Test: test_ok
  Given a thing
  When it runs
  Then it passes
"#;
        let doc = parse_spec_from_str(spec).unwrap();
        let found = doc.sections.iter().find_map(|s| match s {
            Section::CurrentState { content, .. } => Some(content.clone()),
            _ => None,
        });
        assert_eq!(
            found.as_deref(),
            Some("The verifier only runs cargo test today.")
        );
    }

    #[test]
    fn test_parser_accepts_ux_shape_section() {
        let spec = r#"spec: task
name: "sdd"
---

## Intent

Add a thing.

## UX Shape

+----------------+
| Cron Jobs      |
+----------------+

## Completion Criteria

Scenario: ok
  Test: test_ok
  Given a thing
  When it runs
  Then it passes
"#;
        let doc = parse_spec_from_str(spec).unwrap();
        let found = doc.sections.iter().find_map(|s| match s {
            Section::UxShape { content, .. } => Some(content.clone()),
            _ => None,
        });
        let content = found.unwrap();
        assert!(content.contains("| Cron Jobs      |"), "got: {content}");
    }

    #[test]
    fn test_parser_accepts_open_questions_header_alias() {
        let spec = r#"spec: task
name: "sdd"
---

## Intent

Add a thing.

## Open Questions

- which auth flow do we standardize on?
"#;
        let doc = parse_spec_from_str(spec).unwrap();
        let found = doc.sections.iter().find_map(|s| match s {
            Section::Questions { items, .. } => Some(items.clone()),
            _ => None,
        });
        assert_eq!(
            found.unwrap(),
            vec!["which auth flow do we standardize on?".to_string()]
        );
    }

    #[test]
    fn test_parser_rejects_cjk_section_header_with_suggestion() {
        let input = r#"spec: task
name: "x"
---

## 意图

为支付网关添加退款功能。
"#;
        let msg = parse_err_msg(input);
        assert!(msg.contains("keywords must be English"), "{msg}");
        assert!(msg.contains("Intent"), "{msg}");
    }

    #[test]
    fn test_parser_rejects_cjk_scenario_keyword() {
        let input = r#"spec: task
name: "x"
---

## Acceptance Criteria

场景: 全额退款
  Given 输入有效
  When 调用函数
  Then 返回 Ok
"#;
        let msg = parse_err_msg(input);
        assert!(msg.contains("keywords must be English"), "{msg}");
        assert!(msg.contains("Scenario:"), "{msg}");
    }

    #[test]
    fn test_parser_rejects_cjk_step_keyword() {
        let input = r#"spec: task
name: "x"
---

## Acceptance Criteria

Scenario: 全额退款
  假设 存在一笔已完成交易
  When 用户发起退款
  Then 退款成功
"#;
        let msg = parse_err_msg(input);
        assert!(msg.contains("keywords must be English"), "{msg}");
        assert!(msg.contains("Given"), "{msg}");
    }

    #[test]
    fn test_parser_rejects_cjk_test_selector_keys() {
        // Single-line CJK selector: `测试:` -> `Test:`
        let msg = parse_err_msg(
            r#"spec: task
name: "x"
---

## Completion Criteria

Scenario: 绑定
  测试: foo
  When a
  Then b
"#,
        );
        assert!(msg.contains("keywords must be English"), "{msg}");
        assert!(msg.contains("Test:"), "{msg}");

        // CJK field inside an English `Test:` block: `包:` -> `Package:`
        let msg = parse_err_msg(
            r#"spec: task
name: "x"
---

## Completion Criteria

Scenario: 绑定
  Test:
    包: x
    Filter: foo
  When a
  Then b
"#,
        );
        assert!(msg.contains("keywords must be English"), "{msg}");
        assert!(msg.contains("Package:"), "{msg}");

        // CJK field inside an English `Test:` block: `过滤:` -> `Filter:`
        let msg = parse_err_msg(
            r#"spec: task
name: "x"
---

## Completion Criteria

Scenario: 绑定
  Test:
    过滤: y
  When a
  Then b
"#,
        );
        assert!(msg.contains("keywords must be English"), "{msg}");
        assert!(msg.contains("Filter:"), "{msg}");
    }

    #[test]
    fn test_parser_keeps_chinese_free_text() {
        let input = r#"spec: task
name: "退款功能"
---

## Intent

为支付网关添加退款功能。

## Acceptance Criteria

Scenario: 全额退款
  Given 存在一笔金额为 "100.00" 元的已完成交易
  When 用户发起全额退款
  Then 退款状态变为 "processing"
"#;
        let doc = parse_spec_from_str(input).unwrap();
        match &doc.sections[1] {
            Section::AcceptanceCriteria { scenarios, .. } => {
                assert_eq!(scenarios.len(), 1);
                assert_eq!(scenarios[0].name, "全额退款");
                assert_eq!(scenarios[0].steps.len(), 3);
                assert_eq!(
                    scenarios[0].steps[0].text,
                    "存在一笔金额为 \"100.00\" 元的已完成交易"
                );
            }
            other => panic!("expected AcceptanceCriteria, got {other:?}"),
        }
    }
}
