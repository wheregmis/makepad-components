use makepad_components::makepad_widgets::*;
use crate::ui::snippets::TABS_PREVIEW_CODE;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryTabsPage = ShadScrollArea{
        ShadPageTitle{
            text: "Tabs"
        }

        ShadPageSubtitle{
            text: "Composable trigger and content styles for app-level tab state."
        }

        ShadSeparator{}

        ShadTabs{
            tabs_row := ShadTabsList{
                overview_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 4.0

                    tabs_overview_trigger := ShadTabsTrigger{text: "Overview"}
                    tabs_overview_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                usage_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 4.0

                    tabs_usage_trigger := ShadTabsTrigger{text: "Usage"}
                    tabs_usage_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                settings_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 4.0

                    tabs_settings_trigger := ShadTabsTrigger{text: "Settings"}
                    tabs_settings_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            tabs_content_flip := PageFlip{
                width: Fill
                height: Fit
                active_page: @overview_page

                overview_page := ShadTabsContent{
                    ShadSectionHeader{text: "Overview"}
                    ShadFieldDescription{text: "Keep the page shell compact while switching between related content areas."}
                }

                usage_page := ShadTabsContent{
                    ShadSectionHeader{text: "Usage"}
                    ShadFieldDescription{text: "Pair `ShadTabsTrigger` with `PageFlip` or another state holder in app code."}
                }

                settings_page := ShadTabsContent{
                    ShadSectionHeader{text: "Settings"}
                    ShadFieldDescription{text: "This first pass focuses on composition and styling, not a fully stateful tab controller."}
                }
            }
        }

        GalleryCodeSnippetSimple{
            code: #(TABS_PREVIEW_CODE)
        }
    }
}
