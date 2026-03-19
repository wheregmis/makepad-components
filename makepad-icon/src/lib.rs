pub use makepad_widgets;

use makepad_widgets::*;

mod icons;

pub use icons::IconModule;

pub fn register_icons(vm: &mut ScriptVm, modules: &[IconModule]) {
    crate::icons::register_icons(vm, modules);
}

pub fn register_all_icons(vm: &mut ScriptVm) {
    crate::icons::register_all_icons(vm);
}

pub fn script_mod(vm: &mut ScriptVm) {
    crate::icons::register_all_icons(vm);
}
