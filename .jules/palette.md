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

## 2026-03-18 – [Shortcut hints must match real input support]
**Learning:** The gallery shell already opens the command palette on both `Cmd+K` and `Ctrl+K`, but the header only advertised the macOS shortcut and the mobile trigger label was too generic.
**Action:** When a shared launcher supports multiple modifier paths or ambiguous search scope, show the real cross-platform shortcut and name the target explicitly in the shell chrome.
## 2026-03-18 – [Keep primary navigation keyboard reachable]
**Learning:** In this codebase, `grab_key_focus: false` on shared sidebar items silently removes the main catalog navigation from the tab order, and transparent focus colors make the regression easy to miss.
**Action:** For reusable nav controls, keep keyboard focus enabled by default and pair it with a visible focus state that matches hover or active treatment.
## 2026-03-18 – [Gallery examples should not teach placeholder-only fields]
**Learning:** In this repo, the gallery pages act as live documentation. When a preview uses only placeholder text, it quietly teaches an inaccessible pattern even if the code snippet beside it shows the correct labeled-field composition.
**Action:** Keep the live preview aligned with `ShadFieldLabel` and helper text patterns whenever a gallery page demonstrates form inputs.

## 2024-03-19 - [Focus ring contrast consistency]
**Learning:** In Makepad components, thin 1px focus rings using subtle hover colors (like `color_outline_border_hover`) fail accessibility contrast guidelines for keyboard users, whereas other components (like `RadioGroup`) correctly use a 2px `color_primary` stroke.
**Action:** Always ensure focus rings for interactive controls (Checkbox, Toggle, etc.) use a high-contrast primary color and a thicker stroke (e.g. 2px) for clear keyboard visibility.

## 2026-03-19 – [Grouped actions must stay in tab order]
**Learning:** In this codebase, toggling `grab_key_focus` on a styled `ButtonFlat` is not enough to make grouped actions meaningfully keyboard reachable because the upstream widget already manages its own nav stop and click-focus path.
**Action:** When a reusable action row or grouped button needs real tab-stop control, wire keyboard reachability in the component widget itself: add or skip `cx.add_nav_stop(...)` in `draw_walk`, and gate click focus plus key activation in `handle_event`.
