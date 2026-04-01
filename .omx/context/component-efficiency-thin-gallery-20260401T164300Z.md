Task statement

Create a consensus plan for making `makepad-components` more efficient and making `makepad-gallery` a thin wrapper that demonstrates component usage rather than acting like a second UI system.

Desired outcome

- Components own reusable visuals, sizing, and interaction behavior.
- Gallery pages mostly compose and demonstrate `Shad*` components.
- Draw-path work focuses on real submission/render costs, not premature micro-optimization.
- The plan includes a practical execution path for both library architecture and rendering efficiency.

Known facts / evidence

- The gallery shell is centralized in `makepad-gallery/src/ui/root.rs`, with route/page registration generated from `makepad-gallery/src/ui/registry.rs`.
- Gallery page generation is heavily standardized through `makepad-gallery/src/ui/page_macros.rs`, which is a strong seam for keeping gallery pages thin.
- Gallery-specific helper widgets still exist in `makepad-gallery/src/ui/themed_widgets.rs`, especially `GalleryCodeSnippet`, `GalleryActionFlow`, and preview wrappers.
- The gallery command palette is app-owned in `makepad-gallery/src/ui/command_palette.rs`, backed by catalog metadata rather than component-owned behavior.
- The component crate contains many custom shader-backed widgets (`pixel: fn()`) including `button`, `checkbox`, `accordion`, `progress`, `spinner`, `avatar`, `toggle`, `switch`, `popover`, `skeleton`, and others.
- The component crate also has several dynamic `script_apply_eval!` paths (`pagination`, `table`, `command_palette`, `alert`, `sheet`, `select`, `badge`, `surface`, `dialog`, etc.), which are likely hotspots for repeated UI mutation churn.
- Large or potentially draw-heavy surfaces in the repo include:
  - `makepad-components/src/table.rs` using `PortalList`
  - `makepad-components/src/command_palette.rs` using `PortalList`
  - `makepad-gallery/src/ui/themed_widgets.rs` using `CodeView` for snippets
  - `makepad-gallery/src/ui/root.rs` using `RouterWidget` for page routing
- Studio runtime validation already proved valuable: it caught a live-only `ShadSwitch` shader issue that `cargo check` did not catch.
- The Makepad performance guidance says to distinguish redraw-frequency issues from draw submission issues, and to prioritize batching, text reuse, and correct cache boundaries (`new_batch` vs `CachedView`) only after identifying the dominant cost.

Constraints

- Keep the plan in the UI/rendering layer; no platform/runtime-internals detour.
- Preserve the library-first direction already established in the repo.
- Favor composition and reuse before introducing new custom widgets or shaders.
- Use Studio-based measurement/visual verification as part of the plan.
- Avoid turning the gallery into another app framework; it should stay a documentation/demo consumer.

Unknowns / open questions

- Which gallery-only wrappers should remain docs scaffolding vs be promoted into reusable library primitives.
- Which current costs are redraw-driven vs draw-submission-driven in the live gallery.
- Whether some shader-backed components should stay custom or be simplified back toward upstream Makepad widget patterns.
- Whether snippet/code-view rendering is a major gallery-only performance cost worth isolating behind different cache boundaries.

Likely touchpoints

- `makepad-components/src/switch.rs`
- `makepad-components/src/toggle.rs`
- `makepad-components/src/avatar.rs`
- `makepad-components/src/pagination.rs`
- `makepad-components/src/table.rs`
- `makepad-components/src/command_palette.rs`
- `makepad-gallery/src/ui/root.rs`
- `makepad-gallery/src/ui/registry.rs`
- `makepad-gallery/src/ui/page_macros.rs`
- `makepad-gallery/src/ui/themed_widgets.rs`
- Studio remote profiling / screenshots
