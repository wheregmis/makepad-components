use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryButtonGroupPage,
    page: button_group_page,
    title: "Button Group",
    subtitle: "Button groups are grouped leaf actions: the container is presentational, while each named child button still emits its own clicked(actions) event.",
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

            GalleryButtonIconChevronLeft{}

            ShadButtonGroup{
                ShadButtonGroupItem{text: "Archive"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItem{text: "Report"}
            }

            ShadButtonGroup{
                ShadButtonGroupItem{text: "Snooze"}
                ShadButtonGroupSeparator{}
                GalleryButtonIconMoreHorizontal{}
            }
        }

        ShadSectionHeader{ text: "Sizes" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 10.0

            ShadButtonGroup{
                ShadButtonGroupItemSm{text: "Day"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItemSm{text: "Week"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItemSm{text: "Month"}
            }

            ShadButtonGroup{
                ShadButtonGroupItem{text: "Day"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItem{text: "Week"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItem{text: "Month"}
            }

            ShadButtonGroup{
                ShadButtonGroupItemLg{text: "Day"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItemLg{text: "Week"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItemLg{text: "Month"}
            }
        }

        ShadSectionHeader{ text: "Destructive" }

        View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 10.0

            ShadButtonGroup{
                ShadButtonGroupItemDestructiveSm{text: "Small"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItemDestructive{text: "Default"}
                ShadButtonGroupSeparator{}
                ShadButtonGroupItemDestructiveLg{text: "Large"}
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
                GalleryButtonGroupItemIcon{text: "A-"}
                ShadButtonGroupSeparator{}
                GalleryButtonGroupItemIcon{text: "A+"}
            }
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Treat ShadButtonGroup as layout and visual grouping, not as a separate state machine."}
        mod.widgets.GalleryActionFlowStep{text: "2. Name the child buttons you care about, then listen to each with ui.button(cx, ids!(archive_btn)).clicked(actions)."}
        mod.widgets.GalleryActionFlowStep{text: "3. Keep the selected tool or toolbar mode in page state, then re-render whichever button should look active."}
        mod.widgets.GalleryActionFlowStep{text: "4. This scales because business identity stays in your state model, not in the group container."}
    },
}
