use crate::ui::snippets::BADGE_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryBadgePage = ShadScrollYView{
        ShadPageTitle{
            text: "Badge"
        }

        ShadPageSubtitle{
            text: "Shadcn-inspired badge components from makepad-components library"
        }

        ShadHr{}

        badge_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            badge_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                badge_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    badge_demo_tab := mod.widgets.ShadButtonGhost{text: "DEMO" padding: Inset{}}

                    badge_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                badge_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    badge_code_tab := mod.widgets.ShadButtonGhost{text: "CODE" padding: Inset{}}

                    badge_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            badge_preview_panel := mod.widgets.ShadPanel{
                badge_preview_flip := PageFlip{
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
                    align: Align{y: 0.5}
                    spacing: 8.0

                    ShadBadge{
                        label := ShadBadgeLabel{text: "Default"}
                    }

                    ShadBadgeSecondary{
                        label := ShadBadgeSecondaryLabel{text: "Secondary"}
                    }

                    ShadBadgeDestructive{
                        label := ShadBadgeDestructiveLabel{text: "Destructive"}
                    }

                    ShadBadgeOutline{
                        label := ShadBadgeOutlineLabel{text: "Outline"}
                    }
                }
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(BADGE_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
