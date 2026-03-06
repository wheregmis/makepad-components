# makepad-components

Reusable UI components for [Makepad](https://github.com/makepad/makepad), built with the v2 `script_mod!` workflow.

This workspace contains:
- A component library crate (`makepad-components`)
- A reusable icon crate (`makepad-icon`)
- A runnable gallery app (`makepad-example-component-gallery`)

## What You Get

- **Buttons**: shadcn-inspired variants (`default`, `destructive`, `outline`, `secondary`, `ghost`, `link`) plus size presets.
- **Aspect Ratio**: ratio-constrained container for media/content layouts (`ShadAspectRatio`).
- **Accordion**: a composable accordion item widget with open/close state and script-call support (`set_is_open`, `is_open`).
- **Alerts**: shadcn-inspired alert layouts with default and destructive variants.
- **Dialog**: modal with variants — generic (custom body), alert (title + Cancel/Confirm), destructive; `set_open(bool)` / `is_open()` API.
- **Theme tokens**: centralized `shad_theme` color + radius tokens in script space.
- **Icons**: SVG-based icon widgets (`IconCheck`, `IconX`, `IconSearch`).
- **Kbd**: keyboard shortcut key caps (`ShadKbd`, `ShadKbdLabel`, `ShadKbdSeparator`) for displaying shortcuts (e.g. ⌘ ⇧ ⌥ ⌃ or Ctrl + B).
- **Gallery app**: a live catalog demonstrating component usage and styling.

## Workspace Layout

- `components/` → `makepad-components` library
- `makepad-icon/` → `makepad-icon` library
- `gallery/` → `makepad-example-component-gallery` app
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
  - New reusable components belong under `components/src/*.rs` and should be registered from `components::script_mod(vm)` into the `mod.widgets.*` namespace.
  - Gallery-only layout and helper widgets belong under `gallery/src/ui/*.rs` (for example `themed_widgets.rs`) and are registered from `gallery::ui::script_mod(vm)`.

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
wasm-opt --version   # from Binaryen
brotli --version
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

                    Accordion{
                        item_one := AccordionItem{
                            header: View{
                                flow: Right
                                title := Label{text: "Section 1"}
                                View{width: Fill, height: Fit}
                                fold_button := FoldButton{}
                            }
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
- [ ] Calendar
- [x] Card
- [x] Carousel
- [ ] Chart
- [x] Checkbox
- [x] Collapsible
- [ ] Combobox
- [ ] Command
- [ ] Context Menu
- [ ] Date Picker
- [x] Dialog
- [x] Drawer
- [x] Dropdown Menu
- [ ] Empty
- [x] Input & Field
- [x] Hover Card
- [ ] Input OTP
- [x] Kbd
- [x] Label
- [ ] Menubar
- [ ] Native Select
- [ ] Navigation Menu
- [ ] Pagination
- [ ] Popover
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
- [ ] Table
- [x] Tabs
- [ ] Textarea
- [ ] Toggle & Toggle Group
- [x] Tooltip

**Known limitation:** In the gallery app, popup-style selectors (`ShadDropdownMenu`, `ShadSelect`, and the base `DropDown` widget) may not open reliably on click; this appears to be an interaction between the gallery’s layout (flow + `PageFlip`) and the platform’s hit/sweep handling. For working popup interaction examples, see the splash app (`splash_app.md` or the makepad repo’s splash example), which uses a Dock-based layout.

### Buttons (`components/src/button.rs`)

- `ShadButton`
- `ShadButtonDestructive`
- `ShadButtonOutline`
- `ShadButtonSecondary`
- `ShadButtonGhost`
- `ShadButtonLink`
- `ShadButtonSm`
- `ShadButtonLg`
- `ShadButtonIcon`

### Aspect Ratio (`components/src/aspect_ratio.rs`)

- `ShadAspectRatio`

### Accordion (`components/src/accordion.rs`)

- `Accordion` (container `View` preset)
- `AccordionItem` (custom widget)

`AccordionItem` supports:
- `is_open` live field
- script calls: `set_is_open(bool)` and `is_open() -> bool`
- `on_toggle` callback hook

### Alerts (`components/src/alert.rs`)

- `ShadAlert`
- `ShadAlertIcon`
- `ShadAlertContent`
- `ShadAlertTitle`
- `ShadAlertDescription`
- `ShadAlertDestructive`
- `ShadAlertDestructiveIcon`
- `ShadAlertDestructiveTitle`

### Input & Field (`components/src/input.rs`)

- `ShadInput`
- `ShadInputWithIcon`
- `ShadField`
- `ShadFieldLabel`
- `ShadFieldDescription`
- `ShadFieldMessage`

Use `ShadField` as a layout wrapper around `ShadInput` or another form control. Validation/state stays in app code.

### Separator (`components/src/hr.rs`)

- `ShadSeparator`

Compatibility alias: `ShadHr`.

### Scroll Area (`components/src/scroll.rs`)

- `ShadScrollArea` — vertical scroll container
- `ShadScrollAreaX` — horizontal scroll container
- `ShadScrollAreaXY` — two-axis scroll container

Compatibility alias: `ShadScrollYView`.

### Tooltip (`components/src/tooltip.rs`)

- `ShadTooltip`
- `ShadTooltipCallout`

Thin wrappers over Makepad tooltip primitives. Drive them from hover/focus/click in app code via `show_with_options(...)`.

### Radio Group (`components/src/radio_group.rs`)

- `ShadRadioGroup`
- `ShadRadioGroupInline`
- `ShadRadioItem`

Use Makepad `radio_button_set(...).selected(cx, actions)` to keep a single item active.

### Select (`components/src/select.rs`)

- `ShadSelect`
- `ShadSelectItem`

Single-select, non-searchable dropdown built on the popup menu stack.

### Dialog (`components/src/dialog.rs`)

- `ShadDialog` — generic modal with customizable `body` content (closes on backdrop/Escape; wire your own Close button)
- `ShadDialogAlert` — preset with title, description, Cancel/Continue (closes on Cancel, Confirm, or backdrop)
- `ShadDialogAlertDestructive` — same layout with destructive Confirm (e.g. Delete)

Props: `open` (bool). For generic: put content in `overlay +: { content +: { body +: { ... } } }`. For alert variants: customize `title_label`, `description_label` in the template.

Script API: `set_open(bool)` and `is_open() -> bool`.

### Progress (`components/src/progress.rs`)

- `ShadProgress` — default 50%
- `ShadProgress33`, `ShadProgress66`, `ShadProgressFull` — 33%, 66%, 100%
- `ShadProgressIndeterminate` — animated loading bar (continuous sweep)

Use `ShadProgress66{}` for 66%. For custom values, extend `ShadProgressBase` with `draw_bg +: { progress: instance(0.42) }`.

### Slider (`components/src/slider.rs`)

- `ShadSlider` — shadcn-style range slider (extends makepad SliderRoundFlat)

Props: `default`, `min`, `max`, `step`. Uses makepad Slider actions for value changes.

### Tabs (`components/src/tabs.rs`)

- `ShadTabs`
- `ShadTabsList`
- `ShadTabsTrigger`
- `ShadTabsContent`

This wave ships the canonical styling primitives; pair them with `PageFlip` or another app-level state holder.

### Sheet (`components/src/sheet.rs`)

- `ShadSheet`
- `ShadSheetTitle`
- `ShadSheetDescription`

Props: `open` (bool). Script API: `set_open(bool)` and `is_open() -> bool`.

### Resizable (`components/src/resizable.rs`)

- `ShadResizable`

Thin wrapper over Makepad `Splitter` for two-pane layouts.

### Sonner / Toast (`components/src/sonner.rs`)

- `ShadToast` — toast notification card container
- `ShadToastTitle` — title text
- `ShadToastDescription` — optional description (uses `ShadAlertDescription` styling)

Use: `ShadToast{ title := ShadToastTitle{text: "Event created"} }` or add `description := ShadToastDescription{text: "..."}`.

### Spinner (`components/src/spinner.rs`)

- `ShadSpinner` — circular loading indicator (24×24, animated arc)

Use for async operations and loading states.

### Kbd (`components/src/kbd.rs`)

- `ShadKbd` — key cap container (dark grey rounded rect, subtle border)
- `ShadKbdLabel` — text/symbol inside a key (e.g. ⌘, ⇧, Ctrl, B)
- `ShadKbdSeparator` — " + " between keys in a shortcut

Use with a horizontal layout: `ShadKbd{ label := ShadKbdLabel{text: "Ctrl"} }` and `ShadKbdSeparator{}` for shortcuts like "Ctrl + B".

### Sidebar (`components/src/sidebar.rs`)

- `ShadSidebar`
- `ShadSidebarSectionLabel`
- `ShadSidebarItem`

### Theme (`components/src/theme.rs`)

Exports `mod.widgets.shad_theme` with tokens such as:
- `color_primary`, `color_secondary`, `color_background`
- `color_destructive*`, `color_outline_border*`
- `color_muted*`, `color_ghost*`
- `radius`

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

The gallery (`gallery/src/app.rs`) includes:
- Sidebar navigation between component pages
- Accordion showcase with custom header/body content
- Button variant and size matrix
- Aspect ratio examples (16:9, 1:1, 4:3, 9:16)
- Alert default/destructive examples
- Icon preview section

Run it to validate behavior and styling changes quickly.

### Adding a new component + docs page

- **Library component (`Shad*`)**
  - Add or extend a module under `components/src/` (for example `button.rs`, `accordion.rs`).
  - Register the widget in `components::script_mod(vm)` with a `Shad*` name in the `mod.widgets.*` namespace.
  - Use `shad_theme` tokens for colors, radii, and spacing instead of hardcoded values.
- **Gallery docs page (`Gallery*`)**
  - Create a new page script under `gallery/src/ui/` (for example `tooltip_page.rs`) that uses `ShadScrollYView`, `ShadPageTitle`, and `ShadPageSubtitle`.
  - Add a snippet constant to `gallery/src/ui/snippets.rs` and reference it from `GalleryCodeSnippet` on the page.
  - Add a `ShadSidebarItem` entry in `GallerySidebar` and a matching page in `GalleryContentFlip`.
  - Wire the sidebar item to the page in `gallery/src/app.rs` using a `set_page` call in `handle_actions`.

## CI/CD

GitHub Actions workflow: `.github/workflows/wasm-pages.yml`

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
  - `components/src/lib.rs`
  - `components/src/button.rs`
  - `components/src/accordion.rs`

## License

Crates in this workspace are declared as:

`MIT OR Apache-2.0`
