pub use makepad_icon;
pub use makepad_widgets;

use makepad_widgets::*;

pub mod accordion;
pub mod alert;
pub mod aspect_ratio;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod button_group;
pub mod checkbox;
pub mod collapsible;
pub mod input;
pub mod label;
pub mod sidebar;
pub mod theme;

pub fn script_mod(vm: &mut ScriptVm) {
    makepad_icon::script_mod(vm);
    crate::theme::script_mod(vm);
    crate::accordion::script_mod(vm);
    crate::alert::script_mod(vm);
    crate::aspect_ratio::script_mod(vm);
    crate::avatar::script_mod(vm);
    crate::badge::script_mod(vm);
    crate::breadcrumb::script_mod(vm);
    crate::button::script_mod(vm);
    crate::button_group::script_mod(vm);
    crate::checkbox::script_mod(vm);
    crate::collapsible::script_mod(vm);
    crate::input::script_mod(vm);
    crate::label::script_mod(vm);
    crate::sidebar::script_mod(vm);
}
