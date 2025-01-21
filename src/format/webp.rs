use std::io::Cursor;

use image_webp::WebPDecoder;
use log::debug;

use super::Loader;
use crate::image::{ColorType, Image};

pub struct Webp;

impl Loader for Webp {
    fn load(data: &[u8]) -> Image {
        let cursor = Cursor::new(data);

        debug!("Use webp loader");
        let mut decoder = WebPDecoder::new(cursor).unwrap();
        let (width, height) = decoder.dimensions();
        let mut pixels = vec![0; decoder.output_buffer_size().unwrap()];
        decoder.read_image(&mut pixels).unwrap();

        let color_type = if decoder.has_alpha() { ColorType::RGBA } else { ColorType::RGB };

        Image {
            data: pixels,
            width,
            height,
            // TODO
            depth: 8,
            color_type,
        }
    }
}
