use log::debug;
use pcx::Reader;

use super::Loader;
use crate::image::{ColorType, Image};

pub struct Pcx;

impl Loader for Pcx {
    fn load(data: &[u8]) -> Image {
        debug!("Use pcx loader");
        let mut decoder = Reader::from_mem(data).unwrap();
        let buffer_size = 3 * decoder.width() as usize * decoder.height() as usize;
        let mut pixels = vec![0; buffer_size];
        decoder.read_rgb_pixels(&mut pixels).unwrap();

        Image {
            data: pixels,
            width: decoder.width() as u32,
            height: decoder.height() as u32,
            // TODO
            depth: 8,
            color_type: ColorType::RGB,
        }
    }
}
