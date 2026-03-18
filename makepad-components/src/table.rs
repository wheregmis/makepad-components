use crate::internal::actions::widget_action_map;
use crate::models::table::{
    clamp_selected_row, empty_fill_rows as table_empty_fill_rows, resolved_column_count,
    virtual_window_index,
};
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;
use std::sync::Arc;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadTableHeaderViewBase = #(ShadTableHeaderView::register_widget(vm))
    mod.widgets.ShadTableRowViewBase = #(ShadTableRowView::register_widget(vm))
    mod.widgets.ShadTableBase = #(ShadTable::register_widget(vm))

    mod.widgets.ShadTableHeaderView = set_type_default() do mod.widgets.ShadTableHeaderViewBase{
        width: Fit
        height: 36
        fill_color: (shad_theme.color_muted)
        border_color: (shad_theme.color_outline_border)
        text_color: (shad_theme.color_muted_foreground)

        draw_bg +: { color: (shad_theme.color_muted) }

        draw_text +: {
            color: (shad_theme.color_muted_foreground)
            text_style: theme.font_regular{font_size: 10.0}
        }
    }

    mod.widgets.ShadTableRowView = set_type_default() do mod.widgets.ShadTableRowViewBase{
        width: Fit
        height: 40
        text_color: (shad_theme.color_primary)
        border_color: (shad_theme.color_outline_border)
        fill_hover: (shad_theme.color_secondary_hover)
        fill_selected: (shad_theme.color_secondary)
        fill_striped: (shad_theme.color_muted)

        draw_bg +: { color: #0000 }

        draw_text +: {
            color: (shad_theme.color_primary)
            text_style: theme.font_regular{font_size: 11.0}
        }
    }

    mod.widgets.ShadTableEmptyRow = View{
        width: Fill
        height: 40
        flow: Right
        align: Align{x: 0.0, y: 0.5}
        padding: Inset{left: 12.0, right: 12.0, top: 0.0, bottom: 0.0}
        draw_bg.color: #0000

        empty_label := ShadFieldDescription{
            text: ""
        }
    }

    mod.widgets.ShadTable = set_type_default() do mod.widgets.ShadTableBase{
        width: Fill
        height: Fit
        caption: ""
        empty_message: "No rows available."
        selectable: true
        auto_fill_width: true
        text_align: 0.5
        virtual_total_rows: 0
        headers: ["Name" "Email" "Role"]
        rows: []

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
                    spacing: 6.0

                    header := mod.widgets.ShadTableHeaderView{}
                    list := PortalList{
                        width: Fill
                        height: 208
                        flow: Down
                        max_pull_down: 0.0
                        capture_overload: true
                        grab_key_focus: false
                        auto_tail: false
                        selectable: false
                        drag_scrolling: true

                        Item := mod.widgets.ShadTableRowView{}
                        Empty := mod.widgets.ShadTableEmptyRow{}
                    }
                }
            }
        }
    }
}

const DEFAULT_COLUMN_WIDTH: f64 = 160.0;
const MIN_COLUMN_WIDTH: f64 = 80.0;
const TABLE_ROW_HEIGHT: f64 = 40.0;
const CHAR_WIDTH_FACTOR: f64 = 0.6;
const HEADER_FONT_SIZE: f64 = 10.0;
const CELL_FONT_SIZE: f64 = 11.0;
fn estimate_text_width(text: &str, font_size: f64) -> f64 {
    text.len() as f64 * font_size * CHAR_WIDTH_FACTOR
}

fn replace_arc_slice_if_changed<T>(dst: &mut Arc<[T]>, src: &Arc<[T]>) -> bool {
    if Arc::ptr_eq(dst, src) {
        return false;
    }
    *dst = Arc::clone(src);
    true
}

fn sync_vec_if_changed<T: PartialEq + Clone>(dst: &mut Vec<T>, src: &[T]) -> bool {
    if dst.as_slice() == src {
        return false;
    }

    // Optimization: header data is refreshed from table draw/layout sync paths.
    // Previously: `src.to_vec()` rebuilt the Vec storage on every content change.
    // Now: clear + extend reuses the existing allocation when capacity is sufficient.
    dst.clear();
    dst.extend_from_slice(src);
    true
}

/// Ensures the table width buffer tracks the current column count while reusing allocations.
///
/// The common case for virtualized updates is a stable column count, so this returns early and
/// avoids rebuilding the Vec every sync.
fn sync_default_widths(widths: &mut Vec<f64>, column_count: usize, default_width: f64) {
    let previous_len = widths.len();
    if previous_len == column_count {
        if widths.iter().any(|width| *width != default_width) {
            widths.fill(default_width);
        }
        return;
    }
    let shrinking = previous_len > column_count;
    widths.resize(column_count, default_width);
    if shrinking && widths.iter().any(|width| *width != default_width) {
        widths.fill(default_width);
    }
}

fn update_cached_width(cached_width: &mut f64, width: f64) -> bool {
    if *cached_width == width {
        return false;
    }
    *cached_width = width;
    true
}

fn invalidate_cached_width(cached_width: &mut f64) {
    *cached_width = f64::NAN;
}

