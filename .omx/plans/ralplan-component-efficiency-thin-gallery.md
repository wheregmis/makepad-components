# RALPLAN: Component Efficiency + Thin Gallery

## Request

Create a plan for making `makepad-components` more efficient and making `makepad-gallery` a thin wrapper that demonstrates component usage rather than acting like a second UI system.

## Grounding

Context snapshot:
`/Users/wheregmis/Documents/GitHub/makepad-components/.omx/context/component-efficiency-thin-gallery-20260401T164300Z.md`

---

## RALPLAN-DR Summary

### Principles

1. **Measure before optimizing**: use Studio/runtime evidence to separate redraw problems from draw-path problems.
2. **Library owns behavior**: reusable visuals, sizing, and interaction state belong in `makepad-components`, not gallery pages.
3. **Gallery stays declarative**: gallery pages should mostly select, compose, and explain components, not restyle or re-implement them.
4. **Prefer composition over shader proliferation**: only keep custom shader paths that earn their complexity.
5. **Optimize hot clusters, not everything**: target heavy surfaces first (`table`, `command_palette`, code snippets, routed gallery views).

### Decision Drivers

1. **Runtime truth matters**: Studio already caught a live shader issue that `cargo check` missed.
2. **Architecture and performance are coupled**: gallery boundary leaks make it harder to measure and optimize components cleanly.
3. **Draw-path wins must be durable**: improvements should reduce repeated submission/state churn across both the gallery and downstream apps.

### Viable Options

#### Option A — Hotspot-first micro-optimization

Focus immediately on draw counters and patch individual hotspots (`switch`, `table`, code snippets, command palette) without changing gallery/component boundaries much.

- **Pros**
  - Fastest route to short-term counter drops
  - Lower coordination overhead up front
- **Cons**
  - Risks optimizing around the wrong ownership boundaries
  - Gallery may continue to hide component-level inefficiencies behind app-specific wrappers

#### Option B — Architecture-first thin-gallery pass, then optimize

First harden the library/gallery boundary, then run draw-path optimization on the resulting cleaner surface.

- **Pros**
  - Cleaner ownership and easier long-term maintenance
  - Performance work lands in reusable places
- **Cons**
  - Slower to first measured performance win
  - Risks spending time on structural cleanup that might not change hot counters

#### Option C — Hybrid (recommended)

Run a short measurement baseline first, then do a boundary pass and hotspot optimization in parallelized slices, with Studio validation after each slice.

- **Pros**
  - Avoids blind optimization
  - Keeps structural cleanup tied to measured hot areas
  - Best fit for both `$makepad-performance-draw` and `$makepad-ui-patterns`
- **Cons**
  - Requires discipline to preserve scope per slice
  - Needs consistent measurement + visual verification

---

## Architect Review

### Steelman antithesis

The strongest counterargument is that the gallery may not be the real performance problem at all: if redraw frequency or app-shell invalidation dominates, boundary cleanup and draw batching work may deliver only cosmetic counter improvements. A pure measure-first runtime pass could identify that faster.

### Real tradeoff tension

- **Tension**: architectural cleanliness vs immediate measurable wins
- If we refactor boundaries too early, we may delay counter reductions.
- If we optimize too locally, we may entrench gallery-only patterns that keep the library from being the source of truth.

### Synthesis

Use a **baseline-first hybrid plan**:
- first establish representative Studio measurements,
- then optimize and simplify only the clusters that are both **hot** and **architecturally misplaced**.

---

## Critic Evaluation

### Verdict

**APPROVE**

### Why approved

- alternatives were meaningfully considered
- the chosen option is tied to observable evidence
- verification is explicit and testable
- the plan keeps component/library boundaries and draw-path goals aligned

---

## Consensus Plan

### Phase 0 — Baseline and classification

Goal: identify where the cost actually is before changing architecture or batching paths.

Tasks:
- Run representative Studio sessions for:
  - gallery startup shell
  - Avatar page
  - Switch page
  - Pagination page
  - Command Palette open/search
  - Table page with realistic rows
- Record:
  - `draw_calls`
  - `instances`
  - `instance_bytes`
  - `uniform_bytes`
  - `texture_bytes`
- For each page/interaction, classify the dominant issue:
  - redraw-frequency issue
  - draw submission/state churn issue
  - text/snippet rendering issue
  - routing/layout shell issue

