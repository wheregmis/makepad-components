use crate::ui::snippets::BUTTON_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryButtonPageBase = #(GalleryButtonPage::register_widget(vm))

    mod.widgets.GalleryButtonPage = set_type_default() do mod.widgets.GalleryButtonPageBase{
        view := ShadScrollYView{
            ShadPageTitle{
                text: "Button"
            }

            ShadPageSubtitle{
                text: "Buttons are leaf actions: attach an id to the specific button you care about, then listen for clicked(actions) in the page or feature controller."
            }

            ShadHr{}

            button_preview_section := View{
                width: Fill
                height: Fit
                flow: Down

                button_tabs_row := View{
                    width: Fit
                    visible: false
                    height: 0
                    flow: Right
                    spacing: 20.0
                    margin: Inset{top: 4, bottom: 12}

                    button_demo_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        button_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}
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

                        button_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}
                        button_code_indicator := SolidView{
                            width: Fill
                            height: 2
                            visible: false
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }
                }

                button_preview_panel := mod.widgets.ShadPanel{
                    button_preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                        width: Fill
                        height: Fit

                        root_view +: {
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

                            mod.widgets.GalleryActionFlow{
                                body +: {
                                    mod.widgets.GalleryActionFlowStep{text: "1. Give each button that matters its own id, like save_btn or delete_btn."}
                                    mod.widgets.GalleryActionFlowStep{text: "2. Read button clicks with ui.button(cx, ids!(save_btn)).clicked(actions) in the page or feature controller."}
                                    mod.widgets.GalleryActionFlowStep{text: "3. Keep business state outside the button itself; buttons emit the intent, the page decides what to do next."}
                                    mod.widgets.GalleryActionFlowStep{text: "4. Derive disabled, loading, or confirm variants from page state instead of branching in the app shell."}
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
                                    code_view +: { text: #(BUTTON_PREVIEW_CODE) }
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
pub struct GalleryButtonPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GalleryButtonPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
