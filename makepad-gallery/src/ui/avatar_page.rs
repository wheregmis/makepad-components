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

            avatar_preview_section := mod.widgets.GalleryPreviewSection{
                width: Fill
                height: Fit

                preview_panel +: {
                    preview_flip +: {
                        root_view +: {
                            preview_content +: {
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
                        }

                        code_page +: {
                            body +: {
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12.0

                                code_snippet +: {
                                    code: #(AVATAR_PREVIEW_CODE)
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
