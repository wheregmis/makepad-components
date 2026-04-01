# PRD: Component Efficiency and Thin Gallery

## Problem

`makepad-components` is becoming the reusable UI system, but the current gallery still carries enough shell/scaffolding weight that it can obscure where real component inefficiencies live. At the same time, some components likely have draw-path or mutation churn that should be simplified or isolated only after measurement.

## Product goal

Make `makepad-components` the clear source of truth for reusable behavior and visuals while making `makepad-gallery` a thin documentation/demo consumer. Improve rendering efficiency in the highest-value hotspots without premature or architecture-breaking micro-optimizations.

## Desired outcome

- Gallery pages mostly compose and demonstrate `Shad*` components.
- Component behavior and styling stop leaking into gallery-only page logic.
- Draw-path optimizations are tied to Studio evidence.
- Expensive component and gallery surfaces are simplified, batched, or isolated only where measurement justifies it.

## Scope

### In scope

1. Establish a Studio baseline for key pages/interactions:
   - startup shell
   - Avatar
   - Switch
   - Pagination
   - Command Palette
   - Table
2. Thin the gallery architecture:
   - preserve `registry.rs` as the source of truth
   - keep `root.rs` shell/routing focused
   - keep `page_macros.rs` declarative
   - reduce gallery-local behavior duplication
3. Audit and improve component efficiency in hotspot clusters:
   - shader-backed controls
   - dynamic list/paging surfaces
   - gallery-only heavy snippet/preview surfaces
4. Re-verify with Studio screenshots/counters plus build/test evidence.

### Out of scope

- Platform/runtime internals outside the UI/render path
- WASM/browser optimization as the primary target
- Large feature additions unrelated to efficiency or thin-gallery architecture
- Unmeasured speculative batching work

## Acceptance criteria

1. A baseline matrix exists for the representative Studio pages/interactions.
2. `makepad-gallery` is measurably thinner in responsibility: page/example code no longer owns reusable component behavior/styling.
3. At least one validated hotspot cluster is simplified or optimized based on measured evidence.
4. Representative pages still render correctly in Studio after the changes.
5. `cargo check -p makepad-components` passes.
6. `cargo check -p makepad-gallery` passes.
7. `cargo test -p makepad-components --lib` passes.

## Primary touchpoints

- `makepad-gallery/src/ui/root.rs`
- `makepad-gallery/src/ui/registry.rs`
- `makepad-gallery/src/ui/page_macros.rs`
- `makepad-gallery/src/ui/themed_widgets.rs`
- `makepad-components/src/switch.rs`
- `makepad-components/src/toggle.rs`
- `makepad-components/src/checkbox.rs`
- `makepad-components/src/avatar.rs`
- `makepad-components/src/pagination.rs`
- `makepad-components/src/table.rs`
- `makepad-components/src/command_palette.rs`

## Risks

- Studio counters may show redraw-frequency issues instead of draw submission issues, requiring scope adjustment.
- Gallery cleanup could accidentally remove useful docs scaffolding if page-vs-component boundaries are overcorrected.
- Shader simplifications may fix counters but regress interaction/visual fidelity if not visually rechecked in Studio.
