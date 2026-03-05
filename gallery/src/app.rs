use makepad_components::accordion::AccordionItemWidgetRefExt;
use makepad_components::makepad_widgets::*;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    let SidebarItem = ButtonFlatter{
        width: Fill
        height: 32
        draw_text.text_style.font_size: 10
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_bg.color: #0000
        padding: Inset{left: 10, right: 10}
        align: Align{x: 0.0, y: 0.5}
        text: "Item"
    }

    let SectionLabel = Label{
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 9
    }

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.inner_size: vec2(1400 900)
                body +: {
                    flow: Right

                    sidebar := View{
                        width: 280
                        height: Fill
                        flow: Down
                        draw_bg.color: (shad_theme.color_secondary)
                        padding: Inset{top: 14, right: 14, bottom: 14, left: 14}
                        spacing: 10.0

                        Label{
                            text: "Makepad Component\nGallery"
                            draw_text.color: (shad_theme.color_primary)
                            draw_text.text_style.font_size: 13
                        }

                        SectionLabel{text: "Components"}

                        ScrollYView{
                            width: Fill
                            height: Fill
                            flow: Down

                            sidebar_accordion := SidebarItem{text: "Accordion"}
                            sidebar_button := SidebarItem{text: "Button"}
                        }
                    }

                    content_flip := PageFlip{
                        width: Fill
                        height: Fill
                        active_page: @accordion_page

                        accordion_page := View{
                            width: Fill
                            height: Fill
                            flow: Down
                            draw_bg.color: (shad_theme.color_background)
                            padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
                            spacing: 12.0

                            Label{
                                text: "Accordion"
                                draw_text.color: (shad_theme.color_primary)
                                draw_text.text_style.font_size: 18
                            }

                            Label{
                                text: "Accordion component from makepad-components library"
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 10
                            }

                            Hr{}

                            View{
                                width: Fill
                                height: Fit
                                flow: Right
                                spacing: 6.0

                                ButtonFlatter{text: "XSmall"}
                                ButtonFlatter{text: "Small"}
                                size_medium := Button{text: "Medium"}
                                ButtonFlatter{text: "Large"}

                                View{width: Fill, height: Fit}

                                option_multiple := CheckBox{text: "Multiple"}
                                option_icon := CheckBox{text: "Icon"}
                                option_disabled := CheckBox{text: "Disabled"}
                                option_bordered := CheckBox{text: "Bordered"}
                            }

                            Label{
                                text: "Normal"
                                draw_text.color: (shad_theme.color_muted_foreground)
                            }

                            accordion_wrap := RoundedView{
                                width: Fill
                                height: Fit
                                draw_bg.color: (shad_theme.color_secondary)
                                draw_bg.radius: (shad_theme.radius)
                                padding: Inset{top: 8, right: 8, bottom: 8, left: 8}

                                accordion_panel := Accordion{
                                    item_accessible := AccordionItem{
                                        header: View{
                                            width: Fill
                                            height: Fit
                                            flow: Right
                                            align: Align{y: 0.5}
                                            padding: Inset{top: 10, bottom: 10, left: 12, right: 12}
                                            spacing: 8.0

                                            title := Label{text: "Is it accessible?"}
                                            View{width: Fill, height: Fit}
                                            fold_button := FoldButton{}
                                        }
                                        body: View{
                                            width: Fill
                                            height: Fit
                                            flow: Down
                                            padding: Inset{left: 12, right: 12, top: 0, bottom: 12}
                                            Label{
                                                text: "Yes. This accordion is keyboard and mouse friendly by default through FoldHeader/FoldButton behavior."
                                                draw_text.color: (shad_theme.color_muted_foreground)
                                                draw_text.text_style.font_size: 10
                                            }
                                        }
                                    }

                                    item_styled := AccordionItem{
                                        header: View{
                                            width: Fill
                                            height: Fit
                                            flow: Right
                                            align: Align{y: 0.5}
                                            padding: Inset{top: 10, bottom: 10, left: 12, right: 12}
                                            spacing: 8.0

                                            title := Label{text: "Is it styled with complex elements?"}
                                            View{width: Fill, height: Fit}
                                            fold_button := FoldButton{}
                                        }
                                        body: View{
                                            width: Fill
                                            height: Fit
                                            flow: Down
                                            padding: Inset{left: 12, right: 12, top: 0, bottom: 12}
                                            spacing: 8.0

                                            Label{
                                                text: "We can put any view here, like a row with toggles."
                                                draw_text.color: (shad_theme.color_muted_foreground)
                                                draw_text.text_style.font_size: 10
                                            }

                                            View{
                                                width: Fill
                                                height: Fit
                                                flow: Right
                                                spacing: 16

                                                Toggle{text: "Switch"}
                                                CheckBox{text: "Or a CheckBox"}
                                            }
                                        }
                                    }

                                    item_third := AccordionItem{
                                        header: View{
                                            width: Fill
                                            height: Fit
                                            flow: Right
                                            align: Align{y: 0.5}
                                            padding: Inset{top: 10, bottom: 10, left: 12, right: 12}
                                            spacing: 8.0

                                            title := Label{text: "This is third accordion"}
                                            View{width: Fill, height: Fit}
                                            fold_button := FoldButton{}
                                        }
                                        body: View{
                                            width: Fill
                                            height: Fit
                                            flow: Down
                                            padding: Inset{left: 12, right: 12, top: 0, bottom: 12}
                                            Label{
                                                text: "This is third accordion content. It can be any view, like a text view or a button."
                                                draw_text.color: (shad_theme.color_muted_foreground)
                                                draw_text.text_style.font_size: 10
                                            }
                                        }
                                    }
                                }
                            }

                            View{width: Fill, height: Fill}
                        }

                        button_page := View{
                            width: Fill
                            height: Fill
                            flow: Down
                            draw_bg.color: (shad_theme.color_background)
                            padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
                            spacing: 12.0

                            Label{
                                text: "Button"
                                draw_text.color: (shad_theme.color_primary)
                                draw_text.text_style.font_size: 18
                            }

                            Label{
                                text: "Shadcn-inspired button components from makepad-components library"
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 10
                            }

                            Hr{}

                            Label{
                                text: "Variants"
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 10
                            }

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

                            Label{
                                text: "Sizes"
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 10
                            }

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

                            Label{
                                text: "Destructive Sizes"
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 10
                            }

                            View{
                                width: Fill
                                height: Fit
                                flow: Right
                                align: Align{y: 0.5}
                                spacing: 8.0

                                ShadButtonDestructive{
                                    height: 28
                                    padding: Inset{left: 12, right: 12, top: 0, bottom: 0}
                                    draw_text.text_style.font_size: 10
                                    text: "Small"
                                }
                                ShadButtonDestructive{text: "Default"}
                                ShadButtonDestructive{
                                    height: 44
                                    padding: Inset{left: 32, right: 32, top: 0, bottom: 0}
                                    draw_text.text_style.font_size: 13
                                    text: "Large"
                                }
                            }

                            Label{
                                text: "Outline Variations"
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 10
                            }

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

                            View{width: Fill, height: Fill}
                        }
                    }
                }
            }
        }
    }
}

impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        crate::makepad_widgets::script_mod(vm);
        makepad_components::script_mod(vm);
        App::from_script_mod(vm, self::script_mod)
    }

    fn sync_accordion_state(&self, cx: &mut Cx) {
        self.ui
            .accordion_item(cx, ids!(accordion_panel.item_accessible))
            .set_is_open(cx, self.open_accessible);
        self.ui
            .accordion_item(cx, ids!(accordion_panel.item_styled))
            .set_is_open(cx, self.open_styled);
        self.ui
            .accordion_item(cx, ids!(accordion_panel.item_third))
            .set_is_open(cx, self.open_third);
    }

    fn set_single_open(&mut self, item: LiveId, is_open: bool) {
        if is_open {
            self.open_accessible = item == live_id!(item_accessible);
            self.open_styled = item == live_id!(item_styled);
            self.open_third = item == live_id!(item_third);
            return;
        }

        if item == live_id!(item_accessible) {
            self.open_accessible = false;
        } else if item == live_id!(item_styled) {
            self.open_styled = false;
        } else if item == live_id!(item_third) {
            self.open_third = false;
        }
    }

    fn set_multi_open(&mut self, item: LiveId, is_open: bool) {
        if item == live_id!(item_accessible) {
            self.open_accessible = is_open;
        } else if item == live_id!(item_styled) {
            self.open_styled = is_open;
        } else if item == live_id!(item_third) {
            self.open_third = is_open;
        }
    }

    fn normalize_single_mode(&mut self) {
        let open_count =
            self.open_accessible as u8 + self.open_styled as u8 + self.open_third as u8;
        if open_count <= 1 {
            return;
        }

        if self.open_accessible {
            self.open_styled = false;
            self.open_third = false;
        } else if self.open_styled {
            self.open_third = false;
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    allow_multiple: bool,
    #[rust]
    open_accessible: bool,
    #[rust]
    open_styled: bool,
    #[rust]
    open_third: bool,
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(sidebar_accordion)).clicked(actions) {
            self.ui
                .page_flip(cx, ids!(content_flip))
                .set_active_page(cx, live_id!(accordion_page));
        }

        if self.ui.button(cx, ids!(sidebar_button)).clicked(actions) {
            self.ui
                .page_flip(cx, ids!(content_flip))
                .set_active_page(cx, live_id!(button_page));
        }

        if let Some(value) = self
            .ui
            .check_box(cx, ids!(option_multiple))
            .changed(actions)
        {
            self.allow_multiple = value;
            if !self.allow_multiple {
                self.normalize_single_mode();
            }
            self.sync_accordion_state(cx);
        }

        let item_actions = [
            (
                live_id!(item_accessible),
                self.ui
                    .fold_button(cx, ids!(accordion_panel.item_accessible.header.fold_button))
                    .opening(actions),
                self.ui
                    .fold_button(cx, ids!(accordion_panel.item_accessible.header.fold_button))
                    .closing(actions),
            ),
            (
                live_id!(item_styled),
                self.ui
                    .fold_button(cx, ids!(accordion_panel.item_styled.header.fold_button))
                    .opening(actions),
                self.ui
                    .fold_button(cx, ids!(accordion_panel.item_styled.header.fold_button))
                    .closing(actions),
            ),
            (
                live_id!(item_third),
                self.ui
                    .fold_button(cx, ids!(accordion_panel.item_third.header.fold_button))
                    .opening(actions),
                self.ui
                    .fold_button(cx, ids!(accordion_panel.item_third.header.fold_button))
                    .closing(actions),
            ),
        ];

        let mut changed = false;
        for (item, opening, closing) in item_actions {
            if opening {
                if self.allow_multiple {
                    self.set_multi_open(item, true);
                } else {
                    self.set_single_open(item, true);
                }
                changed = true;
            } else if closing {
                if self.allow_multiple {
                    self.set_multi_open(item, false);
                } else {
                    self.set_single_open(item, false);
                }
                changed = true;
            }
        }

        if changed {
            self.sync_accordion_state(cx);
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Startup = event {
            self.allow_multiple = true;
            self.open_accessible = false;
            self.open_styled = true;
            self.open_third = true;
            self.ui
                .check_box(cx, ids!(option_multiple))
                .set_active(cx, true);
            self.sync_accordion_state(cx);
        }

        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());

        // FoldHeader currently reacts to FoldButton actions broadly; re-apply our
        // explicit accordion state after widget action handling to keep icon/body in sync.
        if let Event::Actions(_) = event {
            self.sync_accordion_state(cx);
        }
    }
}
