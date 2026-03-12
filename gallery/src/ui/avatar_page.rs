use crate::ui::snippets::AVATAR_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryAvatarPage = ShadScrollYView{
        ShadPageTitle{
            text: "Avatar"
        }

        ShadPageSubtitle{
            text: "Shadcn-inspired avatar components from makepad-components library"
        }

        ShadHr{}

        avatar_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            avatar_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                avatar_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    avatar_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                    avatar_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                avatar_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    avatar_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                    avatar_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            avatar_preview_panel := mod.widgets.ShadPanel{
                avatar_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                ShadSectionHeader{ text: "Sizes" }

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    align: Align{y: 0.5}
                    spacing: 12.0

                    ShadAvatarSm{
                        fallback := ShadAvatarFallback{text: "SM"}
                    }

                    ShadAvatar{
                        fallback := ShadAvatarFallback{text: "CN"}
                    }

                    ShadAvatarLg{
                        fallback := ShadAvatarFallback{text: "LG"}
                    }
                }

                ShadSectionHeader{ text: "Fallback Variants" }

                View{
                    width: Fill
                    height: Fit
                    flow: Right
                    align: Align{y: 0.5}
                    spacing: 12.0

                    ShadAvatar{
                        fallback := ShadAvatarFallback{text: "JD"}
                    }

                    ShadAvatar{
                        fallback := ShadAvatarFallback{text: "AB"}
                    }

                    ShadAvatar{
                        fallback := ShadAvatarFallback{text: "?"}
                    }
                }
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(AVATAR_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
