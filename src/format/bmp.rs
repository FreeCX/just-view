use log::debug;
use zune_bmp::BmpDecoder;

use super::Loader;
use crate::image::{ColorType, Image};

pub struct Bmp;

impl Loader for Bmp {
    fn load(data: &[u8]) -> Image {
        debug!("Use bmp loader");
        let mut decoder = BmpDecoder::new(data);
        decoder.decode_headers().unwrap();

        let colorspace = decoder.get_colorspace().unwrap();
        let (width, height) = decoder.get_dimensions().unwrap();
        let mut pixels = vec![0; decoder.output_buf_size().unwrap()];
        decoder.decode_into(&mut pixels).unwrap();

        // TODO
        let color_type = if colorspace.has_alpha() { ColorType::RGBA8 } else { ColorType::RGB8 };

        Image { data: pixels, width: width as u32, height: height as u32, color_type }
    }
}
