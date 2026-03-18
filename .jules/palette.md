## 2024-05-24 - Accessibility improvements

**Learning:** Buttons in the Makepad component library lack `cursor: MouseCursor.Hand` by default, making them feel less interactive. Also, buttons need aria labels.
**Action:** Add `cursor: MouseCursor.Hand` to all interactive elements. Wait, buttons might have cursor by default, but I need to check.
## 2024-03-09 – [Add pointer cursor to buttons]
**Learning:** In Makepad, UI elements like buttons may lack an interactive pointer cursor by default.
**Action:** Explicitly adding `cursor: MouseCursor.Hand` to components improves discoverability and usability.

## 2026-03-17 – [Expose keyboard-only actions in shell chrome]
**Learning:** In the gallery shell, the command palette already existed but was hard to discover because it mostly relied on `Cmd/Ctrl + K` and a docs page.
**Action:** When a Makepad app has a global launcher or shortcut-driven feature, add a visible header trigger near navigation controls so mouse and first-run users can find it immediately.

## 2026-03-17 – [Keep live search controls stable]
**Learning:** In the gallery, a live-filtering search field became less obvious when a submit-style `Search` button implied delayed execution and the `Clear` button appeared and disappeared, shifting the toolbar.
**Action:** For live search in this codebase, keep reset actions in a stable slot, disable them when idle, and label any secondary button by what it actually does, like refocusing the field.

## 2026-03-17 – [Escape should recover before dismissing]
**Learning:** In the gallery's shared command palette, treating `Esc` as an immediate close makes keyboard recovery clumsy because users lose their query context and must reopen the modal just to retry.
**Action:** For searchable overlays in this codebase, use a two-step `Esc` flow: clear the current query first while keeping focus in the field, then close only when the query is already empty.
## 2026-03-17 – [Label transient search surfaces]
**Learning:** In the gallery, modal and toolbar search fields can become placeholder-only once users begin typing, which removes the only visible cue about what the field searches.
**Action:** For search UIs in this codebase, keep a persistent visible label or live summary near the field so the search scope stays clear after input starts.

## 2026-03-18 – [Gallery examples should not teach placeholder-only fields]
**Learning:** In this repo, the gallery pages act as live documentation. When a preview uses only placeholder text, it quietly teaches an inaccessible pattern even if the code snippet beside it shows the correct labeled-field composition.
**Action:** Keep the live preview aligned with `ShadFieldLabel` and helper text patterns whenever a gallery page demonstrates form inputs.
