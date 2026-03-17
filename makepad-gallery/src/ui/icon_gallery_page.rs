use crate::ui::page_macros::gallery_stateful_page_shell;
use makepad_components::makepad_widgets::*;
use makepad_components::table::ShadTableWidgetExt;

#[derive(Clone, Copy)]
struct IconGalleryEntry {
    template_id: &'static str,
    icon_name: &'static str,
    widget_name: &'static str,
}

macro_rules! icon_gallery_page_generated {
    ($($icon_rows:tt)*) => {
        gallery_stateful_page_shell! {
            widget: GalleryIconGalleryPage,
            page: icon_gallery_page,
            title: "Icons",
            subtitle: "Generated Lucide icon components from makepad-icon/resources/icons. Search by Lucide asset name or generated Rust widget name.",
            divider: { ShadHr{} },
            preview_spacing: 12.0,
            preview: {
                search_toolbar := View{
                    width: Fill
                    height: Fit
                    flow: Right
                    spacing: 8.0

                    search_field := ShadInputWithIcon{
                        width: Fill
                        input +: {
                            empty_text: "Search icons or widget names..."
                        }
                    }

                    clear_search_btn := ShadButtonOutline{
                        text: "Clear"
                        visible: false
                    }

                    icon_search_btn := ShadButtonSm{text: "Search"}
                }

                icon_results_summary := ShadFieldDescription{
                    text: "Showing all generated icons."
                }

                usage_card := ShadSurfaceMuted{
                    width: Fill
                    height: Fit
                    flow: Down
                    spacing: 8.0
                    padding: Inset{left: 14, right: 14, top: 14, bottom: 14}
                    draw_bg +: {
                        border_size: 1.0
                        border_color: (shad_theme.color_outline_border)
                    }

                    icon_usage_title := ShadSectionHeader{
                        text: "Using IconSearch"
                    }

                    icon_usage_description := ShadFieldDescription{
                        text: "Use the generated widget name directly in script_mod!. The snippet updates to the first visible search match."
                    }

                    icon_usage_snippet := mod.widgets.GalleryCodeSnippet{
                        code: "IconSearch{\n    icon_walk: Walk{width: 18, height: 18}\n    draw_icon.color: (shad_theme.color_primary)\n}\n"
                    }
                }

                icon_table := ShadTable{
                    headers: []
                    rows: []
                    selectable: false
                    empty_message: "No icons matched the current search."
                    table_view := View{
                        width: Fill
                        height: Fit
                        flow: Down
                        spacing: 8.0

                        caption_label := ShadFieldDescription{
                            visible: false
                            text: ""
                        }

                        scroll := ScrollXView{
                            width: Fill
                            height: Fit
                            flow: Down
                            padding: Inset{left: 0.0, right: 0.0, top: 0.0, bottom: 0.0}
                            spacing: 0.0

                            content := View{
                                width: Fit
                                height: Fit
                                flow: Down
                                spacing: 8.0

                                header := mod.widgets.ShadTableHeaderView{}
                                list := PortalList{
                                    width: Fill
                                    height: 420.0
                                    flow: Down
                                    max_pull_down: 0.0
                                    capture_overload: true
                                    grab_key_focus: false
                                    auto_tail: false
                                    selectable: false
                                    drag_scrolling: true

                                    Empty := mod.widgets.ShadTableEmptyRow{}

                                    $($icon_rows)*
                                }
                            }
                        }
                    }
                }
            },
            action_flow: {
                mod.widgets.GalleryActionFlowStep{text: "1. Run `python3 makepad-icon/scripts/download_lucide_icons.py --clean` to refresh SVG assets from Lucide."}
                mod.widgets.GalleryActionFlowStep{text: "2. Build `makepad-icon` or `makepad-gallery`; build.rs scans icons and regenerates the virtualized list rows plus the search metadata automatically."}
                mod.widgets.GalleryActionFlowStep{text: "3. Search by Lucide asset name like `search` or by Rust widget name like `IconSearch` to narrow the virtualized list quickly."}
                mod.widgets.GalleryActionFlowStep{text: "4. Use the widget name shown on each tile directly in script_mod!, and start from the usage snippet panel when you need sizing or color overrides."}
            },
        }
    };
}

// Generated by makepad-gallery/build.rs. Do not edit manually.
include!(concat!(env!("OUT_DIR"), "/icon_preview_rows.rs"));

#[derive(Script, Widget)]
pub struct GalleryIconGalleryPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[rust]
    query: String,
    #[rust]
    template_live_ids: Vec<LiveId>,
    #[rust]
    widget_name_lower: Vec<String>,
    #[rust]
    filtered_template_ids: Vec<LiveId>,
    #[rust]
    filtered_template_ids_scratch: Vec<LiveId>,
    #[rust]
    summary_cache: String,
    #[rust]
    usage_entry_cache: Option<usize>,
}

impl GalleryIconGalleryPage {
    fn normalize_query(query: &str) -> String {
        query.trim().to_ascii_lowercase()
    }

    fn summary_text(query: &str, matches_count: usize) -> String {
        if query.is_empty() {
            format!("Showing all {ICON_GALLERY_TOTAL} generated icons.")
        } else if matches_count == 0 {
            format!("No icons matched \"{query}\". Press Esc to clear the search.")
        } else {
            format!(
                "Showing {matches_count} of {ICON_GALLERY_TOTAL} icons for \"{query}\"."
            )
        }
    }

