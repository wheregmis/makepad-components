# Test Spec: Component Efficiency and Thin Gallery

## Goal

Verify that the efficiency/thin-gallery implementation improves the right surfaces, preserves rendering correctness, and leaves the component/gallery workspace buildable.

## Required evidence

### A. Studio baseline + re-measurement

For these representative interactions:
- gallery startup shell
- Avatar page
- Switch page
- Pagination page
- Command Palette open/search
- Table page

Capture and compare:
- `draw_calls`
- `instances`
- `instance_bytes`
- `uniform_bytes`
- `texture_bytes`

Expected result:
- each target is classified as redraw-dominant or draw-path-dominant
- at least one targeted hotspot shows an evidence-backed improvement or simplification

### B. Thin gallery boundary checks

Expected:
- `makepad-gallery/src/ui/root.rs` remains shell/routing/theme oriented
- `makepad-gallery/src/ui/registry.rs` remains the routing/page metadata source of truth
- `makepad-gallery/src/ui/page_macros.rs` remains the standard page composition path
- gallery pages do not re-own reusable `Shad*` behavior or styling that belongs in the component crate

### C. Runtime visual checks

Use Studio screenshots to confirm:
- Switch page renders correctly across sizes/states
- Avatar page renders correctly across sizes/presence states
- Pagination page renders correctly after any efficiency-related changes
- no obvious color/layout regressions on representative pages

### D. Build / test checks

Run:

```bash
cargo check -p makepad-components
cargo check -p makepad-gallery
cargo test -p makepad-components --lib
```

Expected:
- all commands pass

### E. Diagnostics

Run diagnostics on modified source files and expect no errors.

## Completion rule

Do not treat the work as complete until all of the following are true:
- Studio runtime still launches successfully
- representative screenshots look correct
- build/test evidence is green
- the gallery/component boundary is cleaner than before
- any claimed performance improvement is tied to a measured before/after comparison