Exit criteria:
- a short baseline matrix exists for the above pages/interactions
- each target is tagged runtime-vs-draw dominant

### Phase 1 — Make the gallery thinner

Goal: ensure the gallery acts as a documentation consumer, not a second component system.

Tasks:
- Keep `makepad-gallery/src/ui/registry.rs` as the page metadata source of truth.
- Keep `makepad-gallery/src/ui/root.rs` focused on shell/routing/theme switching/command-palette hosting only.
- Audit `makepad-gallery/src/ui/page_macros.rs` so pages stay declarative and standardized.
- Audit `makepad-gallery/src/ui/themed_widgets.rs`:
  - keep clearly docs-only scaffolding (`GalleryCodeSnippet`, `GalleryActionFlow`, preview wrappers)
  - reject any drift toward component behavior duplication
- Audit page files for page-local styling/state that should instead be:
  - a library prop/variant
  - a reusable `Shad*` helper
  - or removed as gallery-specific duplication

Primary touchpoints:
- `makepad-gallery/src/ui/root.rs`
- `makepad-gallery/src/ui/registry.rs`
- `makepad-gallery/src/ui/page_macros.rs`
- `makepad-gallery/src/ui/themed_widgets.rs`
- representative page files under `makepad-gallery/src/ui/*`

Exit criteria:
- gallery files are mostly orchestration + examples
- component presentation logic stops leaking into page code

### Phase 2 — Component efficiency audit by cluster

Goal: improve reusable component efficiency in the highest-value clusters.

#### Cluster A: shader-backed controls

Audit:
- `makepad-components/src/switch.rs`
- `makepad-components/src/toggle.rs`
- `makepad-components/src/checkbox.rs`
- `makepad-components/src/button.rs`
- `makepad-components/src/avatar.rs`

Look for:
- redundant or overly custom shader logic
- duplicated state fields across similar controls
- shader paths that could fall back to simpler upstream patterns
- repeated `script_apply_eval!` writes that could be reduced

#### Cluster B: dynamic paged/list surfaces

Audit:
- `makepad-components/src/pagination.rs`
- `makepad-components/src/table.rs`
- `makepad-components/src/command_palette.rs`

Look for:
- repeated row/button restyling churn
- unnecessary per-draw mutation of child widgets
- opportunities to stabilize item templates and reduce updates
- virtualization correctness vs excess redraw/update work

#### Cluster C: gallery-only heavy surfaces

Audit:
- `makepad-gallery/src/ui/themed_widgets.rs`
- `CodeView`-backed snippet surfaces
- routed page containers in `makepad-gallery/src/ui/root.rs`

Look for:
- static snippet/code surfaces that should be isolated with `new_batch` or cached more effectively
- preview/code pages that force unnecessary large-subtree redraws
- shell-level layout causing broad damage when only one panel changes

Exit criteria:
- each cluster produces a ranked list of keep/simplify/cache/batch actions

### Phase 3 — Draw-path improvements

Goal: apply only the optimizations justified by measurement.

Allowed moves:
- batch repeated draws when the same draw type/state repeats
- reduce state churn that inflates `uniform_bytes`
- reduce repeated `script_apply_eval!` churn on hot surfaces
- introduce `new_batch` where redraw isolation helps
- use cached surfaces only for expensive mostly-static docs/preview regions
- simplify shader logic if a lighter composition path is good enough

Disallowed moves:
- broad ad hoc caching of large fast-changing views
- custom batching paths without evidence they fix the dominant metric
- gallery-only hacks that do not improve reusable component behavior

### Phase 4 — Verification and guardrails

Goal: prove improvements are real and keep the architecture from regressing.

Tasks:
- rerun the same Studio baseline interactions
- compare counters before/after
- re-run live screenshots for representative pages
- keep `cargo check` and component tests green
- document the ownership rule:
  - library owns component behavior and reusable styling
  - gallery owns docs shell, routing, snippets, and examples only

Exit criteria:
- before/after counters and screenshots are captured
- no regressions in component API behavior

---

## Concrete Workstreams / File Clusters

### Workstream 1 — Measurement + proof

- Studio remote runs and screenshots
- baseline metrics notes / perf log

### Workstream 2 — Thin gallery architecture

- `makepad-gallery/src/ui/root.rs`
- `makepad-gallery/src/ui/registry.rs`
- `makepad-gallery/src/ui/page_macros.rs`
- `makepad-gallery/src/ui/themed_widgets.rs`
- selected `makepad-gallery/src/ui/*_page.rs`

