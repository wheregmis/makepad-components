use makepad_components::makepad_widgets::*;
use crate::ui::snippets::BUTTON_GROUP_PREVIEW_CODE;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryButtonGroupPage = ScrollYView{
        width: Fill
        height: Fill
        flow: Down
        draw_bg.color: (shad_theme.color_background)
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        spacing: 12.0

        Label{
            text: "Button Group"
            draw_text.color: (shad_theme.color_primary)
            draw_text.text_style.font_size: 18
        }

        Label{
            text: "A container that groups related actions with consistent segmented styling"
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
        }

        GalleryHr{}

        button_group_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            button_group_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                button_group_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    button_group_demo_tab := mod.widgets.GalleryPreviewTabButton{text: "DEMO"}

                    button_group_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                button_group_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    button_group_code_tab := mod.widgets.GalleryPreviewTabButton{text: "CODE"}

                    button_group_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            button_group_preview_panel := mod.widgets.GalleryPreviewPanel{
                button_group_preview_flip := PageFlip{
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

                View{
                    width: Fit
                    height: Fit
                    flow: Right
                    spacing: 10.0
                    align: Align{y: 0.5}

                    ShadButtonIcon{text: "←"}

                    ShadButtonGroup{
                        ShadButtonGroupItem{text: "Archive"}
                        ShadButtonGroupSeparator{}
                        ShadButtonGroupItem{text: "Report"}
                    }

                    ShadButtonGroup{
                        ShadButtonGroupItem{text: "Snooze"}
                        ShadButtonGroupSeparator{}
                        ShadButtonGroupItemIcon{text: "⋯"}
                    }
                }

                Label{
                    text: "Sizes"
                    draw_text.color: (shad_theme.color_muted_foreground)
                    draw_text.text_style.font_size: 10
                }

                View{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 10.0

                    ShadButtonGroup{
                        ShadButtonGroupItemSm{text: "Day"}
                        ShadButtonGroupSeparator{}
                        ShadButtonGroupItemSm{text: "Week"}
                        ShadButtonGroupSeparator{}
                        ShadButtonGroupItemSm{text: "Month"}
                    }

                    ShadButtonGroup{
                        ShadButtonGroupItem{text: "Day"}
                        ShadButtonGroupSeparator{}
                        ShadButtonGroupItem{text: "Week"}
                        ShadButtonGroupSeparator{}
                        ShadButtonGroupItem{text: "Month"}
                    }

                    ShadButtonGroup{
                        ShadButtonGroupItemLg{text: "Day"}
                        ShadButtonGroupSeparator{}
                        ShadButtonGroupItemLg{text: "Week"}
                        ShadButtonGroupSeparator{}
                        ShadButtonGroupItemLg{text: "Month"}
                    }
                }

                Label{
                    text: "Toolbar"
                    draw_text.color: (shad_theme.color_muted_foreground)
                    draw_text.text_style.font_size: 10
                }

                View{
                    width: Fit
                    height: Fit
                    flow: Right
                    spacing: 10.0
                    align: Align{y: 0.5}

                    ShadButtonGroup{
                        ShadButtonGroupItem{text: "Bold"}
                        ShadButtonGroupSeparator{}
                        ShadButtonGroupItem{text: "Italic"}
                        ShadButtonGroupSeparator{}
                        ShadButtonGroupItem{text: "Underline"}
                    }

                    ShadButtonGroup{
                        ShadButtonGroupItemIcon{text: "A-"}
                        ShadButtonGroupSeparator{}
                        ShadButtonGroupItemIcon{text: "A+"}
                    }
                }
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(BUTTON_GROUP_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
