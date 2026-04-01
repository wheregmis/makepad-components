use crate::input::ShadSearchInputWidgetRefExt;
use crate::button::ShadButtonWidgetExt;
use crate::internal::actions::{emit_widget_action, widget_action_map};
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

const RESULTS_SCROLL_SPEED: f64 = 18.0;
const RESULTS_MAX_ITEMS_TO_SHOW: usize = 8;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ShadCommandPaletteItem {
    pub title: String,
    pub section: String,
    pub shortcut: String,
    pub keywords: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub enum ShadCommandPaletteAction {
    Selected(usize),
    QueryChanged(String),
    OpenChanged(bool),
    #[default]
    None,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct CommandSearchTerm {
    title: String,
    section: String,
    shortcut: String,
    keywords: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct CommandPaletteRowState {
    command_index: usize,
    show_header: bool,
    is_active: bool,
}

fn search_term_for_item(item: &ShadCommandPaletteItem) -> CommandSearchTerm {
    CommandSearchTerm {
        title: item.title.to_ascii_lowercase(),
        section: item.section.to_ascii_lowercase(),
        shortcut: item.shortcut.to_ascii_lowercase(),
        keywords: item
            .keywords
            .iter()
            .map(|value| value.to_ascii_lowercase())
            .collect(),
    }
}

fn matches_command_query(term: &CommandSearchTerm, query: &str) -> bool {
    query.is_empty()
        || term.title.contains(query)
        || term.section.contains(query)
        || term.shortcut.contains(query)
        || term.keywords.iter().any(|value| value.contains(query))
}

fn command_results_summary(
    query: &str,
    matches_count: usize,
    total: usize,
    item_noun_plural: &str,
    search_help: &str,
) -> String {
    let query = query.trim();
    if query.is_empty() {
        format!("Showing all {total} {item_noun_plural}. {search_help}")
    } else if matches_count == 0 {
        format!("No {item_noun_plural} matched \"{query}\".")
    } else {
        format!("Showing {matches_count} of {total} {item_noun_plural} for \"{query}\".")
    }
}

fn command_palette_secondary_action_label(query: &str) -> &'static str {
    if query.trim().is_empty() {
        "Close"
    } else {
        "Clear"
    }
}

fn sync_cached_row_state<K>(
    cache: &mut HashMap<K, CommandPaletteRowState>,
    key: K,
    next: CommandPaletteRowState,
    item_existed: bool,
) -> bool
where
    K: Eq + Hash + Copy,
{
    if !item_existed {
        cache.remove(&key);
    }
    if cache.get(&key).copied() == Some(next) {
        return false;
    }
    cache.insert(key, next);
    true
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadCommandPaletteBase = #(ShadCommandPalette::register_widget(vm))

    mod.widgets.ShadCommandPaletteRowButton = set_type_default() do mod.widgets.ShadNavButtonBase{
        width: Fill
        height: 36
        grab_key_focus: false
        padding: Inset{left: 0, right: 0, top: 0, bottom: 0}
        align: Align{x: 0.0, y: 0.5}
        text: "Command"
        draw_bg +: {
            color: (shad_theme.color_clear)
            color_hover: (shad_theme.color_ghost_hover)
            color_down: (shad_theme.color_ghost_down)
            color_focus: (shad_theme.color_ghost_hover)
            border_size: 0.0
            border_radius: (shad_theme.radius_lg)
            border_color: (shad_theme.color_clear)
        }
        draw_text.color: (shad_theme.color_primary)
        draw_text.color_hover: (shad_theme.color_primary)
        draw_text.color_down: (shad_theme.color_primary)
        draw_text.color_focus: (shad_theme.color_primary)
        draw_text.text_style.font_size: 13
    }

    mod.widgets.ShadCommandPaletteRow = View{
        width: Fill
        height: Fit
        flow: Down

        header := ShadSectionHeader{
            margin: Inset{left: 12, right: 12, top: 8, bottom: 4}
            visible: false
            draw_text.color: (shad_theme.color_muted_foreground)
            draw_text.text_style.font_size: 10
            text: "Section"
        }

        row := ShadSurface{
            width: Fill
            height: 44
            flow: Right
            align: Align{y: 0.5}
            padding: Inset{left: 14, right: 12, top: 0, bottom: 0}
            spacing: 12.0
            draw_bg +: {
                color: (shad_theme.color_clear)
                border_radius: (shad_theme.radius_lg)
                border_size: 0.0
            }

            button := mod.widgets.ShadCommandPaletteRowButton{}

            shortcut := ShadSectionHeader{
                width: Fit
                draw_text.color: (shad_theme.color_muted_foreground)
                draw_text.text_style.font_size: 10
                text: ""
            }
        }
    }

    mod.widgets.ShadCommandPalette = set_type_default() do mod.widgets.ShadCommandPaletteBase{
        width: Fill
        height: Fill
        open: false
        active_row_color: (shad_theme.color_secondary_hover)
        row_radius: (shad_theme.radius_lg)
        item_noun_plural: "commands"
        search_help: "Search by title, section, or shortcut."

        overlay: Modal{
            bg_view +: {
                draw_bg.color: (shad_theme.color_overlay)
            }

            content +: {
                width: 360
                height: Fit

                panel := ShadSurface{
                    width: Fill
                    height: Fit
                    flow: Down
                    padding: Inset{left: 12, right: 12, top: 12, bottom: 12}
                    spacing: 10.0

                    draw_bg +: {
                        color: (shad_theme.color_popover)
                        border_radius: (shad_theme.radius_xl)
                        border_size: (shad_theme.border_size)
                        border_color: (shad_theme.color_outline_border)
                    }

                    search_label := ShadFieldLabel{
                        text: "Search commands"
                    }

                    search_row := View{
                        width: Fill
                        height: Fit
                        flow: Right
                        align: Align{y: 0.5}
                        spacing: 8.0

                        search_input := ShadSearchInput{
                            width: Fill
                            empty_text: "Search commands..."
                            show_clear_button: false
                        }

                        clear_search_btn := ShadButton{
                            variant: ShadButtonVariant.Ghost
                            text: "Close"
                        }
                    }

                    results_summary := ShadFieldDescription{
                        text: "Showing all 0 commands. Search by title, section, or shortcut."
                    }

                    results_shell := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 2.0

                        results := PortalList{
                            width: Fill
                            height: 320.0
                            flow: Down
                            max_pull_down: 0.0
                            capture_overload: false
                            grab_key_focus: false
                            auto_tail: false
                            selectable: false
                            drag_scrolling: true

                            Item := mod.widgets.ShadCommandPaletteRow{}
                        }

                        empty_state := View{
                            width: Fill
                            height: Fit
                            flow: Down
                            align: Align{x: 0.5}
                            padding: Inset{left: 12, right: 12, top: 18, bottom: 16}
                            spacing: 6.0
                            visible: false

                            empty_title := ShadLabel{
                                draw_text.color: (shad_theme.color_primary)
                                draw_text.text_style.font_size: 13
                                text: "No commands found"
                            }

                            empty_copy := ShadFieldDescription{
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 11
                                text: "Try a component name, a section, or a shortcut tag."
                            }
                        }
                    }

                    footer := View{
                        width: Fill
                        height: Fit
                        flow: Right{wrap: true}
                        spacing: 8.0
                        margin: Inset{top: 4}

                        ShadKbd{ label := ShadKbdLabel{text: "Enter"} }
                        footer_open_label := ShadSectionHeader{
                            draw_text.color: (shad_theme.color_muted_foreground)
                            draw_text.text_style.font_size: 10
                            text: "Open"
                        }

                        ShadKbd{ label := ShadKbdLabel{text: "Esc"} }
                        footer_close_label := ShadSectionHeader{
                            draw_text.color: (shad_theme.color_muted_foreground)
                            draw_text.text_style.font_size: 10
                            text: "Clear / Close"
                        }

                        ShadKbd{ label := ShadKbdLabel{text: "Up/Down"} }
                        footer_move_label := ShadSectionHeader{
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

#[derive(Script, Widget)]
pub struct ShadCommandPalette {
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
    /// Height of the scrollable results viewport (PortalList) inside the modal.
    /// Callers should size this instead of trying to force the overall widget height.
    #[live(320.0)]
    viewport_height: f64,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[live]
    active_row_color: Vec4f,
    #[live]
    row_radius: f64,
    #[live]
    item_noun_plural: ArcStringMut,
    #[live]
    search_help: ArcStringMut,
    #[rust]
    items: Arc<[ShadCommandPaletteItem]>,
    #[rust]
    search_terms: Vec<CommandSearchTerm>,
    #[rust]
    query: String,
    #[rust]
    filtered_indices: Vec<usize>,
    #[rust]
    filtered_indices_scratch: Vec<usize>,
    #[rust]
    active_index: usize,
    #[rust]
    focus_search_on_next_draw: bool,
    #[rust]
    is_synced_open: bool,
    #[rust]
    has_results_cache: Option<bool>,
    #[rust]
    summary_text_cache: String,
    #[rust]
    secondary_action_label_cache: String,
    #[rust]
    row_state_by_uid: HashMap<WidgetUid, CommandPaletteRowState>,
    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}

impl ScriptHook for ShadCommandPalette {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        vm.with_cx_mut(|cx| {
            // Keep the widget reusable: callers own the results viewport height.
            let viewport_height = self.viewport_height;
            let mut results_list = self
                .overlay
                .portal_list(cx, ids!(content.panel.results_shell.results));
            script_apply_eval!(cx, results_list, {
                height: #(viewport_height)
            });

            self.sync_modal_state(cx);
            self.refresh_results(cx);
        });
    }
}

impl ShadCommandPalette {
    fn search_input_ref(&self, cx: &Cx) -> crate::input::ShadSearchInputRef {
        self.overlay
            .shad_search_input(cx, ids!(content.panel.search_row.search_input))
    }

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
        if self.has_results_cache == Some(has_results) {
            return;
        }
        self.has_results_cache = Some(has_results);
        self.overlay
            .widget(cx, ids!(content.panel.results_shell.results))
            .set_visible(cx, has_results);
        self.overlay
            .view(cx, ids!(content.panel.results_shell.empty_state))
            .set_visible(cx, !has_results);
    }

    fn reset_results_position(&mut self, cx: &mut Cx) {
        self.overlay
            .portal_list(cx, ids!(content.panel.results_shell.results))
            .set_first_id(0);
    }

    fn scroll_active_into_view(&mut self, cx: &mut Cx) {
        if self.filtered_indices.is_empty() {
            return;
        }

        self.overlay
            .portal_list(cx, ids!(content.panel.results_shell.results))
            .smooth_scroll_to(
                cx,
                self.active_index,
                RESULTS_SCROLL_SPEED,
                Some(RESULTS_MAX_ITEMS_TO_SHOW),
            );
    }

    fn emit_open_state(&self, cx: &mut Cx, open: bool) {
        emit_widget_action(
            cx,
            &self.action_data,
            self.widget_uid(),
            ShadCommandPaletteAction::OpenChanged(open),
        );
    }

    fn emit_query_changed(&self, cx: &mut Cx, query: String) {
        emit_widget_action(
            cx,
            &self.action_data,
            self.widget_uid(),
            ShadCommandPaletteAction::QueryChanged(query),
        );
    }

    fn emit_selected(&self, cx: &mut Cx, index: usize) {
        emit_widget_action(
            cx,
            &self.action_data,
            self.widget_uid(),
            ShadCommandPaletteAction::Selected(index),
        );
    }

    pub fn set_open(&mut self, cx: &mut Cx, open: bool) {
        if self.open == open {
            return;
        }
        self.open = open;
        self.sync_modal_state(cx);
        self.emit_open_state(cx, open);
        if open {
            self.focus_search_on_next_draw = true;
            self.reset_results_position(cx);
        }
        self.redraw(cx);
    }

    pub fn open(&mut self, cx: &mut Cx) {
        self.set_open(cx, true);
    }

    pub fn close(&mut self, cx: &mut Cx) {
        self.set_open(cx, false);
    }

    pub fn toggle(&mut self, cx: &mut Cx) {
        self.set_open(cx, !self.open);
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn set_items(&mut self, cx: &mut Cx, items: Vec<ShadCommandPaletteItem>) {
        self.search_terms = items.iter().map(search_term_for_item).collect();
        self.items = Arc::from(items);
        self.active_index = 0;
        self.has_results_cache = None;
        self.row_state_by_uid.clear();
        self.refresh_results(cx);
    }

    pub fn set_query(&mut self, cx: &mut Cx, query: &str) {
        self.query.clear();
        self.query.push_str(query);
        self.active_index = 0;
        self.search_input_ref(cx).set_text(cx, query);
        self.refresh_results(cx);
    }

    pub fn query(&self) -> String {
        self.query.clone()
    }

    pub fn selected(&self, actions: &Actions) -> Option<usize> {
        widget_action_map::<ShadCommandPaletteAction, _, _>(actions, self.widget_uid(), |action| {
            if let ShadCommandPaletteAction::Selected(index) = action {
                Some(index)
            } else {
                None
            }
        })
    }

    pub fn query_changed(&self, actions: &Actions) -> Option<String> {
        widget_action_map::<ShadCommandPaletteAction, _, _>(actions, self.widget_uid(), |action| {
            if let ShadCommandPaletteAction::QueryChanged(query) = action {
                Some(query)
            } else {
                None
            }
        })
    }

    pub fn open_changed(&self, actions: &Actions) -> Option<bool> {
        widget_action_map::<ShadCommandPaletteAction, _, _>(actions, self.widget_uid(), |action| {
            if let ShadCommandPaletteAction::OpenChanged(open) = action {
                Some(open)
            } else {
                None
            }
        })
    }

    fn normalize_query(&self) -> String {
        self.query.trim().to_ascii_lowercase()
    }

    fn clear_query(&mut self, cx: &mut Cx) {
        self.query.clear();
        self.active_index = 0;
        self.search_input_ref(cx).clear(cx);
        self.search_input_ref(cx).focus(cx);
        self.refresh_results(cx);
        self.emit_query_changed(cx, String::new());
    }

    fn refresh_results(&mut self, cx: &mut Cx) {
        let query = self.normalize_query();
        let display_query = self.query.trim().to_string();
        let previous_active = self.active_index;
        self.filtered_indices_scratch.clear();

        for (index, item) in self.items.iter().enumerate() {
            let term = &self.search_terms[index];
            if matches_command_query(term, &query) {
                self.filtered_indices_scratch.push(index);
            } else if query.is_empty() && item.title.is_empty() {
                self.filtered_indices_scratch.push(index);
            }
        }

        let results_changed = self.filtered_indices != self.filtered_indices_scratch;
        if results_changed {
            std::mem::swap(
                &mut self.filtered_indices,
                &mut self.filtered_indices_scratch,
            );
            self.row_state_by_uid.clear();
            self.has_results_cache = None;
        }

        if self.filtered_indices.is_empty() {
            self.active_index = 0;
        } else {
            self.active_index = self.active_index.min(self.filtered_indices.len() - 1);
        }
        let active_changed = previous_active != self.active_index;

        self.sync_empty_state(cx);
        let summary_text = command_results_summary(
            &display_query,
            self.filtered_indices.len(),
            self.items.len(),
            self.item_noun_plural.as_ref(),
            self.search_help.as_ref(),
        );
        if self.summary_text_cache != summary_text {
            self.summary_text_cache.clear();
            self.summary_text_cache.push_str(&summary_text);
            self.overlay
                .label(cx, ids!(content.panel.results_summary))
                .set_text(cx, &self.summary_text_cache);
        }

        let secondary_action_label = command_palette_secondary_action_label(&display_query);
        if self.secondary_action_label_cache != secondary_action_label {
            self.secondary_action_label_cache.clear();
            self.secondary_action_label_cache
                .push_str(secondary_action_label);
            self.overlay
                .shad_button(cx, ids!(content.panel.search_row.clear_search_btn))
                .set_text(cx, &self.secondary_action_label_cache);
        }
        self.reset_results_position(cx);
        if results_changed || active_changed {
            self.redraw(cx);
        }
    }

    fn draw_results(&mut self, cx: &mut Cx2d, list: &mut PortalList) {
        list.set_item_range(cx, 0, self.filtered_indices.len());

        while let Some(item_id) = list.next_visible_item(cx) {
            let Some(command_index) = self.filtered_indices.get(item_id).copied() else {
                continue;
            };

            let command = &self.items[command_index];
            let (item, item_existed) = list.item_with_existed(cx, item_id, id!(Item));
            let item = item.as_view();
            let show_header = !command.section.is_empty()
                && (item_id == 0
                    || self
                        .filtered_indices
                        .get(item_id - 1)
                        .is_some_and(|previous| self.items[*previous].section != command.section));

            let mut row = item.view(cx, ids!(row));
            let row_uid = row.widget_uid();
            let next_state = CommandPaletteRowState {
                command_index,
                show_header,
                is_active: item_id == self.active_index,
            };
            if sync_cached_row_state(
                &mut self.row_state_by_uid,
                row_uid,
                next_state,
                item_existed,
            ) {
                item.widget(cx, ids!(header)).set_visible(cx, show_header);
                item.label(cx, ids!(header)).set_text(cx, &command.section);
                item.shad_button(cx, ids!(row.button))
                    .set_text(cx, &command.title);
                item.label(cx, ids!(row.shortcut))
                    .set_text(cx, &command.shortcut);

                let background = if next_state.is_active {
                    self.active_row_color
                } else {
                    Vec4f::all(0.0)
                };
                script_apply_eval!(cx, row, {
                    draw_bg +: {
                        color: #(background)
                        border_radius: #(self.row_radius)
                    }
                });
            }

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
        self.search_input_ref(cx).focus(cx);
        self.scroll_active_into_view(cx);
        self.redraw(cx);
    }

    fn activate(&mut self, cx: &mut Cx) {
        if let Some(command_index) = self.filtered_indices.get(self.active_index).copied() {
            self.emit_selected(cx, command_index);
            self.close(cx);
        }
    }
}

impl Widget for ShadCommandPalette {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.sync_modal_state(cx);

        if self.open {
            let search_input = self.search_input_ref(cx);
            let results = self
                .overlay
                .portal_list(cx, ids!(content.panel.results_shell.results));

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
                        if search_input.key_focus(cx) {
                            self.activate(cx);
                            return;
                        }
                    }
                    KeyCode::Escape => {
                        if self.normalize_query().is_empty() {
                            self.close(cx);
                        } else {
                            self.clear_query(cx);
                        }
                        return;
                    }
                    _ => {}
                }
            }

            self.overlay.handle_event(cx, event, scope);

            if let Event::Actions(actions) = event {
                if let Some(text) = search_input.changed(actions) {
                    self.query = text.clone();
                    self.active_index = 0;
                    self.refresh_results(cx);
                    self.emit_query_changed(cx, text);
                }

                if self
                    .overlay
                .shad_button(cx, ids!(content.panel.search_row.clear_search_btn))
                .clicked(actions)
                {
                    if self.normalize_query().is_empty() {
                        self.close(cx);
                    } else {
                        self.clear_query(cx);
                    }
                    return;
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
                    if item.shad_button(cx, ids!(row.button)).clicked(actions) {
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
            self.refresh_results(&mut *cx);
        }

        self.sync_empty_state(&mut *cx);
        while let Some(step) = self.overlay.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = step.as_portal_list().borrow_mut() {
                self.draw_results(cx, &mut *list);
            }
        }

        if self.focus_search_on_next_draw {
            self.focus_search_on_next_draw = false;
            self.search_input_ref(&*cx).focus(&mut *cx);
        }

        DrawStep::done()
    }
}

impl ShadCommandPaletteRef {
    pub fn set_open(&self, cx: &mut Cx, open: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_open(cx, open);
        }
    }

    pub fn open(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.open(cx);
        }
    }

    pub fn close(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.close(cx);
        }
    }

    pub fn toggle(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.toggle(cx);
        }
    }

    pub fn is_open(&self) -> bool {
        self.borrow().is_some_and(|inner| inner.is_open())
    }

    pub fn set_items(&self, cx: &mut Cx, items: Vec<ShadCommandPaletteItem>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_items(cx, items);
        }
    }

    pub fn set_query(&self, cx: &mut Cx, query: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_query(cx, query);
        }
    }

    pub fn query(&self) -> String {
        self.borrow()
            .map_or_else(String::new, |inner| inner.query())
    }

    pub fn selected(&self, actions: &Actions) -> Option<usize> {
        self.borrow().and_then(|inner| inner.selected(actions))
    }

    pub fn query_changed(&self, actions: &Actions) -> Option<String> {
        self.borrow().and_then(|inner| inner.query_changed(actions))
    }

    pub fn open_changed(&self, actions: &Actions) -> Option<bool> {
        self.borrow().and_then(|inner| inner.open_changed(actions))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        command_palette_secondary_action_label, command_results_summary, matches_command_query,
        search_term_for_item, sync_cached_row_state, CommandPaletteRowState,
        ShadCommandPaletteItem,
    };
    use std::collections::HashMap;

    #[test]
    fn command_palette_query_matches_shortcut_tags_and_keywords() {
        let term = search_term_for_item(&ShadCommandPaletteItem {
            title: "Command Palette".to_string(),
            section: "Navigation".to_string(),
            shortcut: "KB".to_string(),
            keywords: vec!["launcher".to_string()],
        });

        assert!(matches_command_query(&term, "command"));
        assert!(matches_command_query(&term, "navigation"));
        assert!(matches_command_query(&term, "kb"));
        assert!(matches_command_query(&term, "launcher"));
        assert!(!matches_command_query(&term, "dialog"));
    }

    #[test]
    fn command_palette_summary_describes_matches() {
        assert!(command_results_summary("", 12, 12, "commands", "Search.").contains("Showing all"));
        assert!(
            command_results_summary("dialog", 1, 12, "commands", "Search.")
                .contains("Showing 1 of")
        );
        assert!(
            command_results_summary("missing", 0, 12, "commands", "Search.")
                .contains("No commands matched")
        );
    }

    #[test]
    fn command_palette_secondary_action_switches_between_close_and_clear() {
        assert_eq!(command_palette_secondary_action_label(""), "Close");
        assert_eq!(command_palette_secondary_action_label("  "), "Close");
        assert_eq!(command_palette_secondary_action_label("dialog"), "Clear");
    }

    #[test]
    fn command_palette_row_cache_skips_unchanged_updates() {
        let mut cache = HashMap::new();
        let state = CommandPaletteRowState {
            command_index: 3,
            show_header: true,
            is_active: false,
        };

        assert!(sync_cached_row_state(&mut cache, 7_u64, state, false));
        assert!(!sync_cached_row_state(&mut cache, 7_u64, state, true));
    }

    #[test]
    fn command_palette_row_cache_refreshes_reloaded_widgets() {
        let mut cache = HashMap::new();
        let state = CommandPaletteRowState {
            command_index: 3,
            show_header: true,
            is_active: false,
        };

        assert!(sync_cached_row_state(&mut cache, 7_u64, state, false));
        assert!(sync_cached_row_state(&mut cache, 7_u64, state, false));
        assert!(!sync_cached_row_state(&mut cache, 7_u64, state, true));
    }

    #[test]
    fn command_palette_row_cache_reduces_steady_state_updates() {
        const VISIBLE_ROWS: usize = 8;
        const FRAMES: usize = 1_000;
        const WIDGET_UPDATES_PER_ROW: usize = 5;

        let old_updates = VISIBLE_ROWS * FRAMES * WIDGET_UPDATES_PER_ROW;
        let mut new_updates = 0;
        let mut cache = HashMap::new();

        for frame in 0..FRAMES {
            for row in 0..VISIBLE_ROWS {
                let state = CommandPaletteRowState {
                    command_index: row,
                    show_header: row == 0,
                    is_active: row == 0,
                };
                if sync_cached_row_state(&mut cache, row, state, frame != 0) {
                    new_updates += WIDGET_UPDATES_PER_ROW;
                }
            }
        }

        assert_eq!(new_updates, VISIBLE_ROWS * WIDGET_UPDATES_PER_ROW);
        assert_eq!(old_updates, 40_000);
        assert_eq!(new_updates, 40);
    }
}
