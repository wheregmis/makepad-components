pub fn normalize_index(index: usize, slide_count: usize) -> Option<usize> {
    if slide_count == 0 || index >= slide_count {
        None
    } else {
        Some(index)
    }
}

pub fn next_index(current: usize, slide_count: usize) -> usize {
    if slide_count == 0 || current + 1 >= slide_count {
        0
    } else {
        current + 1
    }
}

pub fn prev_index(current: usize, slide_count: usize) -> usize {
    if slide_count == 0 {
        0
    } else if current == 0 {
        slide_count - 1
    } else {
        current - 1
    }
}

#[cfg(test)]
mod tests {
    use super::{next_index, normalize_index, prev_index};

    #[test]
    fn normalizes_bounds() {
        assert_eq!(normalize_index(1, 3), Some(1));
        assert_eq!(normalize_index(3, 3), None);
    }

    #[test]
    fn wraps_forward() {
        assert_eq!(next_index(2, 3), 0);
    }

    #[test]
    fn wraps_backward() {
        assert_eq!(prev_index(0, 3), 2);
    }
}
