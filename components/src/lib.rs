pub use makepad_widgets;

use makepad_widgets::*;

pub mod accordion;
pub mod button;
pub mod theme;

pub fn script_mod(vm: &mut ScriptVm) {
    crate::theme::script_mod(vm);
    crate::accordion::script_mod(vm);
    crate::button::script_mod(vm);
}
