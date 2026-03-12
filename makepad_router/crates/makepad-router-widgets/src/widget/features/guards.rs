use super::super::RouterWidget;

impl RouterWidget {
    pub(crate) fn guards_sync_enabled(&self) -> bool {
        self.cap_guards_sync
    }

    pub(crate) fn guards_async_enabled(&self) -> bool {
        self.cap_guards_async
    }
}
