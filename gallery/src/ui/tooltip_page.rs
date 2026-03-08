use crate::ui::snippets::TOOLTIP_PREVIEW_CODE;
use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryTooltipPage = SolidView{
        width: Fill
        height: Fill
        draw_bg.color: (shad_theme.color_background)
        flow: Overlay

        ShadScrollArea{
            ShadPageTitle{
                text: "Tooltip"
            }

            ShadPageSubtitle{
                text: "Thin wrappers over Makepad tooltip primitives for quick hints and callouts."
            }

            ShadSeparator{}

            ShadPanel{
                View{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 12.0

                    ShadSectionHeader{ text: "Default" }
                    View{
                        width: Fit
                        height: Fit
                        flow: Right
                        spacing: 12.0

                        tooltip_basic_btn := ShadButtonOutline{text: "Show tooltip"}
                        tooltip_callout_btn := ShadButtonOutline{text: "Show callout"}
                    }

                    ShadFieldDescription{
                        text: "The gallery keeps these on click for reliability. The public wrappers can also be driven from hover/focus."
                    }
                }
            }

            GalleryCodeSnippetSimple{
                code: #(TOOLTIP_PREVIEW_CODE)
            }
        }

        basic_tooltip := ShadTooltip{}
        callout_tooltip := ShadTooltipCallout{}
    }
}
