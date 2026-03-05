use crate::ui::snippets::SONNER_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GallerySonnerPage = GalleryComponentPage{
        GalleryPageTitle{
            text: "Sonner / Toast"
        }

        GalleryPageSubtitle{
            text: "Toast notifications with Modal overlay. Use ShadSonner or ShadSonnerWithDescription with set_open(true) to show. Click outside or Escape to dismiss."
        }

        GalleryHr{}

        sonner_preview_section := View{
            width: Fill
            height: Fit
            flow: Down

            sonner_tabs_row := View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 20.0
                margin: Inset{top: 4, bottom: 12}

                sonner_demo_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    sonner_demo_tab := mod.widgets.GalleryPreviewTabButton{text: "DEMO"}

                    sonner_demo_indicator := SolidView{
                        width: Fill
                        height: 2
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }

                sonner_code_tab_group := View{
                    width: Fit
                    height: Fit
                    flow: Down
                    spacing: 6.0

                    sonner_code_tab := mod.widgets.GalleryPreviewTabButton{text: "CODE"}

                    sonner_code_indicator := SolidView{
                        width: Fill
                        height: 2
                        visible: false
                        draw_bg.color: (shad_theme.color_primary)
                    }
                }
            }

            sonner_preview_panel := mod.widgets.GalleryPreviewPanel{
                sonner_preview_flip := PageFlip{
                    width: Fill
                    height: Fit
                    active_page: @demo_page

                    demo_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 16.0

                        View{
                            width: Fill
                            height: Fit
                            flow: Right
                            spacing: 8.0

                            toast_event_btn := ShadButton{text: "Event created"}
                            toast_desc_btn := ShadButton{text: "Toast with description"}
                        }

                        toast_event := ShadSonner{open: false}
                        toast_desc := ShadSonnerWithDescription{open: false}
                    }

                    code_page := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 12.0

                        GalleryCodeSnippet{
                            code: #(SONNER_PREVIEW_CODE)
                        }
                    }
                }
            }
        }
    }
}
