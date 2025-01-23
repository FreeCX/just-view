use log::debug;
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
        // TODO
        let pixels = decoder.decode().unwrap().u8().unwrap();

        Image {
            data: pixels,
            width: width as u32,
            height: height as u32,
            // TODO
            depth: 8,
            color_type: ColorType::RGB,
        }
    }
}
