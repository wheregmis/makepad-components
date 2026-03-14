use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::context_menu::ShadContextMenuWidgetExt;
use makepad_components::makepad_widgets::*;

gallery_stateful_page_shell! {
    widget: GalleryContextMenuPage,
    page: context_menu_page,
    title: "Context Menu",
    subtitle: "Right click or long press a trigger area to reveal contextual actions. Read the selected item with ShadContextMenuRef::changed(actions).",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Basic" }

        context_menu_basic := ShadContextMenu{
            labels: ["Open" "Duplicate" "Share" "Delete"]

            RoundedView{
                width: 360
                height: Fit
                flow: Down
                spacing: 6.0
                padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                draw_bg +: {
                    color: (shad_theme.color_secondary)
                    border_size: 1.0
                    border_radius: (shad_theme.radius)
                    border_color: (shad_theme.color_outline_border)
                }

                ShadLabel{text: "Project brief.md"}
                ShadFieldDescription{text: "Right click this card to open the menu."}
            }
        }

        context_menu_status := ShadFieldDescription{
            text: "No action selected yet."
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Compose the trigger area as the child of ShadContextMenu; the component owns the popup internals."}
        mod.widgets.GalleryActionFlowStep{text: "2. Read changed(actions) from ShadContextMenuRef to get the chosen item index."}
        mod.widgets.GalleryActionFlowStep{text: "3. Translate that index into domain actions like Open, Duplicate, Share, or Delete inside the page/controller."}
        mod.widgets.GalleryActionFlowStep{text: "4. Update visible state, status text, or execute commands without touching popup menu internals."}
    },
}

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryContextMenuPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryContextMenuPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            if let Some(index) = self
                .view
                .shad_context_menu(cx, ids!(context_menu_basic))
                .changed(actions)
            {
                let label = match index {
                    0 => "Open",
                    1 => "Duplicate",
                    2 => "Share",
                    3 => "Delete",
                    _ => "Unknown",
                };
                self.view
                    .label(cx, ids!(context_menu_status))
                    .set_text(cx, &format!("Selected: {}", label));
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
