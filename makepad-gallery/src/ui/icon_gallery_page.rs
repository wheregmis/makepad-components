use makepad_components::makepad_widgets::*;

#[derive(Clone, Copy)]
struct IconGalleryEntry {
    icon_name: &'static str,
    widget_name: &'static str,
    svg_markup: &'static str,
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryIconGalleryRowBase = #(GalleryIconGalleryRow::register_widget(vm))
    mod.widgets.GalleryIconGalleryRow = set_type_default() do mod.widgets.GalleryIconGalleryRowBase{
        width: Fill
        height: 72
        flow: Right
        spacing: 12.0
        align: Align{x: 0.0 y: 0.5}
        padding: Inset{left: 12, right: 12, top: 0, bottom: 0}
        draw_bg +: {
            color: (shad_theme.color_secondary)
            border_size: 1.0
            border_radius: (shad_theme.radius)
            border_color: (shad_theme.color_outline_border)
        }

        preview := RoundedView{
            width: 40
            height: 40
            align: Align{x: 0.5 y: 0.5}
            draw_bg +: {
                color: (shad_theme.color_muted)
                border_radius: 10.0
            }

            preview_icon := Svg{
                width: 18
                height: 18
                animating: false
                draw_svg +: {
                    color: (shad_theme.color_primary)
                }
            }
        }

        copy := View{
            width: Fill
            height: Fit
            flow: Down
            spacing: 4.0

            icon_name_label := ShadLabel{
                text: ""
                draw_text.text_style.font_size: 12.0
            }

            widget_name_label := ShadFieldDescription{
                width: Fill
                text: ""
                draw_text.text_style.font_size: 11.0
            }
        }
    }

    mod.widgets.GalleryIconGalleryPage = set_type_default() do #(GalleryIconGalleryPage::register_widget(vm)){
        width: Fill
        height: Fill

