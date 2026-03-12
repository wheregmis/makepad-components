use crate::ui::snippets::SWITCH_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySwitchPageBase = #(GallerySwitchPage::register_widget(vm))

    mod.widgets.GallerySwitchPage = set_type_default() do mod.widgets.GallerySwitchPageBase{
        width: Fill
        height: Fill

        scroll_area := ShadScrollYView{
            ShadPageTitle{
                text: "Switch"
            }

            ShadPageSubtitle{
                text: "Switches are boolean page state with toggle styling. Read changed(actions), then push external state back with set_active(cx, bool)."
            }

            ShadHr{}

            switch_preview_section := View{
                width: Fill
                height: Fit
                flow: Down
                spacing: 12.0

                switch_tabs_row := View{
                    width: Fit
                    visible: false
                    height: 0
                    flow: Right
                    spacing: 20.0
                    margin: Inset{top: 4, bottom: 12}

                    switch_demo_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        switch_demo_tab := mod.widgets.ShadPreviewTab{text: "DEMO"}

                        switch_demo_indicator := SolidView{
                            width: Fill
                            height: 2
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }

                    switch_code_tab_group := View{
                        width: Fit
                        height: Fit
                        flow: Down
                        spacing: 6.0

                        switch_code_tab := mod.widgets.ShadPreviewTab{text: "CODE"}

                        switch_code_indicator := SolidView{
                            width: Fill
                            height: 2
                            visible: false
                            draw_bg.color: (shad_theme.color_primary)
                        }
                    }
                }

                switch_preview_panel := mod.widgets.ShadPanel{
                    switch_preview_flip := mod.widgets.GalleryPreviewStackNavigation{
                        width: Fill
                        height: Fit

                        root_view +: {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 12.0

                            ShadSectionHeader{ text: "Default" }

                            View{
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12.0

                                email_alerts_switch := ShadSwitch{text: "Enable notifications"}
                                ShadSwitch{text: "Dark mode"}
                                ShadSwitch{text: "Use cellular data"}
                            }

                            ShadHr{}

                            ShadSectionHeader{ text: "Inline with label" }

                            View{
                                width: Fill
                                height: Fit
                                flow: Right
                                spacing: 24.0
                                align: Align{y: 0.5}

                                ShadSwitch{text: "Email alerts"}
                                ShadSwitch{text: "SMS alerts"}
                            }

                            ShadHr{}

                            mod.widgets.GalleryActionFlow{
                                body +: {
                                    mod.widgets.GalleryActionFlowStep{text: "1. ShadSwitch is styled like a switch, but the runtime ref is the same boolean check-box/toggle family."}
                                    mod.widgets.GalleryActionFlowStep{text: "2. Read changes with view.check_box(cx, ids!(email_alerts_switch)).changed(actions)."}
                                    mod.widgets.GalleryActionFlowStep{text: "3. Store that boolean in page state or preferences, not in the app shell."}
                                    mod.widgets.GalleryActionFlowStep{text: "4. Restore the switch from external state with set_active(cx, bool), and inspect active(cx) when reconciling state."}
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
                                    code_view +: { text: #(SWITCH_PREVIEW_CODE) }
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
pub struct GallerySwitchPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
}

impl Widget for GallerySwitchPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
