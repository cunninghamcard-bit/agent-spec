//! Black-box E2E for `finish`: graduation removes consumables only after the
//! contract fully verifies, and never deletes anything on failure.

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

fn goal_spec(report_body: &str) -> String {
    format!(
        r#"spec: task
name: "Registration status codes"
test_command: printf '%s' '{report_body}' > report.xml
test_report: report.xml
---

## Intent

Return deterministic status codes for registration, verified through the
project's own test suite.

## Decisions

- New user registration returns status `201`
- Duplicate email returns status `409`

## Boundaries

### Allowed Changes
- src/**

## Completion Criteria

Scenario: New user gets 201
  Test: new user gets 201
  Given no user with the email exists
  When the client registers the email
  Then the returned status is 201

Scenario: Duplicate email gets 409
  Test: duplicate email gets 409
  Given a user with the email exists
  When the client registers the email again
  Then the returned status is 409
"#
    )
}

const PASSING_REPORT: &str = r#"<testsuites><testsuite><testcase classname="t" name="new user gets 201"/><testcase classname="t" name="duplicate email gets 409"/></testsuite></testsuites>"#;
const FAILING_REPORT: &str = r#"<testsuites><testsuite><testcase classname="t" name="new user gets 201"/><testcase classname="t" name="duplicate email gets 409"><failure message="expected 409 got 200"/></testcase></testsuite></testsuites>"#;

fn write_goal_package(dir: &Path, report_body: &str) -> PathBuf {
    let goal = dir.join("docs/features/registration");
    fs::create_dir_all(&goal).unwrap();
    fs::write(goal.join("spec.md"), goal_spec(report_body)).unwrap();
    fs::write(goal.join("plan.md"), "# Plan\n").unwrap();
    fs::write(goal.join("tasks.md"), "# Tasks\n").unwrap();
    goal
}

#[test]
fn test_cli_finish_removes_plan_and_tasks_e2e() {
    let dir = temp_dir("finish-green");
    let goal = write_goal_package(&dir, PASSING_REPORT);

    let output = run(
        &dir,
        &[
            "finish",
            "docs/features/registration/spec.md",
            "--code",
            ".",
        ],
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "{stdout} {:?}", output);
    assert!(stdout.contains("removed"), "{stdout}");
    assert!(!goal.join("plan.md").exists());
    assert!(!goal.join("tasks.md").exists());
    assert!(goal.join("spec.md").exists());
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_finish_removes_research_consumables_e2e() {
    let dir = temp_dir("finish-research");
    let goal = write_goal_package(&dir, PASSING_REPORT);
    fs::write(goal.join("research.md"), "# R\n").unwrap();
    fs::create_dir_all(goal.join("learning-records")).unwrap();
    fs::write(goal.join("learning-records/0001-trigger.md"), "# LR\n").unwrap();

    let output = run(
        &dir,
        &[
            "finish",
            "docs/features/registration/spec.md",
            "--code",
            ".",
        ],
    );

    assert!(output.status.success(), "{:?}", output);
    assert!(!goal.join("research.md").exists(), "research.md must go");
    assert!(
        !goal.join("learning-records").exists(),
        "learning-records must go"
    );
    assert!(goal.join("spec.md").exists());
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_finish_refuses_on_failing_contract_e2e() {
    let dir = temp_dir("finish-red");
    let goal = write_goal_package(&dir, FAILING_REPORT);

    let output = run(
        &dir,
        &[
            "finish",
            "docs/features/registration/spec.md",
            "--code",
            ".",
        ],
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "finish must fail: {stderr}");
    assert!(stderr.contains("finish aborted"), "{stderr}");
    assert!(stderr.contains("failed"), "{stderr}");
    assert!(goal.join("plan.md").exists(), "plan.md must survive");
    assert!(goal.join("tasks.md").exists(), "tasks.md must survive");
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_finish_retire_removes_goal_directory_e2e() {
    let dir = temp_dir("finish-retire");
    let goal = write_goal_package(&dir, PASSING_REPORT);

    let output = run(
        &dir,
        &[
            "finish",
            "docs/features/registration/spec.md",
            "--code",
            ".",
            "--retire",
        ],
    );

    assert!(output.status.success(), "{:?}", output);
    assert!(!goal.exists(), "goal directory must be removed");
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_finish_bare_spec_reports_nothing_to_clean_e2e() {
    let dir = temp_dir("finish-bare");
    fs::create_dir_all(dir.join("specs")).unwrap();
    fs::write(
        dir.join("specs/registration.spec.md"),
        goal_spec(PASSING_REPORT),
    )
    .unwrap();

    let output = run(
        &dir,
        &["finish", "specs/registration.spec.md", "--code", "."],
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "{stdout} {:?}", output);
    assert!(
        stdout.contains("no goal artifacts needed cleanup"),
        "{stdout}"
    );
    let _ = fs::remove_dir_all(dir);
}
