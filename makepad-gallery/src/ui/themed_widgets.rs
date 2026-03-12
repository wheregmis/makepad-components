use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCodeSnippet = SolidView{
        width: Fill
        height: Fit
        padding: Inset{top: 12, right: 12, bottom: 12, left: 12}
        draw_bg +: {
            color: (shad_theme.color_muted)
            border_radius: (shad_theme.radius)
        }

        code_view := CodeView{
            text: ""
            editor +: {
                width: Fill
                height: Fit
                pad_left_top: vec2(0.0, 0.0)
                empty_page_at_end: false
                show_gutter: false
                draw_bg +: {
                    color: #0000
                }
                draw_text +: {
                    text_style: theme.font_code{
                        font_size: theme.font_size_code
                        line_spacing: theme.font_longform_line_spacing
                    }
                }
            }
        }
    }

    mod.widgets.GalleryActionFlowStep = ShadFieldDescription{
        width: Fill
    }

    mod.widgets.GalleryActionFlow = RoundedView{
        width: Fill
        height: Fit
        flow: Down
        spacing: 8.0
        padding: Inset{top: 14, right: 14, bottom: 14, left: 14}
        draw_bg +: {
            color: (shad_theme.color_muted)
            border_radius: (shad_theme.radius)
            border_size: 1.0
            border_color: (shad_theme.color_outline_border)
        }

        title := ShadSectionHeader{
            text: "Action Flow"
        }

        body := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 8.0
        }
    }

    mod.widgets.GalleryPreviewStackNavigation = View{
        width: Fill
        height: Fit
        flow: Down
        spacing: 16.0

        preview_title := ShadSectionHeader{
            text: "Preview"
        }

        root_view := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0
        }

        code_page := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

            code_title := ShadSectionHeader{
                text: "Code"
            }

            body := View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 12.0
            }
        }
    }

    mod.widgets.GalleryPreviewSection = View{
        width: Fill
        height: Fit
        flow: Down

        tabs_row := View{
            visible: false
            height: 0
            width: Fit
            flow: Right
            spacing: 20.0
            margin: Inset{top: 4, bottom: 12}

            demo_tab_group := View{
                width: Fit
                height: Fit
                flow: Down
                spacing: 6.0

                demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                demo_indicator := SolidView{
                    width: Fill
                    height: 2
                    draw_bg.color: (shad_theme.color_primary)
                }
            }

            code_tab_group := View{
                width: Fit
                height: Fit
                flow: Down
                spacing: 6.0

                code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                code_indicator := SolidView{
                    width: Fill
                    height: 2
                    visible: false
                    draw_bg.color: (shad_theme.color_primary)
                }
            }
        }

        preview_panel := mod.widgets.ShadPanel{
            preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                width: Fill
                height: Fit

                code_page +: {
                    body +: {
                        code_snippet := mod.widgets.GalleryCodeSnippet{}
                    }
                }
            }
        }
    }
}
