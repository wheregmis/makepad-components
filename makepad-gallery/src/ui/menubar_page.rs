use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::button::ShadButtonWidgetExt;
use makepad_components::makepad_widgets::*;
use makepad_components::popover::ShadPopoverWidgetExt;

gallery_stateful_page_shell! {
    widget: GalleryMenubarPage,
    page: menubar_page,
    title: "Menubar",
    subtitle: "Compact application menus built from horizontal triggers and popover-backed menu bodies. `ShadMenubarMenu` reuses `ShadPopover`, so item clicks, close control, and outside-click dismissal follow the same API as popovers.",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        ShadSectionHeader{ text: "Application menu" }

        menubar_demo := ShadMenubar{
            file_menu := ShadMenubarMenu{
                trigger := ShadMenubarTrigger{text: "File"}

                content: ShadMenubarContent{
                    ShadMenubarLabel{text: "Project"}
                    file_new_btn := ShadMenubarItem{text: "New file"}
                    file_open_btn := ShadMenubarItem{text: "Open recent"}
                    ShadMenubarSeparator{}
                    file_share_btn := ShadMenubarItem{text: "Share"}
                }
            }

            edit_menu := ShadMenubarMenu{
                trigger := ShadMenubarTrigger{text: "Edit"}

                content: ShadMenubarContent{
                    ShadMenubarLabel{text: "History"}
                    edit_undo_btn := ShadMenubarItem{text: "Undo"}
                    edit_redo_btn := ShadMenubarItem{text: "Redo"}
                    ShadMenubarSeparator{}
                    edit_find_btn := ShadMenubarItem{text: "Find in files"}
                }
            }

            view_menu := ShadMenubarMenu{
                trigger := ShadMenubarTrigger{text: "View"}

                content: ShadMenubarContent{
                    ShadMenubarLabel{text: "Workspace"}
                    view_toggle_sidebar_btn := ShadMenubarItem{text: "Toggle sidebar"}
                    view_zen_mode_btn := ShadMenubarItem{text: "Enter zen mode"}
                }
            }
        }

        menubar_status := ShadFieldDescription{
            text: "Choose a menubar action."
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. `ShadMenubar` is the horizontal shell; each `ShadMenubarMenu` is a hover-open anchored popover with menubar styling."}
        mod.widgets.GalleryActionFlowStep{text: "2. Put `ShadMenubarItem` buttons inside `content`, then reach them through `content_widget()` from the menu ref."}
        mod.widgets.GalleryActionFlowStep{text: "3. Menubar menus share a hover group, so opening one closes its siblings without page-level glue."}
        mod.widgets.GalleryActionFlowStep{text: "4. Keep command execution in page or app code. The menubar primitives handle layout and popup presentation, not document actions."}
    },
}

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryMenubarPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl GalleryMenubarPage {
    fn set_status(&self, cx: &mut Cx, text: &str) {
        self.view.label(cx, ids!(menubar_status)).set_text(cx, text);
    }
}

impl Widget for GalleryMenubarPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            let file_menu = self.view.shad_popover(cx, ids!(file_menu));
            let file_content = file_menu.content_widget();
            if file_content.shad_button(cx, ids!(file_new_btn)).clicked(actions) {
                file_menu.close(cx);
                self.set_status(cx, "Selected File -> New file");
                return;
            }
            if file_content
                .shad_button(cx, ids!(file_open_btn))
                .clicked(actions)
            {
                file_menu.close(cx);
                self.set_status(cx, "Selected File -> Open recent");
                return;
            }
            if file_content
                .shad_button(cx, ids!(file_share_btn))
                .clicked(actions)
            {
                file_menu.close(cx);
                self.set_status(cx, "Selected File -> Share");
                return;
            }

            let edit_menu = self.view.shad_popover(cx, ids!(edit_menu));
            let edit_content = edit_menu.content_widget();
            if edit_content
                .shad_button(cx, ids!(edit_undo_btn))
                .clicked(actions)
            {
                edit_menu.close(cx);
                self.set_status(cx, "Selected Edit -> Undo");
                return;
            }
            if edit_content
                .shad_button(cx, ids!(edit_redo_btn))
                .clicked(actions)
            {
                edit_menu.close(cx);
                self.set_status(cx, "Selected Edit -> Redo");
                return;
            }
            if edit_content
                .shad_button(cx, ids!(edit_find_btn))
                .clicked(actions)
            {
                edit_menu.close(cx);
                self.set_status(cx, "Selected Edit -> Find in files");
                return;
            }

            let view_menu = self.view.shad_popover(cx, ids!(view_menu));
            let view_content = view_menu.content_widget();
            if view_content
                .shad_button(cx, ids!(view_toggle_sidebar_btn))
                .clicked(actions)
            {
                view_menu.close(cx);
                self.set_status(cx, "Selected View -> Toggle sidebar");
                return;
            }
            if view_content
                .shad_button(cx, ids!(view_zen_mode_btn))
                .clicked(actions)
            {
                view_menu.close(cx);
                self.set_status(cx, "Selected View -> Enter zen mode");
                return;
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
