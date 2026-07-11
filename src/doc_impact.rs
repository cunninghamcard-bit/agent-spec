//! Doc Impact Guard: a mechanical freshness signal for maintained docs.
//!
//! A Markdown document declares which code paths it governs with an
//! invisible marker line:
//!
//! ```markdown
//! <!-- docwright:governs: src/main.rs, skills/** -->
//! ```
//!
//! During `guard`, changes under governed paths without a matching update
//! to the document produce a `documentation impact unresolved` warning.
//! Warning-level only in this phase: guard's exit status is never affected.

use std::fs;
use std::path::{Path, PathBuf};

use crate::spec_verify::path_matches_pattern;

const GOVERNS_MARKER: &str = "<!-- docwright:governs:";

/// A maintained document and the code-path globs it governs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernedDoc {
    /// Path relative to the code root, normalized with `/` separators.
    pub path: String,
    pub globs: Vec<String>,
}

/// Extract governs globs from all marker lines in a document. Markers inside
/// fenced code blocks are documentation examples, not declarations.
pub fn parse_governs_markers(content: &str) -> Vec<String> {
    let mut globs = Vec::new();
    let mut in_fence = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        let Some(rest) = trimmed.strip_prefix(GOVERNS_MARKER) else {
            continue;
        };
        let Some(body) = rest.strip_suffix("-->") else {
            continue;
        };
        for glob in body.split(',') {
            let glob = glob.trim().trim_matches('`');
            if !glob.is_empty() {
                globs.push(glob.to_string());
            }
        }
    }
    globs
}

/// Scan `*.md` at the code root and `docs/**/*.md` for governed documents.
pub fn collect_governed_docs(code_root: &Path) -> Vec<GovernedDoc> {
    let mut files: Vec<PathBuf> = Vec::new();

    if let Ok(entries) = fs::read_dir(code_root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|e| e == "md") {
                files.push(path);
            }
        }
    }
    collect_md_recursive(&code_root.join("docs"), &mut files);

    let mut docs = Vec::new();
    for file in files {
        let Ok(content) = fs::read_to_string(&file) else {
            continue;
        };
        let globs = parse_governs_markers(&content);
        if globs.is_empty() {
            continue;
        }
        let rel = file
            .strip_prefix(code_root)
            .unwrap_or(&file)
            .to_string_lossy()
            .replace('\\', "/");
        docs.push(GovernedDoc { path: rel, globs });
    }
    docs.sort_by(|a, b| a.path.cmp(&b.path));
    docs
}

fn collect_md_recursive(dir: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with('.'))
            {
                continue;
            }
            collect_md_recursive(&path, files);
        } else if path.extension().is_some_and(|e| e == "md") {
            files.push(path);
        }
    }
}

/// The join: one warning per governed document whose paths changed while the
/// document itself did not. `changes` are code-root-relative paths.
pub fn doc_impact_warnings(docs: &[GovernedDoc], changes: &[String]) -> Vec<String> {
    let mut warnings = Vec::new();

    for doc in docs {
        let doc_updated = changes.iter().any(|change| change == &doc.path);
        if doc_updated {
            continue;
        }
        let hit = doc.globs.iter().find_map(|glob| {
            changes
                .iter()
                .find(|change| path_matches_pattern(glob, change))
        });
        if let Some(changed) = hit {
            warnings.push(format!(
                "documentation impact unresolved: {changed} changed; governed by {} (not updated in this change set)",
                doc.path
            ));
        }
    }

    warnings
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn doc(path: &str, globs: &[&str]) -> GovernedDoc {
        GovernedDoc {
            path: path.into(),
            globs: globs.iter().map(|g| g.to_string()).collect(),
        }
    }

    #[test]
    fn test_doc_impact_parses_governs_marker() {
        let content = "\
# Architecture

<!-- docwright:governs: src/agent/**, src/shared/contracts/** -->

Some prose.

<!-- docwright:governs: `src/main.rs` -->
";
        assert_eq!(
            parse_governs_markers(content),
            vec!["src/agent/**", "src/shared/contracts/**", "src/main.rs"]
        );

        assert!(parse_governs_markers("# No markers here\n\nJust prose.\n").is_empty());

        // Markers inside fenced code blocks are documentation examples.
        let fenced = "\
# Doc

```markdown
<!-- docwright:governs: src/example/** -->
```
";
        assert!(parse_governs_markers(fenced).is_empty());
    }

    #[test]
    fn test_doc_impact_warns_when_governed_path_changes() {
        let docs = vec![doc("docs/architecture.md", &["src/**"])];
        let changes = vec!["src/lib.rs".to_string()];

        let warnings = doc_impact_warnings(&docs, &changes);

        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("src/lib.rs"), "{}", warnings[0]);
        assert!(
            warnings[0].contains("docs/architecture.md"),
            "{}",
            warnings[0]
        );
    }

    #[test]
    fn test_doc_impact_quiet_when_doc_updated_together() {
        let docs = vec![doc("docs/architecture.md", &["src/**"])];
        let changes = vec!["src/lib.rs".to_string(), "docs/architecture.md".to_string()];

        assert!(doc_impact_warnings(&docs, &changes).is_empty());
    }

    #[test]
    fn test_doc_impact_quiet_without_overlap() {
        let docs = vec![doc("docs/architecture.md", &["src/agent/**"])];
        let changes = vec!["docs/notes.md".to_string()];

        assert!(doc_impact_warnings(&docs, &changes).is_empty());
    }

    #[test]
    fn test_doc_impact_discovery_scans_root_and_docs() {
        let dir = std::env::temp_dir().join(format!("docwright-doc-impact-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("docs/deep")).unwrap();
        fs::write(
            dir.join("README.md"),
            "<!-- docwright:governs: src/main.rs -->\n",
        )
        .unwrap();
        fs::write(
            dir.join("docs/deep/arch.md"),
            "<!-- docwright:governs: src/agent/** -->\n",
        )
        .unwrap();
        fs::write(dir.join("docs/plain.md"), "no markers\n").unwrap();

        let docs = collect_governed_docs(&dir);

        assert_eq!(docs.len(), 2);
        assert_eq!(docs[0].path, "README.md");
        assert_eq!(docs[1].path, "docs/deep/arch.md");
        let _ = fs::remove_dir_all(&dir);
    }
}
