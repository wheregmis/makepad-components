pub struct PaginationWindow {
    pub pages: Vec<usize>,
    pub show_left_ellipsis: bool,
    pub show_right_ellipsis: bool,
}

pub fn normalized_page_count(page_count: u32) -> usize {
    page_count.max(1) as usize
}

pub fn clamped_current_page(current_page: u32, page_count: usize) -> usize {
    current_page.max(1).min(page_count as u32) as usize
}

pub fn clamped_max_visible_pages(
    max_visible_pages: u32,
    page_count: u32,
    max_page_buttons: usize,
) -> usize {
    max_visible_pages
        .clamp(3, max_page_buttons as u32)
        .min(page_count.max(1)) as usize
}

pub fn compute_window(
    current_page: usize,
    page_count: usize,
    max_visible_pages: usize,
) -> PaginationWindow {
    let max_visible = max_visible_pages.min(page_count);
    if page_count <= max_visible {
        return PaginationWindow {
            pages: (1..=page_count).collect(),
            show_left_ellipsis: false,
            show_right_ellipsis: false,
        };
    }

    let inner_window = max_visible.saturating_sub(2).max(1);
    let mut start = current_page.saturating_sub(inner_window / 2);
    if start < 2 {
        start = 2;
    }

    let mut end = start + inner_window - 1;
    let last_inner_page = page_count - 1;
    if end > last_inner_page {
        end = last_inner_page;
        start = end.saturating_sub(inner_window - 1).max(2);
    }

    let show_left_ellipsis = start > 2;
    let show_right_ellipsis = end < last_inner_page;

    let mut pages = Vec::with_capacity(max_visible);
    pages.push(1);
    for page in start..=end {
        pages.push(page);
    }
    pages.push(page_count);

    PaginationWindow {
        pages,
        show_left_ellipsis,
        show_right_ellipsis,
    }
}

#[cfg(test)]
mod tests {
    use super::{clamped_current_page, clamped_max_visible_pages, compute_window};

    #[test]
    fn clamps_current_page() {
        assert_eq!(clamped_current_page(0, 5), 1);
        assert_eq!(clamped_current_page(9, 5), 5);
    }

    #[test]
    fn clamps_visible_page_count() {
        assert_eq!(clamped_max_visible_pages(1, 10, 7), 3);
        assert_eq!(clamped_max_visible_pages(9, 4, 7), 4);
    }

    #[test]
    fn computes_window_with_ellipses() {
        let window = compute_window(5, 10, 7);
        assert_eq!(window.pages, vec![1, 3, 4, 5, 6, 7, 10]);
        assert!(window.show_left_ellipsis);
        assert!(window.show_right_ellipsis);
    }
}
