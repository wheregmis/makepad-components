
## 2023-10-27 - Caching String Builds in Event Handlers
**Learning:** `handle_event` is called unconditionally on all events in Makepad, which means running `format!` or other string allocations inside it on every call causes extreme churn, even if the result isn't visually changing.
**Action:** When a string built from a Live property is needed for rendering/handling (like converting a code string to Markdown text), cache the raw source text as a `String` in the Rust struct (e.g., `last_code: String`). During `handle_event`, check if `self.code.as_ref() != self.last_code` and only re-allocate/format when it actually changes.

## 2025-02-18 - Caching Script Evals in Draw/Event Loops
**Learning:** In Makepad code, `script_apply_eval!` is computationally expensive. If layout state is continuously applied via this macro inside `handle_event` or `draw_walk` (which happen on every frame or user interaction), it results in unnecessary macro evaluation overhead and CPU churn.
**Action:** Always maintain a local state mirror of the properties governing the layout (like tracking a `.to_string()` for text, or `is_initialized` boolean flag). Check against this cached state before triggering `script_apply_eval!` to ensure the script updates are strictly event-driven.

## 2025-02-19 - Replacing String allocations with static arrays in draw loops
**Learning:** Calling `.to_string()` on numeric types like `u8` or `i32` within Makepad's `draw_walk` function causes an allocation on every frame, which severely hurts rendering performance.
**Action:** When mapping bounded numeric ranges (like days of a month, 1-31) to strings for drawing text, use a pre-allocated array of `&'static str` literals and index into it, avoiding any runtime heap allocations.

## 2025-02-18 - Reusing existing fields and avoiding clones in Layout Sync Loops
**Learning:** In layout sync loops that govern Makepad rendering (such as responding to side-changes in widgets like Sheets), making repeated allocations by calling `.to_string()` to cache states, or cloning `WidgetRef` parameters to pass to `script_apply_eval!`, causes needless heap churn and memory duplication. `script_apply_eval!` correctly borrows existing components.
**Action:** When tracking string changes, use `String::clear()` and `String::push_str()` instead of reallocating with `.to_string()`. When updating widgets, directly pass `self.widget` to `script_apply_eval!` instead of making an implicit clone (`let mut widget = self.widget.clone()`).

## 2026-03-15 - Reusing row buffers in virtualized table updates
**Learning:** In `ShadTableRowView::set_row_data`, replacing row/cell vectors with `to_vec()` during scroll updates creates repeated heap allocations and deallocations in a hot UI path.
**Action:** For virtualized row updates, prefer `clear()` + `extend_from_slice()` on existing `Vec` storage (after change detection) so visible row widgets reuse capacity instead of reallocating every swap.

## 2026-03-18 - PortalList row state should be cached per recycled widget
**Learning:** In `makepad-gallery`'s command palette, `PortalList` reuses row widgets across frames, so blindly reapplying identical `set_text`, `set_visible`, and `script_apply_eval!` updates in `draw_walk` causes avoidable UI churn even when the visible result is unchanged.
**Action:** When a Makepad list recycles widgets, cache the bound row state keyed by `WidgetUid` and only push widget property updates when the item identity or visual state actually changes.
## 2026-03-18 - Caching script width updates in auto-fill table draws
**Learning:** `ShadTable::draw_walk` can re-enter continuously while scrolling or hovering, so calling `script_apply_eval!` for the scroll content width on every frame wastes CPU even when the computed width is unchanged.
**Action:** In Makepad widgets with derived layout values, keep a small Rust-side cache of the last applied value and guard `script_apply_eval!` behind that change check so steady-state redraws skip script work entirely.
## 2026-03-18 - Streaming router paths avoids segment churn
**Learning:** In `makepad-router-core`, formatting route URLs through `Vec<String>` plus `join("/")` clones every static segment and adds an extra heap pass on each navigation/update.
**Action:** Build router paths directly into one pre-sized `String`, and use `LiveId::as_string` for interned dynamic params before falling back to `to_string()`.
## 2025-02-12 – Rust Borrow Checker vs. Global State Clones in Makepad
**Learning:** In Makepad, accessing global state via `cx.global::<T>()` borrows `cx` mutably (or immutably, depending on context). If you try to avoid `.clone()` on the global `T` (which is often a cheap `Rc` or `Arc`) to hold a direct reference, you might hold that borrow across subsequent calls that *also* require `cx` mutably (like `.redraw(cx)` or `.open(cx)`), causing an `E0499` borrow checker error. The original `.clone()` calls were not necessarily naive allocations; they were intentionally bypassing borrow check lifetimes for cheap `Rc`/`Arc` clones.
**Action:** Before removing `.clone()` on global Makepad state to "optimize" it, check if the value is actually an `Rc`/`Arc` (making the clone cheap). If it is, and removing it causes lifetime overlap errors with `cx`, keep the `.clone()`. For real optimizations, look for actual string cloning, buffer recreations, or unoptimized loop iterations (like `take()` on iterators instead of `for index in 0..LEN`).

## 2024-05-19 – Optimize ShadAvatarImage Async Image Load Check
**Learning:** Checking equality of an `Option<PathBuf>` with a newly allocated `PathBuf` by cloning the `Option<PathBuf>` causes an unnecessary heap allocation on every async image load event, leading to reduced performance.
**Action:** When comparing an `Option<PathBuf>` against a `Path` or `PathBuf`, use `.as_deref()` to directly compare references without allocating memory for a new `PathBuf`.

## 2026-03-19 - Nested router URL joins should reuse the first path buffer
**Learning:** `RouterWidget` builds current and preview URLs frequently during browser sync and nested-route resolution. Using `format!` for `join_paths` and then formatting the final URL again adds avoidable heap churn on a hot router path.
**Action:** When extending a route path with child segments, query strings, or hashes, pre-size one `String`, append into it with `push_str`, and reuse the initial path allocation instead of formatting a second URL string.
