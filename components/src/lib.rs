pub use makepad_widgets;
pub use makepad_icon;

use makepad_widgets::*;

pub mod accordion;
pub mod button;

pub fn script_mod(vm: &mut ScriptVm) {
    makepad_icon::script_mod(vm);
    crate::accordion::script_mod(vm);
    crate::button::script_mod(vm);
}
