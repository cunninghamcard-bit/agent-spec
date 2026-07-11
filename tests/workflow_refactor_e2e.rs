//! Black-box E2E for the workflow refactor: staged artifact birth, the
//! flow-grouped help, the docs/ single household (promote, install-hooks,
//! inheritance, guard), and the one-active-goal lifecycle warning.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_dir(label: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "agent-spec-e2e-{label}-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir_all(&dir).expect("temporary directory should be created");
    dir
}

fn run(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_agent-spec"))
        .current_dir(dir)
        .args(args)
        .output()
        .expect("agent-spec process should start")
}

fn git(dir: &Path, args: &[&str]) {
    let output = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .expect("git should start");
    assert!(output.status.success(), "git {args:?}: {output:?}");
}

const PASSING_REPORT: &str = r#"<testsuites><testsuite><testcase classname="t" name="status is ok"/></testsuite></testsuites>"#;

/// Minimal report-mode goal contract that verifies green anywhere.
fn passing_goal_spec(name: &str) -> String {
    format!(
        r#"spec: task
name: "{name}"
test_command: printf '%s' '{PASSING_REPORT}' > report.xml
test_report: report.xml
---

## Intent

Return a deterministic status.

## Boundaries

### Allowed Changes
- src/**

## Completion Criteria

Rule: r-status — status is deterministic

Scenario: Status ok
  Test: status is ok
  Given the service is up
  When the client asks for status
  Then the status is ok
"#
    )
}

#[test]
fn test_cli_plan_out_births_plan_and_tasks_e2e() {
    let dir = temp_dir("plan-birth");
    let goal = dir.join("docs/features/onboarding");
    fs::create_dir_all(&goal).unwrap();
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(goal.join("spec.md"), passing_goal_spec("Onboarding")).unwrap();

    let output = run(
        &dir,
        &[
            "plan",
            "docs/features/onboarding/spec.md",
            "--code",
            ".",
            "--out",
            "docs/features/onboarding/plan.md",
        ],
    );
    assert!(output.status.success(), "{:?}", output);
    assert!(goal.join("plan.md").exists());
    let tasks = fs::read_to_string(goal.join("tasks.md")).unwrap();
    assert!(tasks.contains("## Quality Gates"), "tasks: {tasks}");

    // Hand-curated tasks survive a plan refresh.
    fs::write(goal.join("tasks.md"), "- [x] my curated task\n").unwrap();
    let second = run(
        &dir,
        &[
            "plan",
            "docs/features/onboarding/spec.md",
            "--code",
            ".",
            "--out",
            "docs/features/onboarding/plan.md",
        ],
    );
    assert!(second.status.success(), "{:?}", second);
    assert_eq!(
        fs::read_to_string(goal.join("tasks.md")).unwrap(),
        "- [x] my curated task\n"
    );
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_help_groups_commands_by_flow_e2e() {
    let dir = temp_dir("help-flows");
    let output = run(&dir, &["--help"]);
    assert!(output.status.success(), "{:?}", output);
    let help = String::from_utf8_lossy(&output.stdout);

    for heading in [
        "Adoption (once per repo):",
        "Goal lifecycle (per goal):",
        "Review (per review):",
        "Library governance (periodic):",
        "Probe & AI (on trigger):",
    ] {
        assert!(help.contains(heading), "missing heading {heading}: {help}");
    }
    // Every subcommand appears exactly once as a station.
    let stations: Vec<&str> = help
        .lines()
        .filter_map(|l| {
            let stripped = l.strip_prefix("    ")?;
            if stripped.starts_with(' ') {
                return None;
            }
            stripped.split_whitespace().next()
        })
        .collect();
    for cmd in [
        "parse",
        "lint",
        "verify",
        "matrix",
        "audit",
        "discover",
        "check-structure",
        "gen-integrations",
        "promote",
        "init",
        "lifecycle",
        "brief",
        "contract",
        "integrate",
        "research",
        "finish",
        "guard",
        "explain",
        "stamp",
        "checkpoint",
        "measure-determinism",
        "install-hooks",
        "resolve-ai",
        "plan",
        "graph",
    ] {
        assert_eq!(
            stations.iter().filter(|s| **s == cmd).count(),
            1,
            "{cmd} must have exactly one station: {stations:?}"
        );
    }
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_promote_targets_docs_capabilities_e2e() {
    let dir = temp_dir("promote-docs");
    let goal = dir.join("docs/features/status");
    fs::create_dir_all(&goal).unwrap();
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(goal.join("spec.md"), passing_goal_spec("Status")).unwrap();

    let output = run(
        &dir,
        &[
            "promote",
            "docs/features/status/spec.md",
            "--rule",
            "r-status",
            "--to",
            "core",
            "--code",
            ".",
        ],
    );
    assert!(output.status.success(), "{:?}", output);
    let cap = fs::read_to_string(dir.join("docs/capabilities/core.spec.md")).unwrap();
    assert!(cap.contains("r-status"), "capability: {cap}");
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_install_hooks_targets_docs_home_e2e() {
    let dir = temp_dir("hooks-docs");
    git(&dir, &["init", "-q"]);

    let output = run(&dir, &["install-hooks"]);
    assert!(output.status.success(), "{:?}", output);
    let hook = fs::read_to_string(dir.join(".git/hooks/pre-commit")).unwrap();
    assert!(hook.contains("--spec-dir docs"), "hook: {hook}");
    assert!(!hook.contains("--spec-dir specs"), "hook: {hook}");
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_goal_spec_inherits_relocated_project_constitution_e2e() {
    let dir = temp_dir("inherit-docs");
    fs::create_dir_all(dir.join("docs/features/onboarding")).unwrap();
    fs::write(
        dir.join("docs/project.spec.md"),
        r#"spec: project
name: "Project Constitution"
---

## Constraints

### Must
- Every public behavior carries a regression test
"#,
    )
    .unwrap();
    let goal_spec = passing_goal_spec("Onboarding").replace(
        "name: \"Onboarding\"",
        "name: \"Onboarding\"\ninherits: project",
    );
    fs::write(dir.join("docs/features/onboarding/spec.md"), goal_spec).unwrap();

    let output = run(&dir, &["contract", "docs/features/onboarding/spec.md"]);
    assert!(output.status.success(), "{:?}", output);
    let contract = String::from_utf8_lossy(&output.stdout);
    assert!(
        contract.contains("Every public behavior carries a regression test"),
        "inherited constraint missing: {contract}"
    );
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_guard_collects_docs_contracts_e2e() {
    let dir = temp_dir("guard-docs");
    git(&dir, &["init", "-q"]);
    fs::create_dir_all(dir.join("src")).unwrap();
    for goal in ["alpha", "beta"] {
        let goal_dir = dir.join("docs/features").join(goal);
        fs::create_dir_all(&goal_dir).unwrap();
        fs::write(goal_dir.join("spec.md"), passing_goal_spec(goal)).unwrap();
    }
    assert!(!dir.join("specs").exists());

    let output = run(
        &dir,
        &["guard", "--code", ".", "--change-scope", "worktree"],
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stdout} {stderr}");
    assert!(
        stderr.contains("2 spec(s) passed") || stdout.contains("2 spec(s) passed"),
        "expected both docs contracts verified: {stdout} {stderr}"
    );
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_lifecycle_warns_on_multiple_dirty_goals_e2e() {
    let dir = temp_dir("dirty-goals");
    git(&dir, &["init", "-q"]);
    fs::create_dir_all(dir.join("src")).unwrap();
    for goal in ["alpha", "beta"] {
        let goal_dir = dir.join("docs/features").join(goal);
        fs::create_dir_all(&goal_dir).unwrap();
        fs::write(goal_dir.join("spec.md"), passing_goal_spec(goal)).unwrap();
    }
    git(&dir, &["add", "."]);

    let output = run(
        &dir,
        &["lifecycle", "docs/features/alpha/spec.md", "--code", "."],
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("one active goal per worktree"),
        "warning missing: {stderr}"
    );
    assert!(
        output.status.success(),
        "warning must not block the run: {output:?}"
    );
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_guard_skips_capability_specs_e2e() {
    let dir = temp_dir("guard-capability");
    git(&dir, &["init", "-q"]);
    fs::create_dir_all(dir.join("src")).unwrap();
    let goal_dir = dir.join("docs/features/alpha");
    fs::create_dir_all(&goal_dir).unwrap();
    fs::write(goal_dir.join("spec.md"), passing_goal_spec("alpha")).unwrap();
    // A bare promoted capability: rules without scenarios. Guard's task
    // gates (quality score, verification) must not apply to it.
    fs::create_dir_all(dir.join("docs/capabilities")).unwrap();
    fs::write(
        dir.join("docs/capabilities/core.spec.md"),
        r#"spec: capability
name: "core"
tags: [capability]
---

## Intent

Long-lived behavior truth library for the core capability.

## Completion Criteria

### Rule: r-status — status is deterministic
"#,
    )
    .unwrap();

    let output = run(
        &dir,
        &["guard", "--code", ".", "--change-scope", "worktree"],
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stdout} {stderr}");
    let _ = fs::remove_dir_all(dir);
}
