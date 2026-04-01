Autopilot Spec: Generic Component Library, Thin Gallery

Goal

Turn `makepad-components` into the durable UI system layer and keep `makepad-gallery` as a showcase consumer of that layer.

Requirements

- Reusable components expose generic names, variants, and live properties.
- Theme roles, sizing, and interaction styling live in the library, not ad hoc in gallery pages.
- Gallery-only concepts should not become canonical component names.
- Components should prefer tokenized and configurable values over hidden literals.
- Runtime styling logic should remain lightweight and consistent with Makepad widget patterns.

Technical interpretation

- Treat preview/demo names in the library as API smells.
- Prefer additive API evolution:
  - introduce generic aliases
  - keep compatibility aliases when they already exist publicly
- Follow the `ShadButton` pattern for configurable size metrics when a component currently bakes those values into Rust.
- Where a component already has theme tokens, finish the tokenization before inventing new Rust-side style defaults.

Current execution focus

1. Clean up preview-specific naming in the component crate.
2. Make avatar presentation more configurable and token-driven.
3. Tighten pagination styling so it relies more cleanly on theme/stateful component behavior.
