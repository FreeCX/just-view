use log::debug;
use miniquad::{
    Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, EventHandler, GlContext, KeyCode, KeyMods,
    PassAction, Pipeline, PipelineParams, RenderingBackend, ShaderSource, TextureParams, UniformsSource,
    VertexAttribute, VertexFormat, window,
};

use crate::config::Config;
use crate::image::Size;
use crate::scale::Scale;
use crate::shader;
use crate::vertex::{Vec2, Vertex};

pub struct Window {
    ctx: Box<GlContext>,
    pipeline: Pipeline,
    bindings: Bindings,

    image: Option<Size>,
    config: Config,
    scale: Scale,
}

impl Window {
    pub fn setup(config: Config) -> Window {
        let mut ctx = Box::new(GlContext::new());

        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -1.0, y: -1.0 }, uv: Vec2 { x: 0.0, y: 1.0 } },
            Vertex { pos : Vec2 { x:  1.0, y: -1.0 }, uv: Vec2 { x: 1.0, y: 1.0 } },
            Vertex { pos : Vec2 { x:  1.0, y:  1.0 }, uv: Vec2 { x: 1.0, y: 0.0 } },
            Vertex { pos : Vec2 { x: -1.0, y:  1.0 }, uv: Vec2 { x: 0.0, y: 0.0 } },
        ];
        let vertex_buffer =
            ctx.new_buffer(BufferType::VertexBuffer, BufferUsage::Immutable, BufferSource::slice(&vertices));

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer =
            ctx.new_buffer(BufferType::IndexBuffer, BufferUsage::Immutable, BufferSource::slice(&indices));

        let bindings = Bindings { vertex_buffers: vec![vertex_buffer], index_buffer, images: vec![] };

        let shader = ctx
            .new_shader(ShaderSource::Glsl { vertex: shader::VERTEX, fragment: shader::FRAGMENT }, shader::meta())
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("in_uv", VertexFormat::Float2),
            ],
            shader,
            PipelineParams::default(),
        );

        Window { ctx, pipeline, bindings, config, image: None, scale: Scale::new() }
    }

    fn trigger_fullscreen(&mut self) {
        if !self.config.fullscreen {
            let (w, h) = window::screen_size();
            debug!("Save current window size: {w}x{h}");
            self.config.last_size = Size { w: w as u32, h: h as u32 };
        }

        self.config.fullscreen = !self.config.fullscreen;
        window::set_fullscreen(self.config.fullscreen);

        if !self.config.fullscreen {
            let (w, h) = (self.config.last_size.w, self.config.last_size.h);
            debug!("Restore last window size: {w}x{h}");
            window::set_window_size(w, h);
        }
    }

    fn texture_from_image(&mut self) {
        let image = self.config.filesystem.data();
        let params = TextureParams::from(&image);
        self.bindings.images = vec![self.ctx.new_texture_from_data_and_format(&image.data, params)];
        self.image = Some(Size { w: image.width, h: image.height });
    }
}

impl EventHandler for Window {
    fn update(&mut self) {
        // простое кэширование
        self.config.filesystem.cache();

        // начальная загрузка изображения
        if self.image.is_none() {
            self.texture_from_image();
        }
    }

    fn draw(&mut self) {
        let (w, h) = window::screen_size();
        let aspect = if let Some(img) = &self.image {
            self.scale.calc_aspect(img.w as f32, img.h as f32, w, h)
        } else {
            (1.0, 1.0)
        };

        self.ctx.begin_pass(
            None,
            PassAction::Clear { color: Some(self.config.background.unpack()), depth: None, stencil: None },
        );

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        self.ctx.apply_uniforms(UniformsSource::table(&shader::Uniforms { aspect, offset: (0.0, 0.0) }));
        self.ctx.draw(0, 6, 1);
        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }

    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Escape => window::quit(),
            KeyCode::Delete => debug!("todo: Delete"),
            KeyCode::Equal => {
                self.scale.zoom_in();
                debug!("Zoom in: {:.0} %", self.scale.zoom());
            }
            KeyCode::Minus => {
                self.scale.zoom_out();
                debug!("Zoom out: {:.0} %", self.scale.zoom());
            }
            KeyCode::Up => {
                if self.config.filesystem.first() {
                    debug!("↑ First image");
                    self.texture_from_image();
                }
            }
            KeyCode::Down => {
                if self.config.filesystem.last() {
                    debug!("↓ Last image");
                    self.texture_from_image();
                }
            }
            KeyCode::Left => {
                if self.config.filesystem.prev() {
                    debug!("← Previous image");
                    self.texture_from_image();
                }
            }
            KeyCode::Right => {
                if self.config.filesystem.next() {
                    debug!("→ Next image");
                    self.texture_from_image();
                }
            }
            KeyCode::F => self.trigger_fullscreen(),
            KeyCode::I => debug!("todo: i"),
            KeyCode::R => {
                self.scale.zoom_reset();
                debug!("Zoom reset: {:.0} %", self.scale.zoom());
            }
            _ => (),
        };
    }
}
