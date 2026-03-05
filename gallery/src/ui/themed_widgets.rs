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

    mod.widgets.GalleryCodeSnippetBase = #(GalleryCodeSnippet::register_widget(vm))

    mod.widgets.GalleryCodeSnippet = set_type_default() do mod.widgets.GalleryCodeSnippetBase{
        width: Fill
        height: Fit
        code: ""
        markdown := Markdown{
            width: Fill
            height: Fit
            margin: Inset{top: 4}
            body: ""
            use_code_block_widget: false
            paragraph_spacing: 8
            pre_code_spacing: 8

            draw_text.color: (shad_theme.color_muted_foreground)

            splash_block := SolidView{
                width: Fill
                height: Fit
                flow: Overlay
                margin: Inset{bottom: 8}
                padding: Inset{top: 8, right: 8, bottom: 8, left: 8}
                draw_bg.color: (shad_theme.color_secondary)

                splash_view := Splash{
                    width: Fill
                    height: Fit
                }
            }
        }
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
    rendered_markdown: String,
}

impl GalleryCodeSnippet {
    fn build_markdown(&self) -> String {
        let source = self.code.as_ref().trim();
        if source.is_empty() {
            return String::new();
        }
        format!("```runsplash\n{source}\n```\n\n```rust\n{source}\n```")
    }

    fn sync_markdown(&mut self, cx: &mut Cx) {
        let markdown = self.build_markdown();
        if markdown != self.rendered_markdown {
            let mut md = self.view.markdown(cx, ids!(markdown));
            md.set_text(cx, &markdown);
            self.rendered_markdown = markdown;
        }
    }
}

impl Widget for GalleryCodeSnippet {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.sync_markdown(cx);
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
