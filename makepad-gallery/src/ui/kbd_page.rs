use crate::ui::page_macros::gallery_static_page;
use makepad_components::makepad_widgets::*;

gallery_static_page! {
    widget: GalleryKbdPage,
    page: kbd_page,
    title: "Kbd",
    subtitle: "Keyboard shortcut key caps for displaying shortcuts (e.g. ⌘ ⇧ ⌥ ⌃ or Ctrl + B).",
    divider: { ShadHr{} },
    preview_spacing: 12.0,
    preview: {
        ShadSectionHeader{ text: "Modifier keys" }

        View{
            width: Fit
            height: Fit
            flow: Right
            spacing: 6.0
            align: Align{y: 0.5}

            ShadKbd{ label := ShadKbdLabel{text: "Cmd"} }
            ShadKbd{ label := ShadKbdLabel{text: "Shift"} }
            ShadKbd{ label := ShadKbdLabel{text: "Option"} }
            ShadKbd{ label := ShadKbdLabel{text: "Ctrl"} }
        }

        ShadSectionHeader{ text: "Shortcut" }

        View{
            width: Fit
            height: Fit
            flow: Right
            spacing: 6.0
            align: Align{y: 0.5}

            ShadKbd{ label := ShadKbdLabel{text: "Ctrl"} }
            ShadKbdSeparator{}
            ShadKbd{ label := ShadKbdLabel{text: "B"} }
        }
    },
}
