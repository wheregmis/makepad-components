# makepad-components

Reusable UI components for [Makepad](https://github.com/makepad/makepad), built with the v2 `script_mod!` workflow.

This workspace contains:
- A component library crate (`makepad-components`)
- A reusable icon crate (`makepad-icon`)
- A runnable gallery app (`makepad-example-component-gallery`)

## What You Get

- **Buttons**: shadcn-inspired variants (`default`, `destructive`, `outline`, `secondary`, `ghost`, `link`) plus size presets.
- **Aspect Ratio**: ratio-constrained container for media/content layouts (`ShadAspectRatio`).
- **Avatar**: circular avatar surfaces for photos, fallback initials, and presence dots.
- **Badge**: semantic metadata tags for states like live, beta, internal, and archived.
- **Accordion**: a composable accordion item widget with open-state and animation helpers (`set_open`, `is_open`, `open_changed`, `animation_progress`).
- **Alerts**: inline alert/callout primitives with neutral and destructive variants, composed from an outer shell, optional icon, and content stack.
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

`ShadAspectRatio` is a ratio-constrained layout shell. It decides the frame size; the child content is still responsible for its own fitting, clipping, and visual treatment.

For media, the recommended pattern is:
- let `ShadAspectRatio` own the ratio
- put a clipped child container inside it
- let `Image` fill that child with an explicit fit mode such as `ImageFit.Biggest` for a cover crop

Example:

```rust
ShadAspectRatio{
    width: 320
    ratio: 1.7777777778

    RoundedView{
        width: Fill
        height: Fill
        clip_x: true
        clip_y: true
        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_radius: (shad_theme.radius)
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }

        Image{
            width: Fill
            height: Fill
            fit: ImageFit.Biggest
            src: crate_resource("self://resources/hero.jpg")
        }
    }
}
```

Use `ImageFit.Smallest` when you want the full image visible inside the ratio instead of a cover crop.

### Avatar (`makepad-components/src/avatar.rs`)

- `ShadAvatar`
- `ShadAvatarSm`
- `ShadAvatarLg`
- `ShadAvatarFallback`
- `ShadAvatarImage`
- `ShadAvatarStatus`
- `ShadAvatarStatusOnline`
- `ShadAvatarStatusAway`
- `ShadAvatarStatusBusy`

`ShadAvatar` is a compositional circular surface. Keep `ShadAvatarFallback` in the avatar for empty or loading states, then add `ShadAvatarImage` when you have a real profile photo. The image sits above the fallback, so the fallback stays useful without becoming visible on top of a loaded image.

Use the status helpers only when presence matters to the workflow. `ShadAvatarStatusOnline`, `ShadAvatarStatusAway`, and `ShadAvatarStatusBusy` add a small presence dot anchored to the avatar edge.

Example:

```rust
View{
    width: Fit
    height: Fit
    flow: Right
    align: Align{y: 0.5}
    spacing: 12.0

    ShadAvatar{
        fallback := ShadAvatarFallback{text: "ML"}
        image := ShadAvatarImage{
            src: crate_resource("self://resources/avatar-a.jpg")
        }
        status := ShadAvatarStatusOnline{}
    }

    ShadAvatar{
        fallback := ShadAvatarFallback{text: "JD"}
    }
}
```

Use fallback-only avatars when profile photos are optional, delayed, or privacy-sensitive. Use photo avatars when identity recognition matters and you have a trustworthy local image source.

### Badge (`makepad-components/src/badge.rs`)

- `ShadBadge`
- `ShadBadgeLabel`
- `ShadBadgeSecondary`
- `ShadBadgeSecondaryLabel`
- `ShadBadgeDestructive`
- `ShadBadgeDestructiveLabel`
- `ShadBadgeSuccess`
- `ShadBadgeSuccessLabel`
- `ShadBadgeWarning`
- `ShadBadgeWarningLabel`
- `ShadBadgeOutline`
- `ShadBadgeOutlineLabel`

`ShadBadge` is a presentational metadata label, not an interactive chip or counter. Use it beside surrounding content with short semantic tags like `Live`, `Beta`, `Internal`, `Deprecated`, or `Archived`.

Keep the slot-based label composition so the foreground color stays paired with the badge variant. Use success for positive/live states, warning for caution or preview states, destructive for removal or deprecated states, and outline for quiet archival labels.

Example:

```rust
RoundedView{
    width: 320
    height: Fit
    padding: Inset{top: 16, right: 16, bottom: 16, left: 16}
    draw_bg +: {
        color: (shad_theme.color_muted)
        border_radius: (shad_theme.radius)
        border_size: 1.0
        border_color: (shad_theme.color_outline_border)
    }

    View{
        width: Fill
        height: Fit
        flow: Right
        align: Align{y: 0.5}
        spacing: 12.0

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 2.0

            ShadFieldLabel{text: "Realtime API"}
            ShadFieldDescription{text: "Production webhook delivery is enabled."}
        }

        ShadBadgeSuccess{
            label := ShadBadgeSuccessLabel{text: "Live"}
        }
    }
}
```

If the UI needs click, remove, or filter behavior, keep that behavior on surrounding buttons or controls. The badge itself should remain a compact, text-only label attached to nearby content.

### Breadcrumb (`makepad-components/src/breadcrumb.rs`)

- `ShadBreadcrumb`
- `ShadBreadcrumbLink`
- `ShadBreadcrumbPage`
- `ShadBreadcrumbSeparator`
- `ShadBreadcrumbEllipsis`

`ShadBreadcrumb` is a quiet inline navigation trail for page ancestry and current location. Use it beside or above the page title it describes, not as a segmented control, tag row, or filter bar.

Keep intermediate ancestors interactive with `ShadBreadcrumbLink`, then render the final item with `ShadBreadcrumbPage` so the terminal node reads as the current page instead of another link.

`ShadBreadcrumbEllipsis` is a visual collapse placeholder in this pass. It communicates hidden ancestors in deep hierarchies, but it does not act as an overflow menu trigger yet.

Example:

```rust
RoundedView{
    width: 360
    height: Fit
    flow: Down
    spacing: 10.0
    padding: Inset{top: 16, right: 16, bottom: 16, left: 16}
    draw_bg +: {
        color: (shad_theme.color_muted)
        border_radius: (shad_theme.radius)
        border_size: 1.0
        border_color: (shad_theme.color_outline_border)
    }

    ShadBreadcrumb{
        ShadBreadcrumbLink{ text: "Workspace" }
        ShadBreadcrumbSeparator{}
        ShadBreadcrumbLink{ text: "Settings" }
        ShadBreadcrumbSeparator{}
        ShadBreadcrumbPage{ text: "Billing" }
    }

    View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 4.0

        Label{
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 16
            text: "Billing Settings"
        }
        ShadFieldDescription{
            text: "Manage invoices, tax details, and workspace-level billing contacts."
        }
    }
}
```

### Card (`makepad-components/src/card.rs`)

- `ShadCard`
- `ShadCardHeader`
- `ShadCardTitle`
- `ShadCardDescription`
- `ShadCardContent`
- `ShadCardFooter`

`ShadCard` is a contained surface for one related slice of content. Use it for settings summaries, plan details, metrics, or grouped actions that belong together, not as a generic full-page wrapper.

Header, content, and footer are compositional. Omit any section you do not need, but keep the title and description close to the body they explain so the card still reads as one task or summary.

Example:

```rust
ShadCard{
    width: 380

    header := ShadCardHeader{
        title := ShadCardTitle{text: "Team Access"}
        description := ShadCardDescription{text: "Review seats, pending invites, and billing impact before applying changes."}
    }

    content := ShadCardContent{
        View{
            width: Fill
            height: Fit
            flow: Right
            align: Align{y: 0.5}
            spacing: 12.0

            View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 2.0

                ShadFieldLabel{text: "Current plan"}
                ShadFieldDescription{text: "Pro workspace with advanced sharing controls."}
            }

            ShadBadgeSecondary{
                label := ShadBadgeSecondaryLabel{text: "Pro"}
            }
        }

        ShadHr{}

        ShadFieldDescription{text: "Seats in use: 18 of 25"}
        ShadFieldDescription{text: "Pending invites: 3 awaiting acceptance"}
    }

    footer := ShadCardFooter{
        ShadButtonGhost{text: "Cancel"}
        ShadButton{text: "Review changes"}
    }
}
```

### Carousel (`makepad-components/src/carousel.rs`)

- `ShadCarousel`
- `ShadCarouselDots`
- `ShadCarouselPrevBtn`
- `ShadCarouselNextBtn`

`ShadCarousel` is a bounded three-slide sequence for related highlights or storytelling content. It owns its prev/next buttons, dot indicators, and active slide state internally.

This pass remains fixed to three named slides. When you need custom content, override `slide_0`, `slide_1`, and `slide_2` inside `content_wrap.carousel_flip` rather than trying to supply an arbitrary slide count.

Ref API:
- `next(cx)`
- `prev(cx)`
- `go_to(cx, index)`
- `current() -> Option<usize>`
- `changed(actions) -> Option<usize>`

Example:

```rust
featured_carousel := ShadCarousel{
    width: 720
}

// Controller example (Rust):
// let carousel = self.ui.shad_carousel(cx, ids!(featured_carousel));
//
// if self.ui.button(cx, ids!(open_next_highlight_btn)).clicked(actions) {
//     carousel.next(cx);
// }
//
// if let Some(index) = carousel.changed(actions) {
//     log!("Active highlight changed to {}", index);
// }
```

Use carousel when the panels are ordered and the user benefits from moving through one related narrative. Prefer static cards or a grid when the user needs to compare multiple items at once.

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
- `ShadAlertHeader`
- `ShadAlertIcon`
- `ShadAlertContent`
- `ShadAlertTitle`
- `ShadAlertDescription`
- `ShadAlertDestructive`
- `ShadAlertDestructiveIcon`
- `ShadAlertDestructiveTitle`
- `ShadAlertDestructiveDescription`

`ShadAlert` and `ShadAlertDestructive` are compositional, inline callout shells. They do not own open/close state, dismiss buttons, or action handling.

Render the icon as a sibling of `ShadAlertContent`, then keep the title and description stacked inside `ShadAlertContent`.

Example:

```rust
View{
    width: Fill
    height: Fit
    flow: Down
    spacing: 12.0

    ShadAlert{
        width: Fill
        ShadAlertIcon{}
        ShadAlertContent{
            ShadAlertTitle{text: "Heads up!"}
            ShadAlertDescription{
                text: "You can add components and dependencies to your app using the cli."
            }
        }
    }

    ShadAlertDestructive{
        width: Fill
        ShadAlertDestructiveIcon{}
        ShadAlertContent{
            ShadAlertDestructiveTitle{text: "Error"}
            ShadAlertDestructiveDescription{
                text: "Your session has expired. Please log in again."
            }
        }
    }
}
```

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

- `ShadDialog` — generic blocking modal for short focused workflows
- `ShadDialogAlert` — preset confirm dialog with title, description, Cancel, and Continue
- `ShadDialogAlertDestructive` — destructive confirm dialog with the same shell and destructive primary action
- `ShadDialogHeader`
- `ShadDialogTitle`
- `ShadDialogDescription`
- `ShadDialogContent`
- `ShadDialogFooter`

Use dialog for blocking decisions or one short workflow like renaming, publishing, or deleting. Prefer `ShadSheet` for supporting work that can stay beside the current page, and prefer inline cards for content that should remain visible all the time.

Generic dialogs are composed by overriding `overlay +: { content +: { body +: { ... } } }`:

```rust
rename_dialog := ShadDialog{
    overlay +: {
        content +: {
            body +: {
                dialog_header := ShadDialogHeader{
                    title := ShadDialogTitle{text: "Rename project"}
                    description := ShadDialogDescription{
                        text: "Update the project name shown across navigation and shares."
                    }
                }

                dialog_content := ShadDialogContent{
                    ShadField{
                        ShadFieldLabel{text: "Project name"}
                        ShadInput{empty_text: "Northwind Revamp"}
                    }
                }

                dialog_footer := ShadDialogFooter{
                    rename_cancel_btn := ShadButtonOutline{text: "Cancel"}
                    rename_save_btn := ShadButton{text: "Save changes"}
                }
            }
        }
    }
}
```

Alert variants are built on the same centered shell. Customize them by overriding `dialog_body`, `title_label`, `description_label`, `footer`, `cancel`, and `confirm`.

Props: `open` (bool).
Script API: `set_open(bool)` and `is_open() -> bool`.
Action API: `open_changed(actions) -> Option<bool>`.

Behavior:
- `ShadDialog` closes on backdrop or Escape; page code owns footer actions.
- `ShadDialogAlert` and `ShadDialogAlertDestructive` also close on Cancel and Confirm.

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
- `ShadSonner` — shared toast host / queue
- `ShadSonnerWithDescription` — `ShadSonner` preset with `toast_kind: "description"`
- `ShadSonnerWithClose` — `ShadSonner` preset with `toast_kind: "close"`

Use: `ShadSonner`, `ShadSonnerWithDescription`, and `ShadSonnerWithClose` refs all append into one shared toast queue for the current UI tree. Call `open()` on any of those refs, or use `open_description()` / `open_close()` on a base `ShadSonner` ref when you want one host API.
Action API: `open_changed(actions) -> Option<bool>`.

### Spinner (`makepad-components/src/spinner.rs`)

- `ShadSpinner` — default circular loading indicator (24×24, animated arc)
- `ShadSpinnerSm` — compact inline spinner (16×16)
- `ShadSpinnerLg` — larger panel/loading-state spinner (32×32)

Use spinner for indeterminate waits when the UI is busy but cannot report percentage progress yet. Keep it close to the text or surface it supports instead of centering it in empty space by default.

Use skeleton when the final content structure is already known and you want the loading layout to stay stable. Use progress when completion can be measured.

Example:

```rust
RoundedView{
    width: Fit
    height: Fit
    flow: Right
    spacing: 10.0
    align: Align{y: 0.5}
    padding: Inset{left: 14, right: 14, top: 10, bottom: 10}
    draw_bg +: {
        color: (shad_theme.color_secondary)
        border_radius: (shad_theme.radius)
        border_size: 1.0
        border_color: (shad_theme.color_outline_border)
    }

    ShadSpinnerSm{}

    Label{
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 11
        text: "Saving changes..."
    }
}
```

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

`makepad-icon` now generates icon bindings at build time via `makepad-icon/build.rs`.
Every SVG under `makepad-icon/resources/icons/*.svg` is exported as `Icon<PascalCaseName>`.
For example:
- `check.svg` → `IconCheck`
- `arrow-left-right.svg` → `IconArrowLeftRight`

To sync all Lucide assets:

```bash
cd makepad-icon
python3 scripts/download_lucide_icons.py --clean
```

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
