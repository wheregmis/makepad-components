use crate::ui::snippets::KBD_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryKbdPageBase = #(GalleryKbdPage::register_widget(vm))

    mod.widgets.GalleryKbdPage = set_type_default() do mod.widgets.GalleryKbdPageBase{
        width: Fill
        height: Fill

        scroll_view := ShadScrollYView{
            ShadPageTitle{
                text: "Kbd"
            }

            ShadPageSubtitle{
                text: "Keyboard shortcut key caps for displaying shortcuts (e.g. ⌘ ⇧ ⌥ ⌃ or Ctrl + B)."
            }

            ShadHr{}

            kbd_preview_section := View{
                width: Fill
                height: Fit
                flow: Down

                kbd_tabs_row := View{
                    width: Fit
                    visible: false
                    height: 0
                    flow: Right
                    spacing: 20.0
                    margin: Inset{top: 4, bottom: 12}

                    kbd_demo_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        kbd_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                        kbd_demo_indicator := SolidView{
                            width: Fill
                            height: 2
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }

                    kbd_code_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        kbd_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                        kbd_code_indicator := SolidView{
                            width: Fill
                            height: 2
                            visible: false
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }
                }

                kbd_preview_panel := mod.widgets.ShadPanel{
                    kbd_preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                        width: Fill
                        height: Fit

                        root_view +: {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            ShadSectionHeader{ text: "Modifier keys" }

                            View{
                                width: Fit
                                height: Fit
                                flow: Right
                                spacing: 6.0
                                align: Align{y: 0.5}

                                ShadKbd{ label := ShadKbdLabel{text: "Cmd"} }
                                ShadKbd{ label := ShadKbdLabel{text: "Shift"} }
                                ShadKbd{ label := ShadKbdLabel{text: "Option"} }
                                ShadKbd{ label := ShadKbdLabel{text: "Ctrl"} }
                            }

                            ShadSectionHeader{ text: "Shortcut" }

                            View{
                                width: Fit
                                height: Fit
                                flow: Right
                                spacing: 6.0
                                align: Align{y: 0.5}

                                ShadKbd{ label := ShadKbdLabel{text: "Ctrl"} }
                                ShadKbdSeparator{}
                                ShadKbd{ label := ShadKbdLabel{text: "B"} }
                            }
                        }

                        code_page +: {
                            body +: {
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12.0

                                GalleryCodeSnippet{
                                    code_view +: { text: #(KBD_PREVIEW_CODE) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryKbdPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryKbdPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
