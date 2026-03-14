# ShadAccordionItem Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build `ShadAccordionItem` — a new Rust widget following the `FoldButton` pattern, with self-contained header (title text + rotating chevron via SDF shader), composable body slot, and bottom divider line drawn by shader.

**Architecture:** Replace the current `AccordionItem` (which delegates open/close to an external `FoldButton`) with a new `ShadAccordionItem` that owns its own draw primitives (`draw_bg`, `draw_text`, `draw_icon`) and animator. The header is drawn entirely by the widget — no child `FoldButton` or `Label` needed. The body slot remains a `WidgetRef` for arbitrary content.

**Tech Stack:** Makepad Rust widget system, Splash DSL, SDF2d shaders, Animator derive macro

---

### Key Reference: FoldButton Pattern

Study `/Users/wheregmis/.cargo/git/checkouts/makepad-ec2f134f34cd9f98/8b51533/widgets/src/fold_button.rs` before implementing. The pattern is:

1. `#[derive(Script, ScriptHook, Widget, Animator)]`
2. `draw_bg: DrawQuad` with a custom `pixel` shader using `Sdf2d`
3. `active: f64` field driven by animator
4. `animator_handle_event` in `handle_event`
5. `event.hits(cx, self.draw_bg.area())` for mouse events
6. Emit widget actions via `cx.widget_action_with_data(&self.action_data, uid, Action)`
7. `DrawStep::done()` from `draw_walk`

### Key Reference: Current AccordionItem

The existing code is at `components/src/accordion.rs`. The new widget **replaces** `AccordionItem` entirely — keep `Accordion` (the container `View`) and remove `AccordionItemBase`, `AccordionItem`, and all the old Rust struct/impls.

---

### Task 1: Write the new `ShadAccordionItem` widget in `accordion.rs`

**Files:**
- Modify: `components/src/accordion.rs` (full rewrite)

**Context:**

The new widget needs:
- `draw_bg: DrawQuad` — draws two things: (a) header hover background, (b) 1px bottom divider line
- `draw_text: DrawText` — draws the title string in the header
- `draw_icon: DrawQuad` — draws the chevron triangle via SDF, rotates via `active` instance var
- `body: WidgetRef` — arbitrary body content
- `title: String` — header title text (live property)
- `is_open: bool` — initial open state (live property)
- `active: f64` — animated float (0.0 = closed, 1.0 = open)
- `header_area: Area` — stored area for hit detection
- `animator: Animator` — `hover` and `active` states

**Step 1: Write the complete new `accordion.rs`**

Replace the entire file with:

