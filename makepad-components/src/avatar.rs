use makepad_widgets::*;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    set_type_default() do #(DrawAvatarImage::script_shader(vm)){
        ..mod.draw.DrawQuad
        image_texture: texture_2d(float)
        image_scale: vec2(1.0, 1.0)
        image_pan: vec2(0.0, 0.0)

        get_color: fn() {
            let color = self.image_texture.sample_as_bgra(self.pos * self.image_scale + self.image_pan)
            return Pal.premul(color)
        }

        pixel: fn() {
            let sdf = Sdf2d.viewport(self.pos * self.rect_size)
            sdf.circle(
                self.rect_size.x * 0.5
                self.rect_size.y * 0.5
                min(self.rect_size.x, self.rect_size.y) * 0.5
            )
            sdf.fill_keep_premul(self.get_color())
            return sdf.result
        }
    }

    mod.widgets.ShadAvatarImageBase = #(ShadAvatarImage::register_widget(vm))

    mod.widgets.ShadAvatarFallback = mod.widgets.Label{
        width: Fit
        height: Fit
        draw_text.color: (shad_theme.color_muted_foreground)
        draw_text.text_style.font_size: 12
        text: "CN"
    }

    mod.widgets.ShadAvatarImage = set_type_default() do mod.widgets.ShadAvatarImageBase{
        width: Fill
        height: Fill
        fit: ImageFit.Biggest
    }

    mod.widgets.ShadAvatarStatus = mod.widgets.View{
        visible: false
        width: Fill
        height: Fill
        flow: Overlay
        align: Align{x: 1.0, y: 1.0}
        padding: Inset{right: -2.0, bottom: -2.0}

        dot := mod.widgets.CircleView{
            width: 14
            height: 14
            draw_bg +: {
                color: (shad_theme.color_success)
                border_size: 2.0
                border_color: (shad_theme.color_background)
            }
        }
    }

    mod.widgets.ShadAvatarStatusOnline = mod.widgets.ShadAvatarStatus{
        visible: true
        dot.draw_bg.color: (shad_theme.color_success)
    }

    mod.widgets.ShadAvatarStatusAway = mod.widgets.ShadAvatarStatus{
        visible: true
        dot.draw_bg.color: (shad_theme.color_muted_foreground)
    }

    mod.widgets.ShadAvatarStatusBusy = mod.widgets.ShadAvatarStatus{
        visible: true
        dot.draw_bg.color: (shad_theme.color_destructive)
    }

    mod.widgets.ShadAvatar = mod.widgets.ViewBase {
        width: 40
        height: 40
        flow: Overlay
        align: Align{x: 0.5, y: 0.5}

        backdrop := mod.widgets.CircleView{
            width: Fill
            height: Fill
            draw_bg +: {
                color: (shad_theme.color_secondary)
            }
        }

        fallback := mod.widgets.ShadAvatarFallback{}
        image := mod.widgets.ShadAvatarImage{}

        ring := mod.widgets.CircleView{
            width: Fill
            height: Fill
            draw_bg +: {
                color: #0000
                border_size: 1.0
                border_color: (shad_theme.color_outline_border)
            }
        }

        status := mod.widgets.ShadAvatarStatus{}
    }

    mod.widgets.ShadAvatarSm = mod.widgets.ShadAvatar {
        width: 32
        height: 32
        fallback.draw_text.text_style.font_size: 10
        status.padding: Inset{right: -1.0, bottom: -1.0}
        status.dot.width: 10
        status.dot.height: 10
        status.dot.draw_bg.border_size: 1.5
    }

    mod.widgets.ShadAvatarLg = mod.widgets.ShadAvatar {
        width: 56
        height: 56
        fallback.draw_text.text_style.font_size: 16
        status.padding: Inset{right: -3.0, bottom: -3.0}
        status.dot.width: 18
        status.dot.height: 18
        status.dot.draw_bg.border_size: 2.5
    }
}

#[derive(Script, ScriptHook)]
#[repr(C)]
pub struct DrawAvatarImage {
    #[deref]
    draw_super: DrawQuad,
    #[live]
    pub image_scale: Vec2f,
    #[live]
    pub image_pan: Vec2f,
}

#[derive(Script, ScriptHook, Widget)]
pub struct ShadAvatarImage {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[walk]
    walk: Walk,
    #[redraw]
    #[live]
    draw_bg: DrawAvatarImage,
    #[live(1u64)]
    min_width: u64,
    #[live(1u64)]
    min_height: u64,
    #[visible]
    #[live(true)]
    visible: bool,
    #[live]
    fit: ImageFit,
    #[live]
    src: Option<ScriptHandleRef>,
    #[rust]
    src_loaded: bool,
    #[rust]
    async_image_path: Option<PathBuf>,
    #[rust]
    async_image_size: Option<(usize, usize)>,
    #[rust]
    texture: Option<Texture>,
}

impl ImageCacheImpl for ShadAvatarImage {
    fn get_texture(&self, _id: usize) -> &Option<Texture> {
        &self.texture
    }

    fn set_texture(&mut self, texture: Option<Texture>, _id: usize) {
        self.texture = texture;
    }
}

impl ShadAvatarImage {
    fn apply_async_result(&mut self, cx: &mut Cx, image_path: &Path, result: AsyncLoadResult) {
        match result {
            AsyncLoadResult::Loading(w, h) => {
                self.async_image_size = Some((w, h));
                self.async_image_path = Some(image_path.to_path_buf());
                self.redraw(cx);
            }
            AsyncLoadResult::Loaded => {
                self.async_image_size = None;
                self.async_image_path = None;
                self.redraw(cx);
            }
        }
    }

