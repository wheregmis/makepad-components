use crate::ui::snippets::RESIZABLE_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryResizablePage = ShadScrollArea{
        ShadPageTitle{
            text: "Resizable"
        }

        ShadPageSubtitle{
            text: "Two-pane layouts built on Makepad splitters with a canonical handle treatment."
        }

        ShadSeparator{}

        resizable_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            resizable_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                resizable_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    resizable_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                    resizable_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                resizable_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    resizable_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                    resizable_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            resizable_preview_panel := mod.widgets.ShadPanel{
                resizable_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        ShadSectionHeader{ text: "Horizontal panes" }
                        ShadPanel{
                            height: 240

                            horizontal_resizable := ShadResizable{
                                width: Fill
                                height: Fill
                                axis: SplitterAxis.Horizontal
                                align: SplitterAlign.FromA(180.0)
                                a: RoundedView{
                                    width: Fill
                                    height: Fill
                                    padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                                    flow: Down
                                    spacing: 8.0
                                    draw_bg.color: (shad_theme.color_muted)
                                    draw_bg.border_radius: (shad_theme.radius)
                                    ShadSectionHeader{text: "Navigation"}
                                    ShadFieldDescription{text: "Keep filters, folders, or nav trees here."}
                                }

                                b: RoundedView{
                                    width: Fill
                                    height: Fill
                                    padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                                    flow: Down
                                    spacing: 8.0
                                    draw_bg.color: #0000
                                    draw_bg.border_radius: (shad_theme.radius)
                                    ShadSectionHeader{text: "Content"}
                                    ShadFieldDescription{text: "Main editing or reading surface."}
                                }
                            }
                        }

                        ShadSectionHeader{ text: "Vertical panes" }
                        ShadPanel{
                            height: 260

                            vertical_resizable := ShadResizable{
                                width: Fill
                                height: Fill
                                axis: SplitterAxis.Vertical
                                align: SplitterAlign.FromA(120.0)
                                a: RoundedView{
                                    width: Fill
                                    height: Fill
                                    padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                                    flow: Down
                                    spacing: 8.0
                                    draw_bg.color: (shad_theme.color_muted)
                                    draw_bg.border_radius: (shad_theme.radius)
                                    ShadSectionHeader{text: "Metrics"}
                                    ShadFieldDescription{text: "Compact summary cards or charts."}
                                }

                                b: RoundedView{
                                    width: Fill
                                    height: Fill
                                    padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                                    flow: Down
                                    spacing: 8.0
                                    draw_bg.color: #0000
                                    draw_bg.border_radius: (shad_theme.radius)
                                    ShadSectionHeader{text: "Details"}
                                    ShadFieldDescription{text: "Expanded logs, notes, or tables."}
                                }
                            }
                        }
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(RESIZABLE_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
