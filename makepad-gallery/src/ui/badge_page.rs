use crate::ui::snippets::BADGE_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryBadgePageBase = #(GalleryBadgePage::register_widget(vm))

    mod.widgets.GalleryBadgePage = set_type_default() do mod.widgets.GalleryBadgePageBase{
        view := ShadScrollYView{
            ShadPageTitle{
                text: "Badge"
            }

            ShadPageSubtitle{
                text: "Badge variants showcasing label, secondary, destructive, and outline styles."
            }

            ShadHr{}

            badge_preview_section := mod.widgets.GalleryPreviewSection{
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

                                ShadSectionHeader{ text: "Variants" }

                                View{
                                    width: Fill
                                    height: Fit
                                    flow: Right
                                    spacing: 12.0

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
                        }

                        code_page +: {
                            body +: {
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12.0

                                code_snippet +: {
                                    code: #(BADGE_PREVIEW_CODE)
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
pub struct GalleryBadgePage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryBadgePage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
