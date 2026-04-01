Task statement

Build `makepad-components` as the primary source of truth for highly customized but generic UI components, with `makepad-gallery` acting as a thin showcase layer rather than a second styling system.

Desired outcome

- Library components own reusable styling, state, sizing, and behavior.
- Gallery pages mostly compose and demonstrate library components.
- Preview- or gallery-specific naming and styling do not leak into the component crate unless preserved as compatibility aliases.
- Component internals prefer theme tokens and configurable live props over buried literals.

Known facts/evidence

- `makepad-components/src/button.rs` still exposes `ShadPreviewTab`, a gallery-shaped name that is not used anywhere in-repo.
- `makepad-components/src/avatar.rs` still hardcodes ring and presence-dot sizing/styling in a way that is less configurable than `ShadButton`.
- `makepad-components/src/pagination.rs` already has theme token support for pagination colors, but still mixes in hardcoded clear/border literals and a Rust-side fallback style object.
- Gallery has many page shells and showcase widgets, but the immediate library-facing cleanup is to improve reusable component APIs first.
- `cargo check -p makepad-components` and `cargo check -p makepad-gallery` are the practical fast validation path in this environment.
- Runtime visual validation is limited locally because direct gallery launch on this machine still fails earlier on Metal initialization.

Constraints

- Preserve backward compatibility where reasonable; prefer additive generic aliases over breaking renames.
- Keep work in the UI layer only.
- Use Makepad-native patterns from upstream widgets instead of inventing new DSL conventions.
- Ignore tests as the primary driver for this task; prioritize component API quality, themeability, and UI architecture.

Unknowns/open questions

- Which gallery-specific wrappers should eventually migrate into generic library primitives versus stay gallery-only as documentation scaffolding.
- Whether pagination should graduate to a first-class size/variant API instead of only runtime styling sync.
- Whether avatar sizing should eventually align to the same tokenized control-size system used by buttons and toggles.

Likely codebase touchpoints

- `makepad-components/src/button.rs`
- `makepad-components/src/avatar.rs`
- `makepad-components/src/pagination.rs`
- `makepad-components/src/theme.rs`
- `makepad-gallery/src/ui/*`
