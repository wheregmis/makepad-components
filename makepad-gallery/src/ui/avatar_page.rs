use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryAvatarPage,
    page: avatar_page,
    title: "Avatar",
    subtitle: "Avatar components with badges and fallback variants from makepad-components.",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Sizes" }

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 16.0

            ShadAvatarSm{
                fallback := ShadAvatarFallback{text: "SM"}
            }
            ShadAvatar{
                fallback := ShadAvatarFallback{text: "CN"}
            }
            ShadAvatarLg{
                fallback := ShadAvatarFallback{text: "LG"}
            }
        }

        ShadSectionHeader{ text: "Fallback Variants" }

        View{
            width: Fill
            height: Fit
            flow: Right
            spacing: 16.0

            ShadAvatar{
                fallback := ShadAvatarFallback{text: "JD"}
            }
            ShadAvatar{
                fallback := ShadAvatarFallback{text: "AB"}
            }
            ShadAvatar{
                fallback := ShadAvatarFallback{text: "?"}
            }
        }
    },
}