```rust
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadAccordion = View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 0.0
    }

    mod.widgets.ShadAccordionItemBase = #(ShadAccordionItem::register_widget(vm))

    mod.widgets.ShadAccordionItem = set_type_default() do mod.widgets.ShadAccordionItemBase{
        width: Fill
        height: Fit
        is_open: true
        title: "Accordion Item"

        draw_bg +: {
            hover: instance(0.0)

            color_hover: uniform(shad_theme.color_secondary_hover)
            color_divider: uniform(shad_theme.color_outline_border)

            pixel: fn() {
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)

                // Header hover background (full rect)
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 0.0)
                sdf.fill(mix(vec4(0.0, 0.0, 0.0, 0.0), self.color_hover, self.hover))

                // Bottom divider line (1px at very bottom)
                sdf.rect(0.0, self.rect_size.y - 1.0, self.rect_size.x, 1.0)
                sdf.fill(self.color_divider)

                return sdf.result
            }
        }

        draw_text +: {
            color: uniform(shad_theme.color_primary)
            text_style: theme.font_regular{ font_size: 11.0 }
        }

        draw_icon +: {
            active: instance(1.0)
            hover: instance(0.0)

            color: uniform(shad_theme.color_muted_foreground)
            color_hover: uniform(shad_theme.color_primary)

            pixel: fn() {
                let sz = 3.0
                let c = self.rect_size * 0.5
                let sdf = Sdf2d.viewport(self.pos * self.rect_size)
                sdf.clear(vec4(0.0))

                // Rotate: 0 = pointing right (collapsed), 1 = pointing down (open)
                sdf.rotate(self.active * 0.5 * PI, c.x, c.y)
                sdf.move_to(c.x, c.y - sz)
                sdf.line_to(c.x + sz, c.y + sz)
                sdf.line_to(c.x - sz, c.y + sz)
                sdf.close_path()
                sdf.fill(mix(self.color, self.color_hover, self.hover))

                return sdf.result
            }
        }

        body: View{
            width: Fill
            height: Fit
            flow: Down
            padding: Inset{left: 16, right: 16, top: 0, bottom: 16}
            spacing: 8.0
        }

        animator: Animator{
            hover: {
                default: @off
                off: AnimatorState{
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {hover: 0.0}
                        draw_icon: {hover: 0.0}
                    }
                }
                on: AnimatorState{
                    from: {all: Snap}
                    apply: {
                        draw_bg: {hover: 1.0}
                        draw_icon: {hover: 1.0}
                    }
                }
            }
            active: {
                default: @on
                off: AnimatorState{
                    from: {all: Forward {duration: 0.2}}
                    ease: ExpDecay {d1: 0.96, d2: 0.97}
                    redraw: true
                    apply: {
                        active: 0.0
                        draw_icon: {active: 0.0}
                    }
                }
                on: AnimatorState{
                    from: {all: Forward {duration: 0.2}}
                    ease: ExpDecay {d1: 0.98, d2: 0.95}
                    redraw: true
                    apply: {
                        active: 1.0
                        draw_icon: {active: 1.0}
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ShadAccordionItemAction {
    #[default]
    None,
    Opening,
    Closing,
    Animating(f64),
}

impl ActionTrait for ShadAccordionItemAction {}

#[derive(Script, ScriptHook, Widget, Animator)]
pub struct ShadAccordionItem {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[apply_default]
    animator: Animator,

    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[redraw]
    #[live]
    draw_text: DrawText,
    #[redraw]
    #[live]
    draw_icon: DrawQuad,

    #[find]
    #[redraw]
    #[live]
    body: WidgetRef,

    #[live]
    title: String,
    #[live]
    is_open: bool,
    #[live]
    active: f64,

    #[walk]
    walk: Walk,

    #[rust]
    header_area: Area,

    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}

impl Widget for ShadAccordionItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        let res = self.animator_handle_event(cx, event);
        if res.must_redraw() {
            cx.widget_action_with_data(
                &self.action_data,
                uid,
                ShadAccordionItemAction::Animating(self.active),
            );
            self.draw_bg.redraw(cx);
            self.draw_icon.redraw(cx);
        }

        // Only handle body events when open
        if self.active > 0.0 {
            self.body.handle_event(cx, event, scope);
        }

        match event.hits(cx, self.header_area) {
            Hit::FingerDown(_) => {
                if self.animator_in_state(cx, ids!(active.on)) {
                    self.animator_play(cx, ids!(active.off));
                    cx.widget_action_with_data(
                        &self.action_data,
                        uid,
                        ShadAccordionItemAction::Closing,
                    );
                } else {
                    self.animator_play(cx, ids!(active.on));
                    cx.widget_action_with_data(
                        &self.action_data,
                        uid,
                        ShadAccordionItemAction::Opening,
                    );
                }
            }
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerUp(fe) => {
                if fe.is_over && fe.device.has_hovers() {
                    self.animator_play(cx, ids!(hover.on));
                } else {
                    self.animator_play(cx, ids!(hover.off));
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Header height
        let header_height = 48.0_f64;
        let icon_size = 12.0_f64;
        let padding_h = 16.0_f64;

        // Draw header background (hover bg + divider via shader)
        let header_walk = Walk {
            width: Size::Fill,
            height: Size::Fixed(header_height),
            ..Walk::default()
        };
        self.draw_bg.draw_walk(cx, header_walk);
        self.header_area = self.draw_bg.area();

        // Draw title text — positioned inside header
        let text_walk = Walk {
            width: Size::Fill,
            height: Size::Fixed(header_height),
            margin: Margin {
                left: padding_h,
                right: padding_h + icon_size + 8.0,
                ..Margin::default()
            },
            ..Walk::default()
        };
        self.draw_text.draw_walk(cx, text_walk, Align { x: 0.0, y: 0.5 }, &self.title.clone());

        // Draw chevron icon at right
        let icon_walk = Walk {
            width: Size::Fixed(icon_size),
            height: Size::Fixed(icon_size),
            margin: Margin {
                right: padding_h,
                ..Margin::default()
            },
            ..Walk::default()
        };
        self.draw_icon.draw_walk(cx, icon_walk);

        // Draw body if animating or open
        if self.active > 0.0 {
            let body_walk = self.body.walk(cx);
            self.body.draw_walk(cx, scope, body_walk)?;
        }

        DrawStep::done()
    }
}

impl ShadAccordionItem {
    pub fn set_open(&mut self, cx: &mut Cx, is_open: bool, animate: Animate) {
        self.animator_toggle(cx, is_open, animate, ids!(active.on), ids!(active.off));
    }

    pub fn is_open(&self, cx: &Cx) -> bool {
        self.animator_in_state(cx, ids!(active.on))
    }

    pub fn opening(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            matches!(item.cast::<ShadAccordionItemAction>(), ShadAccordionItemAction::Opening)
        } else {
            false
        }
    }

    pub fn closing(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            matches!(item.cast::<ShadAccordionItemAction>(), ShadAccordionItemAction::Closing)
        } else {
            false
        }
    }
}

impl ShadAccordionItemRef {
    pub fn set_open(&self, cx: &mut Cx, is_open: bool, animate: Animate) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_open(cx, is_open, animate);
        }
    }

    pub fn is_open(&self, cx: &Cx) -> bool {
        self.borrow().map_or(true, |inner| inner.is_open(cx))
    }

    pub fn open_changed(&self, actions: &Actions) -> Option<bool> {
        self.borrow().and_then(|inner| inner.open_changed(actions))
    }

    pub fn animation_progress(&self, actions: &Actions) -> Option<f64> {
        self.borrow()
            .and_then(|inner| inner.animation_progress(actions))
    }
}
```

