use makepad_components::button::ShadButtonWidgetExt;
use makepad_components::sonner::{ShadSonnerWidgetRefExt, SonnerItem, SonnerKind};
use makepad_widgets::*;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    load_all_resources() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.inner_size: vec2(980, 720)
                window.title: "Toast Example"
                pass +: {
                    clear_color: (shad_theme.color_background)
                }
                body +: {
                    flow: Overlay
                    width: Fill
                    height: Fill
                    View{
                        spacing: 16.0
                        open_btn_with_close := ShadButton{text: "Open toast with close"}
                        open_btn_no_close := ShadButton{text: "Open toast without close"}


                    }
                    toast := ShadSonner{
                        width: Fill
                        height: Fill
                        open: false
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self
            .ui
            .shad_button(cx, ids!(open_btn_with_close))
            .clicked(actions)
        {
            let sonner = self.ui.shad_sonner(cx, ids!(toast));
            sonner.enqueue(
                cx,
                SonnerItem {
                    title: "上传失败".to_string(),
                    description: Some("服务器拒绝了请求，请重试。".to_string()),
                    kind: SonnerKind::Error,
                    duration: Some(3.0),
                    show_close: true,
                },
            );
        }
        if self.ui.shad_button(cx, ids!(open_btn_no_close)).clicked(actions) {
            let sonner = self.ui.shad_sonner(cx, ids!(toast));
            sonner.enqueue(
                cx,
                SonnerItem {
                    title: "连接成功".to_string(),
                    description: Some("服务器连接成功。".to_string()),
                    kind: SonnerKind::Success,
                    duration: Some(3.0),
                    show_close: false,
                },
            );
        }
    }
}
impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        makepad_widgets::script_mod(vm);
        makepad_components::theme::script_mod(vm);
        script_eval!(vm, {
            mod.widgets.shad_theme = mod.widgets.shad_themes.dark
        });
        makepad_components::script_mod_without_theme(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
