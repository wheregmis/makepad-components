use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSelectItem = mod.widgets.PopupMenuItem{
        width: Fill
        height: Fit
        align: Align{y: 0.5}
        padding: Inset{left: 24, right: 12, top: 8, bottom: 8}

        draw_text +: {
            color: (shad_theme.color_text)
            color_hover: (shad_theme.color_text)
            color_active: (shad_theme.color_text)
            color_disabled: (shad_theme.color_text_muted)
            text_style.font_size: (shad_theme.control_font_size_md)
        }

        draw_bg +: {
            border_size: 0.0
            border_radius: (shad_theme.radius)
            color: #0000
            color_hover: (shad_theme.color_secondary)
            color_active: (shad_theme.color_secondary_hover)
            color_disabled: #0000
            border_color: #0000
            border_color_hover: #0000
            border_color_active: #0000
            border_color_disabled: #0000
            mark_color: #0000
            mark_color_active: (shad_theme.color_text)
            mark_color_disabled: (shad_theme.color_text_muted)
            color_dither: 0.0
        }
    }

    mod.widgets.ShadSelectPopupMenu = mod.widgets.PopupMenu{
        width: 220
        padding: Inset{left: 6, right: 6, top: 6, bottom: 6}
        menu_item: mod.widgets.ShadSelectItem{}

        draw_bg +: {
            border_size: 1.0
            border_radius: (shad_theme.radius)
            color: (shad_theme.color_surface_popover)
            border_color: (shad_theme.color_border)
            color_dither: 0.0
        }
    }

    mod.widgets.ShadSelectBase = #(ShadSelect::register_widget(vm))

    mod.widgets.ShadSelect = mod.widgets.ShadSelectBase{
        width: 220
        height: (shad_theme.control_height_md)
        align: Align{x: 0.0, y: 0.5}

        padding: Inset{
            left: (shad_theme.control_padding_x_md - 4.0),
            right: (shad_theme.control_padding_x_md + 12.0),
            top: 0,
            bottom: 0
        }

        draw_text +: {
            color: (shad_theme.color_text)
            color_hover: (shad_theme.color_text)
            color_focus: (shad_theme.color_text)
            color_down: (shad_theme.color_text)
            color_disabled: (shad_theme.color_text_muted)
            text_style.font_size: (shad_theme.control_font_size_md)
        }

        draw_bg +: {
            border_radius: (shad_theme.radius)
            border_size: 1.0
            color: (shad_theme.color_surface_default)
            color_hover: (shad_theme.color_surface_default)
            color_focus: (shad_theme.color_surface_default)
            color_down: (shad_theme.color_surface_default)
            color_active: (shad_theme.color_surface_default)
            color_disabled: (shad_theme.color_surface_default)
            border_color: (shad_theme.color_border)
            border_color_hover: (shad_theme.color_border_hover)
            border_color_focus: (shad_theme.color_border_focus)
            border_color_down: (shad_theme.color_border_focus)
            border_color_active: (shad_theme.color_border_focus)
            border_color_disabled: (shad_theme.color_border)
            arrow_color: (shad_theme.color_text)
            arrow_color_hover: (shad_theme.color_text)
            arrow_color_focus: (shad_theme.color_text)
            arrow_color_down: (shad_theme.color_text)
            arrow_color_disabled: (shad_theme.color_text_muted)
        }

        popup_menu: mod.widgets.ShadSelectPopupMenu{}
    }
}

#[derive(Script, Widget)]
pub struct ShadSelect {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    dropdown: DropDown,
}

impl ScriptHook for ShadSelect {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _obj: ScriptValue,
    ) {
        vm.with_cx_mut(|cx| {
            script_apply_eval!(cx, self.dropdown, {
                popup_menu_position: #(PopupMenuPosition::BelowInput)
            });
        });
    }
}

impl Widget for ShadSelect {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.dropdown.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.dropdown.draw_walk(cx, walk);
        DrawStep::done()
    }
}
