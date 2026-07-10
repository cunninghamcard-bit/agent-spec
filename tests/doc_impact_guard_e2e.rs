//! Black-box E2E for the Doc Impact Guard: warnings surface on stderr and
//! never change guard's exit status.

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

const PASSING_SPEC: &str = r#"spec: task
name: "Registration status codes"
test_command: printf '%s' '<testsuites><testsuite><testcase classname="t" name="new user gets 201"/><testcase classname="t" name="duplicate email gets 409"/></testsuite></testsuites>' > report.xml
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
"#;

#[test]
fn test_cli_guard_doc_impact_warns_without_failing_e2e() {
    let dir = temp_dir("doc-impact");
    fs::create_dir_all(dir.join("specs")).unwrap();
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(dir.join("specs/registration.spec.md"), PASSING_SPEC).unwrap();
    fs::write(
        dir.join("ARCHITECTURE.md"),
        "# Architecture\n\n<!-- agent-spec:governs: src/** -->\n\nMaintained doc.\n",
    )
    .unwrap();
    fs::write(dir.join("src/lib.rs"), "pub fn demo() {}\n").unwrap();

    let output = run(
        &dir,
        &[
            "guard",
            "--spec-dir",
            "specs",
            "--code",
            ".",
            "--change",
            "src/lib.rs",
        ],
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("documentation impact unresolved"),
        "stderr: {stderr}"
    );
    assert!(stderr.contains("ARCHITECTURE.md"), "stderr: {stderr}");
    assert!(stderr.contains("src/lib.rs"), "stderr: {stderr}");
    assert!(
        output.status.success(),
        "doc-impact warnings must not gate guard; stderr: {stderr}"
    );

    // Updating the document in the same change set silences the warning.
    let quiet = run(
        &dir,
        &[
            "guard",
            "--spec-dir",
            "specs",
            "--code",
            ".",
            "--change",
            "src/lib.rs",
            "--change",
            "ARCHITECTURE.md",
        ],
    );
    let quiet_stderr = String::from_utf8_lossy(&quiet.stderr);
    assert!(
        !quiet_stderr.contains("documentation impact unresolved"),
        "stderr: {quiet_stderr}"
    );
    assert!(quiet.status.success(), "stderr: {quiet_stderr}");

    let _ = fs::remove_dir_all(dir);
}
