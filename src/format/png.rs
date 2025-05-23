use log::debug;
use png::{Decoder, Transformations};

use super::Loader;
use crate::{
    image::{ColorType, Image},
    transform::grayscale_to_rgb,
};

pub struct Png;

impl Loader for Png {
    fn load(data: &[u8]) -> anyhow::Result<Image> {
        use png::ColorType::*;

        debug!("Use png loader");

        let mut decoder = Decoder::new(data);
        decoder.set_transformations(Transformations::normalize_to_color8());
        let mut reader = decoder.read_info()?;
        let mut pixels = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut pixels)?;

        debug!("Color_type = {:?}", info.color_type);
        let color_type = match info.color_type {
            Rgba | GrayscaleAlpha => ColorType::RGBA8,
            Rgb | Indexed => ColorType::RGB8,
            Grayscale => {
                pixels = grayscale_to_rgb(pixels);
                ColorType::RGB8
            }
        };

        Ok(Image { data: pixels, width: info.width, height: info.height, color_type })
    }
}
