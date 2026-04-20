
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

## 2026-03-21 - Browser base-path inference should slice normalized URLs
**Learning:** `RouterWidget::infer_browser_base_path` scans each leading path segment while syncing browser URLs. Rebuilding a stripped `String` for every candidate prefix added avoidable heap churn inside that probe loop even though the normalized pathname was already available.
**Action:** For prefix-probe loops over normalized URLs, track the current byte offset and pass borrowed suffix slices into route matching instead of re-running String-building helpers per candidate.

## 2026-03-22 - Cache table text positions outside redraws
**Learning:** `ShadTableHeaderView` and `ShadTableRowView` were recomputing `estimate_text_width` and aligned x positions for every visible cell on every redraw, even though widths and text only change when row/header data changes.
**Action:** In virtualized table widgets, precompute per-column text offsets during the data-sync step and reuse them in `draw_walk` so scroll and hover redraws stay glyph-only.

## 2026-03-23 - Stream route pattern parsing during registry rebuilds
**Learning:** `RoutePattern::parse` was allocating a temporary `Vec<&str>` for every route registration just to verify that `**` was trailing. In router live-reload and batch-registration flows, that extra pass shows up as measurable heap churn even before the owned segment strings are built.
**Action:** For parser paths that only need look-ahead validation, prefer a `Peekable` iterator and allocate only the final owned data structure; keep a small ignored benchmark nearby so the win stays documented.

## 2026-03-24 - Nested router tail matching should slice clean suffixes
**Learning:** `RoutePattern::matches_prefix_with_tail` sits on the nested-router resolution path. Rebuilding clean suffixes through `split('/')` and repeated `push_str()` calls adds avoidable heap churn even when the remaining path is already normalized.
**Action:** When matching nested route prefixes, track the unconsumed `&str` remainder directly, fast-path clean suffix copies with one `String` allocation, and fall back to segment normalization only for duplicate or trailing slashes.

## 2026-03-20 - Router query serialization should stream percent-encoding
**Learning:** `makepad-router-core` rebuilt query strings by percent-encoding each key and value into temporary `String`s before copying them into the final URL, which adds heap churn on every router URL update.
**Action:** For hot URL serialization paths, collect borrowed `(&str, &str)` entries, sort those, and write percent-encoded bytes directly into one pre-sized output buffer.

## 2026-03-26 - Browser clean-path prefixing should avoid temporary RouterUrl parsing
**Learning:** `makepad-router-widgets::prefix_clean_browser_base_path` sits on the browser sync path and was allocating three temporary `String`s via `RouterUrl::parse` just to prepend the base path and copy the same parts back out.
**Action:** When a router helper only needs to reassemble an already-normalized URL, split borrowed `&str` slices for path/query/hash and append directly into one destination `String` instead of round-tripping through an owned parsed struct.

## 2024-05-24 – Eliminate Unnecessary Rc Cloning in Hot Paths
**Learning:** In Makepad codebases, referencing global state via `cx.global::<T>()` returns a reference. By default, attempting to hold this reference across mutable operations on `cx` will cause borrow-checker errors. A common, but inefficient, workaround is to `.clone()` the underlying `Rc`/`Arc` wrapper. This cloning increments/decrements the atomic reference counter, causing unnecessary heap-level churn, particularly when done inside event or draw loops (e.g. `on_after_apply`).
**Action:** Always prefer borrowing the `RefCell` state directly from the `cx.global()` reference inside a tight block scope `{ let state = cx.global::<T>().state.borrow(); ... }`. This immediately drops the borrow guard and the reference to the global object before any subsequent mutable `cx` operations, completely eliminating the need to clone the `Rc`/`Arc`.

## 2026-04-03 - Router bool query parsing should not lowercase copies
**Learning:** `makepad-router-core::Route::query_get_bool` sits on the route/widget read path and was allocating a fresh lowercase `String` via `to_ascii_lowercase()` for every boolean query lookup, even though the accepted values are a tiny fixed ASCII set.
**Action:** For hot router parsing helpers, compare the borrowed `&str` with `eq_ignore_ascii_case` against fixed literals instead of normalizing into an owned buffer first; keep a small ignored release benchmark nearby to document the win.

## 2026-04-04 - Reuse table width vectors instead of cloning on length changes
**Learning:** In `makepad-components/src/table.rs`, adjusting column widths used a conditional `.clone()` when the previous and new width vector lengths mismatched. This caused the old vector capacity to be dropped and a completely new heap buffer to be allocated.
**Action:** When updating a `Vec` from another slice or vector in Makepad hot paths (like synchronizing table widths), use `vec.clear(); vec.extend_from_slice(src);` to reuse the existing allocation.

