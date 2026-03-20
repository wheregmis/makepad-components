use crate::ui::catalog;
use makepad_components::makepad_widgets::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::OnceLock;

const RESULTS_SCROLL_SPEED: f64 = 18.0;
const RESULTS_MAX_ITEMS_TO_SHOW: usize = 8;

struct CommandSearchTerm {
    title: String,
    section: String,
    shortcut: String,
}

fn command_search_terms() -> &'static [CommandSearchTerm] {
    static TERMS: OnceLock<Vec<CommandSearchTerm>> = OnceLock::new();
    TERMS
        .get_or_init(|| {
            catalog::entries()
                .iter()
                .map(|entry| CommandSearchTerm {
                    title: entry.title.to_ascii_lowercase(),
                    section: entry.section.to_ascii_lowercase(),
                    shortcut: entry.shortcut.to_ascii_lowercase(),
                })
                .collect()
        })
        .as_slice()
}

fn matches_command_query(term: &CommandSearchTerm, query: &str) -> bool {
    query.is_empty()
        || term.title.contains(query)
        || term.section.contains(query)
        || term.shortcut.contains(query)
}

fn command_results_summary(query: &str, matches_count: usize) -> String {
    let query = query.trim();
    let total = catalog::entries().len();
    if query.is_empty() {
        format!(
            "Showing all {total} gallery components. Search by title, section, or shortcut tag."
        )
    } else if matches_count == 0 {
        format!("No gallery components matched \"{query}\".")
    } else {
        format!("Showing {matches_count} of {total} gallery components for \"{query}\".")
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct CommandPaletteRowState {
    command_index: usize,
    show_header: bool,
    is_active: bool,
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

    mod.widgets.GalleryCommandPaletteBase = #(GalleryCommandPalette::register_widget(vm))

    mod.widgets.GalleryCommandPaletteRow = View{
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
                color: #0000
                border_radius: 10.0
                border_size: 0.0
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

            shortcut := ShadSectionHeader{
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
                        border_radius: 18.0
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    search_label := ShadFieldLabel{
                        text: "Search gallery components"
                    }

                    search_shell := ShadSurface{
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
                            empty_text: "Search components, sections, or shortcut tags..."
                            draw_text.text_style.font_size: 14
                            draw_text.color_empty: (shad_theme.color_muted_foreground)
                        }

                        clear_search_btn := ShadButtonGhost{
                            text: "Clear"
                        }
                    }

                    results_summary := ShadFieldDescription{
                        text: "Showing all gallery components. Search by title, section, or shortcut tag."
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

                            empty_title := ShadLabel{
                                draw_text.color: (shad_theme.color_primary)
                                draw_text.text_style.font_size: 13
                                text: "No commands found"
                            }

                            empty_copy := ShadFieldDescription{
                                draw_text.color: (shad_theme.color_muted_foreground)
                                draw_text.text_style.font_size: 11
                                text: "Try a component like button, a section like forms, or the shortcut tag shown in each row."
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
                        ShadSectionHeader{
                            draw_text.color: (shad_theme.color_muted_foreground)
                            draw_text.text_style.font_size: 10
                            text: "Open"
                        }

                        ShadKbd{ label := ShadKbdLabel{text: "Esc"} }
                        ShadSectionHeader{
                            draw_text.color: (shad_theme.color_muted_foreground)
                            draw_text.text_style.font_size: 10
                            text: "Clear / Close"
                        }

                        ShadKbd{ label := ShadKbdLabel{text: "Up/Down"} }
                        ShadSectionHeader{
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
    row_state_by_uid: HashMap<WidgetUid, CommandPaletteRowState>,
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
        if self.has_results_cache == Some(has_results) {
            return;
        }
        self.has_results_cache = Some(has_results);
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
        self.has_results_cache = None;
        self.row_state_by_uid.clear();
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
        self.has_results_cache = None;
        self.row_state_by_uid.clear();
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
        let display_query = self.query.trim().to_string();
        let search_terms = command_search_terms();
        let previous_active = self.active_index;
        self.filtered_indices_scratch.clear();

        for (index, _command) in catalog::entries().iter().enumerate() {
            if matches_command_query(&search_terms[index], &query) {
                self.filtered_indices_scratch.push(index);
            }
        }
        let results_changed = self.filtered_indices != self.filtered_indices_scratch;
        if results_changed {
            std::mem::swap(
                &mut self.filtered_indices,
                &mut self.filtered_indices_scratch,
            );
            // Optimization: PortalList recycles row widgets across frames and scroll positions.
            // Cache each row widget's bound command/active/header state so unchanged rows skip
            // repeated text/visibility/script updates on every draw.
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
        self.overlay.label(cx, ids!(results_summary)).set_text(
            cx,
            &command_results_summary(&display_query, self.filtered_indices.len()),
        );
        self.overlay
            .button(cx, ids!(clear_search_btn))
            .set_enabled(cx, !query.is_empty());
        self.reset_results_position(cx);
        if results_changed || active_changed {
            self.redraw(cx);
        }
    }

    fn draw_results(&mut self, cx: &mut Cx2d, list: &mut PortalList) {
        list.set_item_range(cx, 0, self.filtered_indices.len());
        let entries = catalog::entries();

        while let Some(item_id) = list.next_visible_item(cx) {
            let Some(command_index) = self.filtered_indices.get(item_id).copied() else {
                continue;
            };

            let command = entries[command_index];
            let (item, item_existed) = list.item_with_existed(cx, item_id, id!(Item));
            let item = item.as_view();
            let show_header = item_id == 0
                || self
                    .filtered_indices
                    .get(item_id - 1)
                    .is_some_and(|previous| entries[*previous].section != command.section);

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
                item.label(cx, ids!(header)).set_text(cx, command.section);
                item.button(cx, ids!(button)).set_text(cx, command.title);
                item.label(cx, ids!(shortcut))
                    .set_text(cx, command.shortcut);

                let background = if next_state.is_active {
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

    fn clear_query(&mut self, cx: &mut Cx) {
        self.query.clear();
        self.active_index = 0;
        self.overlay
            .text_input(cx, ids!(search_input))
            .set_text(cx, "");
        self.refresh_results(cx);
        self.focus_search_on_next_draw = true;
    }
}

#[cfg(test)]
mod tests {
    use super::{
        command_results_summary, matches_command_query, sync_cached_row_state,
        CommandPaletteRowState, CommandSearchTerm,
    };
    use std::collections::HashMap;

    #[test]
    fn command_palette_query_matches_shortcut_tags() {
        let term = CommandSearchTerm {
            title: "command palette".to_string(),
            section: "navigation".to_string(),
            shortcut: "kb".to_string(),
        };

        assert!(matches_command_query(&term, "command"));
        assert!(matches_command_query(&term, "navigation"));
        assert!(matches_command_query(&term, "kb"));
        assert!(!matches_command_query(&term, "dialog"));
    }

    #[test]
    fn command_palette_summary_describes_matches() {
        assert!(command_results_summary("", 12).contains("Showing all"));
        assert!(command_results_summary("dialog", 1).contains("Showing 1 of"));
        assert!(command_results_summary("missing", 0).contains("No gallery components matched"));
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

        for _frame in 0..FRAMES {
            for row in 0..VISIBLE_ROWS {
                let state = CommandPaletteRowState {
                    command_index: row,
                    show_header: row == 0,
                    is_active: row == 0,
                };
                if sync_cached_row_state(&mut cache, row, state, _frame != 0) {
                    new_updates += WIDGET_UPDATES_PER_ROW;
                }
            }
        }

        assert_eq!(new_updates, VISIBLE_ROWS * WIDGET_UPDATES_PER_ROW);
        assert_eq!(old_updates, 40_000);
        assert_eq!(new_updates, 40);
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

            let search_input = self.overlay.text_input(cx, ids!(search_input));
            let results = self.overlay.portal_list(cx, ids!(results));

            self.overlay.handle_event(cx, event, scope);

            if let Event::Actions(actions) = event {
                if let Some(text) = search_input.changed(actions) {
                    self.query = text;
                    self.active_index = 0;
                    self.refresh_results(cx);
                }

                if self
                    .overlay
                    .button(cx, ids!(clear_search_btn))
                    .clicked(actions)
                {
                    self.clear_query(cx);
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
