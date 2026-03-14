use crate::ui::catalog;
use makepad_components::makepad_widgets::*;

const RESULTS_SCROLL_SPEED: f64 = 18.0;
const RESULTS_MAX_ITEMS_TO_SHOW: usize = 8;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryCommandPaletteBase = #(GalleryCommandPalette::register_widget(vm))

    mod.widgets.GalleryCommandPaletteRow = View{
        width: Fill
        height: Fit
        flow: Down

        header := Label{
            margin: Inset{left: 12, right: 12, top: 8, bottom: 4}
            visible: false
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
            text: "Section"
        }

        row := RoundedView{
            width: Fill
            height: 44
            flow: Right
            align: Align{y: 0.5}
            padding: Inset{left: 14, right: 12, top: 0, bottom: 0}
            spacing: 12.0
            draw_bg +: {
                color: #0000
                border_radius: 10.0
            }

            button := ShadButtonGhost{
                width: Fill
                height: 36
                padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
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

            shortcut := Label{
                width: Fit
                draw_text.color: (shad_theme.color_muted_foreground)
                draw_text.text_style.font_size: 10
                text: ""
            }
        }
    }

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

                    results_shell := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 2.0

                        results := PortalList{
                            width: Fill
                            height: 368.0
                            flow: Down
                            max_pull_down: 0.0
                            capture_overload: false
                            grab_key_focus: false
                            auto_tail: false
                            selectable: false
                            drag_scrolling: true

                            Item := mod.widgets.GalleryCommandPaletteRow{}
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

    fn sync_empty_state(&mut self, cx: &mut Cx) {
        let has_results = !self.filtered_indices.is_empty();
        self.overlay
            .widget(cx, ids!(results))
            .set_visible(cx, has_results);
        self.overlay
            .view(cx, ids!(empty_state))
            .set_visible(cx, !has_results);
    }

    fn reset_results_position(&mut self, cx: &mut Cx) {
        self.overlay.portal_list(cx, ids!(results)).set_first_id(0);
    }

    fn scroll_active_into_view(&mut self, cx: &mut Cx) {
        if self.filtered_indices.is_empty() {
            return;
        }

        self.overlay
            .portal_list(cx, ids!(results))
            .smooth_scroll_to(
                cx,
                self.active_index,
                RESULTS_SCROLL_SPEED,
                Some(RESULTS_MAX_ITEMS_TO_SHOW),
            );
    }

    pub fn open(&mut self, cx: &mut Cx) {
        self.open = true;
        self.query.clear();
        self.active_index = 0;
        self.focus_search_on_next_draw = true;
        self.overlay
            .text_input(cx, ids!(search_input))
            .set_text(cx, "");
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

        for (index, command) in catalog::entries().iter().enumerate() {
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

        self.sync_empty_state(cx);
        self.reset_results_position(cx);
        self.redraw(cx);
    }

    fn draw_results(&mut self, cx: &mut Cx2d, list: &mut PortalList) {
        list.set_item_range(cx, 0, self.filtered_indices.len());
        let entries = catalog::entries();

        while let Some(item_id) = list.next_visible_item(cx) {
            let Some(command_index) = self.filtered_indices.get(item_id).copied() else {
                continue;
            };

            let command = entries[command_index];
            let item = list.item(cx, item_id, id!(Item)).as_view();
            let show_header = item_id == 0
                || self
                    .filtered_indices
                    .get(item_id - 1)
                    .is_some_and(|previous| entries[*previous].section != command.section);

            item.widget(cx, ids!(header)).set_visible(cx, show_header);
            item.label(cx, ids!(header)).set_text(cx, command.section);
            item.button(cx, ids!(button)).set_text(cx, command.title);
            item.label(cx, ids!(shortcut))
                .set_text(cx, command.shortcut);

            let background = if item_id == self.active_index {
                self.active_row_color
            } else {
                Vec4f::all(0.0)
            };
            let mut row = item.view(cx, ids!(row));
            script_apply_eval!(cx, row, {
                draw_bg +: {
                    color: #(background)
                    border_radius: 10.0
                }
            });

            item.draw_all(cx, &mut Scope::empty());
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
        self.scroll_active_into_view(cx);
        self.redraw(cx);
    }

    fn activate(&mut self, cx: &mut Cx) {
        if let Some(command_index) = self.filtered_indices.get(self.active_index).copied() {
            cx.widget_action(
                self.uid,
                GalleryCommandPaletteAction::Selected(catalog::entries()[command_index].page),
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
            let results = self.overlay.portal_list(cx, ids!(results));

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

                if !results.any_items_with_actions(actions) {
                    return;
                }

                for (item_id, item) in results.items_with_actions(actions) {
                    if item.button(cx, ids!(button)).clicked(actions) {
                        self.active_index = item_id;
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

        if self.query.is_empty() && self.filtered_indices.is_empty() {
            self.refresh_results(cx);
        }

        self.sync_empty_state(cx);
        while let Some(step) = self.overlay.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = step.as_portal_list().borrow_mut() {
                self.draw_results(cx, &mut *list);
            }
        }

        if self.focus_search_on_next_draw {
            self.focus_search_on_next_draw = false;
            self.overlay
                .text_input(cx, ids!(search_input))
                .set_key_focus(cx);
        }

        DrawStep::done()
    }
}
