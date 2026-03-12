use makepad_components::makepad_widgets::*;

const VISIBLE_SLOT_COUNT: usize = 8;

#[derive(Clone, Copy)]
struct CommandSpec {
    title: &'static str,
    section: &'static str,
    shortcut: &'static str,
    page: LiveId,
}

const COMMANDS: [CommandSpec; 34] = [
    CommandSpec {
        title: "Accordion",
        section: "Components",
        shortcut: "A",
        page: live_id!(accordion_page),
    },
    CommandSpec {
        title: "Alert",
        section: "Components",
        shortcut: "L",
        page: live_id!(alert_page),
    },
    CommandSpec {
        title: "Aspect Ratio",
        section: "Components",
        shortcut: "R",
        page: live_id!(aspect_ratio_page),
    },
    CommandSpec {
        title: "Avatar",
        section: "Components",
        shortcut: "V",
        page: live_id!(avatar_page),
    },
    CommandSpec {
        title: "Badge",
        section: "Components",
        shortcut: "B",
        page: live_id!(badge_page),
    },
    CommandSpec {
        title: "Breadcrumb",
        section: "Components",
        shortcut: "C",
        page: live_id!(breadcrumb_page),
    },
    CommandSpec {
        title: "Button",
        section: "Components",
        shortcut: "U",
        page: live_id!(button_page),
    },
    CommandSpec {
        title: "Button Group",
        section: "Components",
        shortcut: "G",
        page: live_id!(button_group_page),
    },
    CommandSpec {
        title: "Card",
        section: "Components",
        shortcut: "D",
        page: live_id!(card_page),
    },
    CommandSpec {
        title: "Carousel",
        section: "Components",
        shortcut: "O",
        page: live_id!(carousel_page),
    },
    CommandSpec {
        title: "Checkbox",
        section: "Components",
        shortcut: "X",
        page: live_id!(checkbox_page),
    },
    CommandSpec {
        title: "Collapsible",
        section: "Components",
        shortcut: "P",
        page: live_id!(collapsible_page),
    },
    CommandSpec {
        title: "Command Palette",
        section: "Navigation",
        shortcut: "K",
        page: live_id!(command_palette_page),
    },
    CommandSpec {
        title: "Dialog",
        section: "Overlays",
        shortcut: "I",
        page: live_id!(dialog_page),
    },
    CommandSpec {
        title: "Input",
        section: "Forms",
        shortcut: "N",
        page: live_id!(input_page),
    },
    CommandSpec {
        title: "Input OTP",
        section: "Forms",
        shortcut: "6",
        page: live_id!(input_otp_page),
    },
    CommandSpec {
        title: "Kbd",
        section: "Forms",
        shortcut: "K",
        page: live_id!(kbd_page),
    },
    CommandSpec {
        title: "Label",
        section: "Forms",
        shortcut: "M",
        page: live_id!(label_page),
    },
    CommandSpec {
        title: "Progress",
        section: "Feedback",
        shortcut: "P",
        page: live_id!(progress_page),
    },
    CommandSpec {
        title: "Radio Group",
        section: "Forms",
        shortcut: "R",
        page: live_id!(radio_group_page),
    },
    CommandSpec {
        title: "Resizable",
        section: "Layout",
        shortcut: "Z",
        page: live_id!(resizable_page),
    },
    CommandSpec {
        title: "Scroll Area",
        section: "Layout",
        shortcut: "S",
        page: live_id!(scroll_area_page),
    },
    CommandSpec {
        title: "Select",
        section: "Forms",
        shortcut: "E",
        page: live_id!(select_page),
    },
    CommandSpec {
        title: "Separator",
        section: "Layout",
        shortcut: "T",
        page: live_id!(separator_page),
    },
    CommandSpec {
        title: "Sheet",
        section: "Overlays",
        shortcut: "H",
        page: live_id!(sheet_page),
    },
    CommandSpec {
        title: "Skeleton",
        section: "Feedback",
        shortcut: "Q",
        page: live_id!(skeleton_page),
    },
    CommandSpec {
        title: "Sidebar",
        section: "Layout",
        shortcut: "Y",
        page: live_id!(sidebar_page),
    },
    CommandSpec {
        title: "Slider",
        section: "Forms",
        shortcut: "J",
        page: live_id!(slider_page),
    },
    CommandSpec {
        title: "Sonner",
        section: "Feedback",
        shortcut: "F",
        page: live_id!(sonner_page),
    },
    CommandSpec {
        title: "Spinner",
        section: "Feedback",
        shortcut: "W",
        page: live_id!(spinner_page),
    },
    CommandSpec {
        title: "Switch",
        section: "Forms",
        shortcut: "C",
        page: live_id!(switch_page),
    },
    CommandSpec {
        title: "Tabs",
        section: "Navigation",
        shortcut: "T",
        page: live_id!(tabs_page),
    },
    CommandSpec {
        title: "Textarea",
        section: "Forms",
        shortcut: "A",
        page: live_id!(textarea_page),
    },
    CommandSpec {
        title: "Toggle",
        section: "Forms",
        shortcut: "O",
        page: live_id!(toggle_page),
    },
];

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCommandPaletteBase = #(GalleryCommandPalette::register_widget(vm))

    mod.widgets.GalleryCommandPalette = set_type_default() do mod.widgets.GalleryCommandPaletteBase{
        width: Fill
        height: Fill
        open: false
        active_row_color: (shad_theme.color_secondary_hover)

        overlay: Modal{
            bg_view +: {
                draw_bg.color: vec4(0.0, 0.0, 0.0, 0.60)
            }

            content +: {
                width: 620
                height: Fit

                panel := RoundedView{
                    width: Fill
                    height: Fit
                    flow: Down
                    padding: Inset{left: 12, right: 12, top: 12, bottom: 12}
                    spacing: 10.0

                    draw_bg +: {
                        color: (shad_theme.color_popover)
                        border_radius: 18.0
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    search_shell := RoundedView{
                        width: Fill
                        height: Fit
                        flow: Right
                        align: Align{y: 0.5}
                        padding: Inset{left: 14, right: 14, top: 0, bottom: 0}
                        spacing: 10.0

                        draw_bg +: {
                            color: (shad_theme.color_secondary)
                            border_radius: 12.0
                            border_size: 1.0
                            border_color: (shad_theme.color_outline_border)
                        }

                        IconSearch{
                            icon_walk: Walk{width: 18, height: 18}
                            draw_icon.color: (shad_theme.color_muted_foreground)
                        }

                        search_input := ShadInputBorderless{
                            empty_text: "Type a command or search..."
                            draw_text.text_style.font_size: 14
                            draw_text.color_empty: (shad_theme.color_muted_foreground)
                        }
                    }

                    results := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 2.0

                        slot_0 := View{
                            width: Fill
                            height: Fit
                            flow: Down
                            visible: false
                            slot_0_header := Label{
                                margin: Inset{left: 12, right: 12, top: 8, bottom: 4}
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 10
                                text: "Section"
                            }
                            slot_0_row := RoundedView{
                                width: Fill
                                height: 44
                                flow: Overlay
                                draw_bg +: {
                                    color: #0000
                                    border_radius: 10.0
                                }
                                slot_0_button := ShadButtonGhost{
                                    width: Fill
                                    height: Fill
                                    padding: Inset{left: 14, right: 80, top: 0, bottom: 0}
                                    align: Align{x: 0.0, y: 0.5}
                                    text: "Command"
                                    draw_bg +: {
                                        color: #0000
                                        color_hover: #0000
                                        color_down: #0000
                                        color_focus: #0000
                                    }
                                    draw_text.text_style.font_size: 13
                                }
                                slot_0_shortcut_wrap := View{
                                    width: Fill
                                    height: Fill
                                    align: Align{x: 1.0, y: 0.5}
                                    padding: Inset{right: 12}
                                    slot_0_shortcut := Label{
                                        draw_text.color: (shad_theme.color_muted_foreground)
                                        draw_text.text_style.font_size: 10
                                        text: ""
                                    }
                                }
                            }
                        }

                        slot_1 := View{
                            width: Fill
                            height: Fit
                            flow: Down
                            visible: false
                            slot_1_header := Label{
                                margin: Inset{left: 12, right: 12, top: 8, bottom: 4}
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 10
                                text: "Section"
                            }
                            slot_1_row := RoundedView{
                                width: Fill
                                height: 44
                                flow: Overlay
                                draw_bg +: {
                                    color: #0000
                                    border_radius: 10.0
                                }
                                slot_1_button := ShadButtonGhost{
                                    width: Fill
                                    height: Fill
                                    padding: Inset{left: 14, right: 80, top: 0, bottom: 0}
                                    align: Align{x: 0.0, y: 0.5}
                                    text: "Command"
                                    draw_bg +: {
                                        color: #0000
                                        color_hover: #0000
                                        color_down: #0000
                                        color_focus: #0000
                                    }
                                    draw_text.text_style.font_size: 13
                                }
                                slot_1_shortcut_wrap := View{
                                    width: Fill
                                    height: Fill
                                    align: Align{x: 1.0, y: 0.5}
                                    padding: Inset{right: 12}
                                    slot_1_shortcut := Label{
                                        draw_text.color: (shad_theme.color_muted_foreground)
                                        draw_text.text_style.font_size: 10
                                        text: ""
                                    }
                                }
                            }
                        }

                        slot_2 := View{
                            width: Fill
                            height: Fit
                            flow: Down
                            visible: false
                            slot_2_header := Label{
                                margin: Inset{left: 12, right: 12, top: 8, bottom: 4}
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 10
                                text: "Section"
                            }
                            slot_2_row := RoundedView{
                                width: Fill
                                height: 44
                                flow: Overlay
                                draw_bg +: {
                                    color: #0000
                                    border_radius: 10.0
                                }
                                slot_2_button := ShadButtonGhost{
                                    width: Fill
                                    height: Fill
                                    padding: Inset{left: 14, right: 80, top: 0, bottom: 0}
                                    align: Align{x: 0.0, y: 0.5}
                                    text: "Command"
                                    draw_bg +: {
                                        color: #0000
                                        color_hover: #0000
                                        color_down: #0000
                                        color_focus: #0000
                                    }
                                    draw_text.text_style.font_size: 13
                                }
                                slot_2_shortcut_wrap := View{
                                    width: Fill
                                    height: Fill
                                    align: Align{x: 1.0, y: 0.5}
                                    padding: Inset{right: 12}
                                    slot_2_shortcut := Label{
                                        draw_text.color: (shad_theme.color_muted_foreground)
                                        draw_text.text_style.font_size: 10
                                        text: ""
                                    }
                                }
                            }
                        }

                        slot_3 := View{
                            width: Fill
                            height: Fit
                            flow: Down
                            visible: false
                            slot_3_header := Label{
                                margin: Inset{left: 12, right: 12, top: 8, bottom: 4}
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 10
                                text: "Section"
                            }
                            slot_3_row := RoundedView{
                                width: Fill
                                height: 44
                                flow: Overlay
                                draw_bg +: {
                                    color: #0000
                                    border_radius: 10.0
                                }
                                slot_3_button := ShadButtonGhost{
                                    width: Fill
                                    height: Fill
                                    padding: Inset{left: 14, right: 80, top: 0, bottom: 0}
                                    align: Align{x: 0.0, y: 0.5}
                                    text: "Command"
                                    draw_bg +: {
                                        color: #0000
                                        color_hover: #0000
                                        color_down: #0000
                                        color_focus: #0000
                                    }
                                    draw_text.text_style.font_size: 13
                                }
                                slot_3_shortcut_wrap := View{
                                    width: Fill
                                    height: Fill
                                    align: Align{x: 1.0, y: 0.5}
                                    padding: Inset{right: 12}
                                    slot_3_shortcut := Label{
                                        draw_text.color: (shad_theme.color_muted_foreground)
                                        draw_text.text_style.font_size: 10
                                        text: ""
                                    }
                                }
                            }
                        }

                        slot_4 := View{
                            width: Fill
                            height: Fit
                            flow: Down
                            visible: false
                            slot_4_header := Label{
                                margin: Inset{left: 12, right: 12, top: 8, bottom: 4}
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 10
                                text: "Section"
                            }
                            slot_4_row := RoundedView{
                                width: Fill
                                height: 44
                                flow: Overlay
                                draw_bg +: {
                                    color: #0000
                                    border_radius: 10.0
                                }
                                slot_4_button := ShadButtonGhost{
                                    width: Fill
                                    height: Fill
                                    padding: Inset{left: 14, right: 80, top: 0, bottom: 0}
                                    align: Align{x: 0.0, y: 0.5}
                                    text: "Command"
                                    draw_bg +: {
                                        color: #0000
                                        color_hover: #0000
                                        color_down: #0000
                                        color_focus: #0000
                                    }
                                    draw_text.text_style.font_size: 13
                                }
                                slot_4_shortcut_wrap := View{
                                    width: Fill
                                    height: Fill
                                    align: Align{x: 1.0, y: 0.5}
                                    padding: Inset{right: 12}
                                    slot_4_shortcut := Label{
                                        draw_text.color: (shad_theme.color_muted_foreground)
                                        draw_text.text_style.font_size: 10
                                        text: ""
                                    }
                                }
                            }
                        }

                        slot_5 := View{
                            width: Fill
                            height: Fit
                            flow: Down
                            visible: false
                            slot_5_header := Label{
                                margin: Inset{left: 12, right: 12, top: 8, bottom: 4}
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 10
                                text: "Section"
                            }
                            slot_5_row := RoundedView{
                                width: Fill
                                height: 44
                                flow: Overlay
                                draw_bg +: {
                                    color: #0000
                                    border_radius: 10.0
                                }
                                slot_5_button := ShadButtonGhost{
                                    width: Fill
                                    height: Fill
                                    padding: Inset{left: 14, right: 80, top: 0, bottom: 0}
                                    align: Align{x: 0.0, y: 0.5}
                                    text: "Command"
                                    draw_bg +: {
                                        color: #0000
                                        color_hover: #0000
                                        color_down: #0000
                                        color_focus: #0000
                                    }
                                    draw_text.text_style.font_size: 13
                                }
                                slot_5_shortcut_wrap := View{
                                    width: Fill
                                    height: Fill
                                    align: Align{x: 1.0, y: 0.5}
                                    padding: Inset{right: 12}
                                    slot_5_shortcut := Label{
                                        draw_text.color: (shad_theme.color_muted_foreground)
                                        draw_text.text_style.font_size: 10
                                        text: ""
                                    }
                                }
                            }
                        }

                        slot_6 := View{
                            width: Fill
                            height: Fit
                            flow: Down
                            visible: false
                            slot_6_header := Label{
                                margin: Inset{left: 12, right: 12, top: 8, bottom: 4}
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 10
                                text: "Section"
                            }
                            slot_6_row := RoundedView{
                                width: Fill
                                height: 44
                                flow: Overlay
                                draw_bg +: {
                                    color: #0000
                                    border_radius: 10.0
                                }
                                slot_6_button := ShadButtonGhost{
                                    width: Fill
                                    height: Fill
                                    padding: Inset{left: 14, right: 80, top: 0, bottom: 0}
                                    align: Align{x: 0.0, y: 0.5}
                                    text: "Command"
                                    draw_bg +: {
                                        color: #0000
                                        color_hover: #0000
                                        color_down: #0000
                                        color_focus: #0000
                                    }
                                    draw_text.text_style.font_size: 13
                                }
                                slot_6_shortcut_wrap := View{
                                    width: Fill
                                    height: Fill
                                    align: Align{x: 1.0, y: 0.5}
                                    padding: Inset{right: 12}
                                    slot_6_shortcut := Label{
                                        draw_text.color: (shad_theme.color_muted_foreground)
                                        draw_text.text_style.font_size: 10
                                        text: ""
                                    }
                                }
                            }
                        }

                        slot_7 := View{
                            width: Fill
                            height: Fit
                            flow: Down
                            visible: false
                            slot_7_header := Label{
                                margin: Inset{left: 12, right: 12, top: 8, bottom: 4}
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 10
                                text: "Section"
                            }
                            slot_7_row := RoundedView{
                                width: Fill
                                height: 44
                                flow: Overlay
                                draw_bg +: {
                                    color: #0000
                                    border_radius: 10.0
                                }
                                slot_7_button := ShadButtonGhost{
                                    width: Fill
                                    height: Fill
                                    padding: Inset{left: 14, right: 80, top: 0, bottom: 0}
                                    align: Align{x: 0.0, y: 0.5}
                                    text: "Command"
                                    draw_bg +: {
                                        color: #0000
                                        color_hover: #0000
                                        color_down: #0000
                                        color_focus: #0000
                                    }
                                    draw_text.text_style.font_size: 13
                                }
                                slot_7_shortcut_wrap := View{
                                    width: Fill
                                    height: Fill
                                    align: Align{x: 1.0, y: 0.5}
                                    padding: Inset{right: 12}
                                    slot_7_shortcut := Label{
                                        draw_text.color: (shad_theme.color_muted_foreground)
                                        draw_text.text_style.font_size: 10
                                        text: ""
                                    }
                                }
                            }
                        }

                        empty_state := View{
                            width: Fill
                            height: Fit
                            flow: Down
                            align: Align{x: 0.5}
                            padding: Inset{left: 12, right: 12, top: 18, bottom: 16}
                            spacing: 6.0
                            visible: false

                            empty_title := Label{
                                draw_text.color: (shad_theme.color_primary)
                                draw_text.text_style.font_size: 13
                                text: "No commands found"
                            }

                            empty_copy := Label{
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 11
                                text: "Try a different name like button, dialog, or input."
                            }
                        }
                    }

                    footer := View{
                        width: Fill
                        height: Fit
                        flow: Right
                        spacing: 8.0
                        margin: Inset{top: 4}

                        ShadKbd{ label := ShadKbdLabel{text: "Enter"} }
                        Label{
                            draw_text.color: (shad_theme.color_muted_foreground)
                            draw_text.text_style.font_size: 10
                            text: "Open"
                        }

                        ShadKbd{ label := ShadKbdLabel{text: "Esc"} }
                        Label{
                            draw_text.color: (shad_theme.color_muted_foreground)
                            draw_text.text_style.font_size: 10
                            text: "Close"
                        }

                        ShadKbd{ label := ShadKbdLabel{text: "Up/Down"} }
                        Label{
                            draw_text.color: (shad_theme.color_muted_foreground)
                            draw_text.text_style.font_size: 10
                            text: "Move"
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum GalleryCommandPaletteAction {
    #[default]
    None,
    Selected(LiveId),
}

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryCommandPalette {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[find]
    #[redraw]
    #[live]
    overlay: WidgetRef,
    #[live]
    open: bool,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[live]
    active_row_color: Vec4f,
    #[rust]
    query: String,
    #[rust]
    filtered_indices: Vec<usize>,
    #[rust]
    active_index: usize,
    #[rust]
    focus_search_on_next_draw: bool,
    #[rust]
    is_synced_open: bool,
}

impl GalleryCommandPalette {
    fn sync_modal_state(&mut self, cx: &mut Cx) {
        if self.is_synced_open == self.open {
            return;
        }

        if let Some(mut modal) = self.overlay.borrow_mut::<Modal>() {
            if self.open {
                modal.open(cx);
            } else {
                modal.close(cx);
            }
        }

        self.is_synced_open = self.open;
    }

    pub fn open(&mut self, cx: &mut Cx) {
        self.open = true;
        self.query.clear();
        self.active_index = 0;
        self.focus_search_on_next_draw = true;
        self.refresh_results(cx);
        self.sync_modal_state(cx);
    }

    pub fn close(&mut self, cx: &mut Cx) {
        self.open = false;
        self.query.clear();
        self.active_index = 0;
        self.focus_search_on_next_draw = false;
        self.overlay
            .text_input(cx, ids!(search_input))
            .set_text(cx, "");
        self.sync_modal_state(cx);
        self.redraw(cx);
    }

    pub fn toggle(&mut self, cx: &mut Cx) {
        if self.open {
            self.close(cx);
        } else {
            self.open(cx);
        }
    }

    fn normalize_query(&self) -> String {
        self.query.trim().to_ascii_lowercase()
    }

    fn refresh_results(&mut self, cx: &mut Cx) {
        let query = self.normalize_query();
        self.filtered_indices.clear();

        for (index, command) in COMMANDS.iter().enumerate() {
            if query.is_empty()
                || command.title.to_ascii_lowercase().contains(&query)
                || command.section.to_ascii_lowercase().contains(&query)
            {
                self.filtered_indices.push(index);
            }
        }

        if self.filtered_indices.is_empty() {
            self.active_index = 0;
        } else {
            self.active_index = self.active_index.min(self.filtered_indices.len() - 1);
        }

        self.sync_slots(cx);
        self.redraw(cx);
    }

    fn sync_slots(&mut self, cx: &mut Cx) {
        self.overlay
            .view(cx, ids!(empty_state))
            .set_visible(cx, self.filtered_indices.is_empty());

        let mut previous_section = None;
        for slot in 0..VISIBLE_SLOT_COUNT {
            let slot_path = slot_path(slot);
            let header_path = slot_header_path(slot);
            let row_path = slot_row_path(slot);
            let button_path = slot_button_path(slot);
            let shortcut_path = slot_shortcut_path(slot);

            if let Some(command_index) = self.filtered_indices.get(slot).copied() {
                let command = COMMANDS[command_index];
                self.overlay.view(cx, slot_path).set_visible(cx, true);
                self.overlay
                    .button(cx, button_path)
                    .set_text(cx, command.title);
                self.overlay
                    .label(cx, shortcut_path)
                    .set_text(cx, command.shortcut);

                let show_header = previous_section != Some(command.section);
                self.overlay
                    .label(cx, header_path)
                    .set_text(cx, command.section);
                self.overlay
                    .widget(cx, header_path)
                    .set_visible(cx, show_header);
                previous_section = Some(command.section);

                let mut row = self.overlay.view(cx, row_path);
                let background = if slot == self.active_index {
                    self.active_row_color
                } else {
                    Vec4f::all(0.0)
                };
                script_apply_eval!(cx, row, {
                    draw_bg +: {
                        color: #(background)
                        border_radius: 10.0
                    }
                });
            } else {
                self.overlay.view(cx, slot_path).set_visible(cx, false);
            }
        }
    }

    fn move_selection(&mut self, cx: &mut Cx, delta: i32) {
        if self.filtered_indices.is_empty() {
            return;
        }

        let max_index = self.filtered_indices.len() - 1;
        self.active_index = self
            .active_index
            .saturating_add_signed(delta as isize)
            .clamp(0, max_index);
        self.sync_slots(cx);
    }

    fn activate(&mut self, cx: &mut Cx) {
        if let Some(command_index) = self.filtered_indices.get(self.active_index).copied() {
            cx.widget_action(
                self.uid,
                GalleryCommandPaletteAction::Selected(COMMANDS[command_index].page),
            );
            self.close(cx);
        }
    }
}

impl Widget for GalleryCommandPalette {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.sync_modal_state(cx);

        if self.open {
            if let Event::KeyDown(key_event) = event {
                match key_event.key_code {
                    KeyCode::ArrowDown => {
                        self.move_selection(cx, 1);
                        return;
                    }
                    KeyCode::ArrowUp => {
                        self.move_selection(cx, -1);
                        return;
                    }
                    KeyCode::ReturnKey => {
                        self.activate(cx);
                        return;
                    }
                    KeyCode::Escape => {
                        self.close(cx);
                        return;
                    }
                    _ => {}
                }
            }

            let search_input = self.overlay.text_input(cx, ids!(search_input));

            self.overlay.handle_event(cx, event, scope);

            if let Event::Actions(actions) = event {
                if let Some(text) = search_input.changed(actions) {
                    self.query = text;
                    self.active_index = 0;
                    self.refresh_results(cx);
                }

                let content = self.overlay.widget(cx, ids!(content));
                if actions
                    .find_widget_action(content.widget_uid())
                    .is_some_and(|action| matches!(action.cast(), ModalAction::Dismissed))
                {
                    self.close(cx);
                    return;
                }

                for slot in 0..VISIBLE_SLOT_COUNT {
                    if self
                        .overlay
                        .button(cx, slot_button_path(slot))
                        .clicked(actions)
                    {
                        self.active_index = slot;
                        self.activate(cx);
                        return;
                    }
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.sync_modal_state(cx);

        if !self.open {
            return DrawStep::done();
        }

        self.sync_slots(cx);
        while !self.overlay.draw_walk(cx, scope, walk).is_done() {}

        if self.focus_search_on_next_draw {
            self.focus_search_on_next_draw = false;
            self.overlay
                .text_input(cx, ids!(search_input))
                .set_key_focus(cx);
        }

        DrawStep::done()
    }
}

fn slot_path(index: usize) -> &'static [LiveId] {
    match index {
        0 => ids!(slot_0),
        1 => ids!(slot_1),
        2 => ids!(slot_2),
        3 => ids!(slot_3),
        4 => ids!(slot_4),
        5 => ids!(slot_5),
        6 => ids!(slot_6),
        7 => ids!(slot_7),
        _ => ids!(slot_0),
    }
}

fn slot_header_path(index: usize) -> &'static [LiveId] {
    match index {
        0 => ids!(slot_0_header),
        1 => ids!(slot_1_header),
        2 => ids!(slot_2_header),
        3 => ids!(slot_3_header),
        4 => ids!(slot_4_header),
        5 => ids!(slot_5_header),
        6 => ids!(slot_6_header),
        7 => ids!(slot_7_header),
        _ => ids!(slot_0_header),
    }
}

fn slot_row_path(index: usize) -> &'static [LiveId] {
    match index {
        0 => ids!(slot_0_row),
        1 => ids!(slot_1_row),
        2 => ids!(slot_2_row),
        3 => ids!(slot_3_row),
        4 => ids!(slot_4_row),
        5 => ids!(slot_5_row),
        6 => ids!(slot_6_row),
        7 => ids!(slot_7_row),
        _ => ids!(slot_0_row),
    }
}

fn slot_button_path(index: usize) -> &'static [LiveId] {
    match index {
        0 => ids!(slot_0_button),
        1 => ids!(slot_1_button),
        2 => ids!(slot_2_button),
        3 => ids!(slot_3_button),
        4 => ids!(slot_4_button),
        5 => ids!(slot_5_button),
        6 => ids!(slot_6_button),
        7 => ids!(slot_7_button),
        _ => ids!(slot_0_button),
    }
}

fn slot_shortcut_path(index: usize) -> &'static [LiveId] {
    match index {
        0 => ids!(slot_0_shortcut),
        1 => ids!(slot_1_shortcut),
        2 => ids!(slot_2_shortcut),
        3 => ids!(slot_3_shortcut),
        4 => ids!(slot_4_shortcut),
        5 => ids!(slot_5_shortcut),
        6 => ids!(slot_6_shortcut),
        7 => ids!(slot_7_shortcut),
        _ => ids!(slot_0_shortcut),
    }
}
