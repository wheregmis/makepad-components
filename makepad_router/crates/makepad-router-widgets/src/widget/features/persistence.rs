use super::super::RouterWidget;

impl RouterWidget {
    pub(crate) fn persistence_enabled(&self) -> bool {
        self.cap_persistence
    }
}
