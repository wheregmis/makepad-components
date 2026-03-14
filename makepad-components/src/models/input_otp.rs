pub fn visible_cells(cell_count: u32, max_slots: usize) -> usize {
    cell_count.clamp(1, max_slots as u32) as usize
}

pub fn sanitize(value: &str, visible_cells: usize) -> String {
    value
        .chars()
        .filter(|c| c.is_ascii_digit())
        .take(visible_cells)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{sanitize, visible_cells};

    #[test]
    fn clamps_visible_cells() {
        assert_eq!(visible_cells(0, 6), 1);
        assert_eq!(visible_cells(8, 6), 6);
    }

    #[test]
    fn sanitizes_non_numeric_and_truncates() {
        assert_eq!(sanitize("1a2b3c4", 3), "123");
    }
}
