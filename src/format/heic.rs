use libheif_rs::{ColorSpace, HeifContext, LibHeif, RgbChroma};
use log::debug;

use super::Loader;
use crate::image::{ColorType, Image};

pub struct Heic;

impl Loader for Heic {
    fn load(data: &[u8]) -> Image {
        debug!("Use heic loader");

        let lib = LibHeif::new();
        let ctx = HeifContext::read_from_bytes(data).unwrap();
        let handle = ctx.primary_image_handle().unwrap();
        let planes = lib.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None).unwrap();
        let pixels = planes.planes().interleaved.unwrap().data.to_vec();

        Image { data: pixels, width: handle.width(), height: handle.height(), color_type: ColorType::RGB8 }
    }
}
