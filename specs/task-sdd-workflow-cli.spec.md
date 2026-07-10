spec: task
name: "SDD workflow CLI: guard multi-dir, plan --out, init template"
inherits: project
tags: [cli, guard, plan, init, sdd]
---

## Intent

Make the CLI serve DeepChat-style SDD folder layouts: `guard` accepts multiple
spec directories so contracts can live next to their goal folders, `plan`
writes its generated context to a file as a plan.md draft, and the `init` task
template scaffolds the new SDD sections. Documentation records the folder
conventions and lifecycle rules.

## Current State

`guard` scans exactly one `--spec-dir`, `plan` only prints to stdout, and the
task templates predate the Current State / UX Shape / Open Questions sections.

## Decisions

- `guard --spec-dir` becomes repeatable; every given directory is scanned flat, and the default stays `specs`
- `plan --out <path>` writes the rendered plan output to the given file (parent directories created), while stdout behavior stays unchanged when `--out` is absent
- The `init` task templates include `Current State`, `UX Shape`, and `Open Questions` skeleton sections, with `Open Questions` prefilled as "None."
- README and the authoring skill document the SDD conventions: one kebab-case folder per goal, feature/issue/architecture classification, skip rules for trivial work, and the graduation policy (completed contracts are promoted or archived; `specs/` holds active contracts)

## Boundaries

### Allowed Changes
- src/main.rs
- specs/task-sdd-workflow-cli.spec.md
- README.md
- skills/**
- .claude/skills/**
- CHANGELOG.md

### Forbidden
- Do not change the default guard directory away from `specs`
- Do not change plan stdout output when `--out` is not given

## Out of Scope

- Recursive spec-directory scanning (roadmap specs stay out of the default gate)
- Archiving the existing historical contracts under `specs/`

## Completion Criteria

Scenario: guard scans multiple spec directories
  Test:
    Package: agent-spec
    Filter: test_guard_collects_specs_from_multiple_dirs
  Given two directories each containing one spec file
  When guard collects its spec files
  Then specs from both directories are included

Scenario: plan --out writes the plan file
  Test:
    Package: agent-spec
    Filter: test_plan_out_writes_rendered_output_to_file
  Given a task spec and an --out path inside a not-yet-existing directory
  When the plan output is written
  Then the file exists
  And its content equals the rendered plan output

Scenario: init task template scaffolds SDD sections
  Test:
    Package: agent-spec
    Filter: test_init_task_template_includes_sdd_sections
  Given the English task template
  When the template is generated
  Then it contains a "Current State" section
  And it contains an "Open Questions" section prefilled with "None."

Scenario: generated template still parses
  Test:
    Package: agent-spec
    Filter: test_generated_task_templates_parse_for_zh_en_and_both
  Given the generated task templates
  When each template is parsed
  Then parsing succeeds
