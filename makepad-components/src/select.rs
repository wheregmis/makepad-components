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
            color: (shad_theme.color_primary)
            color_hover: (shad_theme.color_primary)
            color_active: (shad_theme.color_primary)
            color_disabled: (shad_theme.color_muted_foreground)
            text_style.font_size: 11
        }

        draw_bg +: {
            border_size: 0.0
            border_radius: 6.0
            color: #0000
            color_hover: (shad_theme.color_secondary)
            color_active: (shad_theme.color_secondary_hover)
            color_disabled: #0000
            border_color: #0000
            border_color_hover: #0000
            border_color_active: #0000
            border_color_disabled: #0000
            mark_color: #0000
            mark_color_active: (shad_theme.color_primary)
            mark_color_disabled: (shad_theme.color_muted_foreground)
            color_dither: 0.0
        }
    }

    mod.widgets.ShadSelectPopupMenu = mod.widgets.PopupMenu{
        width: 220
        padding: Inset{left: 4, right: 4, top: 4, bottom: 4}
        menu_item: mod.widgets.ShadSelectItem{}

        draw_bg +: {
            border_size: 1.0
            border_radius: (shad_theme.radius)
            color: (shad_theme.color_background)
            border_color: (shad_theme.color_outline_border)
            color_dither: 0.0
        }
    }

    mod.widgets.ShadSelect = mod.widgets.DropDownFlat{
        width: 220
        height: 36
        align: Align{x: 0.0, y: 0.5}

        padding: Inset{left: 12, right: 28, top: 0, bottom: 0}

        draw_text +: {
            color: (shad_theme.color_primary)
            color_hover: (shad_theme.color_primary)
            color_focus: (shad_theme.color_primary)
            color_down: (shad_theme.color_primary)
            color_disabled: (shad_theme.color_muted_foreground)
            text_style.font_size: 11
        }

        draw_bg +: {
            border_radius: (shad_theme.radius)
            border_size: 1.0
            color: #0000
            color_hover: (shad_theme.color_ghost_hover)
            color_focus: (shad_theme.color_ghost_hover)
            color_down: (shad_theme.color_ghost_down)
            color_active: (shad_theme.color_ghost_hover)
            color_disabled: #0000
            border_color: (shad_theme.color_outline_border)
            border_color_hover: (shad_theme.color_outline_border_hover)
            border_color_focus: (shad_theme.color_primary)
            border_color_down: (shad_theme.color_primary)
            border_color_active: (shad_theme.color_primary)
            border_color_disabled: (shad_theme.color_outline_border)
            arrow_color: (shad_theme.color_primary)
            arrow_color_hover: (shad_theme.color_primary)
            arrow_color_focus: (shad_theme.color_primary)
            arrow_color_down: (shad_theme.color_primary)
            arrow_color_disabled: (shad_theme.color_muted_foreground)
        }

        popup_menu: mod.widgets.ShadSelectPopupMenu{}
    }
}

#[derive(Clone, Default)]
pub struct ShadSelectRef(pub WidgetRef);

pub trait ShadSelectWidgetExt: Widget {
    fn shad_select(&self, cx: &Cx, path: &[LiveId]) -> ShadSelectRef {
        ShadSelectRef(self.widget(cx, path))
    }
}

impl<T: Widget + ?Sized> ShadSelectWidgetExt for T {}

impl ShadSelectRef {
    pub fn changed(&self, actions: &Actions) -> Option<usize> {
        if let Some(item) = actions.find_widget_action(self.0.widget_uid()) {
            if let DropDownAction::Select(index) = item.cast() {
                return Some(index);
            }
        }
        None
    }

    pub fn changed_label(&self, cx: &Cx, actions: &Actions) -> Option<String> {
        self.0.drop_down(cx, &[]).changed_label(actions)
    }

    pub fn set_selected_item(&self, cx: &mut Cx, item: usize) {
        self.0.drop_down(cx, &[]).set_selected_item(cx, item);
    }

    pub fn selected_item(&self, cx: &Cx) -> usize {
        self.0.drop_down(cx, &[]).selected_item()
    }

    pub fn selected_label(&self, cx: &Cx) -> String {
        self.0.drop_down(cx, &[]).selected_label()
    }

    pub fn set_selected_by_label(&self, label: &str, cx: &mut Cx) {
        self.0.drop_down(cx, &[]).set_selected_by_label(label, cx);
    }

    pub fn set_labels(&self, cx: &mut Cx, labels: Vec<String>) {
        self.0.drop_down(cx, &[]).set_labels(cx, labels);
    }
}