fn calculate_content_based_widths(headers: &[String], rows_data: &[Arc<[String]>]) -> Vec<f64> {
    let column_count = headers.len();
    if column_count == 0 {
        return vec![];
    }

    let mut max_char_counts: Vec<usize> = vec![0; column_count];

    for (col_idx, header) in headers.iter().enumerate() {
        max_char_counts[col_idx] = max_char_counts[col_idx].max(header.len());
    }

    for row in rows_data.iter() {
        for (col_idx, cell) in row.iter().enumerate() {
            if col_idx < column_count {
                max_char_counts[col_idx] = max_char_counts[col_idx].max(cell.len());
            }
        }
    }

    max_char_counts
        .iter()
        .map(|&chars| {
            let header_width = chars as f64 * HEADER_FONT_SIZE * CHAR_WIDTH_FACTOR + 24.0;
            let cell_width = chars as f64 * CELL_FONT_SIZE * CHAR_WIDTH_FACTOR + 24.0;
            header_width.max(cell_width).max(MIN_COLUMN_WIDTH)
        })
        .collect()
}

#[derive(Clone, Debug, Default)]
pub enum ShadTableRowAction {
    Clicked(usize),
    #[default]
    None,
}

#[derive(Clone, Debug, Default)]
pub enum ShadTableAction {
    RowClicked(usize),
    SelectionChanged(usize),
    VirtualWindowRequest(usize),
    #[default]
    None,
}

#[derive(Script, ScriptHook, Widget)]
pub struct ShadTableHeaderView {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[redraw]
    #[live]
    draw_bg: DrawColor,
    #[redraw]
    #[live]
    draw_text: DrawText,
    #[live]
    fill_color: Vec4,
    #[live]
    border_color: Vec4,
    #[live]
    text_color: Vec4,
    #[live(0.5)]
    text_align: f64,
    #[rust]
    area: Area,
    #[rust]
    headers: Vec<String>,
    #[rust]
    widths: Vec<f64>,
}

impl ShadTableHeaderView {
    pub fn set_header_data(
        &mut self,
        cx: &mut Cx,
        headers: &[String],
        widths: &[f64],
        total_width: f64,
        text_align: f64,
    ) {
        let mut changed = false;
        if sync_vec_if_changed(&mut self.headers, headers) {
            changed = true;
        }
        if sync_vec_if_changed(&mut self.widths, widths) {
            changed = true;
        }
        if self.text_align != text_align {
            self.text_align = text_align;
            changed = true;
        }
        if !matches!(self.walk.width, Size::Fixed(width) if width == total_width) {
            self.walk.width = Size::Fixed(total_width);
            changed = true;
        }
        if changed {
            self.area.redraw(cx);
        }
    }
}

impl Widget for ShadTableHeaderView {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        let rect = cx.turtle().rect();
        self.draw_bg.color = self.fill_color;
        self.draw_bg.draw_abs(cx, rect);
        draw_border(cx, &mut self.draw_bg, rect, self.border_color);

        let mut x = rect.pos.x;
        for (index, header) in self.headers.iter().enumerate() {
            let width = self
                .widths
                .get(index)
                .copied()
                .unwrap_or(DEFAULT_COLUMN_WIDTH);
            let text_width = estimate_text_width(header, HEADER_FONT_SIZE);
            let align_offset = (width - text_width) * self.text_align;
            self.draw_text.color = self.text_color;
            self.draw_text
                .draw_abs(cx, dvec2(x + align_offset, rect.pos.y + 12.0), header);
            x += width;
        }

        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct ShadTableRowView {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[redraw]
    #[live]
    draw_bg: DrawColor,
    #[redraw]
    #[live]
    draw_text: DrawText,
    #[live]
    text_color: Vec4,
    #[live]
    border_color: Vec4,
    #[live]
    fill_hover: Vec4,
    #[live]
    fill_selected: Vec4,
    #[live]
    fill_striped: Vec4,
    #[live(0.5)]
    text_align: f64,
    #[rust]
    area: Area,
    #[rust]
    row_index: usize,
    #[rust]
    cells: Arc<[String]>,
    #[rust]
    widths: Arc<[f64]>,
    #[rust]
    selected: bool,
    #[rust]
    striped: bool,
    #[rust]
    hovered: bool,
    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}

impl ShadTableRowView {
    #[allow(clippy::too_many_arguments)]
    pub fn set_row_data(
        &mut self,
        cx: &mut Cx,
        row_index: usize,
        cells: &Arc<[String]>,
        widths: &Arc<[f64]>,
        total_width: f64,
        text_align: f64,
        selected: bool,
        striped: bool,
    ) {
        let mut changed = false;
        if self.row_index != row_index {
            self.row_index = row_index;
            changed = true;
        }
        if replace_arc_slice_if_changed(&mut self.cells, cells) {
            changed = true;
        }
        if replace_arc_slice_if_changed(&mut self.widths, widths) {
            changed = true;
        }
        if self.text_align != text_align {
            self.text_align = text_align;
            changed = true;
        }
        if self.selected != selected {
            self.selected = selected;
            changed = true;
        }
        if self.striped != striped {
            self.striped = striped;
            changed = true;
        }
        if !matches!(self.walk.width, Size::Fixed(width) if width == total_width) {
            self.walk.width = Size::Fixed(total_width);
            changed = true;
        }
        if changed {
            self.area.redraw(cx);
        }
    }

    pub fn clicked(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            matches!(
                item.cast::<ShadTableRowAction>(),
                ShadTableRowAction::Clicked(_)
            )
        } else {
            false
        }
    }
}

