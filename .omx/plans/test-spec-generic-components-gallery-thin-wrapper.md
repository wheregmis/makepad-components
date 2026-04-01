# Test Spec: Generic Components, Thin Gallery

## Scope

Validate that the next library-finish slice keeps `makepad-components` generic, keeps `makepad-gallery` thin, and leaves the workspace buildable.

## Assertions

### A. Switch component surface

- `makepad-components/src/switch.rs` defines `ShadSwitch` as a library-owned reusable component.
- The size API is explicit and uses `ShadControlSize`.
- No gallery-framed comments or API wording remain in the canonical switch surface.

### B. Public docs match code

- `README.md` does not mention removed avatar helpers:
  - `ShadAvatarSm`
  - `ShadAvatarLg`
  - `ShadAvatarStatusOnline`
  - `ShadAvatarStatusAway`
  - `ShadAvatarStatusBusy`
- `README.md` documents avatar usage in terms of:
  - `ShadAvatar`
  - `ShadAvatarFallback`
  - `ShadAvatarImage`
  - `ShadAvatarStatus`
  - `ShadAvatarPresence`
- `README.md` refers to the gallery package as `makepad-gallery`.
- `README.md` mentions pagination sizing through `ShadControlSize`.

### C. Build verification

- `cargo check -p makepad-components`
- `cargo check -p makepad-gallery`

### D. Diagnostics

- Affected files remain free of obvious structural/type issues after the slice is integrated.

## Verification commands

```bash
rg -n "ShadAvatarSm|ShadAvatarLg|ShadAvatarStatusOnline|ShadAvatarStatusAway|ShadAvatarStatusBusy|makepad-example-component-gallery" README.md
cargo check -p makepad-components
cargo check -p makepad-gallery
```

## Completion evidence required

- Zero matches from the stale-API `rg` check above.
- Successful `cargo check` output for both packages.
- Final diff summary tied back to the acceptance criteria.
