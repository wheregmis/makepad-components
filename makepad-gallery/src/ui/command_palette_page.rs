use crate::ui::page_macros::gallery_stateful_page_shell;
use crate::ui::GalleryShellAction;
use makepad_components::makepad_widgets::widget::WidgetActionData;
use makepad_components::makepad_widgets::*;

gallery_stateful_page_shell! {
    widget: GalleryCommandPalettePage,
    page: command_palette_page,
    title: "Command Palette",
    subtitle: "Global launcher for component pages. Press Command/Ctrl + K anywhere in the gallery, or use the trigger below. This page emits an open request; the app shell owns the shared overlay.",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        ShadSectionHeader{ text: "Open the palette" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

            ShadSurface{
                width: Fill
                height: Fit
                flow: Down
                spacing: 10.0
                padding: Inset{left: 18, right: 18, top: 18, bottom: 18}
                draw_bg +: {
                    color: (shad_theme.color_secondary)
                    border_size: 1.0
                    border_color: (shad_theme.color_outline_border)
                }

                ShadLabel{text: "Use the global launcher to jump between components faster."}
                ShadFieldDescription{text: "The same overlay opens from this button or from the keyboard shortcut."}

                View{
                    width: Fit
                    height: Fit
                    flow: Right
                    spacing: 8.0
                    align: Align{y: 0.5}

                    open_command_palette_btn := ShadButton{text: "Open Command Palette"}

                    ShadKbd{ label := ShadKbdLabel{text: "Cmd"} }
                    ShadKbdSeparator{}
                    ShadKbd{ label := ShadKbdLabel{text: "K"} }

                    ShadKbd{ label := ShadKbdLabel{text: "Ctrl"} }
                    ShadKbdSeparator{}
                    ShadKbd{ label := ShadKbdLabel{text: "K"} }
                }
            }

            ShadFieldDescription{
                text: "Expected behavior: search filters live, Up/Down changes selection, Enter opens the highlighted page, and Escape dismisses the modal."
            }

        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. This page does not open the shared palette directly; it emits GalleryShellAction::OpenCommandPalette."}
        mod.widgets.GalleryActionFlowStep{text: "2. The app shell listens for GalleryShellAction::OpenCommandPalette and opens the global overlay."}
        mod.widgets.GalleryActionFlowStep{text: "3. The overlay remains shell-owned, so no page-internal button ids leak into main.rs."}
        mod.widgets.GalleryActionFlowStep{text: "4. When a command is chosen, the palette emits a semantic selection back to the shell for routing."}
    },
}

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryCommandPalettePage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}

impl Widget for GalleryCommandPalettePage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            if self
                .view
                .button(cx, ids!(open_command_palette_btn))
                .clicked(actions)
            {
                cx.widget_action_with_data(
                    &self.action_data,
                    self.widget_uid(),
                    GalleryShellAction::OpenCommandPalette,
                );
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