impl Widget for ShadTableRowView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event.hits(cx, self.area) {
            Hit::FingerHoverIn(_) => {
                self.hovered = true;
                cx.set_cursor(MouseCursor::Hand);
                self.area.redraw(cx);
            }
            Hit::FingerHoverOut(_) => {
                self.hovered = false;
                self.area.redraw(cx);
            }
            Hit::FingerUp(fe) if fe.is_primary_hit() => {
                cx.widget_action_with_data(
                    &self.action_data,
                    self.widget_uid(),
                    ShadTableRowAction::Clicked(self.row_index),
                );
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        let rect = cx.turtle().rect();

        self.draw_bg.color = if self.selected {
            self.fill_selected
        } else if self.hovered {
            self.fill_hover
        } else if self.striped {
            self.fill_striped
        } else {
            vec4(0.0, 0.0, 0.0, 0.0)
        };
        self.draw_bg.draw_abs(cx, rect);
        draw_border(cx, &mut self.draw_bg, rect, self.border_color);

        let mut x = rect.pos.x;
        for (index, cell) in self.cells.iter().enumerate() {
            let width = self
                .widths
                .get(index)
                .copied()
                .unwrap_or(DEFAULT_COLUMN_WIDTH);
            let text_width = estimate_text_width(cell, CELL_FONT_SIZE);
            let align_offset = (width - text_width) * self.text_align;
            self.draw_text.color = self.text_color;
            self.draw_text
                .draw_abs(cx, dvec2(x + align_offset, rect.pos.y + 14.0), cell);
            x += width;
        }

        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }
}

#[derive(Script, Widget)]
pub struct ShadTable {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,

    #[live]
    headers: Vec<String>,
    #[live]
    rows: ScriptValue,
    #[live]
    caption: ArcStringMut,
    #[live]
    empty_message: ArcStringMut,
    #[live(true)]
    selectable: bool,
    #[live(true)]
    auto_fill_width: bool,
    #[live(0.5)]
    text_align: f64,
    #[live]
    virtual_total_rows: usize,

    #[rust]
    rows_data: Vec<Arc<[String]>>,
    #[rust]
    rows_source: ScriptValue,
    #[rust]
    custom_row_template_ids: Arc<[LiveId]>,
    #[rust]
    virtual_window_start: usize,
    #[rust]
    resolved_widths: Vec<f64>,
    #[rust]
    resolved_widths_shared: Arc<[f64]>,
    #[rust]
    stretched_widths: Vec<f64>,
    #[rust]
    stretched_widths_shared: Arc<[f64]>,
    #[rust]
    stretched_source_widths: Arc<[f64]>,
    #[rust]
    stretched_avail_width: f64,
    #[rust]
    applied_content_width: Option<f64>,
    #[rust]
    total_width: f64,
    #[rust]
    selected_row: Option<usize>,

    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}

impl ScriptHook for ShadTable {
    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        let parsed_rows = if self.rows_source != self.rows {
            Some(parse_rows(vm, self.rows))
        } else {
            None
        };
        vm.with_cx_mut(|cx| {
            if let Some(rows) = parsed_rows {
                self.rows_data = rows;
                self.rows_source = self.rows;
            }
            invalidate_content_width_cache(&mut self.applied_content_width);
            if self.virtual_total_rows == 0 {
                self.virtual_window_start = 0;
            } else if self.virtual_window_start >= self.virtual_total_rows {
                self.virtual_window_start = self.virtual_total_rows.saturating_sub(1);
            }
            self.sync_layout(cx);
        });
    }
}

impl ShadTable {
    const VIRTUAL_WINDOW_PRELOAD_MARGIN: usize = 8;

    fn apply_content_width_if_changed(&mut self, cx: &mut Cx, width: f64) {
        if !should_apply_content_width(&mut self.applied_content_width, width) {
            return;
        }

        let mut content = self.view.view(cx, ids!(table_view.scroll.content));
        // Optimization: avoid re-running `script_apply_eval!` on every draw when the table width
        // is unchanged. The auto-fill path can redraw continuously while scrolling, so caching the
        // last applied width removes repeated script evaluation from the steady-state render loop.
        script_apply_eval!(cx, content, {
            width: #(width)
        });
    }

    fn draw_empty_row(&self, cx: &mut Cx2d, list: &mut PortalList, item_id: usize, label: &str) {
        let item = list.item(cx, item_id, id!(Empty)).as_view();
        item.label(cx, ids!(empty_label)).set_text(cx, label);
        item.draw_all(cx, &mut Scope::empty());
    }

    fn has_custom_rows(&self) -> bool {
        !self.custom_row_template_ids.is_empty()
    }

    fn data_row_count(&self) -> usize {
        if self.has_custom_rows() {
            self.custom_row_template_ids.len()
        } else if self.virtual_total_rows > 0 {
            self.virtual_total_rows
        } else {
            self.rows_data.len()
        }
    }

    fn row_for_index(&self, row_index: usize) -> Option<&Arc<[String]>> {
        if self.virtual_total_rows > 0 {
            let local_index =
                virtual_window_index(row_index, self.virtual_window_start, self.rows_data.len())?;
            self.rows_data.get(local_index)
        } else {
            self.rows_data.get(row_index)
        }
    }

