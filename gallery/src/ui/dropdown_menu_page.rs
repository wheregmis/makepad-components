use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryDropdownMenuPage = SolidView{
        width: Fill
        height: Fill
        draw_bg.color: (shad_theme.color_background)

        ScrollYView{
            width: Fill
            height: Fill
            flow: Down
            padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
            spacing: 12.0

            ShadPageTitle{
                text: "Dropdown Menu"
            }

            ShadPageSubtitle{
                text: "Shadcn-inspired dropdown for selecting from a list of options."
            }

            ShadHr{}

            ShadSectionHeader{ text: "Default" }

            View{
                width: Fit
                height: Fit
                flow: Right
                spacing: 12.0

                DropDown{
                    labels: ["Option A" "Option B" "Option C" "Option D"]
                }

                DropDown{
                    labels: ["Small" "Medium" "Large" "Extra Large"]
                }
            }
        }
    }
}
