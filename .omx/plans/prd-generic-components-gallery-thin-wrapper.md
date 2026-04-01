# PRD: Generic Components, Thin Gallery

## Problem

`makepad-components` is meant to be the reusable UI system, but parts of the public API and docs still reflect older gallery-shaped naming and presentation decisions. That makes the library feel less generic than the actual direction of the codebase.

## Product goal

Make `makepad-components` the source of truth for highly customized but generic Makepad UI components, with `makepad-gallery` acting only as a showcase and consumer layer.

## Desired outcome

- Reusable components own sizing, styling, and behavioral APIs.
- Gallery pages demonstrate components without introducing a second styling system.
- Public docs match the actual reusable library surface.
- Preview- or gallery-specific names are either removed from the canonical surface or preserved only as compatibility shims.

## Current facts

- `makepad-components/src/avatar.rs` now exposes `ShadAvatarPresence` and configurable avatar metrics, but `README.md` still documents removed helper names such as `ShadAvatarSm`, `ShadAvatarLg`, and `ShadAvatarStatusOnline`.
- `makepad-components/src/pagination.rs` now supports `ShadControlSize`, and `makepad-gallery/src/ui/pagination_page.rs` already demonstrates the compact size API.
- `makepad-components/src/switch.rs` is already mid-refactor toward a generic, size-aware `ShadSwitch`, but that slice still needs to be validated and folded into the public surface/docs.
- `README.md` still refers to the gallery app as `makepad-example-component-gallery` instead of `makepad-gallery`.

## In-scope for this Ralph slice

1. Finish the generic `ShadSwitch` library surface so it behaves like a first-class reusable component.
2. Update `README.md` so it reflects the actual avatar, pagination, and gallery package APIs.
3. Keep gallery usage thin by relying on library-owned size/state APIs rather than gallery-specific styling workarounds.

## Out of scope for this slice

- New component families
- Major visual redesigns of gallery scaffolding
- Non-UI runtime/platform work
- Breaking public API removals without a compatibility path

## Acceptance criteria

1. `ShadSwitch` exposes a deliberate reusable component API instead of a gallery-framed wrapper.
2. `README.md` no longer documents removed avatar helper names or the stale gallery package name.
3. Pagination docs mention the current size-based API shape.
4. `cargo check -p makepad-components` passes.
5. `cargo check -p makepad-gallery` passes.

## Likely touchpoints

- `makepad-components/src/switch.rs`
- `makepad-components/src/avatar.rs`
- `makepad-components/src/pagination.rs`
- `makepad-components/src/theme.rs`
- `makepad-gallery/src/ui/pagination_page.rs`
- `README.md`
