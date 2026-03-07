use crate::ui::snippets::BREADCRUMB_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryBreadcrumbPage = ShadScrollYView{
        ShadPageTitle{
            text: "Breadcrumb"
        }

        ShadPageSubtitle{
            text: "Displays the path to the current resource using a hierarchy of links."
        }

        ShadHr{}

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

                    breadcrumb_demo_tab := mod.widgets.ShadButtonGhost{text: "DEMO" padding: Inset{}}

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

                    breadcrumb_code_tab := mod.widgets.ShadButtonGhost{text: "CODE" padding: Inset{}}

                    breadcrumb_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            breadcrumb_preview_panel := mod.widgets.ShadPanel{
                breadcrumb_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                ShadSectionHeader{ text: "Default" }

                ShadBreadcrumb{
                    ShadBreadcrumbLink{ text: "Home" }
                    ShadBreadcrumbSeparator{}
                    ShadBreadcrumbLink{ text: "Components" }
                    ShadBreadcrumbSeparator{}
                    ShadBreadcrumbPage{ text: "Breadcrumb" }
                }

                ShadHr{}

                ShadSectionHeader{ text: "Custom Separator" }

                ShadBreadcrumb{
                    ShadBreadcrumbLink{ text: "Home" }
                    ShadBreadcrumbSeparator{ text: "/" }
                    ShadBreadcrumbLink{ text: "Components" }
                    ShadBreadcrumbSeparator{ text: "/" }
                    ShadBreadcrumbPage{ text: "Breadcrumb" }
                }

                ShadHr{}

                ShadSectionHeader{ text: "Collapsed / Ellipsis" }

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