    fn cached_stretched_widths(&mut self, avail: f64) -> Option<(Arc<[f64]>, f64)> {
        if avail <= self.total_width || avail <= 0.0 || self.resolved_widths.is_empty() {
            return None;
        }

        if self.stretched_avail_width != avail
            || !Arc::ptr_eq(&self.stretched_source_widths, &self.resolved_widths_shared)
        {
            self.stretched_widths.clear();
            self.stretched_widths
                .extend_from_slice(self.resolved_widths.as_slice());

            let extra_width = avail - self.total_width;
            let extra_per_column = extra_width / self.stretched_widths.len() as f64;
            for width in &mut self.stretched_widths {
                *width += extra_per_column;
            }

            self.stretched_widths_shared = Arc::from(self.stretched_widths.as_slice());
            self.stretched_source_widths = Arc::clone(&self.resolved_widths_shared);
            self.stretched_avail_width = avail;
        }

        Some((Arc::clone(&self.stretched_widths_shared), avail))
    }

    fn sync_content_width(&mut self, cx: &mut Cx, width: f64) {
        if !update_cached_width(&mut self.applied_content_width, width) {
            return;
        }

        // Optimization: only re-run `script_apply_eval!` when the stretched content width
        // actually changes. Previously the auto-fill draw path re-applied the same width every
        // frame, which forced unnecessary script evaluation in a hot render loop.
        let mut content = self.view.view(cx, ids!(table_view.scroll.content));
        script_apply_eval!(cx, content, {
            width: #(width)
        });
    }

    fn sync_layout(&mut self, cx: &mut Cx) {
        let custom_row_mode = self.has_custom_rows();
        let column_count = if custom_row_mode {
            1
        } else {
            resolved_column_count(&self.headers, &self.rows_data).max(1)
        };

        if custom_row_mode {
            sync_default_widths(
                &mut self.resolved_widths,
                column_count,
                DEFAULT_COLUMN_WIDTH,
            );
        } else if self.auto_fill_width && !self.rows_data.is_empty() && self.virtual_total_rows == 0
        {
            let content_widths = calculate_content_based_widths(&self.headers, &self.rows_data);
            if self.resolved_widths.len() != content_widths.len() {
                self.resolved_widths = content_widths;
            } else {
                for (i, width) in content_widths.iter().enumerate() {
                    self.resolved_widths[i] = *width;
                }
            }
        } else {
            sync_default_widths(
                &mut self.resolved_widths,
                column_count,
                DEFAULT_COLUMN_WIDTH,
            );
        }

        self.total_width = self.resolved_widths.iter().sum::<f64>() + 24.0;
        if self.resolved_widths_shared.as_ref() != self.resolved_widths.as_slice() {
            self.resolved_widths_shared = Arc::from(self.resolved_widths.as_slice());
        }
        self.selected_row = clamp_selected_row(self.selected_row, self.data_row_count());

        self.view
            .label(cx, ids!(table_view.caption_label))
            .set_text(cx, self.caption.as_ref());
        self.view
            .label(cx, ids!(table_view.caption_label))
            .set_visible(cx, !self.caption.as_ref().is_empty());
        self.view
            .widget(cx, ids!(table_view.scroll.content.header))
            .set_visible(cx, !custom_row_mode && !self.headers.is_empty());

        if let Some(mut header) = self
            .view
            .widget_flood(cx, ids!(table_view.scroll.content.header))
            .borrow_mut::<ShadTableHeaderView>()
        {
            header.set_header_data(
                cx,
                &self.headers,
                &self.resolved_widths,
                self.total_width,
                self.text_align,
            );
        }

        self.apply_content_width_if_changed(cx, self.total_width);
    }

    fn empty_fill_rows(list: &PortalList, cx: &Cx2d, used_rows: usize) -> usize {
        table_empty_fill_rows(
            list.area().rect(cx).size.y.max(0.0),
            TABLE_ROW_HEIGHT,
            used_rows,
        )
    }

    fn draw_rows(&mut self, cx: &mut Cx2d, list: &mut PortalList) {
        if self.has_custom_rows() {
            self.draw_custom_rows(cx, list);
            return;
        }

        if self.data_row_count() == 0 {
            let rows = Self::empty_fill_rows(list, cx, 0).max(1);
            list.set_item_range(cx, 0, rows);
            while let Some(item_id) = list.next_visible_item(cx) {
                let label = if item_id == 0 {
                    self.empty_message.as_ref()
                } else {
                    ""
                };
                self.draw_empty_row(cx, list, item_id, label);
            }
            return;
        }

        let row_count = self.data_row_count();
        let empty_rows = Self::empty_fill_rows(list, cx, row_count);
        let item_count = row_count + empty_rows;
        let shared_widths = &self.resolved_widths_shared;
        list.set_item_range(cx, 0, item_count);
        while let Some(item_id) = list.next_visible_item(cx) {
            if item_id >= row_count {
                self.draw_empty_row(cx, list, item_id, "");
                continue;
            }
            let Some(row) = self.row_for_index(item_id) else {
                self.draw_empty_row(cx, list, item_id, "");
                continue;
            };

            let item = list.item(cx, item_id, id!(Item));
            if let Some(mut row_view) = item.borrow_mut::<ShadTableRowView>() {
                row_view.set_row_data(
                    cx,
                    item_id,
                    row,
                    shared_widths,
                    self.total_width,
                    self.text_align,
                    self.selected_row == Some(item_id),
                    item_id & 1 == 1,
                );
            }
            item.draw_all(cx, &mut Scope::empty());
        }
    }

