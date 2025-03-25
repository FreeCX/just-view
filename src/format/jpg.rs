use anyhow::{anyhow, bail};
use log::debug;
use zune_jpeg::JpegDecoder;

use super::Loader;
use crate::{
    image::{ColorType, Image},
    transform::grayscale_to_rgb,
};

pub struct Jpg;

impl Loader for Jpg {
    fn load(data: &[u8]) -> anyhow::Result<Image> {
        debug!("Use jpeg loader");

        let mut decoder = JpegDecoder::new(data);
        let mut pixels = match decoder.decode() {
            Ok(data) => data,
            Err(e) => return Err(anyhow!("{:?}", e)),
        };
        let info = decoder.info().unwrap();

        match (info.components, info.pixel_density) {
            (1, 8) => pixels = grayscale_to_rgb(pixels),
            (3, 8) => (),
            (components, density) => {
                bail!("Type with {components} components and {density} pixel density not supported")
            }
        };

        Ok(Image { data: pixels, width: info.width as u32, height: info.height as u32, color_type: ColorType::RGB8 })
    }
}
