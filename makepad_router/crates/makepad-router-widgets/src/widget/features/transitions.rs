use super::super::RouterWidget;

impl RouterWidget {
    pub(crate) fn transitions_enabled(&self) -> bool {
        self.cap_transitions
    }
}
