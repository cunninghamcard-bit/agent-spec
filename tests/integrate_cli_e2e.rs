//! Black-box E2E for `integrate`: skills for both agent systems, managed
//! policy blocks in AGENTS.md/CLAUDE.md, and init test-binding prefill.

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

#[test]
fn test_cli_integrate_installs_skills_e2e() {
    let dir = temp_dir("integrate-skills");

    let output = run(&dir, &["integrate"]);
    assert!(output.status.success(), "{:?}", output);

    for root in [".agents/skills", ".claude/skills"] {
        for skill in [
            "docwright-sdd",
            "docwright-tool-first",
            "docwright-authoring",
        ] {
            let skill_md = dir.join(root).join(skill).join("SKILL.md");
            assert!(skill_md.is_file(), "missing {}", skill_md.display());
        }
        assert!(
            dir.join(root)
                .join("docwright-tool-first/references/commands.md")
                .is_file()
        );
    }
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_integrate_installs_research_skill_e2e() {
    let dir = temp_dir("integrate-research-skill");

    let output = run(&dir, &["integrate"]);
    assert!(output.status.success(), "{:?}", output);

    for root in [".agents/skills", ".claude/skills"] {
        let skill_md = dir.join(root).join("docwright-research/SKILL.md");
        assert!(skill_md.is_file(), "missing {}", skill_md.display());
        let content = fs::read_to_string(&skill_md).unwrap();
        assert!(content.contains("primary sources"));
    }
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_integrate_writes_policy_blocks_e2e() {
    let dir = temp_dir("integrate-blocks");

    let output = run(&dir, &["integrate"]);
    assert!(output.status.success(), "{:?}", output);

    for declaration in ["AGENTS.md", "CLAUDE.md"] {
        let content = fs::read_to_string(dir.join(declaration)).unwrap();
        assert!(
            content.contains("<!-- docwright:integration:start -->"),
            "{declaration}: {content}"
        );
        assert!(content.contains("<!-- docwright:integration:end -->"));
        assert!(
            content.contains("docwright-sdd skill owns goal classification"),
            "{declaration} must name the workflow owner"
        );
    }
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_integrate_preserves_and_refreshes_e2e() {
    let dir = temp_dir("integrate-idempotent");
    fs::write(
        dir.join("AGENTS.md"),
        "# My Project\n\ncustom team rules above\n",
    )
    .unwrap();

    let first = run(&dir, &["integrate"]);
    assert!(first.status.success(), "{:?}", first);
    fs::OpenOptions::new()
        .append(true)
        .open(dir.join("AGENTS.md"))
        .and_then(|mut f| {
            use std::io::Write;
            f.write_all(b"\ncustom trailing note\n")
        })
        .unwrap();

    let second = run(&dir, &["integrate"]);
    assert!(second.status.success(), "{:?}", second);

    let content = fs::read_to_string(dir.join("AGENTS.md")).unwrap();
    assert!(content.starts_with("# My Project"), "{content}");
    assert!(content.contains("custom team rules above"));
    assert!(content.contains("custom trailing note"));
    assert_eq!(
        content
            .matches("<!-- docwright:integration:start -->")
            .count(),
        1,
        "exactly one managed block: {content}"
    );
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_init_prefills_vitest_binding_for_node_e2e() {
    let dir = temp_dir("init-node-binding");
    fs::write(dir.join("package.json"), "{\"name\":\"app\"}\n").unwrap();

    let output = run(&dir, &["init", "--kind", "feature", "--name", "Onboarding"]);
    assert!(output.status.success(), "{:?}", output);

    let spec = fs::read_to_string(dir.join("docs/features/onboarding/spec.md")).unwrap();
    assert!(
        spec.contains("test_command: pnpm vitest run"),
        "spec: {spec}"
    );
    assert!(spec.contains("test_report: .docwright/report.xml"));
    // Binding sits in the frontmatter, before the separator.
    let frontmatter = spec.split("---").next().unwrap();
    assert!(frontmatter.contains("test_command"));
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn test_cli_init_omits_binding_for_cargo_e2e() {
    let dir = temp_dir("init-cargo-binding");
    fs::write(dir.join("Cargo.toml"), "[package]\nname = \"app\"\n").unwrap();

    let output = run(&dir, &["init", "--kind", "feature", "--name", "Onboarding"]);
    assert!(output.status.success(), "{:?}", output);

    let spec = fs::read_to_string(dir.join("docs/features/onboarding/spec.md")).unwrap();
    assert!(!spec.contains("test_command"), "spec: {spec}");
    let _ = fs::remove_dir_all(dir);
}
