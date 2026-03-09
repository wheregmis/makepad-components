use crate::ui::snippets::DRAWER_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryDrawerPage = ShadScrollYView{
        ShadPageTitle{
            text: "Drawer"
        }

        ShadPageSubtitle{
            text: "Slide-out panel (top). Use set_open(bool) and is_open() to control visibility."
        }

        ShadHr{}

        drawer_preview_section := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

            drawer_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                drawer_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    drawer_demo_tab := mod.widgets.ShadButtonGhost{text: "DEMO" padding: Inset{}}

                    drawer_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                drawer_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    drawer_code_tab := mod.widgets.ShadButtonGhost{text: "CODE" padding: Inset{}}

                    drawer_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            drawer_preview_panel := mod.widgets.ShadPanel{
                drawer_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 24.0

                        ShadSectionHeader{ text: "Default" }

                        open_drawer_btn := mod.widgets.ShadButton{
                            text: "Open drawer"
                        }

                        View{
                            width: Fill
                            height: 280
                            drawer_demo := ShadDrawer{
                                width: Fill
                                height: Fill
                                open: false
                            }
                        }
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(DRAWER_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
