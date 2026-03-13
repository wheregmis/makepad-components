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

            kbd_preview_section := mod.widgets.GalleryPreviewSection{
                width: Fill
                height: Fit

                preview_panel +: {
                    preview_flip +: {
                        root_view +: {
                            preview_content +: {
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
                        }

                        code_page +: {
                            body +: {
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12.0

                                code_snippet +: {
                                    code: #(KBD_PREVIEW_CODE)
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
