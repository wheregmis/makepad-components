use makepad_components::makepad_widgets::*;

pub mod accordion_page;
pub mod alert_page;
pub mod aspect_ratio_page;
pub mod avatar_page;
pub mod badge_page;
pub mod breadcrumb_page;
pub mod button_group_page;
pub mod button_page;
pub mod checkbox_page;
pub mod collapsible_page;
pub mod content_flip;
pub mod input_page;
pub mod label_page;
pub mod root;
pub mod sidebar;
pub mod sidebar_page;
pub mod themed_widgets;

pub fn script_mod(vm: &mut ScriptVm) {
    crate::ui::themed_widgets::script_mod(vm);
    crate::ui::sidebar::script_mod(vm);
    crate::ui::accordion_page::script_mod(vm);
    crate::ui::alert_page::script_mod(vm);
    crate::ui::aspect_ratio_page::script_mod(vm);
    crate::ui::avatar_page::script_mod(vm);
    crate::ui::badge_page::script_mod(vm);
    crate::ui::breadcrumb_page::script_mod(vm);
    crate::ui::button_page::script_mod(vm);
    crate::ui::button_group_page::script_mod(vm);
    crate::ui::checkbox_page::script_mod(vm);
    crate::ui::collapsible_page::script_mod(vm);
    crate::ui::input_page::script_mod(vm);
    crate::ui::label_page::script_mod(vm);
    crate::ui::sidebar_page::script_mod(vm);
    crate::ui::content_flip::script_mod(vm);
    crate::ui::root::script_mod(vm);
}