**Step 2: Build to check for compile errors**

```bash
cargo build -p makepad-components 2>&1 | head -50
```

Expected: compile errors are normal at this stage — note them and fix before moving on. Common issues:
- `DrawText::draw_walk` signature — check the actual Makepad API. Look at how `Label` uses `DrawText` in `makepad_widgets`. The correct call may be `self.draw_text.draw_walk(cx, walk)` without the align/text args — `DrawText` may just use the `text` live property. Read the source if unsure.
- `Animate` import — may need `use makepad_widgets::animator::Animate;`
- `ActionTrait` — verify this trait name in the Makepad source (check `fold_button.rs` imports)
- Walk/Margin field names — verify `Margin::default()` exists

To check DrawText API:
```bash
grep -n "fn draw_walk\|fn draw_text\|pub fn draw" /Users/wheregmis/.cargo/git/checkouts/makepad-ec2f134f34cd9f98/8b51533/draw/src/draw_text.rs | head -20
```

Fix all errors before proceeding to step 3.

**Step 3: Build successfully**

```bash
cargo build -p makepad-components 2>&1 | grep "^error" | wc -l
```

Expected: 0 errors.

**Step 4: Commit**

```bash
git add components/src/accordion.rs
git commit -m "feat: add ShadAccordionItem widget with self-contained header and SDF chevron"
```
Add co-author: `Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>`

---

### Task 2: Update `accordion_page.rs` to use `ShadAccordionItem`

**Files:**
- Modify: `gallery/src/ui/accordion_page.rs`

**Context:**

Replace the `accordion_wrap` RoundedView + `Accordion` + three `AccordionItem` widgets with a `ShadAccordion` + three `ShadAccordionItem` widgets. The new DSL is much simpler — no `header:` slot needed, just `title:` and `body:`.

**Step 1: Replace the accordion section in `accordion_page.rs`**

Replace from `accordion_wrap := RoundedView{` down to the closing `}` of `accordion_wrap` (lines 67–165) with:

```
        accordion_panel := ShadAccordion{
            item_accessible := ShadAccordionItem{
                title: "Is it accessible?"
                is_open: true
                body: View{
                    width: Fill
                    height: Fit
                    flow: Down
                    padding: Inset{left: 16, right: 16, top: 0, bottom: 16}
                    Label{
                        text: "Yes. This accordion is keyboard and mouse friendly by default through FoldHeader/FoldButton behavior."
                        draw_text.color: (shad_theme.color_muted_foreground)
                        draw_text.text_style.font_size: 10
                    }
                }
            }

            item_styled := ShadAccordionItem{
                title: "Is it styled with complex elements?"
                body: View{
                    width: Fill
                    height: Fit
                    flow: Down
                    padding: Inset{left: 16, right: 16, top: 0, bottom: 16}
                    spacing: 8.0
                    Label{
                        text: "We can put any view here, like a row with toggles."
                        draw_text.color: (shad_theme.color_muted_foreground)
                        draw_text.text_style.font_size: 10
                    }
                    View{
                        width: Fill
                        height: Fit
                        flow: Right
                        spacing: 16
                        GalleryToggle{text: "Switch"}
                        GalleryCheckBox{text: "Or a CheckBox"}
                    }
                }
            }

            item_third := ShadAccordionItem{
                title: "This is third accordion"
                body: View{
                    width: Fill
                    height: Fit
                    flow: Down
                    padding: Inset{left: 16, right: 16, top: 0, bottom: 16}
                    Label{
                        text: "This is third accordion content. It can be any view, like a text view or a button."
                        draw_text.color: (shad_theme.color_muted_foreground)
                        draw_text.text_style.font_size: 10
                    }
                }
            }
        }
```