## 2026-04-06 - Plus-only query decoding should skip byte buffers
**Learning:** `makepad-router-core::url::decode_www_form_component` already fast-pathed plain strings, but `+`-only query values still allocated a temporary `Vec<u8>` and rebuilt UTF-8 bytes even though they only needed space substitution.
**Action:** When router query decoding sees `+` but no `%`, use a direct string replacement fast path and keep a release micro-benchmark next to the helper so future refactors preserve the allocation win.
## 2026-04-12 – Prevent Heap Allocations in Router Callbacks (Reverted: Breaking Change)
**Learning:** Passing `Route` by value in callback signatures (`Fn(&mut Cx, Option<Route>, Route)`) causes unnecessary heap allocations on every route transition. Passing by reference (`Option<&Route>`, `&Route`) avoids cloning, but this is a **source-breaking change** to the public API — existing callbacks that move routes into channels/tasks fail to compile. Since `makepad-router-widgets` is v1.0.0, such API changes require a compatibility path or major version bump.
**Action:** Any optimization that changes callback parameter ownership in a public API must be accompanied by a semver bump or a deprecated compatibility overload. The early-exit optimization (`if callbacks.is_empty() { return; }`) is safe and non-breaking and should be retained independently.
## 2026-04-18 - Route actions reordering eliminates RouterAction cloning
**Learning:** In `makepad-router-widgets/src/widget/api.rs`, navigating queues a `RouterAction` and syncs the browser URL. The previous flow called `queue_route_actions` (consuming an owned clone) before `sync_browser_with_action` (which takes a reference). Because `RouterAction` wraps `Route` (which allocates strings and hash maps for parameters), this extra clone caused unnecessary heap allocations on every route change.
**Action:** Reorder synchronous side-effects where possible to let a consuming function run last. By borrowing the `RouterAction` for browser sync first, we can directly move the un-cloned owned value into the queue.
## 2026-04-17 - Eliminate RouterAction Clone in Path Navigation
**Learning:** In Makepad routing, when dispatching a route action,  consumes the action (moving it into a ). However,  only requires a reference (). If they are called in the wrong order (queue then sync), it necessitates an expensive clone of , which intern clones the  object and its heap-allocated parameters/queries.
**Action:** Always perform side-effects that take references before side-effects that take ownership. Swap the execution order of  and  to eliminate unnecessary deep cloning in hot paths.

## 2025-02-14 - Eliminate RouterAction Clone in Path Navigation
**Learning:** In Makepad routing, when dispatching a route action, `queue_route_actions` consumes the action (moving it into a `Vec`). However, `sync_browser_with_action` only requires a reference (`&RouterAction`). If they are called in the wrong order (queue then sync), it necessitates an expensive clone of `RouterAction`, which intern clones the `Route` object and its heap-allocated parameters/queries.
**Action:** Always perform side-effects that take references before side-effects that take ownership. Swap the execution order of `sync_browser_with_action` and `queue_route_actions` to eliminate unnecessary deep cloning in hot paths.
## 2024-04-18 – Eliminate Route clones in makepad-router
**Learning:** In event-heavy UI architectures, routing callback loops (`for callback in &callbacks`) that pass complex values (like `Route`, which contains `HashMap` allocations for queries and params) can cause significant, unnecessary heap churn on every layout or interaction frame.
**Action:** When designing observer or callback APIs (e.g., `on_route_change`), prefer passing arguments by reference (`&T` or `Option<&T>`) instead of consuming `T` or enforcing `.clone()` at the dispatch site, especially for structs holding heap-allocated collections.
## 2026-04-20 – Command Palette Loop Unswitching\n**Learning:** In `makepad-gallery/src/ui/command_palette.rs`, iterating over `catalog::entries()` while conditionally evaluating a fallback path inside a `unwrap_or_else` closure creates unnecessary bounds checks. Although lazy evaluation avoids redundant string allocations when the array lengths match, the loop still incurs branch evaluation overhead.\n**Action:** Use loop unswitching. Iterate directly over the pre-cached `search_terms` slice for the hot path, and handle any uncached trailing entries in a separate fallback loop. This eliminates bounds checks and branch evaluation in the hot path without risking silent result dropping if array lengths differ.
