use crate::internal::actions::{emit_widget_action, widget_action_map};
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadField = mod.widgets.View {
        width: Fill
        height: Fit
        flow: Down
        spacing: 6.0
    }

    mod.widgets.ShadFieldLabel = mod.widgets.Label {
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 11.0
    }

    mod.widgets.ShadFieldDescription = mod.widgets.Label {
        width: Fill
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 10.0
    }

    mod.widgets.ShadFieldMessage = mod.widgets.Label {
        width: Fill
        draw_text.color: (shad_theme.color_destructive)
        draw_text.text_style.font_size: 10.0
    }

    mod.widgets.ShadInput = mod.widgets.TextInput {
        width: Fill
        height: Fit
        padding: Inset{left: 12, right: 12, top: 12, bottom: 12}
        empty_text: "Enter text..."

        draw_bg +: {
            border_radius: (shad_theme.radius)
            border_size: (shad_theme.border_size)

            color: (shad_theme.color_clear)
            color_hover: (shad_theme.color_clear)
            color_focus: (shad_theme.color_clear)
            color_down: (shad_theme.color_clear)
            color_empty: (shad_theme.color_clear)
            color_disabled: (shad_theme.color_muted)

            border_color: (shad_theme.color_outline_border)
            border_color_hover: (shad_theme.color_outline_border_hover)
            border_color_focus: (shad_theme.color_primary)
            border_color_down: (shad_theme.color_primary)
            border_color_empty: (shad_theme.color_outline_border)
            border_color_disabled: (shad_theme.color_outline_border)
        }

        draw_text +: {
            color: (shad_theme.color_primary)
            color_hover: (shad_theme.color_primary)
            color_focus: (shad_theme.color_primary)
            color_down: (shad_theme.color_primary)
            color_empty: (shad_theme.color_muted_foreground)
            color_disabled: (shad_theme.color_muted_foreground)
        }
        draw_text.text_style.font_size: 11.0

        draw_cursor +: {
            color: (shad_theme.color_primary)
        }

        draw_selection +: {
            color: (shad_theme.color_muted)
        }
    }

    // A borderless version of ShadInput to be used inside custom wrappers
    mod.widgets.ShadInputBorderless = mod.widgets.ShadInput {
        draw_bg +: {
            border_size: 0.0
            border_color: (shad_theme.color_clear)
            border_color_hover: (shad_theme.color_clear)
            border_color_focus: (shad_theme.color_clear)
            border_color_down: (shad_theme.color_clear)
            border_color_empty: (shad_theme.color_clear)
            border_color_disabled: (shad_theme.color_clear)
        }
        padding: Inset{left: 0, right: 0, top: 12, bottom: 12}
    }

    mod.widgets.ShadInputShell = mod.widgets.View {
        width: Fill
        height: Fit
        flow: Right
        align: Align{y: 0.5}
        padding: Inset{left: 12, right: 12, top: 0, bottom: 0}
        spacing: 8.0

        draw_bg +: {
            border_radius: (shad_theme.radius)
            border_size: (shad_theme.border_size)

            color: (shad_theme.color_clear)
            border_color: (shad_theme.color_outline_border)
        }

        leading := mod.widgets.View{
            width: Fit
            height: Fit
        }

        input := mod.widgets.ShadInputBorderless {
            width: Fill
        }

        trailing := mod.widgets.View{
            width: Fit
            height: Fit
        }
    }

    // A composite View containing an Icon and a borderless Input field.
    // The shell provides borders and focus rings; callers can override slots.
    mod.widgets.ShadInputWithIcon = mod.widgets.ShadInputShell {
        leading := mod.widgets.IconSearch {
            draw_icon.color: (shad_theme.color_muted_foreground)
        }

        input := mod.widgets.ShadInputBorderless {
            empty_text: "Search..."
        }
    }

    mod.widgets.ShadSearchInputBase = #(ShadSearchInput::register_widget(vm))
    mod.widgets.ShadSearchInput = set_type_default() do mod.widgets.ShadSearchInputBase{
        width: Fill
        height: Fit
        empty_text: "Search..."
        clear_button_text: "Clear"
        show_clear_button: true

        search_shell := mod.widgets.ShadInputShell{
            padding: Inset{left: 14, right: 8, top: 0, bottom: 0}
            spacing: 10.0

            draw_bg +: {
                color: (shad_theme.color_secondary)
                border_radius: 12.0
                border_size: (shad_theme.border_size)
                border_color: (shad_theme.color_outline_border)
            }

            leading := IconSearch{
                icon_walk: Walk{width: 18, height: 18}
                draw_icon.color: (shad_theme.color_muted_foreground)
            }

            input := mod.widgets.ShadInputBorderless{
                width: Fill
                empty_text: "Search..."
                draw_text.text_style.font_size: 14
                draw_text.color_empty: (shad_theme.color_muted_foreground)
            }

            trailing := View{
                width: Fit
                height: Fit

                clear_btn := ShadButton{
                    variant: ShadButtonVariant.Ghost
                    text: "Clear"
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ShadSearchInputAction {
    Changed(String),
    Submitted(String),
    Cleared,
    #[default]
    None,
}

#[derive(Script, Widget)]
pub struct ShadSearchInput {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[live]
    empty_text: ArcStringMut,
    #[live]
    clear_button_text: ArcStringMut,
    #[live(true)]
    show_clear_button: bool,
    #[rust]
    text_cache: String,
    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}

impl ScriptHook for ShadSearchInput {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        vm.with_cx_mut(|cx| {
            self.input_ref(cx)
                .set_empty_text(cx, self.empty_text.as_ref().to_string());
            self.view
                .widget(cx, ids!(search_shell.trailing.clear_btn))
                .set_text(cx, self.clear_button_text.as_ref());
            self.sync_clear_button(cx);
        });
    }
}

impl ShadSearchInput {
    fn input_ref(&self, cx: &Cx) -> TextInputRef {
        self.view.text_input(cx, ids!(search_shell.input))
    }

    fn clear_button_ref(&self, cx: &Cx) -> WidgetRef {
        self.view.widget(cx, ids!(search_shell.trailing.clear_btn))
    }

    fn sync_clear_button(&self, cx: &mut Cx) {
        let clear = self.clear_button_ref(cx);
        clear.set_visible(cx, self.show_clear_button);
        clear.set_disabled(cx, !(self.show_clear_button && !self.text_cache.is_empty()));
    }

    fn clear_internal(&mut self, cx: &mut Cx) {
        self.text_cache.clear();
        self.input_ref(cx).set_text(cx, "");
        self.sync_clear_button(cx);
    }

    fn emit_changed(&self, cx: &mut Cx, text: String) {
        emit_widget_action(
            cx,
            &self.action_data,
            self.widget_uid(),
            ShadSearchInputAction::Changed(text),
        );
    }

    fn emit_submitted(&self, cx: &mut Cx, text: String) {
        emit_widget_action(
            cx,
            &self.action_data,
            self.widget_uid(),
            ShadSearchInputAction::Submitted(text),
        );
    }

    fn emit_cleared(&self, cx: &mut Cx) {
        emit_widget_action(
            cx,
            &self.action_data,
            self.widget_uid(),
            ShadSearchInputAction::Cleared,
        );
    }

    pub fn set_text(&mut self, cx: &mut Cx, text: &str) {
        if self.text_cache == text {
            return;
        }
        self.text_cache.clear();
        self.text_cache.push_str(text);
        self.input_ref(cx).set_text(cx, text);
        self.sync_clear_button(cx);
    }

    pub fn clear(&mut self, cx: &mut Cx) {
        self.clear_internal(cx);
    }

    pub fn text(&self) -> String {
        self.text_cache.clone()
    }

    pub fn focus(&self, cx: &mut Cx) {
        self.input_ref(cx).set_key_focus(cx);
    }

    pub fn changed(&self, actions: &Actions) -> Option<String> {
        widget_action_map::<ShadSearchInputAction, _, _>(actions, self.widget_uid(), |action| {
            if let ShadSearchInputAction::Changed(text) = action {
                Some(text)
            } else {
                None
            }
        })
    }

    pub fn submitted(&self, actions: &Actions) -> Option<String> {
        widget_action_map::<ShadSearchInputAction, _, _>(actions, self.widget_uid(), |action| {
            if let ShadSearchInputAction::Submitted(text) = action {
                Some(text)
            } else {
                None
            }
        })
    }

    pub fn cleared(&self, actions: &Actions) -> bool {
        widget_action_map::<ShadSearchInputAction, _, _>(actions, self.widget_uid(), |action| {
            matches!(action, ShadSearchInputAction::Cleared).then_some(())
        })
        .is_some()
    }
}

impl Widget for ShadSearchInput {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            let input = self.input_ref(cx);
            if let Some(text) = input.changed(actions) {
                self.text_cache = text.clone();
                self.sync_clear_button(cx);
                self.emit_changed(cx, text);
            }

            if let Some((text, _modifiers)) = input.returned(actions) {
                self.text_cache = text.clone();
                self.sync_clear_button(cx);
                self.emit_submitted(cx, text);
            }

            if input.escaped(actions) && !self.text_cache.is_empty() {
                self.clear_internal(cx);
                input.set_key_focus(cx);
                self.emit_cleared(cx);
                self.emit_changed(cx, String::new());
                return;
            }

            if self.show_clear_button
                && actions
                    .find_widget_action(self.clear_button_ref(cx).widget_uid())
                    .is_some_and(|action| matches!(action.cast(), ButtonAction::Clicked(_)))
                && !self.text_cache.is_empty()
            {
                self.clear_internal(cx);
                input.set_key_focus(cx);
                self.emit_cleared(cx);
                self.emit_changed(cx, String::new());
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ShadSearchInputRef {
    pub fn set_text(&self, cx: &mut Cx, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_text(cx, text);
        }
    }

    pub fn clear(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.clear(cx);
        }
    }

    pub fn text(&self) -> String {
        self.borrow().map_or_else(String::new, |inner| inner.text())
    }

    pub fn focus(&self, cx: &mut Cx) {
        if let Some(inner) = self.borrow() {
            inner.focus(cx);
        }
    }

    pub fn key_focus(&self, cx: &Cx) -> bool {
        self.borrow()
            .is_some_and(|inner| inner.input_ref(cx).key_focus(cx))
    }

    pub fn changed(&self, actions: &Actions) -> Option<String> {
        self.borrow().and_then(|inner| inner.changed(actions))
    }

    pub fn submitted(&self, actions: &Actions) -> Option<String> {
        self.borrow().and_then(|inner| inner.submitted(actions))
    }

    pub fn cleared(&self, actions: &Actions) -> bool {
        self.borrow().is_some_and(|inner| inner.cleared(actions))
    }
}
