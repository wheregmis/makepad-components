# makepad-components

Reusable UI components for [Makepad](https://github.com/makepad/makepad), built with the v2 `script_mod!` workflow.

This workspace contains:
- A component library crate (`makepad-components`)
- A reusable icon crate (`makepad-icon`)
- A runnable gallery app (`makepad-example-component-gallery`)

## What You Get

- **Buttons**: shadcn-inspired variants (`default`, `destructive`, `outline`, `secondary`, `ghost`, `link`) plus size presets.
- **Aspect Ratio**: ratio-constrained container for media/content layouts (`ShadAspectRatio`).
- **Accordion**: a composable accordion item widget with open-state and animation helpers (`set_open`, `is_open`, `open_changed`, `animation_progress`).
- **Alerts**: shadcn-inspired alert layouts with default and destructive variants.
- **Dialog**: modal with variants — generic (custom body), alert (title + Cancel/Confirm), destructive; `set_open(bool)` / `is_open()` plus `open_changed(actions)`.
- **Menubar**: compact application menu primitives built on `ShadPopover`, with styled triggers, menu surfaces, separators, and item rows.
- **Navigation Menu**: wide flyout navigation primitives for docs/marketing headers, also built on `ShadPopover` for anchored open/close behavior.
- **Pagination**: stateful page navigator with numbered slots, previous/next controls, compact ellipsis ranges, and `changed(actions)` / `set_page(...)` helpers.
- **Calendar**: single-date calendar widget with controlled month navigation and `set_value(...)` / `value()` / `changed(actions)` helpers.
- **Date Picker**: field-like calendar picker composed from `ShadPopover` + `ShadCalendar`, with `set_value(...)`, `clear()`, `set_open(...)`, and `open_changed(actions)`.
- **Chart**: themed wrappers over Makepad's line, area, and bar chart widgets that accept typed Rust `DataPoint` series.
- **Table**: styled app-owned table shell with typed row click / selection actions and external row/header control.
- **Popover**: anchored overlay with configurable side/alignment, auto-flip when space is tight, outside-click dismissal, popup-content access through `content_widget()`, and `open_changed(actions)`.
- **Theme tokens**: centralized `shad_theme` color + radius tokens in script space, with built-in `light` and `dark` variants.
- **Icons**: SVG-based icon widgets (`IconCheck`, `IconX`, `IconSearch`).
- **Kbd**: keyboard shortcut key caps (`ShadKbd`, `ShadKbdLabel`, `ShadKbdSeparator`) for displaying shortcuts (e.g. ⌘ ⇧ ⌥ ⌃ or Ctrl + B).
- **Gallery app**: a live catalog demonstrating component usage and styling.

## Workspace Layout

- `makepad-components/` → `makepad-components` library
- `makepad-icon/` → `makepad-icon` library
- `makepad-gallery/` → `makepad-example-component-gallery` app
- `.github/workflows/wasm-pages.yml` → GitHub Pages WASM build + deploy

## Architecture & naming conventions

- **Crate roles**:
  - `components`: owns reusable `Shad*` UI widgets and the shared `shad_theme` design tokens.
  - `makepad-icon`: provides icon widgets that can be used by both the component library and apps.
  - `gallery`: owns the documentation/gallery UI that showcases components.
- **Naming**:
  - **`Shad*` widgets** live in the `components` crate and are intended for reuse in any Makepad app (for example `ShadButton`, `ShadAccordionItem`, `ShadSidebar`).
  - **`Gallery*` widgets** live in the `gallery` crate and are only for the docs/gallery experience (for example the code snippet widget). Layout wrappers and preview panels use `Shad*` components from the components crate.
- **File placement**:
  - New reusable components belong under `makepad-components/src/*.rs` and should be registered from `makepad_components::script_mod(vm)` into the `mod.widgets.*` namespace.
  - Gallery-only layout and helper widgets belong under `makepad-gallery/src/ui/*.rs` (for example `themed_widgets.rs`) and are registered from the gallery UI module.
  - `makepad-gallery/src/ui/catalog.rs` is the gallery metadata source of truth for sidebar labels, command-palette entries, route ids/paths, and snippet keys. `root.rs` stays as the explicit router adapter.

## Prerequisites

- Rust stable toolchain
- A working Makepad-compatible environment (system deps vary by OS)
- For WASM builds/deploy: `cargo-makepad`

## Quick Start

### 1) Build everything

```bash
cargo check --workspace
```

### 2) Run the gallery app (native)

```bash
cargo run -p makepad-example-component-gallery --release
```

### 3) Build gallery for web (WASM)

```bash
cargo install --git https://github.com/makepad/makepad.git --branch dev cargo-makepad --locked
cargo makepad wasm install-toolchain
./scripts/build_wasm.sh -p makepad-example-component-gallery --profile small --release --bindgen
```

Expected output directory:

```text
target/makepad-wasm-app/release/makepad-example-component-gallery
```

## Using `makepad-components` in Your App

Add dependencies:

```toml
[dependencies]
makepad-widgets = { git = "https://github.com/makepad/makepad.git", branch = "dev", version = "2.0.0" }
makepad-components = { path = "../components", version = "1.0.0" }
```

Register script modules in `App::run`:

```rust
impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        crate::makepad_widgets::script_mod(vm);
        makepad_components::script_mod(vm);
        App::from_script_mod(vm, self::script_mod)
    }
}
```

Import runtime helpers from the stable module surface:

```rust
use makepad_components::prelude::*;
use makepad_components::calendar::ShadDate;
use makepad_components::dialog::ShadDialogWidgetExt;
use makepad_components::table::ShadTableWidgetExt;
```

Use components in `script_mod!`:

```rust
script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                body +: {
                    flow: Down
                    spacing: 10

                    ShadButton{text: "Default"}
                    ShadButtonOutline{text: "Outline"}

                    ShadAccordion{
                        item_one := ShadAccordionItem{
                            title: "Section 1"
                            body: View{
                                Label{text: "Accordion content"}
                            }
                        }
                    }
                }
            }
        }
    }
}
```

## Migration Notes

- Use `makepad_components::prelude` for shared Makepad types, then import component-specific refs/actions/widget-ext traits from their module paths such as `makepad_components::dialog::*` or `makepad_components::table::*`.
- Overlay widgets expose `open_changed(actions) -> Option<bool>` as the lifecycle helper.
- Expandable widgets expose `open_changed(actions)` and `animation_progress(actions)` as the lifecycle helpers.

## Available Components

### Component Checklist

- [x] Accordion
- [x] Alert
- [x] Aspect Ratio
- [x] Avatar
- [x] Badge
- [x] Breadcrumb
- [x] Button
- [x] Button Group
- [x] Calendar
- [x] Card
- [x] Carousel
- [x] Chart
- [x] Checkbox
- [x] Collapsible
- [x] Command
- [x] Context Menu
- [x] Date Picker
- [x] Dialog
- [x] Input & Field
- [x] Input OTP
- [x] Kbd
- [x] Label
- [x] Menubar
- [x] Navigation Menu
- [x] Pagination
- [x] Popover
- [x] Progress
- [x] Radio Group
- [x] Resizable
- [x] Scroll Area
- [x] Select
- [x] Separator
- [x] Sheet
- [x] Sidebar
- [x] Skeleton
- [x] Slider
- [x] Sonner
- [x] Spinner
- [x] Switch
- [x] Table
- [x] Tabs
- [x] Textarea
- [x] Toggle & Toggle Group

**Known limitation:** In the gallery app, popup-style selectors (`ShadSelect` and the base `DropDown` widget) may not open reliably on click; this appears to be an interaction between the gallery’s layout (flow + `PageFlip`) and the platform’s hit/sweep handling. For working popup interaction examples, see the splash app (`splash_app.md` or the makepad repo’s splash example), which uses a Dock-based layout.

### Buttons (`makepad-components/src/button.rs`)

- `ShadButton`
- `ShadButtonDestructive`
- `ShadButtonOutline`
- `ShadButtonSecondary`
- `ShadButtonGhost`
- `ShadButtonLink`
- `ShadButtonSm`
- `ShadButtonLg`
- `ShadButtonIcon`

### Aspect Ratio (`makepad-components/src/aspect_ratio.rs`)

- `ShadAspectRatio`

### Accordion (`makepad-components/src/accordion.rs`)

- `ShadAccordion` (container `View` preset)
- `ShadAccordionItem` (custom widget)

`ShadAccordionItem` supports:
- `is_open` live field
- script calls: `set_open(bool)` and `is_open() -> bool`
- `open_changed(actions) -> Option<bool>`
- `animation_progress(actions) -> Option<f64>`

### Alerts (`makepad-components/src/alert.rs`)

- `ShadAlert`
- `ShadAlertIcon`
- `ShadAlertContent`
- `ShadAlertTitle`
- `ShadAlertDescription`
- `ShadAlertDestructive`
- `ShadAlertDestructiveIcon`
- `ShadAlertDestructiveTitle`

### Input & Field (`makepad-components/src/input.rs`)

- `ShadInput`
- `ShadInputWithIcon`
- `ShadField`
- `ShadFieldLabel`
- `ShadFieldDescription`
- `ShadFieldMessage`

Use `ShadField` as a layout wrapper around `ShadInput` or another form control. Validation/state stays in app code.

### Calendar (`makepad-components/src/calendar.rs`)

- `ShadCalendar`
- `ShadDate`

`ShadCalendar` supports:
- `set_value(cx, Option<ShadDate>)`
- `clear(cx)`
- `value() -> Option<ShadDate>`
- `set_month(cx, year, month)` plus `prev_month(cx)` / `next_month(cx)`
- `changed(actions) -> Option<ShadDate>`

### Chart (`makepad-components/src/chart.rs`)

- `ShadLineChart`
- `ShadAreaChart`
- `ShadBarChart`
- `DataPoint`

The `Shad*Chart` script types style Makepad's built-in chart widgets. Use the underlying Rust widget types (`LineChart`, `AreaChart`, `BarChart`) to push `Vec<DataPoint>` datasets with `set_data(...)`.

### Date Picker (`makepad-components/src/date_picker.rs`)

- `ShadDatePicker`

`ShadDatePicker` supports:
- `set_value(cx, Option<ShadDate>)`
- `clear(cx)`
- `value() -> Option<ShadDate>`
- `set_open(cx, bool)` / `is_open() -> bool`
- `changed(actions) -> Option<ShadDate>`
- `open_changed(actions) -> Option<bool>`

### Menubar (`makepad-components/src/menubar.rs`)

- `ShadMenubar`
- `ShadMenubarMenu`
- `ShadMenubarTrigger`
- `ShadMenubarContent`
- `ShadMenubarLabel`
- `ShadMenubarHint`
- `ShadMenubarItem`
- `ShadMenubarSeparator`

`ShadMenubarMenu` reuses `ShadPopover` under the hood. Menus open on hover, sibling menus close as the pointer moves across the menubar, and you can still query or close a menu with the normal popover widget ref helpers after an item click.

### Navigation Menu (`makepad-components/src/navigation_menu.rs`)

- `ShadNavigationMenu`
- `ShadNavigationMenuList`
- `ShadNavigationMenuItem`
- `ShadNavigationMenuTrigger`
- `ShadNavigationMenuContent`
- `ShadNavigationMenuSectionLabel`
- `ShadNavigationMenuCallout`
- `ShadNavigationMenuPanel`

`ShadNavigationMenuItem` also reuses `ShadPopover`, but defaults to a wider content surface for grouped links, feature callouts, and site navigation flyouts. Navigation flyouts open on hover and close sibling menus as the pointer moves across the trigger row.

### Table (`makepad-components/src/table.rs`)

- `ShadTable`

`ShadTable` supports:
- `set_headers(cx, Vec<String>)`
- `set_rows(cx, Vec<Vec<String>>)`
- `set_selected_row(cx, Option<usize>)`
- `selected_row() -> Option<usize>`
- `row_clicked(actions) -> Option<usize>`
- `selection_changed(actions) -> Option<usize>`

### Separator (`makepad-components/src/hr.rs`)

- `ShadSeparator`

Compatibility alias: `ShadHr`.

### Scroll Area (`makepad-components/src/scroll.rs`)

- `ShadScrollArea` — vertical scroll container
- `ShadScrollAreaX` — horizontal scroll container
- `ShadScrollAreaXY` — two-axis scroll container

Compatibility alias: `ShadScrollYView`.

### Radio Group (`makepad-components/src/radio_group.rs`)

- `ShadRadioGroup`
- `ShadRadioGroupInline`
- `ShadRadioItem`

Use Makepad `radio_button_set(...).selected(cx, actions)` to keep a single item active.

### Select (`makepad-components/src/select.rs`)

- `ShadSelect`
- `ShadSelectItem`

Single-select, non-searchable dropdown built on the popup menu stack.

### Dialog (`makepad-components/src/dialog.rs`)

- `ShadDialog` — generic modal with customizable `body` content (closes on backdrop/Escape; wire your own Close button)
- `ShadDialogAlert` — preset with title, description, Cancel/Continue (closes on Cancel, Confirm, or backdrop)
- `ShadDialogAlertDestructive` — same layout with destructive Confirm (e.g. Delete)

Props: `open` (bool). For generic: put content in `overlay +: { content +: { body +: { ... } } }`. For alert variants: customize `title_label`, `description_label` in the template.

Script API: `set_open(bool)` and `is_open() -> bool`.
Action API: `open_changed(actions) -> Option<bool>`.

### Progress (`makepad-components/src/progress.rs`)

- `ShadProgress` — default 50%
- `ShadProgress33`, `ShadProgress66`, `ShadProgressFull` — 33%, 66%, 100%
- `ShadProgressIndeterminate` — animated loading bar (continuous sweep)

Use `ShadProgress66{}` for 66%. For custom values, extend `ShadProgressBase` with `draw_bg +: { progress: instance(0.42) }`.

### Slider (`makepad-components/src/slider.rs`)

- `ShadSlider` — shadcn-style range slider (extends makepad SliderRoundFlat)

Props: `default`, `min`, `max`, `step`. Uses makepad Slider actions for value changes.

### Tabs (`makepad-components/src/tabs.rs`)

- `ShadTabs`
- `ShadTabsList`
- `ShadTabsTrigger`
- `ShadTabsContent`

This wave ships the canonical styling primitives; pair them with `PageFlip` or another app-level state holder.

### Sheet (`makepad-components/src/sheet.rs`)

- `ShadSheet`
- `ShadSheetTitle`
- `ShadSheetDescription`

Props: `open` (bool). Script API: `set_open(bool)` and `is_open() -> bool`.
Action API: `open_changed(actions) -> Option<bool>`.

### Resizable (`makepad-components/src/resizable.rs`)

- `ShadResizable`

Thin wrapper over Makepad `Splitter` for two-pane layouts.

### Sonner / Toast (`makepad-components/src/sonner.rs`)

- `ShadToast` — toast notification card container
- `ShadToastTitle` — title text
- `ShadToastDescription` — optional description (uses `ShadAlertDescription` styling)

Use: `ShadToast{ title := ShadToastTitle{text: "Event created"} }` or add `description := ShadToastDescription{text: "..."}`.
Action API: `open_changed(actions) -> Option<bool>`.

### Spinner (`makepad-components/src/spinner.rs`)

- `ShadSpinner` — circular loading indicator (24×24, animated arc)

Use for async operations and loading states.

### Kbd (`makepad-components/src/kbd.rs`)

- `ShadKbd` — key cap container (dark grey rounded rect, subtle border)
- `ShadKbdLabel` — text/symbol inside a key (e.g. ⌘, ⇧, Ctrl, B)
- `ShadKbdSeparator` — " + " between keys in a shortcut

Use with a horizontal layout: `ShadKbd{ label := ShadKbdLabel{text: "Ctrl"} }` and `ShadKbdSeparator{}` for shortcuts like "Ctrl + B".

### Sidebar (`makepad-components/src/sidebar.rs`)

- `ShadSidebar`
- `ShadSidebarSectionLabel`
- `ShadSidebarItem`

### Theme (`makepad-components/src/theme.rs`)

Exports `mod.widgets.shad_themes.light`, `mod.widgets.shad_themes.dark`, and the active `mod.widgets.shad_theme` object with tokens such as:
- `color_primary`, `color_secondary`, `color_background`
- `color_destructive*`, `color_outline_border*`
- `color_muted*`, `color_ghost*`
- `radius`

Switch themes by reassigning `mod.widgets.shad_theme`:

```rust
script_eval!(cx, {
    if mod.state.is_light_theme {
        mod.state.is_light_theme = false
        mod.widgets.shad_theme = mod.widgets.shad_themes.dark
    }
    else {
        mod.state.is_light_theme = true
        mod.widgets.shad_theme = mod.widgets.shad_themes.light
    }
});
```

## Using `makepad-icon`

Add dependency:

```toml
[dependencies]
makepad-icon = { path = "../makepad-icon", version = "1.0.0" }
```

Register and use:

```rust
makepad_icon::script_mod(vm);
```

```rust
script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                body +: {
                    flow: Right
                    spacing: 8
                    IconCheck{}
                    IconX{}
                    IconSearch{}
                }
            }
        }
    }
}
```

Backed by SVG assets in:
- `makepad-icon/resources/icons/check.svg`
- `makepad-icon/resources/icons/x.svg`
- `makepad-icon/resources/icons/search.svg`

## Gallery App

The gallery (`makepad-gallery/src/main.rs`) includes:
- Sidebar navigation between component pages
- Accordion showcase with custom header/body content
- Button variant and size matrix
- Aspect ratio examples (16:9, 1:1, 4:3, 9:16)
- Alert default/destructive examples
- Icon preview section

Run it to validate behavior and styling changes quickly.

### Adding a new component + docs page

- **Library component (`Shad*`)**
  - Add or extend a module under `makepad-components/src/` (for example `button.rs`, `accordion.rs`).
  - Register the widget in `makepad_components::script_mod(vm)` with a `Shad*` name in the `mod.widgets.*` namespace.
  - Use `shad_theme` tokens for colors, radii, and spacing instead of hardcoded values.
- **Gallery docs page (`Gallery*`)**
  - Create a new page script under `makepad-gallery/src/ui/` that uses `ShadScrollYView`, `ShadPageTitle`, and `ShadPageSubtitle`.
  - Add a snippet constant to `makepad-gallery/src/ui/snippets.rs` and reference it from `GalleryCodeSnippet` on the page.
  - Add a `ShadSidebarItem` entry in `GallerySidebar` and a matching page in `GalleryContentFlip`.
  - Wire the sidebar item to the page in `makepad-gallery/src/main.rs` using a `set_page` call in `handle_actions`.

## CI/CD

GitHub Actions workflow: `.github/workflows/wasm-pages.yml`

For clean-path deep links on GitHub Pages, deploy both `index.html` and a copy of it as `404.html`. The Pages workflow in this repo patches `index.html` for subpath hosting and then reuses that patched app shell as `404.html` so refreshes on routes like `/scroll-area` still bootstrap the app.

On pushes to `main`/`master` (or manual dispatch), it:
- Installs Rust + system dependencies
- Installs `cargo-makepad`
- Builds selected package to WASM using profile `small`
- Publishes output to GitHub Pages

Manual dispatch supports an optional `package` input (defaults to `makepad-example-component-gallery`).

## Build Profiles

Workspace defines a size-optimized profile:

- `[profile.small]` in root `Cargo.toml`
- `opt-level = 'z'`
- `lto = true`
- `codegen-units = 1`
- `panic = 'abort'`
- `strip = true`

## Development Notes

- The project uses Makepad v2 script syntax (`script_mod!`) and `Name: value` style.
- Component registration order matters: register base widgets, then component modules, then app UI.
- If extending this library, mirror existing patterns in:
  - `makepad-components/src/lib.rs`
  - `makepad-components/src/button.rs`
  - `makepad-components/src/accordion.rs`

## License

Crates in this workspace are declared as:

`MIT OR Apache-2.0`
