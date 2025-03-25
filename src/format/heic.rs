use anyhow::Context;
use libheif_rs::{ColorSpace, HeifContext, LibHeif, RgbChroma};
use log::debug;

use super::Loader;
use crate::image::{ColorType, Image};

pub struct Heic;

impl Loader for Heic {
    fn load(data: &[u8]) -> anyhow::Result<Image> {
        debug!("Use heic loader");

        let lib = LibHeif::new();
        let ctx = HeifContext::read_from_bytes(data)?;
        let handle = ctx.primary_image_handle()?;
        let planes = lib.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;
        let pixels = planes.planes().interleaved.with_context(|| format!("Missing interleaved plane"))?.data.to_vec();

        Ok(Image { data: pixels, width: handle.width(), height: handle.height(), color_type: ColorType::RGB8 })
    }
}