    fn usage_code(widget_name: &str) -> String {
        format!(
            "{widget_name}{{\n    icon_walk: Walk{{width: 18, height: 18}}\n    draw_icon.color: (shad_theme.color_primary)\n}}\n"
        )
    }

    fn sync_empty_usage_preview(&self, cx: &mut Cx, query: &str) {
        self.view
            .label(cx, ids!(icon_usage_title))
            .set_text(cx, "No matching icon");
        self.view
            .label(cx, ids!(icon_usage_description))
            .set_text(
                cx,
                &format!(
                    "No generated icon matched \"{query}\". Clear or broaden the search to restore the usage snippet."
                ),
            );
        self.view
            .widget(cx, ids!(icon_usage_snippet.container.code_view))
            .set_text(
                cx,
                "// No icon snippet available while the current search has no matches.\n",
            );
    }

    fn sync_usage_preview(&self, cx: &mut Cx, entry: &IconGalleryEntry) {
        self.view
            .label(cx, ids!(icon_usage_title))
            .set_text(cx, &format!("Using {}", entry.widget_name));
        self.view
            .label(cx, ids!(icon_usage_description))
            .set_text(
                cx,
                &format!(
                    "Lucide asset \"{}\" generates the `{}` widget. Drop it into script_mod! and override icon_walk or draw_icon.color as needed.",
                    entry.icon_name, entry.widget_name
                ),
            );
        self.view
            .widget(cx, ids!(icon_usage_snippet.container.code_view))
            .set_text(cx, &Self::usage_code(entry.widget_name));
    }

    fn ensure_filter_cache(&mut self) {
        if self.template_live_ids.len() == ICON_GALLERY_ENTRIES.len()
            && self.widget_name_lower.len() == ICON_GALLERY_ENTRIES.len()
        {
            return;
        }

        self.template_live_ids = ICON_GALLERY_ENTRIES
            .iter()
            .map(|entry| LiveId::from_str(entry.template_id))
            .collect();
        self.widget_name_lower = ICON_GALLERY_ENTRIES
            .iter()
            .map(|entry| entry.widget_name.to_ascii_lowercase())
            .collect();
        self.filtered_template_ids.clear();
        self.filtered_template_ids_scratch.clear();
    }

    fn apply_filter(&mut self, cx: &mut Cx) {
        self.ensure_filter_cache();
        let display_query = self.query.trim().to_string();
        let query = Self::normalize_query(&self.query);
        let mut matches_count = 0;
        let mut first_match_index = None;
        let mut changed = false;
        self.filtered_template_ids_scratch.clear();

        for (index, entry) in ICON_GALLERY_ENTRIES.iter().enumerate() {
            let matches = query.is_empty()
                || entry.icon_name.contains(&query)
                || self.widget_name_lower[index].contains(&query);
            if matches {
                self.filtered_template_ids_scratch
                    .push(self.template_live_ids[index]);
                matches_count += 1;
                if first_match_index.is_none() {
                    first_match_index = Some(index);
                }
            }
        }

        if self.filtered_template_ids != self.filtered_template_ids_scratch {
            std::mem::swap(
                &mut self.filtered_template_ids,
                &mut self.filtered_template_ids_scratch,
            );
            self.view
                .shad_table(cx, ids!(icon_table))
                .set_custom_row_templates(cx, self.filtered_template_ids.clone());
            changed = true;
        }

        let summary = Self::summary_text(&display_query, matches_count);
        if self.summary_cache != summary {
            self.summary_cache = summary;
            self.view
                .label(cx, ids!(icon_results_summary))
                .set_text(cx, &self.summary_cache);
            changed = true;
        }

        self.view
            .button(cx, ids!(clear_search_btn))
            .set_visible(cx, !query.is_empty());

        match first_match_index {
            Some(target_entry_index) => {
                if self.usage_entry_cache != Some(target_entry_index) {
                    self.usage_entry_cache = Some(target_entry_index);
                    self.sync_usage_preview(cx, &ICON_GALLERY_ENTRIES[target_entry_index]);
                    changed = true;
                }
            }
            None => {
                if self.usage_entry_cache.is_some() || !query.is_empty() {
                    self.usage_entry_cache = None;
                    self.sync_empty_usage_preview(cx, &display_query);
                    changed = true;
                }
            }
        }

        if changed {
            self.view.redraw(cx);
        }
    }
}

impl ScriptHook for GalleryIconGalleryPage {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        vm.with_cx_mut(|cx| {
            self.apply_filter(cx);
        });
    }
}

impl Widget for GalleryIconGalleryPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            let search_input = self.view.text_input(cx, ids!(search_field.input));
            let mut next_query = None;

            if let Some(text) = search_input.changed(actions) {
                next_query = Some(text);
            }
            if let Some((text, _modifiers)) = search_input.returned(actions) {
                next_query = Some(text);
            }
            if search_input.escaped(actions) && !self.query.is_empty() {
                search_input.set_text(cx, "");
                search_input.set_key_focus(cx);
                next_query = Some(String::new());
            }
            if self.view.button(cx, ids!(icon_search_btn)).clicked(actions) {
                search_input.set_key_focus(cx);
                next_query = Some(search_input.text());
            }
            if self.view.button(cx, ids!(clear_search_btn)).clicked(actions) {
                search_input.set_text(cx, "");
                search_input.set_key_focus(cx);
                next_query = Some(String::new());
            }

            if let Some(query) = next_query {
                self.query = query;
                self.apply_filter(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