    fn draw_rows_with_widths(
        &mut self,
        cx: &mut Cx2d,
        list: &mut PortalList,
        widths: &Arc<[f64]>,
        total_width: f64,
    ) {
        if self.has_custom_rows() {
            self.draw_custom_rows(cx, list);
            return;
        }

        if self.data_row_count() == 0 {
            let rows = Self::empty_fill_rows(list, cx, 0).max(1);
            list.set_item_range(cx, 0, rows);
            while let Some(item_id) = list.next_visible_item(cx) {
                let label = if item_id == 0 {
                    self.empty_message.as_ref()
                } else {
                    ""
                };
                self.draw_empty_row(cx, list, item_id, label);
            }
            return;
        }

        let row_count = self.data_row_count();
        let empty_rows = Self::empty_fill_rows(list, cx, row_count);
        let item_count = row_count + empty_rows;
        list.set_item_range(cx, 0, item_count);
        while let Some(item_id) = list.next_visible_item(cx) {
            if item_id >= row_count {
                self.draw_empty_row(cx, list, item_id, "");
                continue;
            }
            let Some(row) = self.row_for_index(item_id) else {
                self.draw_empty_row(cx, list, item_id, "");
                continue;
            };

            let item = list.item(cx, item_id, id!(Item));
            if let Some(mut row_view) = item.borrow_mut::<ShadTableRowView>() {
                row_view.set_row_data(
                    cx,
                    item_id,
                    row,
                    widths,
                    total_width,
                    self.text_align,
                    self.selected_row == Some(item_id),
                    item_id & 1 == 1,
                );
            }
            item.draw_all(cx, &mut Scope::empty());
        }
    }

    fn draw_custom_rows(&mut self, cx: &mut Cx2d, list: &mut PortalList) {
        let row_count = self.data_row_count();
        if row_count == 0 {
            let rows = Self::empty_fill_rows(list, cx, 0).max(1);
            list.set_item_range(cx, 0, rows);
            while let Some(item_id) = list.next_visible_item(cx) {
                let label = if item_id == 0 {
                    self.empty_message.as_ref()
                } else {
                    ""
                };
                self.draw_empty_row(cx, list, item_id, label);
            }
            return;
        }

        let empty_rows = Self::empty_fill_rows(list, cx, row_count);
        list.set_item_range(cx, 0, row_count + empty_rows);
        while let Some(item_id) = list.next_visible_item(cx) {
            if item_id >= row_count {
                self.draw_empty_row(cx, list, item_id, "");
                continue;
            }
            let Some(template) = self.custom_row_template_ids.get(item_id).copied() else {
                self.draw_empty_row(cx, list, item_id, "");
                continue;
            };
            list.item(cx, item_id, template)
                .draw_all(cx, &mut Scope::empty());
        }
    }

    fn maybe_request_virtual_window(&self, cx: &mut Cx, list: &PortalListRef) {
        if self.has_custom_rows() || self.virtual_total_rows == 0 || self.rows_data.is_empty() {
            return;
        }

        let window_len = self.rows_data.len();
        let first_visible = list.first_id();
        let visible_rows = list.visible_items().max(1);
        let window_end = self.virtual_window_start.saturating_add(window_len);

        let needs_previous = first_visible
            < self
                .virtual_window_start
                .saturating_add(Self::VIRTUAL_WINDOW_PRELOAD_MARGIN);
        let needs_next = first_visible
            .saturating_add(visible_rows)
            .saturating_add(Self::VIRTUAL_WINDOW_PRELOAD_MARGIN)
            >= window_end;

        if !needs_previous && !needs_next {
            return;
        }

        let max_start = self.virtual_total_rows.saturating_sub(window_len);
        let requested_start = first_visible.min(max_start);
        if requested_start == self.virtual_window_start {
            return;
        }

        cx.widget_action_with_data(
            &self.action_data,
            self.widget_uid(),
            ShadTableAction::VirtualWindowRequest(requested_start),
        );
    }

    pub fn set_headers(&mut self, cx: &mut Cx, headers: Vec<String>) {
        self.headers = headers;
        self.sync_layout(cx);
        self.view.redraw(cx);
    }

    pub fn set_caption(&mut self, cx: &mut Cx, caption: String) {
        self.caption.set(&caption);
        self.sync_layout(cx);
        self.view.redraw(cx);
    }

    pub fn set_rows(&mut self, cx: &mut Cx, rows: Vec<Vec<String>>) {
        self.custom_row_template_ids = Arc::default();
        self.virtual_total_rows = 0;
        self.virtual_window_start = 0;
        self.rows_data = into_arc_rows(rows);
        self.rows_source = ScriptValue::default();
        self.selected_row = clamp_selected_row(self.selected_row, self.data_row_count());
        self.sync_layout(cx);
        self.view.redraw(cx);
    }

    pub fn set_virtual_total_rows(&mut self, cx: &mut Cx, total_rows: usize) {
        let cleared_custom_rows = !self.custom_row_template_ids.is_empty();
        self.custom_row_template_ids = Arc::default();
        if self.virtual_total_rows == total_rows && !cleared_custom_rows {
            return;
        }
        self.virtual_total_rows = total_rows;
        self.rows_data.clear();
        self.rows_source = ScriptValue::default();
        if total_rows == 0 {
            self.virtual_window_start = 0;
        } else if self.virtual_window_start >= total_rows {
            self.virtual_window_start = total_rows.saturating_sub(1);
        }
        self.selected_row = clamp_selected_row(self.selected_row, self.data_row_count());
        self.sync_layout(cx);
        self.view.redraw(cx);
    }

