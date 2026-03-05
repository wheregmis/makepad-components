# Theme Consistency Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace all hardcoded hex colors with `shad_theme` tokens and style unstyled native Makepad widgets (FoldButton, CheckBox, Toggle, Hr, ButtonFlatter) to match the dark shadcn aesthetic.

**Architecture:** All changes are in the Splash DSL layer — no Rust widget code changes. A new `gallery/src/ui/themed_widgets.rs` defines gallery-specific themed overrides for native widgets. Component library files fix their own hardcoded literals. Gallery pages then use these themed variants.

**Tech Stack:** Makepad, Splash scripting DSL, Rust (for module registration only)

---

### Task 1: Fix hardcoded colors in `ShadButtonLink`

**Files:**
- Modify: `components/src/button.rs` (lines 117–119)

**Context:** `ShadButtonLink` uses three hardcoded hex colors instead of `shad_theme` tokens. These need to become:
- `#a1a1aa` → `(shad_theme.color_muted_foreground)`
- `#fafafa` → `(shad_theme.color_primary)`
- `#d4d4d8` → `(shad_theme.color_primary_down)`

Note: `shad_theme.color_primary_down` does not exist yet — check `components/src/theme.rs`. If missing, use `(shad_theme.color_primary_hover)` as the closest alternative. Looking at theme.rs the defined tokens are:
- `color_primary`, `color_primary_hover`, `color_primary_down`, `color_primary_foreground`

So `color_primary_down` does exist. Use it.

**Step 1: Edit `ShadButtonLink` in `components/src/button.rs`**

Replace lines 117–119:
```
        draw_text.color: #a1a1aa
        draw_text.color_hover: #fafafa
        draw_text.color_down: #d4d4d8
```
With:
```
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.color_hover: (shad_theme.color_primary)
        draw_text.color_down: (shad_theme.color_primary_down)
```

**Step 2: Build to verify**

```bash
cargo build -p makepad-components
```
Expected: compiles without errors.

**Step 3: Commit**

```bash
git add components/src/button.rs
git commit -m "fix: replace hardcoded colors in ShadButtonLink with shad_theme tokens"
```

---

### Task 2: Fix hardcoded color in `button_page.rs`

**Files:**
- Modify: `gallery/src/ui/button_page.rs` (line 115)

**Context:** The "Makepad Icon Crate" section label uses `#9f9f9f` instead of the theme token.

**Step 1: Edit `gallery/src/ui/button_page.rs`**

Replace:
```
            draw_text.color: #9f9f9f
```
With:
```
            draw_text.color: (shad_theme.color_muted_foreground)
```

**Step 2: Build to verify**

```bash
cargo build -p gallery
```
Expected: compiles without errors.

**Step 3: Commit**

```bash
git add gallery/src/ui/button_page.rs
git commit -m "fix: replace hardcoded label color in button_page with shad_theme token"
```

---

### Task 3: Create `themed_widgets.rs` with gallery-specific widget overrides

**Files:**
- Create: `gallery/src/ui/themed_widgets.rs`

**Context:** Native Makepad widgets (`FoldButton`, `CheckBox`, `Toggle`, `Hr`) use default Makepad styling which looks wrong on the dark background. We define themed gallery variants here. Study `shad_theme` token names from `components/src/theme.rs` before writing — use only tokens that exist there.

The available tokens from `theme.rs`:
- `color_primary`, `color_primary_hover`, `color_primary_down`, `color_primary_foreground`
- `color_secondary`, `color_secondary_hover`, `color_secondary_down`, `color_secondary_foreground`
- `color_muted`, `color_muted_foreground`
- `color_destructive`, `color_destructive_hover`, `color_destructive_down`, `color_destructive_foreground`
- `color_ghost_hover`, `color_ghost_down`
- `color_outline_border`, `color_outline_border_hover`, `color_outline_border_down`
- `color_background`
- `radius`

**Step 1: Create `gallery/src/ui/themed_widgets.rs`**

```rust
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryHr = Hr{
        draw_bg.color: (shad_theme.color_outline_border)
    }

    mod.widgets.GalleryCheckBox = CheckBox{
        draw_text.color: (shad_theme.color_primary)
        draw_text.color_hover: (shad_theme.color_primary)
        draw_text.text_style.font_size: 10
        draw_check.color: (shad_theme.color_muted_foreground)
        draw_check.color_hover: (shad_theme.color_primary)
    }

    mod.widgets.GalleryToggle = Toggle{
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 10
    }
}
```

Note: `FoldButton` styling is controlled via `draw_bg` and `draw_icon` — but since `FoldButton` is embedded inside `AccordionItem` which is a component we own, we style it at the accordion level in Task 4 instead.

**Step 2: Build to verify**

```bash
cargo build -p gallery
```
Expected: compile error because `themed_widgets` module is not registered yet. That's fine — fix in Task 4.

Actually — register it first so the build works. See Task 4 Step 1 before running the build here. Come back to verify after Task 4 Step 1.

---

### Task 4: Register `themed_widgets` in `gallery/src/ui/mod.rs`

**Files:**
- Modify: `gallery/src/ui/mod.rs`

**Step 1: Add module declaration and registration**

Add `pub mod themed_widgets;` to the module list and `crate::ui::themed_widgets::script_mod(vm);` to `script_mod`. The file should look like:

