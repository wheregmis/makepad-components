use crate::ui::snippets::COMMAND_PALETTE_PREVIEW_CODE;
use makepad_components::makepad_widgets::widget::WidgetActionData;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCommandPalettePageBase = #(GalleryCommandPalettePage::register_widget(vm))

    mod.widgets.GalleryCommandPalettePage = set_type_default() do mod.widgets.GalleryCommandPalettePageBase{
        width: Fill
        height: Fill

        scroll_view := ShadScrollYView{
            ShadPageTitle{
                text: "Command Palette"
            }

            ShadPageSubtitle{
                text: "Global launcher for component pages. Press Command/Ctrl + K anywhere in the gallery, or use the trigger below. This page emits an open request; the app shell owns the shared overlay."
            }

            ShadHr{}

            command_palette_preview_section := View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 12.0

                command_palette_tabs_row := View{
                    width: Fit
                    visible: false
                    height: 0
                    flow: Right
                    spacing: 20.0
                    margin: Inset{top: 4, bottom: 12}

                    command_palette_demo_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        command_palette_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                        command_palette_demo_indicator := SolidView{
                            width: Fill
                            height: 2
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }

                    command_palette_code_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        command_palette_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                        command_palette_code_indicator := SolidView{
                            width: Fill
                            height: 2
                            visible: false
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }
                }

                command_palette_preview_panel := mod.widgets.ShadPanel{
                    command_palette_preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                        width: Fill
                        height: Fit

                        root_view +: {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 16.0

                            ShadSectionHeader{ text: "Open the palette" }

                            View{
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12.0

                                RoundedView{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 10.0
                                    padding: Inset{left: 18, right: 18, top: 18, bottom: 18}
                                    draw_bg +: {
                                        color: (shad_theme.color_secondary)
                                        border_size: 1.0
                                        border_radius: (shad_theme.radius)
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

                                mod.widgets.GalleryActionFlow{
                                    body +: {
                                        mod.widgets.GalleryActionFlowStep{text: "1. This page does not open the shared palette directly; it emits GalleryCommandPalettePageAction::OpenRequested."}
                                        mod.widgets.GalleryActionFlowStep{text: "2. The app shell listens to command_palette_page.open_requested(actions) and opens the global overlay."}
                                        mod.widgets.GalleryActionFlowStep{text: "3. The overlay remains shell-owned, so no page-internal button ids leak into main.rs."}
                                        mod.widgets.GalleryActionFlowStep{text: "4. When a command is chosen, the palette emits a semantic selection back to the shell for routing."}
                                    }
                                }
                            }
                        }

                        code_page +: {
                            body +: {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            GalleryCodeSnippet{
                                code_view +: { text: #(COMMAND_PALETTE_PREVIEW_CODE) }
                            }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum GalleryCommandPalettePageAction {
    OpenRequested,
    #[default]
    None,
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

impl GalleryCommandPalettePage {
    pub fn open_requested(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            matches!(
                item.cast::<GalleryCommandPalettePageAction>(),
                GalleryCommandPalettePageAction::OpenRequested
            )
        } else {
            false
        }
    }
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
                    GalleryCommandPalettePageAction::OpenRequested,
                );
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl GalleryCommandPalettePageRef {
    pub fn open_requested(&self, actions: &Actions) -> bool {
        self.borrow()
            .is_some_and(|inner| inner.open_requested(actions))
    }
}
