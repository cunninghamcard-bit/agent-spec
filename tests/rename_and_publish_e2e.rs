//! Black-box E2E for the rename: the binary, install-hooks, and generated
//! integration files present under the new docwright identity, with no
//! leftover agent-spec references.

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

fn git(dir: &Path, args: &[&str]) {
    let output = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .expect("git should start");
    assert!(output.status.success(), "git {args:?}: {output:?}");
}

#[test]
fn test_cli_binary_reports_docwright_identity_e2e() {
    let dir = temp_dir("identity");
    let version = run(&dir, &["--version"]);
    assert!(version.status.success(), "{:?}", version);
    let version_out = String::from_utf8_lossy(&version.stdout);
    assert!(version_out.starts_with("docwright"), "{version_out}");

    let help = run(&dir, &["--help"]);
    assert!(help.status.success(), "{:?}", help);
    let help_out = String::from_utf8_lossy(&help.stdout);
    assert!(help_out.contains("docwright <COMMAND>"), "{help_out}");
    assert!(
        !help_out.contains("agent-spec"),
        "help output must not mention the old identity: {help_out}"
    );
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_install_hooks_uses_docwright_identity_e2e() {
    let dir = temp_dir("hooks-identity");
    git(&dir, &["init", "-q"]);

    let output = run(&dir, &["install-hooks"]);
    assert!(output.status.success(), "{:?}", output);
    let hook = fs::read_to_string(dir.join(".git/hooks/pre-commit")).unwrap();
    assert!(hook.contains("docwright guard"), "hook: {hook}");
    assert!(!hook.contains("agent-spec"), "hook: {hook}");
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_integrate_generated_files_carry_new_identity_e2e() {
    let dir = temp_dir("gen-integrations-identity");
    let output = run(&dir, &["gen-integrations", "--target", "all", "--out", "."]);
    assert!(output.status.success(), "{:?}", output);

    for name in ["AGENTS.md", ".cursorrules", "docwright-tool-first.md"] {
        let content = fs::read_to_string(dir.join(name)).unwrap();
        assert!(
            content.contains("docwright"),
            "{name} must mention docwright: {content}"
        );
        assert!(
            !content.contains("agent-spec"),
            "{name} must not mention the old identity: {content}"
        );
    }
    let _ = fs::remove_dir_all(dir);
}
