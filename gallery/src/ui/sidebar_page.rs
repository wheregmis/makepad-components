use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySidebarPage = ShadScrollYView{
        ShadPageTitle{
            text: "Sidebar"
        }

        ShadPageSubtitle{
            text: "Composable sidebar primitives used by the gallery navigation."
        }

        ShadHr{}

        sidebar_preview_section := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

            sidebar_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                sidebar_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    sidebar_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                    sidebar_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                sidebar_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    sidebar_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                    sidebar_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            sidebar_preview_panel := mod.widgets.ShadPanel{
                sidebar_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    spacing: 12.0
                    align: Align{y: 0.0}

                    ShadSidebar{
                        width: 300
                        height: 320
                        ShadLabel{
                            text: "Acme Inc"
                            draw_text.text_style.font_size: 12
                        }
                        ShadSidebarSectionLabel{text: "Platform"}
                        ShadSidebarItem{text: "Playground"}
                        ShadSidebarItem{text: "History"}
                        ShadSidebarItem{text: "Settings"}
                    }

                    View{
                        width: Fill
                        height: 320
                        draw_bg.color: #0000
                        draw_bg.border_size: 1.0
                        draw_bg.border_color: (shad_theme.color_outline_border)
                        draw_bg.border_radius: (shad_theme.radius)
                    }
                }
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: "mod.widgets.ShadSidebar{\n    width: 300\n    Label{text: \"Acme Inc\"}\n    ShadSidebarSectionLabel{text: \"Platform\"}\n    ShadSidebarItem{text: \"Playground\"}\n    ShadSidebarItem{text: \"History\"}\n}"
                        }
                    }
                }
            }
        }
    }
}
