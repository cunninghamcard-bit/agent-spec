# 0003 — Single spec home

- **Finding**: research.md F3 (The two spec homes)
- **Question**: What is the relationship between the specs/ library and
  the docs/*/ goal packages?
- **Teaching required**: two rounds. The user rejected the first two
  framings ("你没有讲清楚啊，你这个还是让我做选择，不是跟我讲内容啊")
  until the mechanics were taught with concrete repo files: the two homes
  are a track-switch artifact (pre-07-11 work → specs/, post → docs/);
  guard treats both identically; task-organized contracts can't answer
  "what are the rules now" (the historical-spec-all-red defect);
  specs/capabilities/ was never populated — promote has never run.
- **Recommendation**: grow the capability library as the current-state
  source of truth (OpenSpec semantics via the upstream promote channel).
- **User's answer**: **Override** — 只维护 docs/。docs/*/ goal packages
  are the single maintained home for contracts; the capability library
  stays unbuilt; promote is not adopted into the workflow.
- **Overrides**: The recommendation and upstream's living-spec-library
  design intent. Disposal of the 43 historical specs/ contracts is a
  follow-up decision (record 0004).
