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

#[cfg(test)]
mod tests {
    use super::{clamp_selected_row, default_widths, empty_fill_rows, resolved_column_count};

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
}