    pub fn set_virtual_window(&mut self, cx: &mut Cx, start_row: usize, rows: Vec<Vec<String>>) {
        let cleared_custom_rows = !self.custom_row_template_ids.is_empty();
        self.custom_row_template_ids = Arc::default();
        if self.virtual_total_rows == 0 {
            self.set_rows(cx, rows);
            return;
        }
        self.rows_data = into_arc_rows(rows);
        self.rows_source = ScriptValue::default();
        let row_count = self.data_row_count();
        let clamped_start = if row_count == 0 {
            0
        } else {
            start_row.min(row_count.saturating_sub(1))
        };
        self.virtual_window_start = clamped_start;
        let max_window_len = row_count.saturating_sub(clamped_start);
        debug_assert!(
            self.rows_data.len() <= max_window_len,
            "set_virtual_window received {} rows but only {} fit into the declared virtual_total_rows={} from start_row={}",
            self.rows_data.len(),
            max_window_len,
            self.virtual_total_rows,
            clamped_start
        );
        if self.rows_data.len() > max_window_len {
            self.rows_data.truncate(max_window_len);
        }
        self.selected_row = clamp_selected_row(self.selected_row, row_count);
        let column_count = resolved_column_count(&self.headers, &self.rows_data).max(1);
        if cleared_custom_rows || self.resolved_widths.len() != column_count {
            self.sync_layout(cx);
        }
        self.view.redraw(cx);
    }

    pub fn set_custom_row_templates(&mut self, cx: &mut Cx, templates: Arc<[LiveId]>) {
        let changed = self.custom_row_template_ids.as_ref() != templates.as_ref();
        self.custom_row_template_ids = templates;
        self.virtual_total_rows = 0;
        self.virtual_window_start = 0;
        self.rows_data.clear();
        self.rows_source = ScriptValue::default();
        self.selected_row = None;
        if changed {
            self.sync_layout(cx);
            self.view.redraw(cx);
        }
    }

    pub fn set_selected_row(&mut self, cx: &mut Cx, selected_row: Option<usize>) {
        if self.selected_row == selected_row {
            return;
        }
        self.selected_row = selected_row;
        if let Some(row) = selected_row {
            cx.widget_action_with_data(
                &self.action_data,
                self.widget_uid(),
                ShadTableAction::SelectionChanged(row),
            );
        }
        self.view.redraw(cx);
    }

    pub fn selected_row(&self) -> Option<usize> {
        self.selected_row
    }

    pub fn row_clicked(&self, actions: &Actions) -> Option<usize> {
        widget_action_map::<ShadTableAction, _, _>(actions, self.widget_uid(), |action| {
            if let ShadTableAction::RowClicked(index) = action {
                Some(index)
            } else {
                None
            }
        })
    }

    pub fn selection_changed(&self, actions: &Actions) -> Option<usize> {
        widget_action_map::<ShadTableAction, _, _>(actions, self.widget_uid(), |action| {
            if let ShadTableAction::SelectionChanged(index) = action {
                Some(index)
            } else {
                None
            }
        })
    }

    pub fn virtual_window_request(&self, actions: &Actions) -> Option<usize> {
        widget_action_map::<ShadTableAction, _, _>(actions, self.widget_uid(), |action| {
            if let ShadTableAction::VirtualWindowRequest(start) = action {
                Some(start)
            } else {
                None
            }
        })
    }
}

impl Widget for ShadTable {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        let list = self
            .view
            .portal_list(cx, ids!(table_view.scroll.content.list));
        let list_widget = self.view.widget(cx, ids!(table_view.scroll.content.list));
        self.view.handle_event(cx, event, scope);

        if let Event::Scroll(scroll_event) = event {
            if list_widget.point_hits_area(cx, scroll_event.abs) {
                scroll_event.handled_y.set(true);
            }
        }

