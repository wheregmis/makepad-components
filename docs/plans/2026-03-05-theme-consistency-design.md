# Theme Consistency Pass Design

**Date:** 2026-03-05
**Branch:** copilot/add-alerts-component
**Goal:** Use `shad_theme` tokens consistently across the whole app — no hardcoded hex colors, and native Makepad widgets styled to match the dark shadcn aesthetic.

## Problem

Two categories of inconsistency:

### A. Hardcoded colors
- `components/src/button.rs` — `ShadButtonLink` uses literal `#a1a1aa`, `#fafafa`, `#d4d4d8` instead of theme tokens
- `gallery/src/ui/button_page.rs:115` — "Makepad Icon Crate" label uses `#9f9f9f`

### B. Unstyled native widgets
- `FoldButton` in accordion items — Makepad default light styling looks jarring on dark bg
- `CheckBox` in accordion page toolbar (`option_icon`, `option_disabled`, `option_bordered`)
- `Toggle` inside accordion body content
- Toolbar `ButtonFlatter` widgets (XSmall, Small, Large) — no theme colors
- `Hr` dividers in all gallery pages

## Design

All changes stay in the DSL/Splash layer — no Rust changes needed.

### Fix A: Hardcoded colors

**`components/src/button.rs` — `ShadButtonLink`:**
- `#a1a1aa` → `(shad_theme.color_muted_foreground)`
- `#fafafa` → `(shad_theme.color_primary)`
- `#d4d4d8` → `(shad_theme.color_primary_down)`

**`gallery/src/ui/button_page.rs:115`:**
- `#9f9f9f` → `(shad_theme.color_muted_foreground)`

### Fix B: Styled gallery widgets

Create `gallery/src/ui/themed_widgets.rs` with themed overrides:

- `GalleryFoldButton` — arrow icon colored `color_muted_foreground`, no visible bg
- `GalleryCheckBox` — label text `color_primary`, check indicator using theme colors
- `GalleryToggle` — styled to match dark theme
- `GalleryHr` — thin line using `color_outline_border`

Use these in `accordion_page.rs` instead of bare `FoldButton`, `CheckBox`, `Toggle`, `Hr`. Swap toolbar `ButtonFlatter` widgets to `ShadButtonGhost` (already themed).

Register `themed_widgets.rs` in `gallery/src/ui/mod.rs`.

## Files to Touch

| File | Change |
|---|---|
| `components/src/button.rs` | Fix 3 hardcoded colors in `ShadButtonLink` |
| `gallery/src/ui/button_page.rs` | Fix 1 hardcoded color |
| `gallery/src/ui/accordion_page.rs` | Use themed widget variants, swap toolbar buttons |
| `gallery/src/ui/mod.rs` | Register `themed_widgets` module |
| `gallery/src/ui/themed_widgets.rs` (new) | Define `GalleryCheckBox`, `GalleryToggle`, `GalleryFoldButton`, `GalleryHr` |

## Success Criteria

- Zero hardcoded hex colors remaining in gallery or component files
- All interactive widgets in the gallery visually match the dark shadcn palette
- App builds cleanly with `cargo run`
