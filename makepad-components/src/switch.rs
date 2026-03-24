use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    // Single switch/toggle component: themed Toggle (same as gallery uses)
    mod.widgets.ShadSwitch = Toggle{
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 10

        draw_bg.border_color: (shad_theme.color_outline_border)
        draw_bg.border_color_active: (shad_theme.color_outline_border)
        draw_bg.border_color_hover: (shad_theme.color_outline_border_hover)
    }
}

#[derive(Clone, Default)]
pub struct ShadSwitchRef(pub WidgetRef);

pub trait ShadSwitchWidgetExt: Widget {
    fn shad_switch(&self, cx: &Cx, path: &[LiveId]) -> ShadSwitchRef {
        ShadSwitchRef(self.widget(cx, path))
    }
}

impl<T: Widget + ?Sized> ShadSwitchWidgetExt for T {}

impl ShadSwitchRef {
    pub fn changed(&self, actions: &Actions) -> Option<bool> {
        self.0
            .borrow::<CheckBox>()
            .and_then(|inner| inner.changed(actions))
    }

    pub fn active(&self, cx: &Cx) -> bool {
        self.0
            .borrow::<CheckBox>()
            .is_some_and(|inner| inner.active(cx))
    }

    pub fn set_active(&self, cx: &mut Cx, value: bool) {
        if let Some(mut inner) = self.0.borrow_mut::<CheckBox>() {
            inner.set_active(cx, value);
        }
    }
}
