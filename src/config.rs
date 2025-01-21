use crate::color::Color;
use crate::fs::Filesystem;
use crate::image::Size;

pub struct Config {
    pub fullscreen: bool,
    pub zoom: f32,
    pub last_size: Size,
    pub background: Color,
    pub filesystem: Filesystem,
}

impl Config {
    pub fn new(fullscreen: bool, background: Color, filesystem: Filesystem) -> Config {
        Config { fullscreen, background, filesystem, zoom: 1.0, last_size: Size { w: 0, h: 0 } }
    }
}
