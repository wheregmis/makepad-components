use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryCardPage,
    page: card_page,
    title: "Card",
    subtitle: "Shadcn-inspired card component from makepad-components library",
    divider: { ShadHr{} },
    preview_spacing: 16.0,
    preview: {
        ShadSectionHeader{ text: "Default" }

        mod.widgets.ShadCard{
            header := mod.widgets.ShadCardHeader{
                title := mod.widgets.ShadCardTitle{text: "Card title"}
                description := mod.widgets.ShadCardDescription{text: "Card description goes here."}
            }
            content := mod.widgets.ShadCardContent{
                ShadLabel{
                    text: "Card content area. Put any widgets here."
                    draw_text.text_style.font_size: 14
                }
            }
            footer := mod.widgets.ShadCardFooter{
                mod.widgets.ShadButton{text: "Cancel"}
                mod.widgets.ShadButton{text: "Save"}
            }
        }
    },
}
