use super::super::RouterWidget;

impl RouterWidget {
    pub(crate) fn nested_enabled(&self) -> bool {
        self.cap_nested
    }
}
