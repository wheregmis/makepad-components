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

            GalleryButtonDestructiveSm{text: "Small"}
            ShadButtonDestructive{text: "Default"}
            GalleryButtonDestructiveLg{text: "Large"}
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
            flow: Right{wrap: true}
            align: Align{y: 0.5}
            spacing: 8.0

            ShadButtonIconSm{text: "✓"}
            ShadButtonIcon{text: "✓"}
            ShadButtonIconLg{text: "✓"}

            GalleryButtonIconChevronLeft{}

            GalleryButtonIconChevronRight{}

            GalleryButtonIconX{}

            GalleryButtonIconMoreHorizontal{}
        }
    },
    action_flow: {
        mod.widgets.GalleryActionFlowStep{text: "1. Give each button that matters its own id, like save_btn or delete_btn."}
        mod.widgets.GalleryActionFlowStep{text: "2. Read button clicks with ui.button(cx, ids!(save_btn)).clicked(actions) in the page or feature controller."}
        mod.widgets.GalleryActionFlowStep{text: "3. Keep business state outside the button itself; buttons emit the intent, the page decides what to do next."}
        mod.widgets.GalleryActionFlowStep{text: "4. Derive disabled, loading, or confirm variants from page state instead of branching in the app shell."}
    },
}
