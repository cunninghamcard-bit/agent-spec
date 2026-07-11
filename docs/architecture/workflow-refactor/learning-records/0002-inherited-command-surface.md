# 0002 — The inherited command surface

- **Finding**: research.md F2 (The inherited command surface)
- **Question**: How does this refactor treat the 16 non-core subcommands?
- **Recommendation (first attempt, rejected)**: git-style demotion by
  this session's usage — core 9 / utilities 8 / hidden 8.
- **User's pushback**: "为什么说我们有多余的命令？我们没做过删除命令的
  feature啊" — the premise was wrong. Verified against upstream/main:
  all 16 are upstream inheritance we never evaluated; we added only
  research, finish, integrate. Usage-to-date is not an evaluation.
- **Recommendation (reframed)**: inventory before tiering — audit each
  inherited command against the SDD loop, adopt the ones that fill felt
  holes (stamp, promote, discover), then tier help around the resulting
  loop. Demotion only after audit; deletion never.
- **User's answer**: Confirmed — 先盘家底再分层.
- **Overrides**: The original F2 recommendation and its
  "redundant commands" framing.
