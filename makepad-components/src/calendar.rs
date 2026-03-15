use crate::internal::script_args::string_arg;
pub use crate::models::calendar::ShadDate;
use crate::models::calendar::{days_in_month, shift_month, weekday_from_civil};
use makepad_widgets::widget::WidgetActionData;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.ShadCalendarBase = #(ShadCalendar::register_widget(vm))

    mod.widgets.ShadCalendar = set_type_default() do mod.widgets.ShadCalendarBase{
        width: 280
        height: 304
        value: ""

        color_background: (shad_theme.color_background)
        color_primary: (shad_theme.color_primary)
        color_primary_foreground: (shad_theme.color_primary_foreground)
        color_muted_foreground: (shad_theme.color_muted_foreground)
        color_outline_border: (shad_theme.color_outline_border)
        color_hover: (shad_theme.color_secondary_hover)

        draw_bg +: { color: (shad_theme.color_background) }
        draw_cell_bg +: { color: #0000 }

        draw_header_text +: {
            color: (shad_theme.color_primary)
            text_style: theme.font_regular{font_size: 11.0}
        }

        draw_weekday_text +: {
            color: (shad_theme.color_muted_foreground)
            text_style: theme.font_regular{font_size: 10.0}
        }

        draw_day_text +: {
            color: (shad_theme.color_primary)
            text_style: theme.font_regular{font_size: 11.0}
        }

        draw_nav_text +: {
            color: (shad_theme.color_primary)
            text_style: theme.font_regular{font_size: 14.0}
        }
    }
}

const WEEKDAY_LABELS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const MONTH_LABELS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const DAY_LABELS: [&str; 31] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17",
    "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31",
];

