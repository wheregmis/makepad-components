use makepad_components::makepad_widgets::*;

pub mod accordion_page;
pub mod alert_page;
pub mod button_page;
pub mod content_flip;
pub mod root;
pub mod sidebar;
pub mod themed_widgets;

pub fn script_mod(vm: &mut ScriptVm) {
    crate::ui::themed_widgets::script_mod(vm);
    crate::ui::sidebar::script_mod(vm);
    crate::ui::accordion_page::script_mod(vm);
    crate::ui::alert_page::script_mod(vm);
    crate::ui::button_page::script_mod(vm);
    crate::ui::content_flip::script_mod(vm);
    crate::ui::root::script_mod(vm);
}
