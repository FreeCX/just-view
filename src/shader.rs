use miniquad::{ShaderMeta, UniformBlockLayout, UniformDesc, UniformType};

pub const VERTEX: &str = r#"#version 100
    attribute vec2 in_pos;
    attribute vec2 in_uv;

    uniform vec2 aspect;
    uniform vec2 offset;

    varying lowp vec2 uv;

    void main() {
        gl_Position = vec4(in_pos * aspect + offset, 0, 1);
        uv = in_uv;
    }"#;

pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 uv;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = texture2D(tex, uv);
    }"#;

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("aspect", UniformType::Float2),
                UniformDesc::new("offset", UniformType::Float2),
            ],
        },
    }
}

#[repr(C)]
pub struct Uniforms {
    pub aspect: (f32, f32),
    pub offset: (f32, f32),
}