### Workstream 3 — Control/shader simplification

- `makepad-components/src/switch.rs`
- `makepad-components/src/toggle.rs`
- `makepad-components/src/checkbox.rs`
- `makepad-components/src/button.rs`
- `makepad-components/src/avatar.rs`

### Workstream 4 — Dynamic list/paging performance

- `makepad-components/src/pagination.rs`
- `makepad-components/src/table.rs`
- `makepad-components/src/command_palette.rs`

### Workstream 5 — Docs/guardrails

- `README.md`
- gallery page/snippet examples where API ownership must be clarified

---

## Verification Strategy

### Build / code correctness

- `cargo check -p makepad-components`
- `cargo check -p makepad-gallery`
- `cargo test -p makepad-components --lib`

### Studio runtime verification

Run the gallery through Studio remote and verify:
- startup shell loads cleanly
- Switch, Avatar, Pagination, Command Palette, and Table pages render correctly
- screenshots show no color/layout regressions

### Draw-performance verification

For each representative interaction:
- capture Studio counters before change
- capture counters after change
- compare:
  - `draw_calls`
  - `instances`
  - `instance_bytes`
  - `uniform_bytes`
  - `texture_bytes`

### Decision rule

- if counters improve but frame smoothness does not, the next pass should shift to `$makepad-performance-runtime`
- if architecture improves but counters do not, do not keep adding batching complexity without new evidence

---

## ADR

### Decision

Adopt a **baseline-first hybrid plan**: measure first, then thin the gallery and optimize the hottest component/draw-path clusters in parallelized slices.

### Drivers

- Studio/runtime evidence catches issues static checks miss
- library-first ownership is already the repo direction
- draw optimization is only worth doing where counters justify it

### Alternatives considered

- hotspot-first optimization only
- architecture-first cleanup only

### Why chosen

It preserves evidence-first performance work while ensuring improvements land in reusable component boundaries instead of gallery-local scaffolding.

### Consequences

- requires a measurement discipline before each major optimization slice
- likely produces fewer but higher-confidence performance changes
- keeps the gallery simpler over time

### Follow-ups

- add a lightweight performance note or baseline log for key gallery interactions
- add architecture guidance to keep gallery pages thin

---

## Available-agent-types roster

Recommended execution roster:
- `executor`
- `architect`
- `critic`
- `performance-reviewer`
- `test-engineer`
- `verifier`
- `designer`
- `code-reviewer`

Optional support:
- `explore`
- `researcher`

---

## Follow-up staffing guidance

### If executed via `$ralph`

Recommended lane sequence:
1. **executor** — implementation lane, **high**
2. **performance-reviewer** — counter interpretation / hotspot review, **medium**
3. **architect** — boundary sanity + reuse review, **high**
4. **test-engineer** or **verifier** — proof lane, **medium**

Suggested launch:

```text
$ralph implement the baseline-first hybrid plan for component efficiency and thin-gallery architecture in makepad-components
```

Use Ralph when:
- you want one tightly controlled sequential owner
- you expect a few larger integrated refactor slices
- you want repeated verification after each slice

### If executed via `$team`

Recommended practical staffing:
- **Lane 1: gallery-boundary cleanup** — executor, medium/high
- **Lane 2: component hotspot optimization** — executor, high
- **Lane 3: verification + Studio/perf evidence** — executor or test-engineer-style lane, medium

Because current `omx team` uses one worker role per launch, use executor workers with role-specific task assignments from the leader.

Suggested launch:

```text
omx team 3:executor "Execute the baseline-first hybrid plan for component efficiency and thin-gallery architecture in makepad-components; split work into gallery-boundary cleanup, component hotspot optimization, and Studio/perf verification lanes."
```

or

```text
$team 3:executor "Execute the baseline-first hybrid plan for component efficiency and thin-gallery architecture in makepad-components; split work into gallery-boundary cleanup, component hotspot optimization, and Studio/perf verification lanes."
```

### Team verification path

The verification lane must own:
- Studio remote run of the gallery
- screenshot spot checks of representative pages
- metric comparison for representative interactions
- `cargo check -p makepad-components`
- `cargo check -p makepad-gallery`
- `cargo test -p makepad-components --lib`

The leader should not close the run until:
- gallery-boundary tasks are complete
- hotspot tasks are complete
- verification evidence is recorded and green
