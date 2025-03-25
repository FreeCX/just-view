use anyhow::anyhow;
use log::debug;
use zune_jpeg::JpegDecoder;

use super::Loader;
use crate::image::{ColorType, Image};

pub struct Jpg;

impl Loader for Jpg {
    fn load(data: &[u8]) -> anyhow::Result<Image> {
        debug!("Use jpeg loader");

        let mut decoder = JpegDecoder::new(data);
        let pixels = match decoder.decode() {
            Ok(data) => data,
            Err(e) => return Err(anyhow!("{:?}", e)),
        };
        let info = decoder.info().unwrap();

        Ok(Image {
            data: pixels,
            width: info.width as u32,
            height: info.height as u32,
            // TODO
            color_type: ColorType::RGB8,
        })
    }
}
