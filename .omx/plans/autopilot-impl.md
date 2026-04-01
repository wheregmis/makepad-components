Autopilot Implementation Plan

1. Add a generic tab-trigger alias in the component library and preserve the existing preview-oriented name only as a compatibility shim.
2. Refactor `ShadAvatar` to expose configurable ring/presence metrics and clear-color usage through live properties instead of buried literals.
3. Reduce hardcoded pagination styling by routing clear/border sizing through resolved theme values and component-managed styling.
4. Run `cargo check -p makepad-components`.
5. Run `cargo check -p makepad-gallery`.
6. Record follow-up targets for the next component-first pass.