#[derive(Clone, Debug, Default)]
pub enum ShadCalendarAction {
    Changed(ShadDate),
    #[default]
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CalendarCell {
    date: ShadDate,
    in_month: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CalendarTarget {
    Prev,
    Next,
    Cell(usize),
}

#[derive(Clone, Copy)]
struct CalendarLayoutInfo {
    gap: f64,
    cell_width: f64,
    cell_height: f64,
    prev_rect: Rect,
    next_rect: Rect,
    title_pos: DVec2,
    weekday_y: f64,
    grid_origin: DVec2,
}

#[derive(Script, Widget)]
pub struct ShadCalendar {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,

    #[live]
    value: ArcStringMut,
    #[live]
    color_background: Vec4,
    #[live]
    color_primary: Vec4,
    #[live]
    color_primary_foreground: Vec4,
    #[live]
    color_muted_foreground: Vec4,
    #[live]
    color_outline_border: Vec4,
    #[live]
    color_hover: Vec4,

    #[redraw]
    #[live]
    draw_bg: DrawColor,
    #[redraw]
    #[live]
    draw_cell_bg: DrawColor,
    #[redraw]
    #[live]
    draw_header_text: DrawText,
    #[redraw]
    #[live]
    draw_weekday_text: DrawText,
    #[redraw]
    #[live]
    draw_day_text: DrawText,
    #[redraw]
    #[live]
    draw_nav_text: DrawText,

    #[rust]
    area: Area,
    #[rust]
    value_date: Option<ShadDate>,
    #[rust]
    visible_year: i32,
    #[rust]
    visible_month: u8,
    #[rust]
    today: Option<ShadDate>,
    #[rust]
    hovered_target: Option<CalendarTarget>,

    #[action_data]
    #[rust]
    action_data: WidgetActionData,
}

impl ScriptHook for ShadCalendar {
    fn on_after_new(&mut self, _vm: &mut ScriptVm) {
        self.today = ShadDate::try_today_utc();
        let (year, month) = self
            .today
            .map(|date| (date.year, date.month))
            .unwrap_or_else(ShadDate::fallback_visible_month);
        self.visible_year = year;
        self.visible_month = month;
    }

    fn on_after_apply(
        &mut self,
        vm: &mut ScriptVm,
        _apply: &Apply,
        _scope: &mut Scope,
        _value: ScriptValue,
    ) {
        vm.with_cx_mut(|cx| {
            let parsed = ShadDate::parse_iso(self.value.as_ref());
            self.value_date = parsed;
            if let Some(date) = parsed {
                self.visible_year = date.year;
                self.visible_month = date.month;
            }
            self.area.redraw(cx);
        });
    }
}

impl ShadCalendar {
    fn layout_info(&self, rect: Rect) -> CalendarLayoutInfo {
        let padding = 12.0;
        let gap = 4.0;
        let header_height = 28.0;
        let weekday_height = 18.0;
        let cell_width = ((rect.size.x - padding * 2.0) - gap * 6.0) / 7.0;
        let cell_height =
            ((rect.size.y - padding * 2.0 - header_height - weekday_height) - gap * 7.0) / 6.0;
        let prev_rect = Rect {
            pos: dvec2(rect.pos.x + padding, rect.pos.y + padding),
            size: dvec2(24.0, header_height),
        };
        let next_rect = Rect {
            pos: dvec2(
                rect.pos.x + rect.size.x - padding - 24.0,
                rect.pos.y + padding,
            ),
            size: dvec2(24.0, header_height),
        };
        CalendarLayoutInfo {
            gap,
            cell_width,
            cell_height,
            prev_rect,
            next_rect,
            title_pos: dvec2(
                prev_rect.pos.x + prev_rect.size.x + 12.0,
                rect.pos.y + padding + 7.0,
            ),
            weekday_y: rect.pos.y + padding + header_height + gap + 4.0,
            grid_origin: dvec2(
                rect.pos.x + padding,
                rect.pos.y + padding + header_height + gap + weekday_height + gap,
            ),
        }
    }

    fn month_title(&self) -> String {
        format!(
            "{} {}",
            MONTH_LABELS[(self.visible_month.saturating_sub(1)) as usize],
            self.visible_year
        )
    }

    fn month_grid(&self) -> Vec<CalendarCell> {
        let first_weekday = weekday_from_civil(self.visible_year, self.visible_month, 1) as usize;
        let current_days = days_in_month(self.visible_year, self.visible_month) as usize;
        let (prev_year, prev_month) = shift_month(self.visible_year, self.visible_month, -1);
        let prev_days = days_in_month(prev_year, prev_month) as usize;
        let (next_year, next_month) = shift_month(self.visible_year, self.visible_month, 1);
        let mut cells = Vec::with_capacity(42);

        for index in 0..42 {
            let cell = if index < first_weekday {
                let day = (prev_days - first_weekday + index + 1) as u8;
                CalendarCell {
                    date: ShadDate {
                        year: prev_year,
                        month: prev_month,
                        day,
                    },
                    in_month: false,
                }
            } else if index < first_weekday + current_days {
                let day = (index - first_weekday + 1) as u8;
                CalendarCell {
                    date: ShadDate {
                        year: self.visible_year,
                        month: self.visible_month,
                        day,
                    },
                    in_month: true,
                }
            } else {
                let day = (index - first_weekday - current_days + 1) as u8;
                CalendarCell {
                    date: ShadDate {
                        year: next_year,
                        month: next_month,
                        day,
                    },
                    in_month: false,
                }
            };
            cells.push(cell);
        }
        cells
    }

    fn target_from_abs(&self, cx: &Cx, abs: DVec2) -> Option<CalendarTarget> {
        let rect = self.area.rect(cx);
        if !rect.contains(abs) {
            return None;
        }

        let layout = self.layout_info(rect);
        if layout.prev_rect.contains(abs) {
            return Some(CalendarTarget::Prev);
        }
        if layout.next_rect.contains(abs) {
            return Some(CalendarTarget::Next);
        }

        let grid_width = layout.cell_width * 7.0 + layout.gap * 6.0;
        let grid_height = layout.cell_height * 6.0 + layout.gap * 5.0;
        let grid_rect = Rect {
            pos: layout.grid_origin,
            size: dvec2(grid_width, grid_height),
        };
        if !grid_rect.contains(abs) {
            return None;
        }

        let local_x = abs.x - grid_rect.pos.x;
        let local_y = abs.y - grid_rect.pos.y;
        let stride_x = layout.cell_width + layout.gap;
        let stride_y = layout.cell_height + layout.gap;
        let col = (local_x / stride_x).floor() as usize;
        let row = (local_y / stride_y).floor() as usize;
        if col >= 7 || row >= 6 {
            return None;
        }
        let cell_origin_x = col as f64 * stride_x;
        let cell_origin_y = row as f64 * stride_y;
        if local_x > cell_origin_x + layout.cell_width
            || local_y > cell_origin_y + layout.cell_height
        {
            return None;
        }

        Some(CalendarTarget::Cell(row * 7 + col))
    }

    fn set_hovered_target(&mut self, cx: &mut Cx, target: Option<CalendarTarget>) {
        if self.hovered_target != target {
            self.hovered_target = target;
            self.area.redraw(cx);
        }
    }

    pub fn set_value(&mut self, cx: &mut Cx, value: Option<ShadDate>) {
        if self.value_date == value {
            return;
        }
        self.value_date = value;
        if let Some(date) = value {
            self.visible_year = date.year;
            self.visible_month = date.month;
            cx.widget_action_with_data(
                &self.action_data,
                self.widget_uid(),
                ShadCalendarAction::Changed(date),
            );
        }
        self.area.redraw(cx);
    }

    pub fn clear(&mut self, cx: &mut Cx) {
        if self.value_date.take().is_some() {
            self.area.redraw(cx);
        }
    }

    pub fn value(&self) -> Option<ShadDate> {
        self.value_date
    }

    pub fn set_month(&mut self, cx: &mut Cx, year: i32, month: u8) {
        let clamped_month = month.clamp(1, 12);
        if self.visible_year == year && self.visible_month == clamped_month {
            return;
        }
        self.visible_year = year;
        self.visible_month = clamped_month;
        self.area.redraw(cx);
    }

    pub fn next_month(&mut self, cx: &mut Cx) {
        let (year, month) = shift_month(self.visible_year, self.visible_month, 1);
        self.set_month(cx, year, month);
    }

    pub fn prev_month(&mut self, cx: &mut Cx) {
        let (year, month) = shift_month(self.visible_year, self.visible_month, -1);
        self.set_month(cx, year, month);
    }

    pub fn visible_month(&self) -> (i32, u8) {
        (self.visible_year, self.visible_month)
    }

    pub fn changed(&self, actions: &Actions) -> Option<ShadDate> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let ShadCalendarAction::Changed(value) = item.cast() {
                return Some(value);
            }
        }
        None
    }
}

