pub use makepad_widgets;

use makepad_widgets::*;

mod icons;

pub use icons::{IconMetadata, ICON_METADATA};

pub fn script_mod(vm: &mut ScriptVm) {
    crate::icons::script_mod(vm);
}
