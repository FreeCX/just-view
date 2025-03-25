use std::io::Cursor;

use anyhow::Context;
use image_webp::WebPDecoder;
use log::debug;

use super::Loader;
use crate::image::{ColorType, Image};

pub struct Webp;

impl Loader for Webp {
    fn load(data: &[u8]) -> anyhow::Result<Image> {
        let cursor = Cursor::new(data);

        debug!("Use webp loader");
        let mut decoder = WebPDecoder::new(cursor)?;
        let (width, height) = decoder.dimensions();
        let mut pixels = vec![0; decoder.output_buffer_size().with_context(|| format!("Missing buffer size info"))?];
        decoder.read_image(&mut pixels)?;
        let color_type = if decoder.has_alpha() { ColorType::RGBA8 } else { ColorType::RGB8 };

        Ok(Image { data: pixels, width, height, color_type })
    }
}