        page_root := GalleryPageRoot{
            width: Fill
            height: Fill

            ShadPageTitle{
                text: "Icons"
            }

            ShadPageSubtitle{
                text: "Generated Lucide icon components from makepad-icon/resources/icons. Search by Lucide asset name or generated Rust widget name."
            }

            ShadHr{}

            preview_section := mod.widgets.GalleryPreviewSection{
                width: Fill
                height: Fit

                preview_panel +: {
                    preview_content_wrapper +: {
                        preview_flip +: {
                            root_view +: {
                                preview_content +: {
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 12.0

                                    search_controls := ShadField{
                                        width: Fill

                                        search_label := ShadFieldLabel{
                                            text: "Search icons or widget names"
                                        }

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
                                            }

                                            icon_search_btn := ShadButtonSm{text: "Focus search"}
                                        }
                                    }

                                    search_hint := ShadFieldDescription{
                                        text: "Results update as you type. Press Esc or Clear to reset the search."
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

                                    icon_results_panel := ShadSurfaceMuted{
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: 8.0
                                        padding: Inset{left: 12, right: 12, top: 12, bottom: 12}
                                        draw_bg +: {
                                            border_size: 1.0
                                            border_color: (shad_theme.color_outline_border)
                                        }

                                        icon_results_list := PortalList{
                                            width: Fill
                                            height: 420.0
                                            flow: Down
                                            max_pull_down: 0.0
                                            capture_overload: true
                                            grab_key_focus: false
                                            auto_tail: false
                                            selectable: false
                                            drag_scrolling: true

                                            Item := mod.widgets.GalleryIconGalleryRow{}
                                            Empty := mod.widgets.ShadTableEmptyRow{}
                                        }
                                    }
                                }

                                action_flow +: {
                                    visible: true
                                    mod.widgets.GalleryActionFlow{
                                        body +: {
                                            mod.widgets.GalleryActionFlowStep{text: "1. Run `python3 makepad-icon/scripts/download_lucide_icons.py --clean` to refresh SVG assets from Lucide."}
                                            mod.widgets.GalleryActionFlowStep{text: "2. Build `makepad-icon` or `makepad-gallery`; build.rs scans icons and regenerates the icon metadata automatically."}
                                            mod.widgets.GalleryActionFlowStep{text: "3. Search by Lucide asset name like `search` or by Rust widget name like `IconSearch` to narrow the virtualized list quickly."}
                                            mod.widgets.GalleryActionFlowStep{text: "4. Use the widget name shown on each row directly in script_mod!, and start from the usage snippet panel when you need sizing or color overrides."}
                                        }
                                    }
                                }
                            }

                            code_page +: {
                                body +: {
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 12.0

                                    code_snippet +: {
                                        code: #(crate::ui::snippets::snippet_code_for_page(live_id!(icon_gallery_page)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// Generated by makepad-gallery/build.rs. Do not edit manually.
include!(concat!(env!("OUT_DIR"), "/icon_gallery_data.rs"));

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryIconGalleryRow {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[rust]
    entry_index: Option<usize>,
}

impl GalleryIconGalleryRow {
    fn set_entry(&mut self, cx: &mut Cx, entry_index: usize) {
        if self.entry_index == Some(entry_index) {
            return;
        }

        self.entry_index = Some(entry_index);
        let entry = &ICON_GALLERY_ENTRIES[entry_index];
        self.view
            .label(cx, ids!(copy.icon_name_label))
            .set_text(cx, entry.icon_name);
        self.view
            .label(cx, ids!(copy.widget_name_label))
            .set_text(cx, entry.widget_name);

        if let Some(mut icon) = self
            .view
            .widget(cx, ids!(preview.preview_icon))
            .borrow_mut::<Svg>()
        {
            icon.draw_svg.load_from_str(entry.svg_markup);
        }
    }
}

impl Widget for GalleryIconGalleryRow {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Script, Widget)]
pub struct GalleryIconGalleryPage {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[rust]
    query: String,
    #[rust]
    widget_name_lower: Vec<String>,
    #[rust]
    filtered_indices: Vec<usize>,
    #[rust]
    filtered_indices_scratch: Vec<usize>,
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
            format!("Showing {matches_count} of {ICON_GALLERY_TOTAL} icons for \"{query}\".")
        }
    }

    fn usage_code(widget_name: &str) -> String {
        format!(
            "{widget_name}{{\n    icon_walk: Walk{{width: 18, height: 18}}\n    draw_icon.color: (shad_theme.color_primary)\n}}\n"
        )
    }

    fn ensure_filter_cache(&mut self) {
        if self.widget_name_lower.len() == ICON_GALLERY_ENTRIES.len() {
            return;
        }

        self.widget_name_lower = ICON_GALLERY_ENTRIES
            .iter()
            .map(|entry| entry.widget_name.to_ascii_lowercase())
            .collect();
        self.filtered_indices.clear();
        self.filtered_indices_scratch.clear();
        self.filtered_indices_scratch
            .reserve(ICON_GALLERY_ENTRIES.len());
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

    fn reset_results_position(&self, cx: &mut Cx) {
        self.view
            .portal_list(cx, ids!(icon_results_list))
            .set_first_id(0);
    }

    fn apply_filter(&mut self, cx: &mut Cx) {
        self.ensure_filter_cache();

        let query = Self::normalize_query(&self.query);
        let mut matches_count = 0;
        let mut first_match_index = None;
        let mut changed = false;
        self.filtered_indices_scratch.clear();

        for (index, entry) in ICON_GALLERY_ENTRIES.iter().enumerate() {
            let matches = query.is_empty()
                || entry.icon_name.contains(&query)
                || self.widget_name_lower[index].contains(&query);
            if matches {
                self.filtered_indices_scratch.push(index);
                matches_count += 1;
                if first_match_index.is_none() {
                    first_match_index = Some(index);
                }
            }
        }

        if self.filtered_indices != self.filtered_indices_scratch {
            self.filtered_indices.clear();
            self.filtered_indices
                .extend_from_slice(self.filtered_indices_scratch.as_slice());
            self.reset_results_position(cx);
            changed = true;
        }

        let summary = Self::summary_text(self.query.trim(), matches_count);
        if self.summary_cache != summary {
            self.summary_cache = summary;
            self.view
                .label(cx, ids!(icon_results_summary))
                .set_text(cx, &self.summary_cache);
            changed = true;
        }

        self.view
            .button(cx, ids!(clear_search_btn))
            .set_enabled(cx, !query.is_empty());

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
                    self.sync_empty_usage_preview(cx, self.query.trim());
                    changed = true;
                }
            }
        }

        if changed {
            self.view.redraw(cx);
        }
    }

    fn draw_results(&mut self, cx: &mut Cx2d, list: &mut PortalList) {
        let mut empty_scope = Scope::empty();

        if self.filtered_indices.is_empty() {
            list.set_item_range(cx, 0, 1);
            if let Some(item_id) = list.next_visible_item(cx) {
                let item = list.item(cx, item_id, id!(Empty)).as_view();
                item.label(cx, ids!(empty_label))
                    .set_text(cx, "No icons matched the current search.");
                item.draw_all(cx, &mut empty_scope);
            }
            return;
        }

        list.set_item_range(cx, 0, self.filtered_indices.len());
        while let Some(item_id) = list.next_visible_item(cx) {
            let Some(entry_index) = self.filtered_indices.get(item_id).copied() else {
                continue;
            };

            let item = list.item(cx, item_id, id!(Item));
            if let Some(mut row) = item.borrow_mut::<GalleryIconGalleryRow>() {
                row.set_entry(cx, entry_index);
            }
            item.draw_all(cx, &mut empty_scope);
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
            }
            if self
                .view
                .button(cx, ids!(clear_search_btn))
                .clicked(actions)
            {
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
        while let Some(step) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = step.as_portal_list().borrow_mut() {
                self.draw_results(cx, &mut *list);
            }
        }
        DrawStep::done()
    }
}
