pub fn resolved_column_count(headers: &[String], rows: &[Vec<String>]) -> usize {
    headers
        .len()
        .max(rows.iter().map(|row| row.len()).max().unwrap_or(0))
}

pub fn default_widths(column_count: usize, default_column_width: f64) -> Vec<f64> {
    vec![default_column_width; column_count]
}

pub fn clamp_selected_row(selected_row: Option<usize>, row_count: usize) -> Option<usize> {
    selected_row.filter(|row| *row < row_count)
}

pub fn empty_fill_rows(viewport_height: f64, row_height: f64, used_rows: usize) -> usize {
    if viewport_height <= 0.0 {
        return 1usize.saturating_sub(used_rows);
    }
    let visible_rows = ((viewport_height / row_height).ceil() as usize).max(1);
    visible_rows.saturating_sub(used_rows)
}

/// Maps a global row index into a local index within a virtualized row window.
///
/// Returns `Some(local_index)` when `global_row` is inside the window that starts at
/// `window_start` and spans `window_len` rows; otherwise returns `None`.
pub fn virtual_window_index(
    global_row: usize,
    window_start: usize,
    window_len: usize,
) -> Option<usize> {
    let local = global_row.checked_sub(window_start)?;
    (local < window_len).then_some(local)
}

#[cfg(test)]
mod tests {
    use super::{
        clamp_selected_row, default_widths, empty_fill_rows, resolved_column_count,
        virtual_window_index,
    };

    #[test]
    fn resolves_column_count_from_rows() {
        let headers = vec!["Name".to_string()];
        let rows = vec![vec!["A".to_string(), "B".to_string()]];
        assert_eq!(resolved_column_count(&headers, &rows), 2);
    }

    #[test]
    fn clamps_selection_to_existing_rows() {
        assert_eq!(clamp_selected_row(Some(2), 2), None);
        assert_eq!(clamp_selected_row(Some(1), 2), Some(1));
    }

    #[test]
    fn creates_default_widths() {
        assert_eq!(default_widths(3, 160.0), vec![160.0, 160.0, 160.0]);
    }

    #[test]
    fn computes_empty_fill_rows() {
        assert_eq!(empty_fill_rows(120.0, 40.0, 1), 2);
    }

    #[test]
    fn maps_global_row_into_virtual_window() {
        assert_eq!(virtual_window_index(100, 95, 10), Some(5));
        assert_eq!(virtual_window_index(95, 95, 10), Some(0));
        assert_eq!(virtual_window_index(104, 95, 10), Some(9));
    }

    #[test]
    fn returns_none_for_rows_outside_virtual_window() {
        assert_eq!(virtual_window_index(94, 95, 10), None);
        assert_eq!(virtual_window_index(105, 95, 10), None);
        assert_eq!(virtual_window_index(95, 95, 0), None);
    }
}