        if let Event::Actions(actions) = event {
            if matches!(
                actions
                    .find_widget_action(list.widget_uid())
                    .map(|action| action.cast::<PortalListAction>()),
                Some(PortalListAction::Scroll)
            ) {
                self.maybe_request_virtual_window(cx, &list);
            }

            if !list.any_items_with_actions(actions) {
                return;
            }
            for (_item_id, item) in list.items_with_actions(actions) {
                if let Some(action) = actions.find_widget_action(item.widget_uid()) {
                    if let ShadTableRowAction::Clicked(index) = action.cast() {
                        cx.widget_action_with_data(
                            &self.action_data,
                            uid,
                            ShadTableAction::RowClicked(index),
                        );
                        if self.selectable {
                            self.set_selected_row(cx, Some(index));
                        }
                    }
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let auto_filled = if self.auto_fill_width && !self.resolved_widths.is_empty() {
            cx.begin_turtle(walk, self.layout);
            let avail = cx.turtle().rect().size.x;
            cx.end_turtle();

            if let Some((shared_widths, new_total)) = self.cached_stretched_widths(avail) {
                if let Some(mut header) = self
                    .view
                    .widget_flood(cx, ids!(table_view.scroll.content.header))
                    .borrow_mut::<ShadTableHeaderView>()
                {
                    header.set_header_data(
                        cx,
                        &self.headers,
                        &shared_widths,
                        new_total,
                        self.text_align,
                    );
                }

                self.apply_content_width_if_changed(cx, new_total);

                while let Some(step) = self.view.draw_walk(cx, scope, walk).step() {
                    if let Some(mut list) = step.as_portal_list().borrow_mut() {
                        self.draw_rows_with_widths(cx, &mut list, &shared_widths, new_total);
                    }
                }
                return DrawStep::done();
            }
            false
        } else {
            false
        };

        if !auto_filled {
            while let Some(step) = self.view.draw_walk(cx, scope, walk).step() {
                if let Some(mut list) = step.as_portal_list().borrow_mut() {
                    self.draw_rows(cx, &mut list);
                }
            }
        }
        DrawStep::done()
    }
}

impl ShadTableRef {
    pub fn set_headers(&self, cx: &mut Cx, headers: Vec<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_headers(cx, headers);
        }
    }

    pub fn set_caption(&self, cx: &mut Cx, caption: String) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_caption(cx, caption);
        }
    }

    pub fn set_rows(&self, cx: &mut Cx, rows: Vec<Vec<String>>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_rows(cx, rows);
        }
    }

    pub fn set_virtual_total_rows(&self, cx: &mut Cx, total_rows: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_virtual_total_rows(cx, total_rows);
        }
    }

    pub fn set_virtual_window(&self, cx: &mut Cx, start_row: usize, rows: Vec<Vec<String>>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_virtual_window(cx, start_row, rows);
        }
    }

    pub fn set_custom_row_templates(&self, cx: &mut Cx, templates: Arc<[LiveId]>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_custom_row_templates(cx, templates);
        }
    }

    pub fn set_selected_row(&self, cx: &mut Cx, selected_row: Option<usize>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_selected_row(cx, selected_row);
        }
    }

    pub fn selected_row(&self) -> Option<usize> {
        self.borrow().and_then(|inner| inner.selected_row())
    }

    pub fn row_clicked(&self, actions: &Actions) -> Option<usize> {
        self.borrow().and_then(|inner| inner.row_clicked(actions))
    }

    pub fn selection_changed(&self, actions: &Actions) -> Option<usize> {
        self.borrow()
            .and_then(|inner| inner.selection_changed(actions))
    }

    pub fn virtual_window_request(&self, actions: &Actions) -> Option<usize> {
        self.borrow()
            .and_then(|inner| inner.virtual_window_request(actions))
    }
}

fn into_arc_rows(rows: Vec<Vec<String>>) -> Vec<Arc<[String]>> {
    rows.into_iter().map(Arc::<[String]>::from).collect()
}

fn parse_rows(vm: &mut ScriptVm, value: ScriptValue) -> Vec<Arc<[String]>> {
    let Some(obj) = value.as_object() else {
        return Vec::new();
    };

    let mut rows = Vec::new();
    vm.vec_with(obj, |vm, vec| {
        for entry in vec {
            let Some(row_obj) = entry.value.as_object() else {
                continue;
            };
            let mut row = Vec::new();
            vm.vec_with(row_obj, |vm, row_vec| {
                for cell in row_vec {
                    let mut value = String::new();
                    vm.bx.heap.cast_to_string(cell.value, &mut value);
                    row.push(value);
                }
            });
            rows.push(Arc::from(row));
        }
    });
    rows
}

fn should_apply_content_width(last_applied_width: &mut Option<f64>, width: f64) -> bool {
    if matches!(last_applied_width, Some(previous) if previous.to_bits() == width.to_bits()) {
        return false;
    }
    *last_applied_width = Some(width);
    true
}

fn invalidate_content_width_cache(last_applied_width: &mut Option<f64>) {
    *last_applied_width = None;
}

fn draw_border(cx: &mut Cx2d, draw: &mut DrawColor, rect: Rect, color: Vec4) {
    draw.color = color;
    draw.draw_abs(
        cx,
        Rect {
            pos: rect.pos,
            size: dvec2(rect.size.x, 1.0),
        },
    );
    draw.draw_abs(
        cx,
        Rect {
            pos: dvec2(rect.pos.x, rect.pos.y + rect.size.y - 1.0),
            size: dvec2(rect.size.x, 1.0),
        },
    );
    draw.draw_abs(
        cx,
        Rect {
            pos: rect.pos,
            size: dvec2(1.0, rect.size.y),
        },
    );
    draw.draw_abs(
        cx,
        Rect {
            pos: dvec2(rect.pos.x + rect.size.x - 1.0, rect.pos.y),
            size: dvec2(1.0, rect.size.y),
        },
    );
}

#[cfg(test)]
mod tests {
    use super::{
        invalidate_content_width_cache, replace_arc_slice_if_changed, should_apply_content_width,
        sync_default_widths,
    };
    use std::hint::black_box;
    use std::sync::Arc;
    use std::time::Instant;

    #[test]
    fn replace_arc_slice_noop_when_identical() {
        let row: Arc<[String]> = Arc::from(vec!["alpha".to_string(), "beta".to_string()]);
        let mut dst = Arc::clone(&row);
        assert!(!replace_arc_slice_if_changed(&mut dst, &row));
        assert!(Arc::ptr_eq(&dst, &row));
    }

    #[test]
    fn replace_arc_slice_switches_reference() {
        let mut dst: Arc<[String]> = Arc::from(vec!["old".to_string(), "values".to_string()]);
        let src: Arc<[String]> = Arc::from(vec!["new".to_string(), "row".to_string()]);
        assert!(replace_arc_slice_if_changed(&mut dst, &src));
        assert!(Arc::ptr_eq(&dst, &src));
    }

