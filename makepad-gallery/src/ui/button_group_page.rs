use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryButtonGroupPage,
    page: button_group_page,
    title: "Button Group",
    subtitle: "Button groups are grouped leaf actions: the container is presentational, while each ShadButtonGroupItem still uses `variant:` and `size:` like a regular button and emits its own clicked(actions) event.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Default" }

        View{
            width: Fit
            height: Fit
            flow: Right
            spacing: 10.0
            align: Align{y: 0.5}

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

            ShadButtonGroup{
                ShadButtonGroupItem{text: "Archive"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItem{text: "Report"}
            }

            ShadButtonGroup{
                ShadButtonGroupItem{text: "Snooze"}
                ShadButtonGroupSeparator{}
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
        }

        ShadSectionHeader{ text: "Sizes via size" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 10.0

            ShadButtonGroup{
                ShadButtonGroupItem{size: ShadControlSize.Small text: "Day"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItem{size: ShadControlSize.Small text: "Week"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItem{size: ShadControlSize.Small text: "Month"}
            }

            ShadButtonGroup{
                ShadButtonGroupItem{text: "Day"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItem{text: "Week"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItem{text: "Month"}
            }

            ShadButtonGroup{
                ShadButtonGroupItem{size: ShadControlSize.Large text: "Day"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItem{size: ShadControlSize.Large text: "Week"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItem{size: ShadControlSize.Large text: "Month"}
            }
        }

        ShadSectionHeader{ text: "Destructive" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 10.0

            ShadButtonGroup{
                ShadButtonGroupItem{
                    variant: ShadButtonVariant.Destructive
                    size: ShadControlSize.Small
                    text: "Small"
                }
                ShadButtonGroupSeparator{}
                ShadButtonGroupItem{
                    variant: ShadButtonVariant.Destructive
                    text: "Default"
                }
                ShadButtonGroupSeparator{}
                ShadButtonGroupItem{
                    variant: ShadButtonVariant.Destructive
                    size: ShadControlSize.Large
                    text: "Large"
                }
            }
        }

        ShadSectionHeader{ text: "Toolbar" }

        View{
            width: Fit
            height: Fit
            flow: Right
            spacing: 10.0
            align: Align{y: 0.5}

            ShadButtonGroup{
                ShadButtonGroupItem{text: "Bold"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItem{text: "Italic"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItem{text: "Underline"}
            }

            ShadButtonGroup{
                ShadButtonGroupItem{
                    width: 36
                    spacing: 0.0
                    padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
                    text: "A-"
                }
                ShadButtonGroupSeparator{}
                ShadButtonGroupItem{
                    width: 36
                    spacing: 0.0
                    padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
                    text: "A+"
                }
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Treat ShadButtonGroup as layout and visual grouping, not as a separate state machine."}
        mod.widgets.GalleryActionFlowStep{text: "2. Configure each ShadButtonGroupItem with `variant:` and `size:` just like a standalone button when the grouped action needs emphasis or density changes."}
        mod.widgets.GalleryActionFlowStep{text: "3. Name the child buttons you care about, then listen to each with ui.shad_button(cx, ids!(archive_btn)).clicked(actions)."}
        mod.widgets.GalleryActionFlowStep{text: "4. Keep the selected tool or toolbar mode in page state, then re-render whichever button should look active."}
        mod.widgets.GalleryActionFlowStep{text: "5. This scales because business identity stays in your state model, not in the group container."}
    },
}
