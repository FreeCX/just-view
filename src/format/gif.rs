use gif::DecodeOptions;
use log::debug;

use super::Loader;
use crate::image::{ColorType, Image};

pub struct Gif;

impl Loader for Gif {
    fn load(data: &[u8]) -> Image {
        debug!("Use gif loader");

        let mut options = DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::RGBA);

        let mut decoder = options.read_info(data).unwrap();
        let frame = decoder.read_next_frame().unwrap().unwrap();
        let pixels = frame.buffer.to_vec();

        Image {
            data: pixels,
            width: decoder.width() as u32,
            height: decoder.height() as u32,
            color_type: ColorType::RGBA8,
        }
    }
}
