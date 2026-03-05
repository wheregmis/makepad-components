pub use makepad_widgets;

use makepad_widgets::*;

mod icons;

pub fn script_mod(vm: &mut ScriptVm) {
    crate::icons::script_mod(vm);
}
