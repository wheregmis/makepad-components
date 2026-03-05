
## 2023-10-27 - Caching String Builds in Event Handlers
**Learning:** `handle_event` is called unconditionally on all events in Makepad, which means running `format!` or other string allocations inside it on every call causes extreme churn, even if the result isn't visually changing.
**Action:** When a string built from a Live property is needed for rendering/handling (like converting a code string to Markdown text), cache the raw source text as a `String` in the Rust struct (e.g., `last_code: String`). During `handle_event`, check if `self.code.as_ref() != self.last_code` and only re-allocate/format when it actually changes.
