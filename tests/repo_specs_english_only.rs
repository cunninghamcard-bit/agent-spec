//! Regression guard: the repo's own spec corpus must use English structural
//! keywords only. Descriptive prose may be any language.

use std::fs;
use std::path::{Path, PathBuf};

const CJK_COLON_KEYWORDS: &[&str] = &[
    "场景", "示例", "例子", "规则", "测试", "过滤", "层级", "替身", "命中", "审核", "模式", "标签",
    "前置", "包",
];

const CJK_SECTION_HEADERS: &[&str] = &[
    "已定决策",
    "验收标准",
    "完成条件",
    "排除范围",
    "允许修改",
    "禁止做",
    "必须做",
    "待澄清",
    "意图",
    "约束",
    "决策",
    "边界",
];

const CJK_STEP_KEYWORDS: &[&str] = &["假设", "那么", "并且", "但是", "当"];

fn spec_files(dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.ends_with(".spec.md") || n.ends_with(".spec"))
        })
        .collect()
}

fn violations_in(path: &Path) -> Vec<String> {
    let Ok(content) = fs::read_to_string(path) else {
        return vec![format!("{}: unreadable", path.display())];
    };

    let mut violations = Vec::new();
    let mut in_criteria = false;

    for (idx, line) in content.lines().enumerate() {
        let line_no = idx + 1;
        let stripped = line.trim().trim_start_matches('#').trim();

        if line.trim_start().starts_with('#') {
            if stripped.starts_with("Completion Criteria")
                || stripped.starts_with("Acceptance Criteria")
            {
                in_criteria = true;
            } else if line.trim_start().starts_with("## ") {
                in_criteria = false;
            }
            for &cjk in CJK_SECTION_HEADERS {
                if stripped == cjk {
                    violations.push(format!(
                        "{}:{line_no}: CJK section header '{cjk}'",
                        path.display()
                    ));
                }
            }
        }

        for &cjk in CJK_COLON_KEYWORDS {
            for colon in [":", "："] {
                let prefix = format!("{cjk}{colon}");
                if stripped.starts_with(&prefix) {
                    violations.push(format!(
                        "{}:{line_no}: CJK keyword '{prefix}'",
                        path.display()
                    ));
                }
            }
        }

        if in_criteria {
            let trimmed = line.trim();
            for &cjk in CJK_STEP_KEYWORDS {
                if trimmed.starts_with(cjk) {
                    violations.push(format!(
                        "{}:{line_no}: CJK step keyword '{cjk}'",
                        path.display()
                    ));
                }
            }
        }
    }

    violations
}

#[test]
fn test_repo_specs_use_english_structural_keywords() {
    let mut all = Vec::new();
    for dir in ["specs", "specs/roadmap", "examples"] {
        for file in spec_files(Path::new(dir)) {
            all.extend(violations_in(&file));
        }
    }

    assert!(
        all.is_empty(),
        "CJK structural keywords found in spec corpus:\n{}",
        all.join("\n")
    );
}
