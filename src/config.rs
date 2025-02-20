use crate::color::Color;
use crate::fs::Filesystem;
use crate::image::Size;

pub struct Config {
    pub fullscreen: bool,
    pub last_size: Size,
    pub background: Color,
    pub filesystem: Filesystem,
}

impl Config {
    pub fn new(fullscreen: bool, background: Color, filesystem: Filesystem) -> Config {
        Config { fullscreen, background, filesystem, last_size: Size { w: 0, h: 0 } }
    }
}
