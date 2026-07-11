//! Black-box E2E for `research`: scaffold on first run, refresh only the
//! managed codebase-state region afterwards.

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
        "docwright-e2e-{label}-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir_all(&dir).expect("temporary directory should be created");
    dir
}

fn run(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_docwright"))
        .current_dir(dir)
        .args(args)
        .output()
        .expect("docwright process should start")
}

const GOAL_SPEC: &str = r#"spec: task
name: "Onboarding"
---

## Intent

Add onboarding.

## Boundaries

### Allowed Changes
- src/**

## Completion Criteria

Scenario: ok
  Test: test_ok
  Given a thing
  When it runs
  Then it passes
"#;

#[test]
fn test_cli_research_scaffolds_and_refreshes_e2e() {
    let dir = temp_dir("research");
    let goal = dir.join("docs/features/onboarding");
    fs::create_dir_all(&goal).unwrap();
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(goal.join("spec.md"), GOAL_SPEC).unwrap();
    fs::write(dir.join("src/lib.rs"), "//! Demo lib\npub fn demo() {}\n").unwrap();

    // First run scaffolds.
    let first = run(
        &dir,
        &[
            "research",
            "docs/features/onboarding/spec.md",
            "--code",
            ".",
        ],
    );
    assert!(first.status.success(), "{:?}", first);
    let research = fs::read_to_string(goal.join("research.md")).unwrap();
    for section in [
        "## Unknowns",
        "## Industry Norms & Prior Art",
        "## Current Codebase State",
        "## Findings",
    ] {
        assert!(research.contains(section), "missing {section}: {research}");
    }
    assert!(research.contains("src/lib.rs"), "scan missing: {research}");

    // Hand-written prose outside the region must survive a refresh.
    let edited = research.replace(
        "- [UNFILLED: findings from primary sources, one bullet per claim, each with\n  its Source]",
        "- vitest is the norm. Source: local survey.",
    );
    fs::write(goal.join("research.md"), edited).unwrap();
    fs::write(
        dir.join("src/extra.rs"),
        "//! Extra module\npub fn extra() {}\n",
    )
    .unwrap();

    let second = run(
        &dir,
        &[
            "research",
            "docs/features/onboarding/spec.md",
            "--code",
            ".",
        ],
    );
    assert!(second.status.success(), "{:?}", second);
    let refreshed = fs::read_to_string(goal.join("research.md")).unwrap();
    assert!(
        refreshed.contains("- vitest is the norm. Source: local survey."),
        "hand prose lost: {refreshed}"
    );
    assert!(
        refreshed.contains("src/extra.rs"),
        "region not refreshed: {refreshed}"
    );

    let _ = fs::remove_dir_all(dir);
}
