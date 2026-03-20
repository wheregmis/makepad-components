# Responsive Makepad Components Guide

This document describes how responsiveness is handled in this workspace and the practical patterns we use for mobile-friendly Makepad apps.

## How responsiveness works in Makepad

Makepad does not use CSS media queries. Responsiveness is built from a few core layout primitives plus runtime window geometry events:

- `width: Fill` and `height: Fill` let widgets expand to the available space.
- `width: Fit` and `height: Fit` let widgets hug their content.
- `flow: Down` stacks content vertically, which is usually the safest default for mobile.
- `flow: Right{wrap: true}` allows fixed-width siblings to wrap onto the next line when horizontal space runs out.
- `Event::WindowGeomChange(...)` lets app code react to the actual window size and switch shell behavior, such as showing a mobile header or collapsing a sidebar.
- Overlay widgets should use conservative widths plus viewport padding so they stay usable on narrow screens.

In this repo, the gallery app already uses `WindowGeomChange` to switch between desktop and small-screen navigation modes in `makepad-gallery/src/main.rs`. The component suite itself uses `Fill`/`Fit` sizing and bounded overlay widths so controls can shrink cleanly inside app-owned containers.

## Responsive rules for this component suite

### 1. Prefer container-led sizing

Reusable widgets should usually let the parent decide width. In practice that means:

- form controls should usually be placed in a `width: Fill` field/container, with explicit control overrides when the surrounding shell is `Fit`-sized;
- overlays should use a moderate bounded width instead of very wide desktop-first defaults;
- preview/demo shells should use `width: Fill` unless a deliberate bounded width is part of the example.

### 2. Stack first, wrap second

For mobile layouts, use this decision order:

1. Try `flow: Down` for sections, forms, and explanatory content.
2. Use `flow: Right{wrap: true}` for rows of fixed-width cards, badges, chips, or keycaps.
3. Avoid `wrap: true` when children depend on `width: Fill`, because wrapped `Right` flows do not support fill-width children.

### 3. Bound wide storytelling surfaces

Components like carousels, cards, sheets, dialogs, and command palettes need explicit width decisions. The goal is to keep them readable on desktop without exceeding a phone viewport.

Current workspace guidance:

- dialogs and sheets should stay around the mid-300px range by default;
- command/search overlays should stay narrow enough for portrait phones;
- large gallery demos should use `Fill` and let their internal content establish rhythm.

### 4. Keep app-shell responsiveness in app state

App-shell changes belong to the app, not to individual reusable widgets. In the gallery, the app shell owns:

- switching between desktop and mobile headers;
- opening and closing the sidebar on small screens;
- hiding the main content when the mobile sidebar is open.

That pattern should be reused for product apps: keep route, sidebar, and page-shell decisions in app state, while individual components stay layout-friendly and reusable.

## Changes applied in this responsiveness pass

### Gallery shell and pages

- Wide gallery demo shells were changed from fixed desktop widths to `Fill` where appropriate.
- Card, aspect-ratio, keyboard shortcut, label, sheet, dialog, and sidebar demos were adjusted to stack or wrap more gracefully.
- The command palette overlay was narrowed for portrait/mobile use.

### Component defaults

- `ShadDialog`, `ShadSheet`, and `ShadNavigationMenuContent` use smaller default widths that are safer on mobile.
- Gallery form demos now opt into `width: Fill` at the usage site for `ShadSelect` and `ShadDatePicker`, which avoids collapses inside `Fit`-sized parents while keeping the underlying components reusable.

## Practical checklist for new components/pages

When adding a new component or gallery page, verify all of the following:

- Does the main demo container use `Fill` unless a bounded width is essential?
- If a row must wrap, are all direct children fixed-width or fit-width?
- If a control belongs in a form, is it placed inside a `width: Fill` field/container, and does the control get an explicit fill-width override when needed?
- If an overlay opens on top of the UI, is its default width phone-safe?
- If the app shell changes on mobile, is that behavior owned by app state via `WindowGeomChange`?
- Does the page still read clearly in a narrow portrait layout?

## Recommended responsive patterns

### Responsive form field

```rust
ShadField{
    width: Fill
    ShadFieldLabel{text: "Status"}
    ShadSelect{
        width: Fill
        labels: ["Pending" "In Progress" "Done"]
    }
}
```

### Wrapping card row

```rust
View{
    width: Fill
    height: Fit
    flow: Right{wrap: true}
    spacing: 12.0

    ShadSurface{width: 220}
    ShadSurface{width: 220}
    ShadSurface{width: 220}
}
```

### Mobile-aware shell logic

```rust
fn update_screen_mode(&mut self, cx: &mut Cx, window_width: f64) {
    let is_small_screen = window_width < 900.0;
    if self.is_small_screen != is_small_screen {
        self.is_small_screen = is_small_screen;
        self.sidebar_open = !is_small_screen;
    }
    self.apply_responsive_visibility(cx);
}
```

## One important Makepad caveat

`flow: Right{wrap: true}` is useful, but it is not a CSS flexbox replacement. Direct children in a wrapped row should not use `width: Fill`. If you need flexible full-width children, switch to `flow: Down` on small/narrow layouts or give each wrapped child a fixed width.
