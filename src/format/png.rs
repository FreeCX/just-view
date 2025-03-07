use log::debug;
use png::{Decoder, Transformations};

use super::Loader;
use crate::image::{ColorType, Image};

pub struct Png;

impl Loader for Png {
    fn load(data: &[u8]) -> Image {
        use png::ColorType::*;

        debug!("Use png loader");
        let mut decoder = Decoder::new(data);
        decoder.set_transformations(Transformations::normalize_to_color8());
        let mut reader = decoder.read_info().unwrap();
        let mut pixels = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut pixels).unwrap();

        debug!("Color_type = {:?}", info.color_type);
        let color_type = match info.color_type {
            Rgba | GrayscaleAlpha => ColorType::RGBA8,
            Grayscale | Rgb | Indexed => ColorType::RGB8,
        };

        Image { data: pixels, width: info.width, height: info.height, color_type }
    }
}
