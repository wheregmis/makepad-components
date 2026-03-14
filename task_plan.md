# Task Plan: UI Workspace Refactor

## Goal
Refactor the Makepad UI workspace into cleaner component families and shared gallery metadata while keeping the workspace buildable throughout the change.

## Phases
- [x] Phase 1: Capture current state and shared refactor rules
- [x] Phase 2: Extract shared component internals and normalize APIs
- [x] Phase 3: Restructure public surfaces and gallery metadata
- [x] Phase 4: Update docs, verify, and deliver

## Key Questions
1. Which current files already contain in-flight additions that must be preserved?
2. Which widget families can share controllers without changing runtime behavior?
3. Which gallery concerns should move behind a single catalog without weakening routing?

## Decisions Made
- Use `makepad-ui-patterns` for script/module patterns and `planning-with-files` for persistent tracking.
- Keep router registration explicit in `root.rs`; move metadata duplication out first.
- Treat the existing dirty worktree as source material, not as something to reset.

## Errors Encountered
- The gallery catalog array length was initially wrong (`44` vs `43`) and was corrected after the first workspace check.
- The existing WASM build script still documented the old gallery package name; validation succeeded after rerunning it with `-p makepad-gallery`.

## Status
**Completed** - shared internals/models were extracted, overlay/expandable APIs were normalized, gallery metadata moved into a catalog/snippet-key system, docs were updated, and workspace/tests/WASM packaging passed. Studio-driven manual smoke testing was not run in this turn.
