spec: task
name: "git 根发现移除进程 cwd 兜底"
inherits: project
tags: [guard, verify, vcs, fail-fast]
---

## Intent

`guard` 与单命令的 git 仓库根发现目前在 `--code` 与 spec 路径都不在仓库内时
兜底到进程当前目录，导致变更集悄悄来自无关仓库，也让单元测试依赖宿主仓库的
暂存状态而随机失败。移除该兜底：路径不在 git 仓库内就返回空变更集。

## Decisions

- `find_guard_repo_root` 候选仅为 `--code` 与 `--spec-dir`，不再包含进程当前目录
- `find_command_repo_root` 候选仅为 `--code` 与 spec 路径，不再包含进程当前目录
- 两者都找不到仓库时保持现状：返回空变更集，不报错
- 常规用法不受影响：`--code` 默认值本就是当前目录

## Boundaries

### Allowed Changes
- src/main.rs
- specs/task-remove-cwd-repo-fallback.spec.md

### Forbidden
- 不要改变 `--code` 或 spec 路径本身在 git 仓库内时的发现结果
- 不要把"找不到仓库"从返回空变更集改成报错

## Completion Criteria

Scenario: 路径都不在仓库内时不再摸到进程 cwd 的仓库
  Test:
    Package: agent-spec
    Filter: test_repo_root_discovery_ignores_process_cwd
  Given spec 目录与 code 目录都是临时的非 git 目录且进程 cwd 位于一个 git 仓库内
  When 解析 guard 的仓库根
  Then 返回 None

Scenario: 非 git 目录返回空变更集且不受宿主暂存状态影响
  Test:
    Package: agent-spec
    Filter: test_resolve_guard_change_paths_returns_empty_outside_git_repo
  Given spec 目录与 code 目录都是临时的非 git 目录且宿主仓库存在暂存变更
  When 以 staged scope 解析 guard 变更集
  Then 返回空变更集
