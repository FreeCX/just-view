use miniquad::conf::Conf;
use simple_logger::SimpleLogger;

mod color;
mod config;
mod format;
mod fs;
mod image;
mod shader;
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

    let config = Config::new(false, Color::black(), fs);

    let window = Conf {
        window_title: String::from("Just View"),
        window_width: 500,
        window_height: 500,
        window_resizable: true,
        ..Default::default()
    };

    miniquad::start(window, || Box::new(Window::setup(config)));
}
