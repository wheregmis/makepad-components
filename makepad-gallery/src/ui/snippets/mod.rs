use crate::ui::catalog::{self, GallerySnippetKey};
use makepad_components::makepad_widgets::*;

pub mod components;
pub mod data;
pub mod feedback;
pub mod forms;
pub mod layout;
pub mod navigation;
pub mod overlays;

pub use components::{
    ACCORDION_PREVIEW_CODE, AVATAR_PREVIEW_CODE, BADGE_PREVIEW_CODE, BUTTON_GROUP_PREVIEW_CODE,
    BUTTON_PREVIEW_CODE, CARD_PREVIEW_CODE, CAROUSEL_PREVIEW_CODE, COLLAPSIBLE_PREVIEW_CODE,
};
pub use data::{CHART_PREVIEW_CODE, TABLE_PREVIEW_CODE};
pub use feedback::{
    ALERT_PREVIEW_CODE, PROGRESS_PREVIEW_CODE, SKELETON_PREVIEW_CODE, SONNER_PREVIEW_CODE,
    SPINNER_PREVIEW_CODE,
};
pub use forms::{
    CALENDAR_PREVIEW_CODE, CHECKBOX_PREVIEW_CODE, DATE_PICKER_PREVIEW_CODE, INPUT_OTP_PREVIEW_CODE,
    INPUT_PREVIEW_CODE, KBD_PREVIEW_CODE, LABEL_PREVIEW_CODE, RADIO_GROUP_PREVIEW_CODE,
    SELECT_PREVIEW_CODE, SLIDER_PREVIEW_CODE, SWITCH_PREVIEW_CODE, TEXTAREA_PREVIEW_CODE,
    TOGGLE_PREVIEW_CODE,
};
pub use layout::{
    ASPECT_RATIO_PREVIEW_CODE, RESIZABLE_PREVIEW_CODE, SCROLL_AREA_PREVIEW_CODE,
    SEPARATOR_PREVIEW_CODE,
};
pub use navigation::{
    BREADCRUMB_PREVIEW_CODE, COMMAND_PALETTE_PREVIEW_CODE, MENUBAR_PREVIEW_CODE,
    NAVIGATION_MENU_PREVIEW_CODE, PAGINATION_PREVIEW_CODE, SIDEBAR_PREVIEW_CODE, TABS_PREVIEW_CODE,
};
pub use overlays::{
    CONTEXT_MENU_PREVIEW_CODE, DIALOG_PREVIEW_CODE, POPOVER_PREVIEW_CODE, SHEET_PREVIEW_CODE,
};

pub fn snippet_code(key: GallerySnippetKey) -> &'static str {
    match key {
        GallerySnippetKey::Accordion => ACCORDION_PREVIEW_CODE,
        GallerySnippetKey::Alert => ALERT_PREVIEW_CODE,
        GallerySnippetKey::AspectRatio => ASPECT_RATIO_PREVIEW_CODE,
        GallerySnippetKey::Avatar => AVATAR_PREVIEW_CODE,
        GallerySnippetKey::Badge => BADGE_PREVIEW_CODE,
        GallerySnippetKey::Breadcrumb => BREADCRUMB_PREVIEW_CODE,
        GallerySnippetKey::Button => BUTTON_PREVIEW_CODE,
        GallerySnippetKey::ButtonGroup => BUTTON_GROUP_PREVIEW_CODE,
        GallerySnippetKey::Calendar => CALENDAR_PREVIEW_CODE,
        GallerySnippetKey::Card => CARD_PREVIEW_CODE,
        GallerySnippetKey::Carousel => CAROUSEL_PREVIEW_CODE,
        GallerySnippetKey::Chart => CHART_PREVIEW_CODE,
        GallerySnippetKey::Checkbox => CHECKBOX_PREVIEW_CODE,
        GallerySnippetKey::Collapsible => COLLAPSIBLE_PREVIEW_CODE,
        GallerySnippetKey::CommandPalette => COMMAND_PALETTE_PREVIEW_CODE,
        GallerySnippetKey::ContextMenu => CONTEXT_MENU_PREVIEW_CODE,
        GallerySnippetKey::DatePicker => DATE_PICKER_PREVIEW_CODE,
        GallerySnippetKey::Dialog => DIALOG_PREVIEW_CODE,
        GallerySnippetKey::Input => INPUT_PREVIEW_CODE,
        GallerySnippetKey::InputOtp => INPUT_OTP_PREVIEW_CODE,
        GallerySnippetKey::Kbd => KBD_PREVIEW_CODE,
        GallerySnippetKey::Label => LABEL_PREVIEW_CODE,
        GallerySnippetKey::Menubar => MENUBAR_PREVIEW_CODE,
        GallerySnippetKey::NavigationMenu => NAVIGATION_MENU_PREVIEW_CODE,
        GallerySnippetKey::Pagination => PAGINATION_PREVIEW_CODE,
        GallerySnippetKey::Popover => POPOVER_PREVIEW_CODE,
        GallerySnippetKey::Progress => PROGRESS_PREVIEW_CODE,
        GallerySnippetKey::RadioGroup => RADIO_GROUP_PREVIEW_CODE,
        GallerySnippetKey::Resizable => RESIZABLE_PREVIEW_CODE,
        GallerySnippetKey::ScrollArea => SCROLL_AREA_PREVIEW_CODE,
        GallerySnippetKey::Select => SELECT_PREVIEW_CODE,
        GallerySnippetKey::Separator => SEPARATOR_PREVIEW_CODE,
        GallerySnippetKey::Sheet => SHEET_PREVIEW_CODE,
        GallerySnippetKey::Sidebar => SIDEBAR_PREVIEW_CODE,
        GallerySnippetKey::Skeleton => SKELETON_PREVIEW_CODE,
        GallerySnippetKey::Slider => SLIDER_PREVIEW_CODE,
        GallerySnippetKey::Sonner => SONNER_PREVIEW_CODE,
        GallerySnippetKey::Spinner => SPINNER_PREVIEW_CODE,
        GallerySnippetKey::Switch => SWITCH_PREVIEW_CODE,
        GallerySnippetKey::Table => TABLE_PREVIEW_CODE,
        GallerySnippetKey::Tabs => TABS_PREVIEW_CODE,
        GallerySnippetKey::Textarea => TEXTAREA_PREVIEW_CODE,
        GallerySnippetKey::Toggle => TOGGLE_PREVIEW_CODE,
    }
}

pub fn snippet_code_for_page(page: LiveId) -> &'static str {
    catalog::entry_for_page(page)
        .map(|entry| snippet_code(entry.snippet))
        .unwrap_or("")
}
