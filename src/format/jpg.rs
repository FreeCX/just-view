use zune_jpeg::JpegDecoder;

use super::Loader;
use crate::image::{ColorType, Image};

pub struct Jpg;

impl Loader for Jpg {
    fn load(data: &[u8]) -> Image {
        println!("Use jpeg loader");
        let mut decoder = JpegDecoder::new(data);
        let pixels = decoder.decode().unwrap();
        let info = decoder.info().unwrap();

        Image {
            data: pixels,
            width: info.width as u32,
            height: info.height as u32,
            // TODO
            depth: info.components,
            color_type: ColorType::RGB,
        }
    }
}
