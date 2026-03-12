pub use makepad_icon;
pub use makepad_widgets;

use makepad_widgets::*;

pub mod accordion;
pub mod alert;
pub mod dialog;
pub use dialog::{
    ShadDialog, ShadDialogAction, ShadDialogRef, ShadDialogWidgetExt, ShadDialogWidgetRefExt,
};
pub use sonner::{
    ShadSonner, ShadSonnerAction, ShadSonnerRef, ShadSonnerWidgetExt, ShadSonnerWidgetRefExt,
};
pub mod aspect_ratio;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod button_group;
pub mod card;
pub mod carousel;
pub use carousel::{
    ShadCarousel, ShadCarouselAction, ShadCarouselRef, ShadCarouselWidgetExt,
    ShadCarouselWidgetRefExt,
};
pub mod checkbox;
pub mod collapsible;
pub mod context_menu;
pub use context_menu::{
    ShadContextMenuAction, ShadContextMenuRef, ShadContextMenuWidgetExt,
    ShadContextMenuWidgetRefExt,
};
pub mod hr;
pub mod input;
pub mod input_otp;
pub use input_otp::{
    ShadInputOtp, ShadInputOtpAction, ShadInputOtpRef, ShadInputOtpWidgetExt,
    ShadInputOtpWidgetRefExt,
};
pub mod kbd;
pub mod label;
pub mod pagination;
pub mod panel;
pub use pagination::{
    ShadPagination, ShadPaginationAction, ShadPaginationRef, ShadPaginationWidgetExt,
    ShadPaginationWidgetRefExt,
};
pub mod popover;
pub use popover::{
    ShadPopover, ShadPopoverAction, ShadPopoverRef, ShadPopoverWidgetExt, ShadPopoverWidgetRefExt,
};
pub mod progress;
pub mod radio_group;
pub mod resizable;
pub mod scroll;
pub mod select;
pub mod sheet;
pub use sheet::{
    ShadSheet, ShadSheetAction, ShadSheetRef, ShadSheetWidgetExt, ShadSheetWidgetRefExt,
};
pub mod sidebar;
pub mod skeleton;
pub mod slider;
pub mod sonner;
pub mod spinner;
pub mod switch;
pub mod tabs;
pub mod textarea;
pub mod theme;
pub mod toggle;

pub fn script_mod_without_theme(vm: &mut ScriptVm) {
    makepad_icon::script_mod(vm);
    crate::accordion::script_mod(vm);
    crate::alert::script_mod(vm);
    crate::aspect_ratio::script_mod(vm);
    crate::avatar::script_mod(vm);
    crate::badge::script_mod(vm);
    crate::breadcrumb::script_mod(vm);
    crate::tabs::script_mod(vm);
    crate::button::script_mod(vm);
    crate::button_group::script_mod(vm);
    crate::card::script_mod(vm);
    crate::carousel::script_mod(vm);
    crate::checkbox::script_mod(vm);
    crate::collapsible::script_mod(vm);
    crate::context_menu::script_mod(vm);
    crate::dialog::script_mod(vm);

    crate::skeleton::script_mod(vm);
    crate::kbd::script_mod(vm);
    crate::hr::script_mod(vm);
    crate::input::script_mod(vm);
    crate::input_otp::script_mod(vm);
    crate::label::script_mod(vm);
    crate::panel::script_mod(vm);
    crate::pagination::script_mod(vm);
    crate::popover::script_mod(vm);
    crate::progress::script_mod(vm);
    crate::radio_group::script_mod(vm);
    crate::resizable::script_mod(vm);
    crate::select::script_mod(vm);
    crate::sheet::script_mod(vm);
    crate::scroll::script_mod(vm);
    crate::sidebar::script_mod(vm);
    crate::slider::script_mod(vm);
    crate::sonner::script_mod(vm);
    crate::spinner::script_mod(vm);
    crate::switch::script_mod(vm);
    crate::textarea::script_mod(vm);
    crate::toggle::script_mod(vm);
}

pub fn script_mod(vm: &mut ScriptVm) {
    crate::theme::script_mod(vm);
    crate::script_mod_without_theme(vm);
}
