use makepad_components::makepad_widgets::*;
use crate::ui::snippets::BUTTON_PREVIEW_CODE;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryButtonPage = ShadScrollYView{
        ShadPageTitle{
            text: "Button"
        }

        ShadPageSubtitle{
            text: "Shadcn-inspired button components from makepad-components library"
        }

        ShadHr{}

        button_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            button_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                button_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    button_demo_tab := mod.widgets.ShadButtonGhost{text: "DEMO" padding: Inset{}}

                    button_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                button_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    button_code_tab := mod.widgets.ShadButtonGhost{text: "CODE" padding: Inset{}}

                    button_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            button_preview_panel := mod.widgets.ShadPanel{
                button_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                ShadSectionHeader{ text: "Variants" }

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    spacing: 8.0

                    ShadButton{text: "Default"}
                    ShadButtonDestructive{text: "Destructive"}
                    ShadButtonOutline{text: "Outline"}
                    ShadButtonSecondary{text: "Secondary"}
                    ShadButtonGhost{text: "Ghost"}
                    ShadButtonLink{text: "Link"}
                }

                ShadSectionHeader{ text: "Sizes" }

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    align: Align{y: 0.5}
                    spacing: 8.0

                    ShadButtonSm{text: "Small"}
                    ShadButton{text: "Default"}
                    ShadButtonLg{text: "Large"}
                }

                ShadSectionHeader{ text: "Destructive Sizes" }

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    align: Align{y: 0.5}
                    spacing: 8.0

                    ShadButtonDestructive{
                        height: 28
                        padding: Inset{left: 12, right: 12, top: 0, bottom: 0}
                        draw_text.text_style.font_size: 10
                        text: "Small"
                    }
                    ShadButtonDestructive{text: "Default"}
                    ShadButtonDestructive{
                        height: 44
                        padding: Inset{left: 32, right: 32, top: 0, bottom: 0}
                        draw_text.text_style.font_size: 13
                        text: "Large"
                    }
                }

                ShadSectionHeader{ text: "Outline Variations" }

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    align: Align{y: 0.5}
                    spacing: 8.0

                    ShadButtonOutline{text: "Outline"}
                    ShadButtonGhost{text: "Ghost"}
                    ShadButtonLink{text: "Link"}
                }

                ShadSectionHeader{ text: "Icons" }

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    align: Align{y: 0.5}
                    spacing: 14.0

                    IconCheck{}
                    IconX{}
                    IconSearch{}
                    IconInfo{}
                    IconChevronLeft{}
                    IconChevronRight{}
                    IconChevronDown{}
                }

                ShadSectionHeader{ text: "Icon Buttons" }

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    align: Align{y: 0.5}
                    spacing: 8.0

                    ShadButtonIcon{text: "✓"}

                    IconButtonChevronLeft{
                        width: 36
                        height: 36
                        draw_bg +: {
                            color: #0000
                            color_hover: (shad_theme.color_ghost_hover)
                            color_down: (shad_theme.color_ghost_down)
                            border_size: 1.0
                            border_radius: (shad_theme.radius)
                            border_color: (shad_theme.color_outline_border)
                        }
                        draw_icon.color: (shad_theme.color_primary)
                    }

                    IconButtonChevronRight{
                        width: 36
                        height: 36
                        draw_bg +: {
                            color: #0000
                            color_hover: (shad_theme.color_ghost_hover)
                            color_down: (shad_theme.color_ghost_down)
                            border_size: 1.0
                            border_radius: (shad_theme.radius)
                            border_color: (shad_theme.color_outline_border)
                        }
                        draw_icon.color: (shad_theme.color_primary)
                    }

                    IconButtonX{
                        width: 36
                        height: 36
                        draw_bg +: {
                            color: #0000
                            color_hover: (shad_theme.color_ghost_hover)
                            color_down: (shad_theme.color_ghost_down)
                            border_size: 0.0
                            border_radius: (shad_theme.radius)
                        }
                        draw_icon.color: (shad_theme.color_muted_foreground)
                    }
                }
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(BUTTON_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
