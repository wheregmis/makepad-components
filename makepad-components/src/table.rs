use crate::internal::actions::widget_action_map;
use crate::models::table::{
    clamp_selected_row, default_widths, empty_fill_rows as table_empty_fill_rows,
    resolved_column_count,
};
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

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
        width: Fit
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
                        width: Fit
                        height: 208
                        flow: Down
                        max_pull_down: 0.0
                        capture_overload: false
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
const TABLE_ROW_HEIGHT: f64 = 40.0;

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
        headers: Vec<String>,
        widths: Vec<f64>,
        total_width: f64,
    ) {
        self.headers = headers;
        self.widths = widths;
        self.walk.width = Size::Fixed(total_width);
        self.area.redraw(cx);
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

        let mut x = rect.pos.x + 12.0;
        for (index, header) in self.headers.iter().enumerate() {
            let width = self
                .widths
                .get(index)
                .copied()
                .unwrap_or(DEFAULT_COLUMN_WIDTH);
            self.draw_text.color = self.text_color;
            self.draw_text
                .draw_abs(cx, dvec2(x, rect.pos.y + 12.0), header);
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
    #[rust]
    area: Area,
    #[rust]
    row_index: usize,
    #[rust]
    cells: Vec<String>,
    #[rust]
    widths: Vec<f64>,
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
    pub fn set_row_data(
        &mut self,
        cx: &mut Cx,
        row_index: usize,
        cells: Vec<String>,
        widths: Vec<f64>,
        total_width: f64,
        selected: bool,
        striped: bool,
    ) {
        self.row_index = row_index;
        self.cells = cells;
        self.widths = widths;
        self.selected = selected;
        self.striped = striped;
        self.walk.width = Size::Fixed(total_width);
        self.area.redraw(cx);
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

        let mut x = rect.pos.x + 12.0;
        for (index, cell) in self.cells.iter().enumerate() {
            let width = self
                .widths
                .get(index)
                .copied()
                .unwrap_or(DEFAULT_COLUMN_WIDTH);
            self.draw_text.color = self.text_color;
            self.draw_text
                .draw_abs(cx, dvec2(x, rect.pos.y + 14.0), cell);
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

    #[rust]
    rows_data: Vec<Vec<String>>,
    #[rust]
    resolved_widths: Vec<f64>,
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
        let rows = parse_rows(vm, self.rows);
        vm.with_cx_mut(|cx| {
            self.rows_data = rows;
            self.sync_layout(cx);
        });
    }
}

impl ShadTable {
    fn compute_widths(&self) -> Vec<f64> {
        default_widths(
            resolved_column_count(&self.headers, &self.rows_data),
            DEFAULT_COLUMN_WIDTH,
        )
    }

    fn sync_layout(&mut self, cx: &mut Cx) {
        self.resolved_widths = self.compute_widths();
        self.total_width = self.resolved_widths.iter().sum::<f64>() + 24.0;
        self.selected_row = clamp_selected_row(self.selected_row, self.rows_data.len());

        self.view
            .label(cx, ids!(table_view.caption_label))
            .set_text(cx, self.caption.as_ref());
        self.view
            .label(cx, ids!(table_view.caption_label))
            .set_visible(cx, !self.caption.as_ref().is_empty());

        if let Some(mut header) = self
            .view
            .widget_flood(cx, ids!(table_view.scroll.content.header))
            .borrow_mut::<ShadTableHeaderView>()
        {
            header.set_header_data(
                cx,
                self.headers.clone(),
                self.resolved_widths.clone(),
                self.total_width,
            );
        }

        let mut content = self.view.view(cx, ids!(table_view.scroll.content));
        script_apply_eval!(cx, content, {
            width: #(self.total_width)
        });
        let mut empty = self
            .view
            .view(cx, ids!(table_view.scroll.content.list.Empty));
        script_apply_eval!(cx, empty, {
            width: #(self.total_width)
        });
    }

    fn empty_fill_rows(list: &PortalList, cx: &Cx2d, used_rows: usize) -> usize {
        table_empty_fill_rows(list.area().rect(cx).size.y.max(0.0), TABLE_ROW_HEIGHT, used_rows)
    }

    fn draw_rows(&mut self, cx: &mut Cx2d, list: &mut PortalList) {
        if self.rows_data.is_empty() {
            let rows = Self::empty_fill_rows(list, cx, 0).max(1);
            list.set_item_range(cx, 0, rows);
            while let Some(item_id) = list.next_visible_item(cx) {
                let mut item = list.item(cx, item_id, id!(Empty)).as_view();
                let label = if item_id == 0 {
                    self.empty_message.as_ref()
                } else {
                    ""
                };
                item.label(cx, ids!(empty_label)).set_text(cx, label);
                script_apply_eval!(cx, item, {
                    width: #(self.total_width)
                });
                item.draw_all(cx, &mut Scope::empty());
            }
            return;
        }

        let empty_rows = Self::empty_fill_rows(list, cx, self.rows_data.len());
        let item_count = self.rows_data.len() + empty_rows;
        list.set_item_range(cx, 0, item_count);
        while let Some(item_id) = list.next_visible_item(cx) {
            let Some(row) = self.rows_data.get(item_id) else {
                let mut item = list.item(cx, item_id, id!(Empty)).as_view();
                item.label(cx, ids!(empty_label)).set_text(cx, "");
                script_apply_eval!(cx, item, {
                    width: #(self.total_width)
                });
                item.draw_all(cx, &mut Scope::empty());
                continue;
            };

            let item = list.item(cx, item_id, id!(Item));
            if let Some(mut row_view) = item.borrow_mut::<ShadTableRowView>() {
                row_view.set_row_data(
                    cx,
                    item_id,
                    row.clone(),
                    self.resolved_widths.clone(),
                    self.total_width,
                    self.selected_row == Some(item_id),
                    item_id & 1 == 1,
                );
            }
            item.draw_all(cx, &mut Scope::empty());
        }
    }

    pub fn set_headers(&mut self, cx: &mut Cx, headers: Vec<String>) {
        self.headers = headers;
        self.sync_layout(cx);
        self.view.redraw(cx);
    }

    pub fn set_rows(&mut self, cx: &mut Cx, rows: Vec<Vec<String>>) {
        self.rows_data = rows;
        self.selected_row = clamp_selected_row(self.selected_row, self.rows_data.len());
        self.sync_layout(cx);
        self.view.redraw(cx);
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
}

impl Widget for ShadTable {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        let list = self
            .view
            .portal_list(cx, ids!(table_view.scroll.content.list));
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
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
        while let Some(step) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = step.as_portal_list().borrow_mut() {
                self.draw_rows(cx, &mut *list);
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

    pub fn set_rows(&self, cx: &mut Cx, rows: Vec<Vec<String>>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_rows(cx, rows);
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
}

fn parse_rows(vm: &mut ScriptVm, value: ScriptValue) -> Vec<Vec<String>> {
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
            rows.push(row);
        }
    });
    rows
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
