use makepad_widgets::*;
use std::sync::Arc;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadRadioGroup = View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 10.0
    }

    mod.widgets.ShadRadioGroupInline = View{
        width: Fit
        height: Fit
        flow: Right
        spacing: 16.0
        align: Align{y: 0.5}
    }

    mod.widgets.ShadRadioItem = RadioButtonFlat{
        width: Fit
        height: Fit
        padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
        label_walk +: {
            margin: theme.mspace_h_1{left: 18.0}
        }

        draw_bg +: {
            size: 16.0
            color: #0000
            color_hover: #0000
            color_down: #0000
            color_focus: #0000
            color_disabled: #0000
            color_active: #0000

            border_color: (shad_theme.color_outline_border)
            border_color_hover: (shad_theme.color_outline_border_hover)
            border_color_down: (shad_theme.color_primary)
            border_color_focus: (shad_theme.color_primary)
            border_color_active: (shad_theme.color_primary)
            border_color_disabled: (shad_theme.color_outline_border)

            mark_color: #0000
            mark_color_active: (shad_theme.color_primary)
            mark_color_disabled: (shad_theme.color_muted_foreground)
        }

        draw_text +: {
            color: (shad_theme.color_primary)
            color_hover: (shad_theme.color_primary)
            color_down: (shad_theme.color_primary)
            color_active: (shad_theme.color_primary)
            color_focus: (shad_theme.color_primary)
            color_disabled: (shad_theme.color_muted_foreground)
            text_style.font_size: 11.0
        }
    }
}

#[derive(Clone, Default)]
pub struct ShadRadioGroupRef {
    root: WidgetRef,
    item_paths: Vec<Arc<[LiveId]>>,
}

pub trait ShadRadioGroupWidgetExt: Widget {
    fn shad_radio_group<'a, I>(&self, cx: &Cx, item_paths: I) -> ShadRadioGroupRef
    where
        I: IntoIterator<Item = &'a [LiveId]>,
    {
        ShadRadioGroupRef {
            root: self.widget(cx, &[]),
            item_paths: item_paths.into_iter().map(Arc::from).collect(),
        }
    }
}

impl<T: Widget + ?Sized> ShadRadioGroupWidgetExt for T {}

impl ShadRadioGroupRef {
    pub fn selected(&self, cx: &mut Cx, actions: &Actions) -> Option<usize> {
        let paths: Vec<&[LiveId]> = self.item_paths.iter().map(|path| path.as_ref()).collect();
        self.root
            .radio_button_set(cx, paths.as_slice())
            .selected(cx, actions)
    }

    pub fn active_index(&self, cx: &Cx) -> Option<usize> {
        self.item_paths
            .iter()
            .position(|path| self.root.radio_button(cx, path.as_ref()).active(cx))
    }

    pub fn set_selected(&self, cx: &mut Cx, selected: Option<usize>) {
        for (index, path) in self.item_paths.iter().enumerate() {
            self.root
                .radio_button(cx, path.as_ref())
                .set_active(cx, selected == Some(index));
        }
    }
}
