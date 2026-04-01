# Baseline: Component Efficiency + Thin Gallery

Date: 2026-04-01

## Scope

Representative gallery/runtime targets selected from the approved plan:

- gallery startup shell
- Avatar page
- Switch page
- Pagination page
- Command Palette open/search
- Table page

## Runtime observations

### Startup shell

- Fresh Studio rerun on current leader state succeeds.
- The gallery shell renders correctly after restoring the runtime-safe `root.rs` route expansion shape.
- The shell remains visually correct with the thin-gallery cleanup already integrated.

### Avatar

- Avatar page rendered successfully in Studio during the earlier live verification pass.
- No obvious color/layout regressions were observed after the avatar API generalization work.

### Switch

- Switch rendering previously failed only at runtime due to shader IO declarations.
- That runtime issue was fixed and the live switch now renders correctly again in Studio.

### Command Palette

- The command palette is a likely high-value hotspot because it drives:
  - a modal overlay
  - a `PortalList`
  - per-row header/title/shortcut updates
  - search summary and clear/close label churn
- Current local optimization:
  - cache the visible summary label text
  - cache the secondary-action button label text
  - avoid rewriting those strings when they have not changed

### Gallery snippet / docs shell

- Static docs surfaces are likely over-invalidated when preview interactions change.
- Current local optimization:
  - `new_batch` on `code_page`
  - `new_batch` on `action_flow`
- Goal:
  - isolate static docs/presentation redraw from live preview interaction where possible

## Counter extraction path

Counter path identified from team support lane:

- query issuance:
  - `makepad/studio/desktop/src/app_tabs.rs`
- results handling:
  - `makepad/studio/desktop/src/app_messages.rs`
- counter rendering:
  - `makepad/studio/desktop/src/desktop_profiler_view.rs`
- sample payload:
  - `makepad/platform/studio/src/hub_protocol.rs` (`GPUSample`)
- metric collection:
  - `makepad/platform/src/draw_list.rs`

Relevant fields:

- `draw_calls`
- `instances`
- `instance_bytes`
- `uniform_bytes`
- `texture_bytes`

## Ranked hotspot map

Based on component structure and team support evidence:

1. **Table**
   - many visible row/cell draws
   - glyph/background churn
   - virtualization and width-sync behavior matter
2. **Command Palette**
   - overlay list churn on open/query changes
   - row content/state updates
3. **Avatar**
   - image/status/ring/fallback layering
   - possible texture churn
4. **Pagination**
   - repeated small control updates via `script_apply_eval!`
5. **Switch / Toggle / Checkbox**
   - lowest individual priority after the runtime shader fix

## Current implemented slices

### Thin-gallery architecture

- integrated worker-produced cleanup to keep wrapper modules dependent on registry projections only
- avoided the more aggressive `root.rs` macro rewrite because Studio runtime rejected it

### Efficiency slice

- `makepad-components/src/command_palette.rs`
  - reduced unnecessary text writes for summary / close button labels
- `makepad-gallery/src/ui/themed_widgets.rs`
  - isolated static code/action-flow regions with `new_batch`

## Verification on current leader state

- `cargo check -p makepad-components` ✅
- `cargo check -p makepad-gallery` ✅
- `cargo test -p makepad-components --lib` ✅
- Studio rerun / screenshot on current leader state ✅

## Known gap

- No durable numeric before/after QueryProfiler capture was produced by the team lane.
- Next local performance slice should either:
  - collect numeric profiler samples for command palette / pagination / table, or
  - implement one more clearly justified hotspot optimization with accompanying Studio evidence.
