# 0007 — Capability library, homed in docs/

- **Finding**: research.md F3, re-grilled after the full command
  inventory (supersedes record 0003)
- **Question**: Adopt upstream's full closed loop (goal packages as
  proposals, capability library as the truth layer, promote as the
  channel)?
- **Recommendation**: adopt, with the library at its upstream location
  specs/capabilities/.
- **User's answer**: adopt the loop, but re-home it — "把specs/ 换成docs/
  就行了啊，不就改个str的事？". The capability library lives at
  docs/capabilities/; promote's target path changes accordingly. Single
  household (docs/) and the upstream vision merge: docs/ holds goal
  packages (proposals, mortal) and capabilities/ (truth layer, growing).
- **Overrides**: record 0003 (library unbuilt, promote not adopted) —
  revoked in record 0006. promote enters the loop after finish; audit
  becomes the library's periodic health check.