impl Widget for ShadCalendar {
    fn script_call(
        &mut self,
        vm: &mut ScriptVm,
        method: LiveId,
        args: ScriptValue,
    ) -> ScriptAsyncResult {
        if method == live_id!(set_value) {
            if let Some(next) = string_arg(vm, args) {
                vm.with_cx_mut(|cx| self.set_value(cx, ShadDate::parse_iso(&next)));
            }
            return ScriptAsyncResult::Return(NIL);
        }
        if method == live_id!(clear) {
            vm.with_cx_mut(|cx| self.clear(cx));
            return ScriptAsyncResult::Return(NIL);
        }
        if method == live_id!(value) {
            if let Some(date) = self.value_date {
                let value = date.format_iso();
                if let Some(inline) = ScriptValue::from_inline_string(&value) {
                    return ScriptAsyncResult::Return(inline);
                }
                return ScriptAsyncResult::Return(vm.bx.heap.new_string_from_str(&value));
            }
            return ScriptAsyncResult::Return(NIL);
        }
        ScriptAsyncResult::MethodNotFound
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event.hits(cx, self.area) {
            Hit::FingerHoverIn(fh) => {
                cx.set_cursor(MouseCursor::Hand);
                self.set_hovered_target(cx, self.target_from_abs(cx, fh.abs));
            }
            Hit::FingerMove(fm) => {
                if self.target_from_abs(cx, fm.abs).is_some() {
                    cx.set_cursor(MouseCursor::Hand);
                }
                self.set_hovered_target(cx, self.target_from_abs(cx, fm.abs));
            }
            Hit::FingerHoverOut(_) => self.set_hovered_target(cx, None),
            Hit::FingerUp(fe) if fe.is_primary_hit() => {
                if let Some(target) = self.target_from_abs(cx, fe.abs) {
                    match target {
                        CalendarTarget::Prev => self.prev_month(cx),
                        CalendarTarget::Next => self.next_month(cx),
                        CalendarTarget::Cell(index) => {
                            if let Some(cell) = self.month_grid().get(index).copied() {
                                self.set_value(cx, Some(cell.date));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        let rect = cx.turtle().rect();
        let layout = self.layout_info(rect);
        let cells = self.month_grid();

        self.draw_bg.color = self.color_background;
        self.draw_bg.draw_abs(cx, rect);
        draw_border(cx, &mut self.draw_bg, rect, self.color_outline_border);

        self.draw_cell_bg.color = if self.hovered_target == Some(CalendarTarget::Prev) {
            self.color_hover
        } else {
            vec4(0.0, 0.0, 0.0, 0.0)
        };
        self.draw_cell_bg.draw_abs(cx, layout.prev_rect);

        self.draw_cell_bg.color = if self.hovered_target == Some(CalendarTarget::Next) {
            self.color_hover
        } else {
            vec4(0.0, 0.0, 0.0, 0.0)
        };
        self.draw_cell_bg.draw_abs(cx, layout.next_rect);

        self.draw_nav_text.color = self.color_primary;
        self.draw_nav_text
            .draw_abs(cx, layout.prev_rect.pos + dvec2(8.0, 5.0), "<");
        self.draw_nav_text
            .draw_abs(cx, layout.next_rect.pos + dvec2(8.0, 5.0), ">");

        self.draw_header_text.color = self.color_primary;
        self.draw_header_text
            .draw_abs(cx, layout.title_pos, &self.month_title());

        for (index, label) in WEEKDAY_LABELS.iter().enumerate() {
            let label_x = layout.grid_origin.x
                + index as f64 * (layout.cell_width + layout.gap)
                + layout.cell_width * 0.5
                - 10.0;
            self.draw_weekday_text.color = self.color_muted_foreground;
            self.draw_weekday_text
                .draw_abs(cx, dvec2(label_x, layout.weekday_y), label);
        }

        for (index, cell) in cells.iter().enumerate() {
            let row = index / 7;
            let col = index % 7;
            let cell_rect = Rect {
                pos: dvec2(
                    layout.grid_origin.x + col as f64 * (layout.cell_width + layout.gap),
                    layout.grid_origin.y + row as f64 * (layout.cell_height + layout.gap),
                ),
                size: dvec2(layout.cell_width, layout.cell_height),
            };
            let is_selected = self.value_date == Some(cell.date);
            let is_today = self.today == Some(cell.date);
            let is_hovered = self.hovered_target == Some(CalendarTarget::Cell(index));

            self.draw_cell_bg.color = if is_selected {
                self.color_primary
            } else if is_hovered {
                self.color_hover
            } else {
                vec4(0.0, 0.0, 0.0, 0.0)
            };
            self.draw_cell_bg.draw_abs(cx, cell_rect);
            if is_today && !is_selected {
                draw_border(cx, &mut self.draw_bg, cell_rect, self.color_primary);
            } else if is_hovered {
                draw_border(cx, &mut self.draw_bg, cell_rect, self.color_outline_border);
            }

            self.draw_day_text.color = if is_selected {
                self.color_primary_foreground
            } else if cell.in_month {
                self.color_primary
            } else {
                self.color_muted_foreground
            };
            // Optimization: avoid repeated string allocations in UI draw_walk loops
            // Previously: allocated a new String using `cell.date.day.to_string()` on every frame
            // Now: reuse a static array of string literals representing days 1 to 31
            let day_text = DAY_LABELS[(cell.date.day.clamp(1, 31) - 1) as usize];
            let text_x = cell_rect.pos.x + cell_rect.size.x * 0.5
                - if cell.date.day < 10 { 3.5 } else { 7.0 };
            let text_y = cell_rect.pos.y + cell_rect.size.y * 0.5 - 6.0;
            self.draw_day_text
                .draw_abs(cx, dvec2(text_x, text_y), day_text);
        }

        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }
}

impl ShadCalendarRef {
    pub fn set_value(&self, cx: &mut Cx, value: Option<ShadDate>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_value(cx, value);
        }
    }

    pub fn clear(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.clear(cx);
        }
    }

    pub fn value(&self) -> Option<ShadDate> {
        self.borrow().and_then(|inner| inner.value())
    }

    pub fn set_month(&self, cx: &mut Cx, year: i32, month: u8) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_month(cx, year, month);
        }
    }

    pub fn next_month(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.next_month(cx);
        }
    }

    pub fn prev_month(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.prev_month(cx);
        }
    }

    pub fn visible_month(&self) -> Option<(i32, u8)> {
        self.borrow().map(|inner| inner.visible_month())
    }

    pub fn changed(&self, actions: &Actions) -> Option<ShadDate> {
        self.borrow().and_then(|inner| inner.changed(actions))
    }
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
