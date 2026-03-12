use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySkeletonPage = ShadScrollYView{
        ShadPageTitle{
            text: "Skeleton"
        }

        ShadPageSubtitle{
            text: "Use to show a placeholder while content is loading."
        }

        ShadHr{}

        skeleton_preview_section := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 12.0

            skeleton_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                skeleton_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    skeleton_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                    skeleton_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                skeleton_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    skeleton_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                    skeleton_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            skeleton_preview_panel := mod.widgets.ShadPanel{
                skeleton_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                ShadSectionHeader{ text: "Preview" }

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    spacing: 12.0
                    align: Align{x: 0.0, y: 0.5}

                    ShadSkeleton{
                        width: 48
                        height: 48
                        draw_bg.border_radius: 24.0
                    }

                    View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 8.0

                        ShadSkeleton{
                            width: 200
                            height: 16
                        }
                        ShadSkeleton{
                            width: 150
                            height: 16
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
                            code: "View{\n    width: Fill\n    height: Fit\n    flow: Right\n    spacing: 12.0\n    align: Align{x: 0.0, y: 0.5}\n\n    ShadSkeleton{\n        width: 48\n        height: 48\n        draw_bg.border_radius: 24.0\n    }\n\n    View{\n        width: Fit\n        height: Fit\n        flow: Down\n        spacing: 8.0\n\n        ShadSkeleton{\n            width: 200\n            height: 16\n        }\n        ShadSkeleton{\n            width: 150\n            height: 16\n        }\n    }\n}"
                        }
                    }
                }
            }
        }
    }
}
