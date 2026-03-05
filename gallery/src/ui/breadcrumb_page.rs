use makepad_components::makepad_widgets::*;
use crate::ui::snippets::BREADCRUMB_PREVIEW_CODE;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryBreadcrumbPage = ScrollYView{
        width: Fill
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_background)
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        spacing: 12.0

        Label{
            text: "Breadcrumb"
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 18
        }

        Label{
            text: "Displays the path to the current resource using a hierarchy of links."
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        GalleryHr{}

        breadcrumb_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            breadcrumb_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                breadcrumb_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    breadcrumb_demo_tab := mod.widgets.GalleryPreviewTabButton{text: "DEMO"}

                    breadcrumb_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                breadcrumb_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    breadcrumb_code_tab := mod.widgets.GalleryPreviewTabButton{text: "CODE"}

                    breadcrumb_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            breadcrumb_preview_panel := mod.widgets.GalleryPreviewPanel{
                breadcrumb_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                Label{
                    text: "Default"
                    draw_text.color: (shad_theme.color_muted_foreground)
                    draw_text.text_style.font_size: 10
                }

                ShadBreadcrumb{
                    ShadBreadcrumbLink{ text: "Home" }
                    ShadBreadcrumbSeparator{}
                    ShadBreadcrumbLink{ text: "Components" }
                    ShadBreadcrumbSeparator{}
                    ShadBreadcrumbPage{ text: "Breadcrumb" }
                }

                GalleryHr{}

                Label{
                    text: "Custom Separator"
                    draw_text.color: (shad_theme.color_muted_foreground)
                    draw_text.text_style.font_size: 10
                }

                ShadBreadcrumb{
                    ShadBreadcrumbLink{ text: "Home" }
                    ShadBreadcrumbSeparator{ text: "/" }
                    ShadBreadcrumbLink{ text: "Components" }
                    ShadBreadcrumbSeparator{ text: "/" }
                    ShadBreadcrumbPage{ text: "Breadcrumb" }
                }

                GalleryHr{}

                Label{
                    text: "Collapsed / Ellipsis"
                    draw_text.color: (shad_theme.color_muted_foreground)
                    draw_text.text_style.font_size: 10
                }

                ShadBreadcrumb{
                    ShadBreadcrumbLink{ text: "Home" }
                    ShadBreadcrumbSeparator{}
                    ShadBreadcrumbEllipsis{}
                    ShadBreadcrumbSeparator{}
                    ShadBreadcrumbLink{ text: "Components" }
                    ShadBreadcrumbSeparator{}
                    ShadBreadcrumbPage{ text: "Breadcrumb" }
                }
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(BREADCRUMB_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
