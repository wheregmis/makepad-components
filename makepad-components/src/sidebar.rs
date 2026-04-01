use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadSidebar = View{
        width: 280
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_secondary)
        padding: Inset{top: 14, right: 14, bottom: 14, left: 14}
        spacing: 10.0
    }

    mod.widgets.ShadSidebarSectionLabel = Label{
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 9
    }

    mod.widgets.ShadSidebarItem = mod.widgets.ShadNavButtonBase{
        width: Fill
        height: 32
        padding: Inset{left: 10, right: 10}
        align: Align{x: 0.0, y: 0.5}
        reset_hover_on_click: true
        grab_key_focus: true
        draw_bg +: {
            color: (shad_theme.color_clear)
            color_hover: (shad_theme.color_secondary_hover)
            color_down: (shad_theme.color_secondary_down)
            color_focus: (shad_theme.color_secondary_hover)
            color_active: (shad_theme.color_secondary_hover)
            border_radius: (shad_theme.radius)
            border_size: 0.0
            border_color: (shad_theme.color_clear)
            border_color_active: (shad_theme.color_clear)
        }
        draw_text +: {
            color: (shad_theme.color_muted_foreground)
            color_hover: (shad_theme.color_primary)
            color_down: (shad_theme.color_primary)
            color_focus: (shad_theme.color_primary)
            color_active: (shad_theme.color_primary)
            text_style.font_size: 10.0
        }
        text: "Item"
    }
}

#[derive(Clone, Default)]
pub struct ShadSidebarItemRef(pub WidgetRef);

pub trait ShadSidebarWidgetExt {
    fn shad_sidebar_item(&self, cx: &Cx, path: &[LiveId]) -> ShadSidebarItemRef;
}

impl<T: Widget + ?Sized> ShadSidebarWidgetExt for T {
    fn shad_sidebar_item(&self, cx: &Cx, path: &[LiveId]) -> ShadSidebarItemRef {
        ShadSidebarItemRef(self.widget(cx, path))
    }
}

impl ShadSidebarItemRef {
    pub fn clicked(&self, actions: &Actions) -> bool {
        actions
            .find_widget_action(self.0.widget_uid())
            .is_some_and(|action| matches!(action.cast(), ButtonAction::Clicked(_)))
    }

    pub fn set_active(&self, cx: &mut Cx, active: bool) {
        if let Some(mut inner) = self.0.borrow_mut::<crate::button::ShadNavButton>() {
            inner.set_active(cx, active, Animate::Yes);
        }
    }

    pub fn is_active(&self) -> bool {
        self.0
            .borrow::<crate::button::ShadNavButton>()
            .is_some_and(|inner| inner.is_active())
    }
}
