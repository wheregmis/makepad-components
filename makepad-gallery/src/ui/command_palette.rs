use crate::ui::catalog;
use makepad_components::command_palette::ShadCommandPaletteItem;
use makepad_components::makepad_widgets::*;

pub fn catalog_command_items() -> Vec<ShadCommandPaletteItem> {
    catalog::entries()
        .iter()
        .map(|entry| ShadCommandPaletteItem {
            title: entry.title.to_string(),
            section: entry.section.to_string(),
            shortcut: entry.shortcut.to_string(),
            keywords: vec![
                entry.sidebar_label.to_string(),
                entry.route.trim_start_matches('/').to_string(),
            ],
        })
        .collect()
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCommandPalette = mod.widgets.ShadCommandPalette{
        item_noun_plural: "gallery components"
        search_help: "Search by title, section, or shortcut tag."
        viewport_height: 320.0

        overlay +: {
            content +: {
                panel +: {
                    search_label +: {
                        text: "Search gallery components"
                    }

                    search_row +: {
                        search_input +: {
                            empty_text: "Search components, sections, or shortcut tags..."
                        }
                    }

                    results_shell +: {
                        empty_state +: {
                            empty_title +: {
                                text: "No components found"
                            }
                            empty_copy +: {
                                text: "Try a component like button, a section like forms, or the shortcut tag shown in each row."
                            }
                        }
                    }

                    footer +: {
                        footer_open_label +: {
                            text: "Open"
                        }
                        footer_close_label +: {
                            text: "Clear / Close"
                        }
                        footer_move_label +: {
                            text: "Move"
                        }
                    }
                }
            }
        }
    }
}
