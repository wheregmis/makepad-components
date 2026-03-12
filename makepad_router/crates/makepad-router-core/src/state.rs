#![allow(clippy::question_mark)]

use crate::navigation::NavigationHistory;
use makepad_micro_serde::*;

/// Serializable router state (history + optional URL override).
#[allow(clippy::question_mark)]
#[derive(Clone, Debug, Default, PartialEq, Eq, SerBin, DeBin, SerRon, DeRon)]
pub struct RouterState {
    /// Navigation history stack.
    pub history: NavigationHistory,
    /// Optional URL/path override used when displaying the not-found route.
    pub url_path_override: Option<String>,
}
