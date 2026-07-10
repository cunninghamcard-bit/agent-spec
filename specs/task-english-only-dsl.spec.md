spec: task
name: "English-only structural keywords (breaking)"
inherits: project
tags: [parser, dsl, breaking, english-only]
---

## Intent

Adopt the specwright fork's judgment: structural DSL keywords are English-only.
The parser hard-rejects CJK structural keywords with an actionable error naming
the English replacement, so a mixed-dialect spec fails fast instead of being
half-parsed. Descriptive free text (scenario names, step prose, quoted params,
frontmatter values) may remain in any language. The repo's own spec corpus is
migrated to English keywords in the same change.

## Decisions

- `keywords.rs` stops matching CJK structural keywords as valid syntax; a detection table maps every previously supported CJK keyword to its English replacement
- Parse failure message format: `keywords must be English; '<cjk>' is not recognized — use '<english>'`
- Rejection covers: section headers, boundary subsection headers, `场景:`/`示例:`/`例子:`, `规则:`, `测试:`, selector fields (`包:`/`过滤:`/`层级:`/`替身:`/`命中:`), step keywords (`假设`/`当`/`那么`/`并且`/`但是`), `审核:`, `模式:`, `标签:`, `前置:`
- Step-keyword rejection applies only inside scenario bodies, where those tokens were previously structural
- Free text stays language-neutral: scenario titles, step prose after an English keyword, quoted parameters, and frontmatter `name:` may contain any language
- All specs under `specs/` and `specs/roadmap/` are migrated to English structural keywords with prose preserved
- A regression test walks `specs/**/*.spec.md` and fails if any CJK structural keyword reappears
- Version bumps to 0.4.0 to signal the breaking change

## Boundaries

### Allowed Changes
- src/**
- specs/**
- examples/**
- skills/**
- .claude/skills/**
- tests/**
- README.md
- CHANGELOG.md
- Cargo.toml
- Cargo.lock

### Forbidden
- Do not change verdict semantics or verification behavior
- Do not reject CJK content in free text positions

## Out of Scope

- Translating Chinese prose inside existing specs (only keywords migrate)
- docs/index.html landing page rewrite

## Completion Criteria

Scenario: CJK section header is rejected with the English replacement named
  Test:
    Package: agent-spec
    Filter: test_parser_rejects_cjk_section_header_with_suggestion
  Given a spec whose section header is "## 意图"
  When the spec is parsed
  Then parsing fails
  And the error names 'Intent' as the replacement

Scenario: CJK scenario keyword is rejected
  Test:
    Package: agent-spec
    Filter: test_parser_rejects_cjk_scenario_keyword
  Given a spec containing a "场景:" line in the acceptance section
  When the spec is parsed
  Then parsing fails
  And the error names 'Scenario:' as the replacement

Scenario: CJK step keyword inside a scenario body is rejected
  Test:
    Package: agent-spec
    Filter: test_parser_rejects_cjk_step_keyword
  Given a scenario body containing a line starting with "假设"
  When the spec is parsed
  Then parsing fails
  And the error names 'Given' as the replacement

Scenario: CJK test selector keys are rejected
  Test:
    Package: agent-spec
    Filter: test_parser_rejects_cjk_test_selector_keys
  Given a scenario using "测试:" or a "包:"/"过滤:" block
  When the spec is parsed
  Then parsing fails
  And the error names the English selector key as the replacement

Scenario: English DSL with Chinese free text still parses
  Test:
    Package: agent-spec
    Filter: test_parser_keeps_chinese_free_text
  Given a spec using English keywords whose scenario title and step prose are Chinese
  When the spec is parsed
  Then parsing succeeds
  And the scenario title retains the Chinese text

Scenario: Repo spec corpus uses English structural keywords only
  Test:
    Package: agent-spec
    Filter: test_repo_specs_use_english_structural_keywords
  Given the checked-in specs under "specs/" and "specs/roadmap/"
  When every file is scanned for CJK structural keywords
  Then no file contains one
