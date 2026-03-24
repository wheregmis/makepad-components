use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryButtonPage,
    page: button_page,
    title: "Button",
    subtitle: "Buttons are leaf actions: attach an id to the specific button you care about, then listen for clicked(actions) in the page or feature controller.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Variants" }

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 8.0

            ShadButton{text: "Default"}
            ShadButton{variant: ShadButtonVariant.Destructive text: "Destructive"}
            ShadButton{variant: ShadButtonVariant.Outline text: "Outline"}
            ShadButton{variant: ShadButtonVariant.Secondary text: "Secondary"}
            ShadButton{variant: ShadButtonVariant.Ghost text: "Ghost"}
            ShadButton{variant: ShadButtonVariant.Link text: "Link"}
        }

        ShadSectionHeader{ text: "Sizes" }

        View{
            width: Fill
            height: Fit
            flow: Right
            align: Align{y: 0.5}
            spacing: 8.0

            ShadButton{size: ShadControlSize.Small text: "Small"}
            ShadButton{text: "Default"}
            ShadButton{size: ShadControlSize.Large text: "Large"}
        }

        ShadSectionHeader{ text: "Destructive Sizes" }

        View{
            width: Fill
            height: Fit
            flow: Right
            align: Align{y: 0.5}
            spacing: 8.0

            ShadButton{
                variant: ShadButtonVariant.Destructive
                size: ShadControlSize.Small
                text: "Small"
            }
            ShadButton{variant: ShadButtonVariant.Destructive text: "Default"}
            ShadButton{
                variant: ShadButtonVariant.Destructive
                size: ShadControlSize.Large
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

            ShadButton{variant: ShadButtonVariant.Outline text: "Outline"}
            ShadButton{variant: ShadButtonVariant.Ghost text: "Ghost"}
            ShadButton{variant: ShadButtonVariant.Link text: "Link"}
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
            flow: Right{wrap: true}
            align: Align{y: 0.5}
            spacing: 8.0

            ShadButtonIconSm{text: "✓"}
            ShadButtonIcon{text: "✓"}
            ShadButtonIconLg{text: "✓"}

            IconButtonChevronLeft{
                width: 36
                height: 36
                spacing: 0.0
                padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
                draw_bg +: {
                    color: #0000
                    color_hover: (shad_theme.color_ghost_hover)
                    color_down: (shad_theme.color_ghost_down)
                    color_focus: (shad_theme.color_ghost_hover)
                    color_disabled: (shad_theme.color_disabled)
                    border_size: 1.0
                    border_radius: (shad_theme.radius)
                    border_color: (shad_theme.color_outline_border)
                }
                draw_icon.color: (shad_theme.color_primary)
            }

            IconButtonChevronRight{
                width: 36
                height: 36
                spacing: 0.0
                padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
                draw_bg +: {
                    color: #0000
                    color_hover: (shad_theme.color_ghost_hover)
                    color_down: (shad_theme.color_ghost_down)
                    color_focus: (shad_theme.color_ghost_hover)
                    color_disabled: (shad_theme.color_disabled)
                    border_size: 1.0
                    border_radius: (shad_theme.radius)
                    border_color: (shad_theme.color_outline_border)
                }
                draw_icon.color: (shad_theme.color_primary)
            }

            IconButtonX{
                width: 36
                height: 36
                spacing: 0.0
                padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
                draw_bg +: {
                    color: #0000
                    color_hover: (shad_theme.color_ghost_hover)
                    color_down: (shad_theme.color_ghost_down)
                    color_focus: (shad_theme.color_ghost_hover)
                    color_disabled: (shad_theme.color_disabled)
                    border_size: 0.0
                    border_radius: (shad_theme.radius)
                    border_color: #0000
                }
                draw_icon.color: (shad_theme.color_muted_foreground)
            }

            IconButtonMoreHorizontal{
                width: 36
                height: 36
                spacing: 0.0
                padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
                draw_bg +: {
                    color: #0000
                    color_hover: (shad_theme.color_ghost_hover)
                    color_down: (shad_theme.color_ghost_down)
                    color_focus: (shad_theme.color_ghost_hover)
                    color_disabled: (shad_theme.color_disabled)
                    border_size: 0.0
                    border_radius: (shad_theme.radius)
                    border_color: #0000
                }
                draw_icon.color: (shad_theme.color_primary)
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Give each button that matters its own id, like save_btn or delete_btn."}
        mod.widgets.GalleryActionFlowStep{text: "2. Read button clicks with ui.button(cx, ids!(save_btn)).clicked(actions) in the page or feature controller."}
        mod.widgets.GalleryActionFlowStep{text: "3. Keep business state outside the button itself; buttons emit the intent, the page decides what to do next."}
        mod.widgets.GalleryActionFlowStep{text: "4. Derive disabled, loading, or confirm variants from page state instead of branching in the app shell."}
    },
}
