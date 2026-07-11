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

#[test]
fn test_cli_init_feature_creates_spec_only_e2e() {
    let dir = temp_dir("feature");
    let output = run(
        &dir,
        &["init", "--kind", "feature", "--name", "Plugins Hub"],
    );
    assert!(output.status.success(), "{:?}", output);

    // Staged birth: init creates the contract only; plan.md and tasks.md
    // are born later by `plan --out`.
    let goal = dir.join("docs/features/plugins-hub");
    let spec = fs::read_to_string(goal.join("spec.md")).unwrap();
    assert!(!goal.join("plan.md").exists());
    assert!(!goal.join("tasks.md").exists());
    assert!(spec.contains("spec: task"));
    assert!(spec.contains("tags: [feature, sdd]"));
    assert!(spec.contains("```plantuml"));

    let parse = run(&dir, &["parse", goal.join("spec.md").to_str().unwrap()]);
    assert!(parse.status.success(), "{:?}", parse);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("next: agent-spec lint"));
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_init_architecture_creates_sdd_package_e2e() {
    let dir = temp_dir("architecture");
    let output = run(
        &dir,
        &[
            "init",
            "--kind",
            "architecture",
            "--name",
            "Agent Runtime Split",
        ],
    );
    assert!(output.status.success(), "{:?}", output);
    let goal = dir.join("docs/architecture/agent-runtime-split");
    assert!(goal.join("spec.md").exists());
    assert!(!goal.join("plan.md").exists());
    assert!(!goal.join("tasks.md").exists());
    assert!(
        fs::read_to_string(goal.join("spec.md"))
            .unwrap()
            .contains("tags: [architecture, sdd]")
    );
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_init_issue_creates_single_spec_e2e() {
    let dir = temp_dir("issue");
    let output = run(
        &dir,
        &[
            "init",
            "--kind",
            "issue",
            "--name",
            "Session Restore Jitter",
        ],
    );
    assert!(output.status.success(), "{:?}", output);
    let goal = dir.join("docs/issues/session-restore-jitter");
    let spec = fs::read_to_string(goal.join("spec.md")).unwrap();
    assert!(!goal.join("plan.md").exists());
    assert!(!goal.join("tasks.md").exists());
    for section in [
        "### Impact",
        "### Suspected Root Cause",
        "### Fix Plan",
        "### Validation",
        "## Open Questions",
    ] {
        assert!(spec.contains(section), "missing {section}");
    }
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_init_sdd_package_refuses_partial_overwrite_e2e() {
    let dir = temp_dir("overwrite");
    let goal = dir.join("docs/features/plugins-hub");
    fs::create_dir_all(&goal).unwrap();
    fs::write(goal.join("plan.md"), "keep me\n").unwrap();

    let output = run(
        &dir,
        &["init", "--kind", "feature", "--name", "Plugins Hub"],
    );
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("refusing to update existing goal directory")
    );
    assert_eq!(
        fs::read_to_string(goal.join("plan.md")).unwrap(),
        "keep me\n"
    );
    assert!(!goal.join("spec.md").exists());
    assert!(!goal.join("tasks.md").exists());
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_init_rejects_invalid_sdd_kind_e2e() {
    let dir = temp_dir("invalid-kind");
    let output = run(&dir, &["init", "--kind", "product", "--name", "Bad Kind"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("feature"));
    assert!(stderr.contains("issue"));
    assert!(stderr.contains("architecture"));
    assert!(!dir.join("docs").exists());
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_init_requires_kind_and_name_e2e() {
    let dir = temp_dir("required-args");
    let missing_kind = run(&dir, &["init", "--name", "missing-kind"]);
    let missing_name = run(&dir, &["init", "--kind", "feature"]);

    assert!(!missing_kind.status.success());
    assert!(String::from_utf8_lossy(&missing_kind.stderr).contains("--kind"));
    assert!(!missing_name.status.success());
    assert!(String::from_utf8_lossy(&missing_name.stderr).contains("--name"));
    assert!(!dir.join("missing-kind.spec.md").exists());
    assert!(!dir.join("docs").exists());
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_agent_spec_sdd_skill_routes_cli_workflow() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let sdd = fs::read_to_string(root.join("skills/agent-spec-sdd/SKILL.md")).unwrap();
    let claude_sdd =
        fs::read_to_string(root.join(".claude/skills/agent-spec-sdd/SKILL.md")).unwrap();
    let tool_first =
        fs::read_to_string(root.join("skills/agent-spec-tool-first/SKILL.md")).unwrap();
    let authoring = fs::read_to_string(root.join("skills/agent-spec-authoring/SKILL.md")).unwrap();

    assert!(sdd.contains("deepchat-sdd"));
    assert_eq!(sdd, claude_sdd);
    assert!(sdd.contains("agent-spec init --kind"));
    assert!(sdd.contains("`spec.md` is the authoritative"));
    assert!(sdd.contains("E2E evidence"));
    assert!(sdd.contains("staged birth"));
    assert!(sdd.contains("One active goal per worktree"));
    assert!(tool_first.contains("exact CLI execution, verdict interpretation, and retry"));
    assert!(authoring.contains("public CLI or product E2E tests"));

    let dir = temp_dir("integration-guidance");
    let output = run(
        &dir,
        &[
            "gen-integrations",
            "--target",
            "agents",
            "--out",
            dir.to_str().unwrap(),
        ],
    );
    assert!(output.status.success(), "{:?}", output);
    let generated = fs::read_to_string(dir.join("AGENTS.md")).unwrap();
    assert!(generated.contains("agent-spec init --kind"));
    assert!(!generated.contains("Presenter"));
    assert!(!generated.contains("GitHub-label"));
    let _ = fs::remove_dir_all(dir);
}