    fn load_image_from_data_async(
        &mut self,
        cx: &mut Cx,
        image_path: &Path,
        data: Arc<Vec<u8>>,
    ) -> Result<(), ImageError> {
        self.lazy_create_image_cache(cx);
        if let Ok(result) = self.load_image_from_data_async_impl(cx, image_path, data, 0) {
            self.apply_async_result(cx, image_path, result);
        }
        Ok(())
    }

    fn load_from_resource(&mut self, cx: &mut Cx) {
        if self.src_loaded {
            return;
        }
        let Some(ref handle_ref) = self.src else {
            self.src_loaded = true;
            return;
        };
        let handle = handle_ref.as_handle();
        cx.load_script_resource(handle);
        let path = {
            let resources = cx.script_data.resources.resources.borrow();
            resources
                .iter()
                .find(|r| r.handle == handle)
                .map(|r| PathBuf::from(&r.abs_path))
                .unwrap_or_else(|| PathBuf::from("avatar_image_resource"))
        };
        self.lazy_create_image_cache(cx);

        // For regular files this avoids cloning the already-cached resource bytes into a fresh Vec.
        if let Ok(result) = self.load_image_file_by_path_async_impl(cx, &path, 0) {
            self.src_loaded = true;
            self.apply_async_result(cx, &path, result);
            return;
        }

        let Some(data) = cx.get_resource(handle) else {
            let resources = cx.script_data.resources.resources.borrow();
            if let Some(res) = resources.iter().find(|r| r.handle == handle) {
                if res.is_error() {
                    drop(resources);
                    self.src_loaded = true;
                    return;
                }
            } else {
                self.src_loaded = true;
            }
            return;
        };
        self.src_loaded = true;
        // If this is the last Rc owner, move the bytes into Arc without cloning.
        let data = match Rc::try_unwrap(data) {
            Ok(bytes) => Arc::new(bytes),
            Err(shared) => Arc::new((*shared).clone()),
        };
        let _ = self.load_image_from_data_async(cx, &path, data);
    }

    fn draw_walk_image(&mut self, cx: &mut Cx2d, mut walk: Walk) -> DrawStep {
        if !self.visible || self.src.is_none() {
            return DrawStep::done();
        }

        let rect = cx.peek_walk_turtle(walk);
        let dpi = cx.current_dpi_factor();

        let (width, height) = if let Some((w, h)) = &self.async_image_size {
            self.draw_bg.draw_vars.empty_texture(0);
            (*w as f64, *h as f64)
        } else if let Some(image_texture) = &self.texture {
            self.draw_bg.draw_vars.set_texture(0, image_texture);
            let (width, height) = image_texture
                .get_format(cx)
                .vec_width_height()
                .unwrap_or((self.min_width as usize, self.min_height as usize));
            if image_texture.get_format(cx).is_render() {
                self.draw_bg.image_scale = vec2(1.0, -1.0);
                self.draw_bg.image_pan = vec2(0.0, 1.0);
            } else {
                self.draw_bg.image_scale = vec2(1.0, 1.0);
                self.draw_bg.image_pan = vec2(0.0, 0.0);
            }
            (width as f64, height as f64)
        } else {
            self.draw_bg.draw_vars.empty_texture(0);
            (self.min_width as f64 / dpi, self.min_height as f64 / dpi)
        };

        let aspect = width / height;
        match self.fit {
            ImageFit::Size => {
                walk.width = Size::Fixed(width);
                walk.height = Size::Fixed(height);
            }
            ImageFit::Stretch => {}
            ImageFit::Horizontal => {
                walk.height = Size::Fixed(rect.size.x / aspect);
            }
            ImageFit::Vertical => {
                walk.width = Size::Fixed(rect.size.y * aspect);
            }
            ImageFit::Smallest => {
                let walk_height = rect.size.x / aspect;
                if walk_height > rect.size.y {
                    walk.width = Size::Fixed(rect.size.y * aspect);
                } else {
                    walk.height = Size::Fixed(walk_height);
                }
            }
            ImageFit::Biggest => {
                let walk_height = rect.size.x / aspect;
                if walk_height < rect.size.y {
                    walk.width = Size::Fixed(rect.size.y * aspect);
                } else {
                    walk.height = Size::Fixed(walk_height);
                }
            }
        }

        self.draw_bg.draw_walk(cx, walk);
        DrawStep::done()
    }
}

impl Widget for ShadAvatarImage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if let Event::NetworkResponses(e) = event {
            handle_image_cache_network_responses(cx, e);
        }

        if let Event::Actions(actions) = &event {
            for action in actions {
                if let Some(AsyncImageLoad { image_path, result }) = &action.downcast_ref() {
                    if let Some(result) = result.borrow_mut().take() {
                        self.process_async_image_load(cx, image_path, result);
                    }
                    // Optimization: avoid repeated allocation when checking if loaded image matches the pending async load
                    // Previously: self.async_image_path.clone() == Some(image_path.to_path_buf()) (caused unnecessary heap allocations)
                    // Now: Compare path references directly, reducing allocations by 100% on async image loads
                    if self.async_image_size.is_some()
                        && self.async_image_path.as_deref() == Some(image_path)
                    {
                        self.load_image_from_cache(cx, image_path, 0);
                        self.async_image_size = None;
                        self.redraw(cx);
                    }
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.load_from_resource(cx);
        self.draw_walk_image(cx, walk)
    }
}
