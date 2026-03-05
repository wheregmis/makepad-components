use makepad_widgets::*;

#[derive(Clone)]
enum DrawState {
    DrawHeader,
    DrawBody,
}

use makepad_script::ScriptFnRef;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.Accordion = View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 0.0
    }

    mod.widgets.AccordionItemBase = #(AccordionItem::register_widget(vm))

    mod.widgets.AccordionItem = set_type_default() do mod.widgets.AccordionItemBase{
        width: Fill
        height: Fit
        body_walk: Walk{width: Fill, height: Fit}
        is_open: true

        header: View{
            width: Fill
            height: Fit
            flow: Right
            align: Align{y: 0.5}
            padding: Inset{top: 12, bottom: 12, left: 12, right: 12}
            spacing: 8.0

            title := Label{
                text: "Accordion Item"
                draw_text.color: (shad_theme.color_primary)
                draw_text.text_style.font_size: 11
            }

            View{width: Fill, height: Fit}

            fold_button := FoldButton{
                width: 16
                height: 16
            }
        }

        body: View{
            width: Fill
            height: Fit
            flow: Down
            padding: Inset{left: 12, right: 12, top: 0, bottom: 12}
            spacing: 8.0

            body_text := Label{
                text: "Accordion content"
                draw_text.color: (shad_theme.color_muted_foreground)
                draw_text.text_style.font_size: 10
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct AccordionItem {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,

    #[rust]
    draw_state: DrawStateWrap<DrawState>,
    #[rust]
    area: Area,
    #[find]
    #[redraw]
    #[live]
    header: WidgetRef,
    #[find]
    #[redraw]
    #[live]
    body: WidgetRef,
    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
    #[live]
    body_walk: Walk,
    #[live]
    is_open: bool,
    #[live]
    on_toggle: ScriptFnRef,
}

impl Widget for AccordionItem {
    fn script_call(
        &mut self,
        vm: &mut ScriptVm,
        method: LiveId,
        args: ScriptValue,
    ) -> ScriptAsyncResult {
        if method == live_id!(set_is_open) {
            if let Some(args_obj) = args.as_object() {
                let trap = vm.bx.threads.cur().trap.pass();
                let value = vm.bx.heap.vec_value(args_obj, 0, trap);
                if let Some(is_open) = value.as_bool() {
                    vm.with_cx_mut(|cx| {
                        self.set_is_open(cx, is_open);
                    });
                }
            }
            return ScriptAsyncResult::Return(NIL);
        }
        if method == live_id!(is_open) {
            return ScriptAsyncResult::Return(ScriptValue::from_bool(self.is_open));
        }
        ScriptAsyncResult::MethodNotFound
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.header.handle_event(cx, event, scope);
        if self.is_open {
            self.body.handle_event(cx, event, scope);
        }

        if let Event::Actions(actions) = event {
            let uid = self.widget_uid();
            let fold_button = self.header.fold_button(cx, ids!(fold_button));
            let opening = fold_button.opening(actions);
            let closing = fold_button.closing(actions);

            if opening && !self.is_open {
                self.is_open = true;
                self.area.redraw(cx);
                cx.widget_to_script_call(
                    uid,
                    NIL,
                    self.source.clone(),
                    self.on_toggle.clone(),
                    &[ScriptValue::from_bool(true)],
                );
            } else if closing && self.is_open {
                self.is_open = false;
                self.area.redraw(cx);
                cx.widget_to_script_call(
                    uid,
                    NIL,
                    self.source.clone(),
                    self.on_toggle.clone(),
                    &[ScriptValue::from_bool(false)],
                );
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.draw_state.begin(cx, DrawState::DrawHeader) {
            cx.begin_turtle(walk, self.layout);
        }
        if let Some(DrawState::DrawHeader) = self.draw_state.get() {
            let header_walk = self.header.walk(cx);
            self.header.draw_walk(cx, scope, header_walk)?;

            if self.is_open {
                cx.begin_turtle(self.body_walk, Layout::flow_down());
                self.draw_state.set(DrawState::DrawBody);
            } else {
                cx.end_turtle_with_area(&mut self.area);
                self.draw_state.end();
            }
        }
        if let Some(DrawState::DrawBody) = self.draw_state.get() {
            let body_walk = self.body.walk(cx);
            self.body.draw_walk(cx, scope, body_walk)?;
            cx.end_turtle();
            cx.end_turtle_with_area(&mut self.area);
            self.draw_state.end();
        }
        DrawStep::done()
    }
}

impl AccordionItem {
    pub fn set_is_open(&mut self, cx: &mut Cx, is_open: bool) {
        self.is_open = is_open;
        let fold_button = self.header.fold_button(cx, ids!(fold_button));
        fold_button.set_is_open(cx, is_open, animator::Animate::No);
        self.area.redraw(cx);
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }
}

impl AccordionItemRef {
    pub fn set_is_open(&self, cx: &mut Cx, is_open: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_is_open(cx, is_open);
        }
    }

    pub fn is_open(&self) -> bool {
        self.borrow().map_or(true, |inner| inner.is_open())
    }
}
