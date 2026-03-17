## 2024-05-24 - Accessibility improvements

**Learning:** Buttons in the Makepad component library lack `cursor: MouseCursor.Hand` by default, making them feel less interactive. Also, buttons need aria labels.
**Action:** Add `cursor: MouseCursor.Hand` to all interactive elements. Wait, buttons might have cursor by default, but I need to check.
## 2024-03-09 – [Add pointer cursor to buttons]
**Learning:** In Makepad, UI elements like buttons may lack an interactive pointer cursor by default.
**Action:** Explicitly adding `cursor: MouseCursor.Hand` to components improves discoverability and usability.

## 2026-03-17 – [Expose keyboard-only actions in shell chrome]
**Learning:** In the gallery shell, the command palette already existed but was hard to discover because it mostly relied on `Cmd/Ctrl + K` and a docs page.
**Action:** When a Makepad app has a global launcher or shortcut-driven feature, add a visible header trigger near navigation controls so mouse and first-run users can find it immediately.
