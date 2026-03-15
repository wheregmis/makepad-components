## 2024-05-24 - Accessibility improvements

**Learning:** Buttons in the Makepad component library lack `cursor: MouseCursor.Hand` by default, making them feel less interactive.
**Action:** Add `cursor: MouseCursor.Hand` to all interactive elements. Wait, buttons might have cursor by default, but I need to check.
## 2024-03-09 – [Add pointer cursor to buttons]
**Learning:** In Makepad, UI elements like buttons may lack an interactive pointer cursor by default.
**Action:** Explicitly adding `cursor: MouseCursor.Hand` to components improves discoverability and usability.
## 2024-11-20 – [Aria labels not supported]
**Learning:** Makepad currently does not support `aria_label` or `area-labels` for components upstream.
**Action:** Do not attempt to add `aria_label` properties to Makepad components, as this syntax is invalid and unsupported by the engine.
