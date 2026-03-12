use crate::ui::snippets::KBD_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryKbdPage = ShadScrollYView{
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
                height: Fit
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
                kbd_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
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

                            ShadKbd{ label := IconCommand{ icon_walk: Walk{width: 12 height: 12} draw_icon.color: (shad_theme.color_kbd_foreground) } }
                            ShadKbd{ label := IconShift{ icon_walk: Walk{width: 12 height: 12} draw_icon.color: (shad_theme.color_kbd_foreground) } }
                            ShadKbd{ label := IconOption{ icon_walk: Walk{width: 12 height: 12} draw_icon.color: (shad_theme.color_kbd_foreground) } }
                            ShadKbd{ label := IconControl{ icon_walk: Walk{width: 12 height: 12} draw_icon.color: (shad_theme.color_kbd_foreground) } }
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

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(KBD_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
