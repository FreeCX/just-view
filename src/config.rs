use crate::fs::{self, Filesystem};

#[derive(Default)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

#[derive(Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Default)]
pub struct Config {
    pub fullscreen: bool,
    pub last_size: Size,
    pub background: Color,
    pub filesystem: fs::Filesystem,
}

impl Color {
    pub fn black() -> Color {
        Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }
}

impl Config {
    pub fn new(fullscreen: bool, background: Color, fs: Filesystem) -> Config {
        Config { fullscreen, background, filesystem: fs, ..Default::default() }
    }
}
