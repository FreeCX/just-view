use anyhow::Context;
use gif::DecodeOptions;
use log::debug;

use super::Loader;
use crate::image::{ColorType, Image};

pub struct Gif;

impl Loader for Gif {
    fn load(data: &[u8]) -> anyhow::Result<Image> {
        debug!("Use gif loader");

        let mut options = DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::RGBA);

        let mut decoder = options.read_info(data)?;
        let frame = decoder.read_next_frame()?.with_context(|| format!("Cannot read frame"))?;
        let pixels = frame.buffer.to_vec();

        Ok(Image {
            data: pixels,
            width: decoder.width() as u32,
            height: decoder.height() as u32,
            color_type: ColorType::RGBA8,
        })
    }
}
