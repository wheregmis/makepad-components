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
                width: 560

                panel +: {
                    draw_bg +: {
                        border_radius: 22.0
                        border_color: (shad_theme.color_outline_border_hover)
                    }

                    body +: {
                        padding: Inset{left: 16, right: 16, top: 16, bottom: 16}
                        spacing: 14.0

                        search_label +: {
                            text: "Search gallery components"
                            draw_text.text_style.font_size: 12
                        }

                        search_row +: {
                            search_input +: {
                                empty_text: "Search components, sections, or shortcut tags..."

                                search_shell +: {
                                    padding: Inset{left: 16, right: 10, top: 0, bottom: 0}
                                    draw_bg +: {
                                        color: (shad_theme.color_secondary)
                                        border_radius: 16.0
                                        border_color: (shad_theme.color_outline_border_hover)
                                    }

                                    input +: {
                                        draw_text.text_style.font_size: 15
                                    }
                                }
                            }

                            clear_search_btn +: {
                                variant: ShadButtonVariant.Outline
                                height: 42
                                padding: Inset{left: 14, right: 14, top: 0, bottom: 0}
                            }
                        }

                        results_summary +: {
                            draw_text.text_style.font_size: 11
                            draw_text.color: (shad_theme.color_muted_foreground)
                        }

                        results_shell +: {
                            draw_bg +: {
                                color: (shad_theme.color_muted)
                                border_radius: 20.0
                                border_color: (shad_theme.color_outline_border_hover)
                            }

                            body +: {
                                padding: Inset{left: 8, right: 8, top: 8, bottom: 8}

                                results_table +: {
                                    empty_message: "No components found for that query."
                                    auto_fill_width: true

                                    table_view +: {
                                        spacing: 0.0
                                        header +: {
                                            height: 0
                                            fill_color: (shad_theme.color_clear)
                                            border_color: (shad_theme.color_clear)
                                            text_color: (shad_theme.color_clear)
                                        }

                                        list +: {
                                            Item +: {
                                                height: 52
                                                fill_hover: (shad_theme.color_ghost_hover)
                                                fill_selected: (shad_theme.color_secondary_hover)
                                                fill_striped: (shad_theme.color_clear)
                                                border_color: (shad_theme.color_clear)
                                                text_color: (shad_theme.color_primary)
                                            }

                                            Empty +: {
                                                height: 92
                                                draw_bg.color: (shad_theme.color_clear)

                                                empty_label +: {
                                                    draw_text.color: (shad_theme.color_muted_foreground)
                                                    draw_text.text_style.font_size: 12.0
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        footer +: {
                            margin: Inset{top: 8}
                            spacing: 12.0

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
}
