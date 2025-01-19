use std::fs;

use miniquad::{
    window, Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, EventHandler, GlContext, KeyCode, Pipeline,
    PipelineParams, RenderingBackend, ShaderSource, TextureParams, UniformsSource, VertexAttribute, VertexFormat,
};

use crate::shader;
use crate::vertex::{Vec2, Vertex};
use crate::{
    config::{Config, Size},
    format,
};

pub struct Window {
    ctx: Box<GlContext>,
    pipeline: Pipeline,
    bindings: Bindings,

    img: Size,

    config: Config,
}

impl Window {
    pub fn new(config: Config) -> Window {
        let mut ctx = Box::new(GlContext::new());

        // TODO: сохранять пропорции изображения
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

        // TODO
        let file = "demo.png";
        let mime_type = mime_guess::MimeGuess::from_path(file).first().unwrap();
        let data = fs::read(file).unwrap();
        let img = format::load_by_mime(&mime_type.subtype(), &data);
        let params = TextureParams {
            kind: miniquad::TextureKind::Texture2D,
            format: miniquad::TextureFormat::RGB8,
            wrap: miniquad::TextureWrap::Clamp,
            min_filter: miniquad::FilterMode::Linear,
            mag_filter: miniquad::FilterMode::Linear,
            mipmap_filter: miniquad::MipmapFilterMode::None,
            width: img.width,
            height: img.height,
            allocate_mipmaps: false,
            sample_count: 0,
        };
        let img_info = Size { w: img.width, h: img.height };
        let texture = ctx.new_texture_from_data_and_format(&img.data, params);

        let bindings = Bindings { vertex_buffers: vec![vertex_buffer], index_buffer, images: vec![texture] };

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

        Window { ctx, pipeline, bindings, img: img_info, config }
    }

    fn trigger_fullscreen(&mut self) {
        if !self.config.fullscreen {
            let (w, h) = window::screen_size();
            println!("Save current window size: {w}x{h}");
            self.config.last_size = Size { w: w as u32, h: h as u32 };
        }

        self.config.fullscreen = !self.config.fullscreen;
        window::set_fullscreen(self.config.fullscreen);

        if !self.config.fullscreen {
            let (w, h) = (self.config.last_size.w, self.config.last_size.h);
            println!("Restore last window size: {w}x{h}");
            window::set_window_size(w, h);
        }
    }
}

impl EventHandler for Window {
    fn update(&mut self) {}

    fn draw(&mut self) {
        let (w, h) = window::screen_size();

        let img_aspect = (w / self.img.w as f32).min(h / self.img.h as f32);
        let ix = (self.img.w as f32 * img_aspect) / w;
        let iy = (self.img.h as f32 * img_aspect) / h;

        self.ctx.begin_default_pass(Default::default());

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        self.ctx.apply_uniforms(UniformsSource::table(&shader::Uniforms { aspect: (ix, iy) }));
        self.ctx.draw(0, 6, 1);
        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }

    fn key_down_event(&mut self, keycode: miniquad::KeyCode, _keymods: miniquad::KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Escape => window::quit(),
            KeyCode::Delete => println!("todo: Delete"),
            KeyCode::Equal => println!("todo: +"),
            KeyCode::Minus => println!("todo: -"),
            KeyCode::Left => println!("todo: ←"),
            KeyCode::Right => println!("todo: →"),
            KeyCode::F => self.trigger_fullscreen(),
            KeyCode::I => println!("todo: i"),
            KeyCode::R => println!("todo: r"),
            _ => (),
        };
    }
}
