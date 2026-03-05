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
- **Theme tokens**: centralized `shad_theme` color + radius tokens in script space.
- **Icons**: SVG-based icon widgets (`IconCheck`, `IconX`, `IconSearch`).
- **Gallery app**: a live catalog demonstrating component usage and styling.

## Workspace Layout

- `components/` → `makepad-components` library
- `makepad-icon/` → `makepad-icon` library
- `gallery/` → `makepad-example-component-gallery` app
- `.github/workflows/wasm-pages.yml` → GitHub Pages WASM build + deploy

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
cargo makepad wasm build --brotli --bindgen -p makepad-example-component-gallery --release --profile=small
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
- [ ] Alert Dialog
- [x] Aspect Ratio
- [x] Avatar
- [x] Badge
- [x] Breadcrumb
- [x] Button
- [x] Button Group
- [ ] Calendar
- [ ] Card
- [ ] Carousel
- [ ] Chart
- [x] Checkbox
- [x] Collapsible
- [ ] Combobox
- [ ] Command
- [ ] Context Menu
- [ ] Data Table
- [ ] Date Picker
- [ ] Dialog
- [ ] Direction
- [ ] Drawer
- [ ] Dropdown Menu
- [ ] Empty
- [ ] Field
- [ ] Hover Card
- [x] Input
- [ ] Input Group
- [ ] Input OTP
- [ ] Item
- [ ] Kbd
- [x] Label
- [ ] Menubar
- [ ] Native Select
- [ ] Navigation Menu
- [ ] Pagination
- [ ] Popover
- [ ] Progress
- [ ] Radio Group
- [ ] Resizable
- [ ] Scroll Area
- [ ] Select
- [ ] Separator
- [ ] Sheet
- [ ] Sidebar
- [ ] Skeleton
- [ ] Slider
- [ ] Sonner
- [ ] Spinner
- [ ] Switch
- [ ] Table
- [ ] Tabs
- [ ] Textarea
- [ ] Toast
- [ ] Toggle
- [ ] Toggle Group
- [ ] Tooltip

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
