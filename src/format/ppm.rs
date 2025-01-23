use log::debug;
use zune_core::result::DecodingResult;
use zune_ppm::PPMDecoder;

use super::Loader;
use crate::image::{ColorType, Image};

pub struct Ppm;

impl Loader for Ppm {
    fn load(data: &[u8]) -> Image {
        debug!("Use ppm loader");
        let mut decoder = PPMDecoder::new(data);
        decoder.decode_headers().unwrap();
        let (width, height) = decoder.get_dimensions().unwrap();
        let (pixels, color_type) = match decoder.decode().unwrap() {
            DecodingResult::U8(data) => (data, ColorType::RGB8),
            DecodingResult::U16(_) => panic!("format u16 not supported"),
            DecodingResult::F32(_) => panic!("format f32 not supported"),
            _ => panic!("incorret image"),
        };

        Image { data: pixels, width: width as u32, height: height as u32, color_type }
    }
}
