use miniquad::conf::Conf;
use simple_logger::SimpleLogger;

mod color;
mod config;
mod format;
mod fs;
mod image;
mod scale;
mod shader;
mod transform;
mod vertex;
mod window;

use crate::color::Color;
use crate::config::Config;
use crate::fs::Filesystem;
use crate::window::Window;

fn main() {
    SimpleLogger::new().with_level(log::LevelFilter::Debug).init().unwrap();

    let filename: Option<String> = std::env::args().skip(1).take(1).next();
    let fs = match filename {
        Some(file) => Filesystem::setup(&file),
        None => Filesystem::default(),
    };

    let config = Config::new(true, Color::black(), fs);
    let window = Conf {
        window_title: String::from("Just View"),
        window_width: config.last_size.w as i32,
        window_height: config.last_size.h as i32,
        window_resizable: true,
        fullscreen: config.fullscreen,
        ..Default::default()
    };

    miniquad::start(window, || Box::new(Window::setup(config)));
}
