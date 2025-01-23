use log::debug;
use png::Decoder;

use super::Loader;
use crate::image::{ColorType, Image};

pub struct Png;

impl Loader for Png {
    fn load(data: &[u8]) -> Image {
        debug!("Use png loader");
        let decoder = Decoder::new(data);
        let mut reader = decoder.read_info().unwrap();
        let mut pixels = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut pixels).unwrap();

        let color_type = match info.color_type {
            png::ColorType::Rgb => ColorType::RGB8,
            png::ColorType::Rgba => ColorType::RGBA8,
            // TODO
            other => panic!("Color type {other:?} not supported!"),
        };

        Image { data: pixels, width: info.width, height: info.height, color_type }
    }
}
