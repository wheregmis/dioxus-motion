# Task Plan: Reduce complexity + cleanup

## Goal
Make the codebase simpler to maintain by removing obvious overengineering, fixing clippy issues, and documenting follow-up refactors.

## Phases
- [ ] Phase 1: Establish baseline (fmt/test/clippy) and identify hotspots
- [ ] Phase 2: Do safe cleanups (warnings, clippy, dead code, small refactors)
- [ ] Phase 3: Simplify core abstractions (only if API-compatible)
- [ ] Phase 4: Verify (tests, docs) and summarize changes

## Key Questions
1. Which public APIs are stable/required?
2. Is peak performance (pooling/state machine) a hard requirement or optional?
3. Are there target platforms/features that must stay green (web/desktop/transitions)?

## Decisions Made
- Start with clippy-clean + low-risk refactors before larger architectural simplifications.

## Errors Encountered
- None yet.

## Status
**Currently in Phase 2** - doing safe cleanups (clippy + small simplifications).
