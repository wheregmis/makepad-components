use makepad_components::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.GalleryHr = Hr{
        draw_bg.color: (shad_theme.color_outline_border)
    }

    mod.widgets.GalleryCheckBox = CheckBox{
        draw_text.color: (shad_theme.color_primary)
        draw_text.color_hover: (shad_theme.color_primary)
        draw_text.text_style.font_size: 10
        draw_bg.color: (shad_theme.color_muted_foreground)
        draw_bg.color_hover: (shad_theme.color_secondary_hover)
    }

    mod.widgets.GalleryToggle = Toggle{
        draw_text.color: (shad_theme.color_primary)
        draw_text.text_style.font_size: 10
    }

    mod.widgets.GalleryPreviewTabButton = ButtonFlat{
        width: Fit
        height: Fit
        margin: Inset{}
        padding: Inset{}

        draw_bg +: {
            color: #0000
            color_hover: #0000
            color_down: #0000
            color_focus: #0000
            border_size: 0.0
        }

        draw_text +: {
            color: (shad_theme.color_muted_foreground)
            color_hover: (shad_theme.color_primary)
            color_down: (shad_theme.color_primary)
            color_focus: (shad_theme.color_primary)
            text_style.font_size: 10
        }
    }

    mod.widgets.GalleryCodeSnippetBase = #(GalleryCodeSnippet::register_widget(vm))

    mod.widgets.GalleryCodeSnippet = set_type_default() do mod.widgets.GalleryCodeSnippetBase{
        width: Fill
        height: Fit
        code: ""
        markdown := Markdown{
            width: Fill
            height: Fit
            margin: Inset{top: 4, bottom: 8}
            body: ""
            use_code_block_widget: false
            paragraph_spacing: 8
            pre_code_spacing: 8
            draw_text.color: (shad_theme.color_muted_foreground)
        }
    }

    mod.widgets.GalleryPreviewPanel = SolidView{
        width: Fill
        height: Fit
        padding: Inset{top: 20, right: 20, bottom: 20, left: 20}
        draw_bg.color: #0000
        draw_bg.border_size: 1.0
        draw_bg.border_color: (shad_theme.color_outline_border)
        draw_bg.border_radius: (shad_theme.radius)
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct GalleryCodeSnippet {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[live]
    code: ArcStringMut,
    #[rust]
    last_code: String,
}

impl GalleryCodeSnippet {
    fn build_markdown(&self) -> String {
        let source = self.code.as_ref().trim();
        if source.is_empty() {
            return String::new();
        }

        format!("```rust\n{source}\n```")
    }

    fn sync_markdown(&mut self, cx: &mut Cx) {
        let current_code = self.code.as_ref().trim();
        if current_code != self.last_code {
            self.last_code = current_code.to_string();
            let markdown = self.build_markdown();
            self.view.markdown(cx, ids!(markdown)).set_text(cx, &markdown);
        }
    }
}

impl Widget for GalleryCodeSnippet {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.sync_markdown(cx);
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.sync_markdown(cx);
        self.view.draw_walk(cx, scope, walk)
    }
}