    #[test]
    fn replace_arc_slice_performance_comparison() {
        // Performance comparison helper: this prints timings for manual verification.
        // It intentionally does not assert wall-clock durations to avoid flaky CI failures.
        const BENCHMARK_ITERATIONS: usize = 50_000;
        let source_a: Arc<[String]> =
            Arc::from(vec!["A".to_string(), "B".to_string(), "C".to_string()]);
        let source_b: Arc<[String]> =
            Arc::from(vec!["X".to_string(), "Y".to_string(), "Z".to_string()]);

        let mut old = Arc::clone(&source_b);
        let old_start = Instant::now();
        for _ in 0..BENCHMARK_ITERATIONS {
            // Previous row-change path: compare full row, then clone all strings.
            if old.as_ref() != source_a.as_ref() {
                old = Arc::from(source_a.as_ref().to_vec());
            }
            if old.as_ref() != source_b.as_ref() {
                old = Arc::from(source_b.as_ref().to_vec());
            }
            black_box(&old);
        }
        let old_elapsed = old_start.elapsed();

        let mut optimized = Arc::clone(&source_b);
        let new_start = Instant::now();
        for _ in 0..BENCHMARK_ITERATIONS {
            // Optimized row-change path: pointer swaps only.
            replace_arc_slice_if_changed(&mut optimized, &source_a);
            replace_arc_slice_if_changed(&mut optimized, &source_b);
            black_box(&optimized);
        }
        let new_elapsed = new_start.elapsed();

        println!(
            "replace_arc_slice_if_changed benchmark: old={old_elapsed:?}, new={new_elapsed:?}"
        );
    }

    #[test]
    fn test_sync_default_widths_reuses_allocation() {
        let mut widths = vec![160.0, 160.0, 160.0, 160.0];
        let ptr_before = widths.as_ptr();
        let capacity_before = widths.capacity();
        sync_default_widths(&mut widths, 4, 160.0);
        assert_eq!(widths.as_ptr(), ptr_before);
        assert_eq!(widths.capacity(), capacity_before);
    }

    #[test]
    fn test_sync_default_widths_performance_comparison() {
        // Performance comparison helper: this prints timings for manual verification.
        // It intentionally does not assert wall-clock durations to avoid flaky CI failures.
        const BENCHMARK_ITERATIONS: usize = 100_000;
        const COLUMN_COUNT: usize = 8;

        let old_start = Instant::now();
        let mut old = Vec::new();
        for _ in 0..BENCHMARK_ITERATIONS {
            old = vec![160.0; COLUMN_COUNT];
            black_box(&old);
        }
        let old_elapsed = old_start.elapsed();

        let new_start = Instant::now();
        let mut optimized = vec![160.0; COLUMN_COUNT];
        let optimized_capacity = optimized.capacity();
        for _ in 0..BENCHMARK_ITERATIONS {
            sync_default_widths(&mut optimized, COLUMN_COUNT, 160.0);
            black_box(&optimized);
        }
        let new_elapsed = new_start.elapsed();
        assert_eq!(optimized.capacity(), optimized_capacity);
        assert_eq!(old, optimized);

        println!("sync_default_widths benchmark: old={old_elapsed:?}, new={new_elapsed:?}");
    }

    #[test]
    fn test_sync_default_widths_resizes_with_existing_capacity() {
        let mut widths = vec![160.0; 8];
        widths.reserve(8);
        let capacity_before = widths.capacity();

        sync_default_widths(&mut widths, 12, 160.0);
        assert_eq!(widths.len(), 12);
        assert_eq!(widths.capacity(), capacity_before);
        assert!(widths.iter().all(|width| *width == 160.0));

        sync_default_widths(&mut widths, 8, 160.0);
        assert_eq!(widths.len(), 8);
        assert_eq!(widths.capacity(), capacity_before);
        assert!(widths.iter().all(|width| *width == 160.0));
    }

    #[test]
    fn content_width_apply_cache_skips_steady_state_updates() {
        const FRAME_COUNT: usize = 120_000;

        let old_start = Instant::now();
        let mut uncached_updates = 0usize;
        for _ in 0..FRAME_COUNT {
            uncached_updates += 1;
            black_box(uncached_updates);
        }
        let old_elapsed = old_start.elapsed();

        let new_start = Instant::now();
        let mut cached_width = None;
        let mut cached_updates = 0usize;
        for _ in 0..FRAME_COUNT {
            if should_apply_content_width(&mut cached_width, 960.0) {
                cached_updates += 1;
            }
            black_box(cached_updates);
        }
        let new_elapsed = new_start.elapsed();

        assert_eq!(uncached_updates, FRAME_COUNT);
        assert_eq!(cached_updates, 1);
        println!(
            "content_width_apply_cache benchmark: frames={FRAME_COUNT}, uncached_updates={uncached_updates}, cached_updates={cached_updates}, old={old_elapsed:?}, new={new_elapsed:?}"
        );
    }

    #[test]
    fn content_width_apply_cache_reapplies_after_invalidation() {
        let mut cached_width = None;

        assert!(should_apply_content_width(&mut cached_width, 960.0));
        assert!(!should_apply_content_width(&mut cached_width, 960.0));

        invalidate_content_width_cache(&mut cached_width);

        assert!(should_apply_content_width(&mut cached_width, 960.0));
    }
}
