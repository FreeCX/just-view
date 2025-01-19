use miniquad::conf::Conf;

mod config;
mod format;
mod fs;
mod image;
mod shader;
mod vertex;
mod window;

use crate::config::{Color, Config};
use crate::fs::Filesystem;
use crate::window::Window;

fn main() {
    let filename: Option<String> = std::env::args().skip(1).take(1).next();

    let fs = match filename {
        Some(file) => Filesystem::load(&file),
        None => Filesystem::default(),
    };

    let config = Config::new(false, Color::black(), fs);

    let window = Conf {
        window_title: String::from("Just Viewer"),
        window_width: 500,
        window_height: 500,
        window_resizable: true,
        ..Default::default()
    };

    miniquad::start(window, || Box::new(Window::new(config)));
}
