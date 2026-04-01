use makepad_widgets::*;
use std::sync::Arc;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadTabs = View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 12.0
    }

    mod.widgets.ShadTabsList = RoundedView{
        width: Fit
        height: Fit
        flow: Right
        spacing: 4.0
        padding: Inset{left: 4, right: 4, top: 4, bottom: 4}

        draw_bg +: {
            color: (shad_theme.color_muted)
            border_radius: (shad_theme.radius)
            border_size: (shad_theme.border_size)
            border_color: (shad_theme.color_outline_border)
        }
    }

    mod.widgets.ShadTabsTrigger = ButtonFlat{
        height: 32
        padding: Inset{left: 12, right: 12, top: 0, bottom: 0}

        draw_bg +: {
            border_radius: (shad_theme.radius)
            border_size: 0.0
            color: (shad_theme.color_clear)
            color_hover: (shad_theme.color_ghost_hover)
            color_down: (shad_theme.color_ghost_down)
            color_disabled: (shad_theme.color_disabled)
        }

        draw_text +: {
            color: (shad_theme.color_muted_foreground)
            color_hover: (shad_theme.color_primary)
            color_down: (shad_theme.color_primary)
            color_disabled: (shad_theme.color_disabled_foreground)
            text_style.font_size: 11.0
        }
    }

    mod.widgets.ShadTabsIndicator = SolidView{
        width: Fill
        height: 2
        draw_bg.color: (shad_theme.color_primary)
    }

    mod.widgets.ShadTabsContent = RoundedView{
        width: Fill
        height: Fit
        flow: Down
        spacing: 10.0
        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}

        draw_bg +: {
            color: (shad_theme.color_clear)
            border_radius: (shad_theme.radius)
            border_size: (shad_theme.border_size)
            border_color: (shad_theme.color_outline_border)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShadTabSpec {
    pub value: LiveId,
    trigger_path: Arc<[LiveId]>,
    indicator_path: Arc<[LiveId]>,
}

impl ShadTabSpec {
    pub fn new(value: LiveId, trigger_path: &[LiveId], indicator_path: &[LiveId]) -> Self {
        Self {
            value,
            trigger_path: Arc::from(trigger_path),
            indicator_path: Arc::from(indicator_path),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ShadTabsController {
    selected: LiveId,
    tabs: Arc<[ShadTabSpec]>,
}

impl ShadTabsController {
    pub fn new(selected: LiveId, tabs: Vec<ShadTabSpec>) -> Self {
        Self {
            selected,
            tabs: Arc::from(tabs),
        }
    }

    pub fn selected(&self) -> LiveId {
        self.selected
    }

    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }

    pub fn set_selected(&mut self, cx: &mut Cx, root: &WidgetRef, selected: LiveId) -> bool {
        if self.selected == selected {
            self.sync(cx, root);
            return false;
        }
        self.selected = selected;
        self.sync(cx, root);
        true
    }

    pub fn sync(&self, cx: &mut Cx, root: &WidgetRef) {
        for tab in self.tabs.iter() {
            root.view(cx, tab.indicator_path.as_ref())
                .set_visible(cx, tab.value == self.selected);
        }
    }

    pub fn changed(&mut self, cx: &mut Cx, root: &WidgetRef, actions: &Actions) -> Option<LiveId> {
        let selected = self.tabs.iter().find_map(|tab| {
            root.button(cx, tab.trigger_path.as_ref())
                .clicked(actions)
                .then_some(tab.value)
        });

        if let Some(selected) = selected {
            self.set_selected(cx, root, selected);
        }

        selected
    }
}