Also remove the `accordion_wrap` RoundedView wrapper — `ShadAccordion` is the container now. Remove the "Normal" label too, or keep it as a section header if desired.

**Step 2: Build the gallery**

```bash
cargo build -p makepad-example-component-gallery 2>&1 | grep "^error" | head -20
```

Fix any DSL errors. Common issues:
- `ShadAccordion` / `ShadAccordionItem` not found — check that `script_mod` in `accordion.rs` uses `ShadAccordion` and `ShadAccordionItem` (not old `Accordion`/`AccordionItem` names)
- If the old `AccordionItem`/`Accordion` names are referenced elsewhere, update those too

**Step 3: Run the app and visually verify**

```bash
cargo run -p gallery
```

Check:
- Accordion page shows 3 items
- Clicking a header toggles open/close
- Chevron rotates smoothly
- Body content visible when open, hidden when closed
- Hover shows subtle background
- Bottom divider line visible between items

**Step 4: Commit**

```bash
git add gallery/src/ui/accordion_page.rs
git commit -m "feat: update accordion page to use ShadAccordionItem"
```
Add co-author: `Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>`

---

### Task 3: Fix draw_walk layout if headers/body don't align correctly

**Context:**

The `draw_walk` approach in Task 1 draws items sequentially using `draw_walk` calls, but the turtle/layout system in Makepad requires careful use of `begin_turtle`/`end_turtle` to stack things vertically. If the header and body overlap, or the icon doesn't appear at the right edge, the draw logic needs adjusting.

**Step 1: If layout is wrong, study how existing widgets handle multi-element draw**

Look at how `ButtonFlat` draws icon + text:
```bash
grep -n "draw_walk\|begin_turtle\|end_turtle\|icon_walk\|label_walk" /Users/wheregmis/.cargo/git/checkouts/makepad-ec2f134f34cd9f98/8b51533/widgets/src/button.rs | head -30
```

Also check `DrawText::draw_walk` signature:
```bash
grep -n "pub fn draw_walk\|pub fn draw_text_walk" /Users/wheregmis/.cargo/git/checkouts/makepad-ec2f134f34cd9f98/8b51533/draw/src/draw_text.rs | head -10
```

**Step 2: Fix layout issues**

The most reliable approach if direct `draw_walk` calls cause layout problems: wrap the header in a `begin_turtle` / `end_turtle` block with `Layout::flow_right()`, then draw body below with `Layout::flow_down()`. Study how `AccordionItem`'s original `draw_walk` used `DrawState` and `begin_turtle`.

**Step 3: Rebuild and verify visually**

```bash
cargo run -p gallery
```

**Step 4: Commit fixes**

```bash
git add components/src/accordion.rs
git commit -m "fix: correct ShadAccordionItem draw layout"
```
Add co-author: `Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>`

---

### Task 4: Final verification

**Step 1: Full build**

```bash
cargo build
```

Expected: zero errors.

**Step 2: Grep to confirm old types are gone**

```bash
grep -rn "AccordionItem\b\|AccordionItemRef\b" components/src/ gallery/src/ --include="*.rs"
```

Expected: zero results (the old type is fully replaced).

**Step 3: Run app, check all three pages still work**

```bash
cargo run -p gallery
```

- Accordion page: 3 items, click to toggle, chevron animates, body shows/hides
- Button page: all button variants render correctly
- Alert page: both alert variants render correctly

**Step 4: Commit if any final fixes needed**

```bash
git add -p
git commit -m "fix: final ShadAccordionItem cleanup"
```
Add co-author: `Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>`
