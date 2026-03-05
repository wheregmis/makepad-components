pub use makepad_widgets;
pub use makepad_icon;

use makepad_widgets::*;

pub mod accordion;
pub mod alert;
pub mod avatar;
pub mod badge;
pub mod button;
pub mod checkbox;
pub mod theme;

pub fn script_mod(vm: &mut ScriptVm) {
    makepad_icon::script_mod(vm);
    crate::theme::script_mod(vm);
    crate::accordion::script_mod(vm);
    crate::alert::script_mod(vm);
    crate::avatar::script_mod(vm);
    crate::badge::script_mod(vm);
    crate::button::script_mod(vm);
    crate::checkbox::script_mod(vm);
}
