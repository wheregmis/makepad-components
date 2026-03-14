# ShadAccordionItem Design

**Date:** 2026-03-05
**Goal:** Build a new `ShadAccordionItem` Rust widget from scratch following the `FoldButton` pattern, with self-contained header (title text + rotating chevron via SDF shader), composable body slot, bottom divider line, and smooth open/close animation.

## Architecture

### Two new Rust types in `components/src/accordion.rs`

**`ShadAccordionItem`** — main widget

- `#[derive(Script, ScriptHook, Widget, Animator)]`
- `draw_bg: DrawQuad` — draws bottom divider line (1px `shad_theme.color_outline_border`) via SDF; also draws header hover background
- `draw_text: DrawText` — draws title text in header
- `draw_icon: DrawQuad` — draws rotating chevron arrow via SDF triangle (same technique as `FoldButton`)
- `body: WidgetRef` — arbitrary body content slot
- `title: String` — header title text
- `is_open: bool` — initial state
- `active: f64` — animated float (0.0=closed, 1.0=open), drives chevron rotation and body visibility
- `animator: Animator` — `hover` and `active` states

**`ShadAccordion`** — thin DSL-only container (no Rust needed, just a `View` wrapper in script_mod)

### Actions

```rust
pub enum ShadAccordionItemAction {
    None,
    Opening,
    Closing,
    Animating(f64),
}
```

Callers: `item.open_changed(actions)`, `item.animation_progress(actions)`

### DSL Usage

```
ShadAccordion {
    ShadAccordionItem {
        title: "Is it accessible?"
        is_open: true
        body: View { Label { text: "Yes. It adheres to WAI-ARIA." } }
    }
    ShadAccordionItem {
        title: "Is it styled?"
        body: View { ... }
    }
}
```

## Shader Details

### `draw_bg` pixel shader
- Draws a 1px horizontal line at the bottom of the item (divider) using `shad_theme.color_outline_border`
- Header area: transparent fill normally, `shad_theme.color_secondary_hover` on hover (driven by `hover` instance var)

### `draw_icon` pixel shader (chevron)
- SDF triangle, same approach as `FoldButton`
- `active` instance var: 0.0 = pointing right ▶ (collapsed), 1.0 = pointing down ▼ (open)
- Rotation: `sdf.rotate(self.active * 0.5 * PI, cx, cy)`
- Color: `shad_theme.color_muted_foreground`, brightens to `shad_theme.color_primary` on hover

### `draw_text`
- Title text color: `shad_theme.color_primary`
- Font size: 11

## Animator

```
hover:
  off: Forward 0.1s → draw_bg.hover: 0.0, draw_icon.hover: 0.0
  on:  Snap       → draw_bg.hover: 1.0, draw_icon.hover: 1.0

active:
  default: @on (starts open)
  off: Forward 0.2s ExpDecay → active: 0.0, draw_icon.active: 0.0
  on:  Forward 0.2s ExpDecay → active: 1.0, draw_icon.active: 1.0
```

## Draw Logic (`draw_walk`)

```
1. Begin turtle (item walk)
2. Draw header row: draw_bg (hover bg) | draw_text (title) | draw_icon (chevron)
3. If active > 0.0: draw body WidgetRef
4. Draw divider: draw_bg bottom 1px line
5. End turtle
```

Body is shown when `active > 0.0` (while animating AND when open).

## Event Logic (`handle_event`)

- `animator_handle_event` → if `must_redraw()`, emit `Animating(self.active)`, redraw
- `event.hits(cx, header_area)`:
  - `FingerDown`: toggle `active.on/off`, emit `Opening`/`Closing`
  - `FingerHoverIn`: set cursor Hand, play `hover.on`
  - `FingerHoverOut`: play `hover.off`
  - `FingerUp`: restore hover state

## Files to Touch

| File | Change |
|---|---|
| `components/src/accordion.rs` | Replace `AccordionItem` with `ShadAccordionItem` + `ShadAccordionItemRef` |
| `components/src/lib.rs` | No change needed (accordion module already exported) |
| `gallery/src/ui/accordion_page.rs` | Update to use `ShadAccordionItem` with `title:` and `body:` |

## Success Criteria

- Widget registers correctly via `script_mod`
- Header click toggles open/close with smooth chevron rotation
- Bottom divider renders between items
- Header shows hover background on mouse over
- Body content renders when open, hides when closed
- `cargo run -p gallery` shows working accordion page
