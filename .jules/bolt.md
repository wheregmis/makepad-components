
## 2023-10-27 - Caching String Builds in Event Handlers
**Learning:** `handle_event` is called unconditionally on all events in Makepad, which means running `format!` or other string allocations inside it on every call causes extreme churn, even if the result isn't visually changing.
**Action:** When a string built from a Live property is needed for rendering/handling (like converting a code string to Markdown text), cache the raw source text as a `String` in the Rust struct (e.g., `last_code: String`). During `handle_event`, check if `self.code.as_ref() != self.last_code` and only re-allocate/format when it actually changes.

## 2025-02-18 - Caching Script Evals in Draw/Event Loops
**Learning:** In Makepad code, `script_apply_eval!` is computationally expensive. If layout state is continuously applied via this macro inside `handle_event` or `draw_walk` (which happen on every frame or user interaction), it results in unnecessary macro evaluation overhead and CPU churn.
**Action:** Always maintain a local state mirror of the properties governing the layout (like tracking a `.to_string()` for text, or `is_initialized` boolean flag). Check against this cached state before triggering `script_apply_eval!` to ensure the script updates are strictly event-driven.
