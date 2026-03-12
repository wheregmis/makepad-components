use crate::ui::snippets::AVATAR_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryAvatarPageBase = #(GalleryAvatarPage::register_widget(vm))

    mod.widgets.GalleryAvatarPage = set_type_default() do mod.widgets.GalleryAvatarPageBase{
        view := ShadScrollYView{
            ShadPageTitle{
                text: "Avatar"
            }

            ShadPageSubtitle{
                text: "Avatar components with badges and fallback variants from makepad-components."
            }

            ShadHr{}

            avatar_preview_section := View{
                width: Fill
                height: Fit
                flow: Down

                avatar_tabs_row := View{
                    width: Fit
                    visible: false
                    height: 0
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
                    avatar_preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                        width: Fill
                        height: Fit

                        root_view +: {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            ShadSectionHeader{ text: "Sizes" }

                            View{
                                width: Fill
                                height: Fit
                                flow: Right
                                spacing: 16.0

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
                                spacing: 16.0

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

                        code_page +: {
                            body +: {
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12.0

                                GalleryCodeSnippet{
                                    code_view +: { text: #(AVATAR_PREVIEW_CODE) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryAvatarPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryAvatarPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
