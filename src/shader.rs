use miniquad::{ShaderMeta, UniformBlockLayout, UniformDesc, UniformType};

pub const VERTEX: &str = r#"#version 100
    attribute vec2 in_pos;
    attribute vec2 in_uv;

    uniform vec2 aspect;

    varying lowp vec2 texcoord;

    void main() {
        gl_Position = vec4(in_pos * aspect, 0, 1);
        texcoord = in_uv;
    }"#;

pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }"#;

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout { uniforms: vec![UniformDesc::new("aspect", UniformType::Float2)] },
    }
}

#[repr(C)]
pub struct Uniforms {
    pub aspect: (f32, f32),
}