```rust
use makepad_components::makepad_widgets::*;

pub mod accordion_page;
pub mod alert_page;
pub mod button_page;
pub mod content_flip;
pub mod root;
pub mod sidebar;
pub mod themed_widgets;

pub fn script_mod(vm: &mut ScriptVm) {
    crate::ui::themed_widgets::script_mod(vm);
    crate::ui::sidebar::script_mod(vm);
    crate::ui::accordion_page::script_mod(vm);
    crate::ui::alert_page::script_mod(vm);
    crate::ui::button_page::script_mod(vm);
    crate::ui::content_flip::script_mod(vm);
    crate::ui::root::script_mod(vm);
}
```

Important: `themed_widgets` must be registered **before** the pages that use its widgets, so it goes first in `script_mod`.

**Step 2: Build to verify**

```bash
cargo build -p gallery
```
Expected: compiles without errors.

**Step 3: Commit**

```bash
git add gallery/src/ui/themed_widgets.rs gallery/src/ui/mod.rs
git commit -m "feat: add themed_widgets with GalleryHr, GalleryCheckBox, GalleryToggle"
```

---

### Task 5: Apply themed widgets in `accordion_page.rs`

**Files:**
- Modify: `gallery/src/ui/accordion_page.rs`

**Context:** The accordion page has:
1. Toolbar buttons (`ButtonFlatter{text: "XSmall"}` etc.) — swap to `ShadButtonGhost` which is already themed
2. `CheckBox` widgets (`option_icon`, `option_disabled`, `option_bordered`) — swap to `GalleryCheckBox`
3. `Toggle` in accordion body — swap to `GalleryToggle`
4. `Hr{}` divider — swap to `GalleryHr`
5. The accordion `Label` with no color override at line ~47 (`text: "Normal"`) — verify it has `draw_text.color: (shad_theme.color_muted_foreground)` (it does in the file)

**Step 1: Update toolbar buttons**

In `accordion_page.rs`, replace the toolbar `ButtonFlatter` widgets and the existing `Button`:
```
            ButtonFlatter{text: "XSmall"}
            ButtonFlatter{text: "Small"}
            size_medium := Button{text: "Medium"}
            ButtonFlatter{text: "Large"}
```
With `ShadButtonGhost` variants:
```
            ShadButtonGhost{
                height: 28
                padding: Inset{left: 10, right: 10, top: 0, bottom: 0}
                draw_text.text_style.font_size: 9
                text: "XSmall"
            }
            ShadButtonGhost{
                height: 28
                padding: Inset{left: 10, right: 10, top: 0, bottom: 0}
                draw_text.text_style.font_size: 10
                text: "Small"
            }
            size_medium := ShadButton{text: "Medium"}
            ShadButtonGhost{
                height: 44
                padding: Inset{left: 14, right: 14, top: 0, bottom: 0}
                draw_text.text_style.font_size: 13
                text: "Large"
            }
```

**Step 2: Update CheckBox widgets**

Replace:
```
            option_icon := CheckBox{text: "Icon"}
            option_disabled := CheckBox{text: "Disabled"}
            option_bordered := CheckBox{text: "Bordered"}
```
With:
```
            option_icon := GalleryCheckBox{text: "Icon"}
            option_disabled := GalleryCheckBox{text: "Disabled"}
            option_bordered := GalleryCheckBox{text: "Bordered"}
```

**Step 3: Update Toggle**

Replace:
```
                            Toggle{text: "Switch"}
                            CheckBox{text: "Or a CheckBox"}
```
With:
```
                            GalleryToggle{text: "Switch"}
                            GalleryCheckBox{text: "Or a CheckBox"}
```

**Step 4: Update Hr divider**

Replace:
```
        Hr{}
```
With:
```
        GalleryHr{}
```

**Step 5: Build and run**

```bash
cargo run -p gallery
```
Expected: app runs, accordion page toolbar shows ghost-styled size buttons, checkboxes and toggle match the dark theme, divider line is subtle.

**Step 6: Commit**

```bash
git add gallery/src/ui/accordion_page.rs
git commit -m "feat: apply themed widgets across accordion page"
```

---

### Task 6: Apply `GalleryHr` in other gallery pages

**Files:**
- Modify: `gallery/src/ui/alert_page.rs`
- Modify: `gallery/src/ui/button_page.rs`

**Context:** Both `alert_page.rs` and `button_page.rs` use bare `Hr{}`. Replace with `GalleryHr{}`.

**Step 1: Update `alert_page.rs`**

Replace `Hr{}` with `GalleryHr{}`.

**Step 2: Update `button_page.rs`**

Replace `Hr{}` with `GalleryHr{}`.

**Step 3: Build and run**

```bash
cargo run -p gallery
```
Expected: all three gallery pages show the themed divider line.

**Step 4: Commit**

```bash
git add gallery/src/ui/alert_page.rs gallery/src/ui/button_page.rs
git commit -m "feat: use GalleryHr in alert and button pages"
```

---

### Task 7: Final verification

**Step 1: Full build**

```bash
cargo build
```
Expected: zero errors, zero warnings about unused variables.

**Step 2: Run and visually verify each page**

```bash
cargo run -p gallery
```

Check each page:
- **Accordion page:** toolbar buttons are ghost-styled, checkboxes/toggle match dark theme, accordion fold arrows visible
- **Button page:** ShadButtonLink text is muted gray, icon labels are muted gray
- **Alert page:** both alert variants look correct

**Step 3: Grep for any remaining hardcoded hex colors in gallery and components source**

```bash
grep -rn '#[0-9a-fA-F]\{6\}\|#[0-9a-fA-F]\{3\}' components/src/ gallery/src/ --include="*.rs"
```

Any remaining hits that are NOT `#0000` (transparent) or intentional neutrals should be replaced with theme tokens.

**Step 4: Commit if any final fixes**

```bash
git add -p
git commit -m "fix: remove remaining hardcoded colors found in final review"
```
