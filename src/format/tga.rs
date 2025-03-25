use anyhow::anyhow;
use log::debug;
use tinytga::RawTga;

use super::Loader;
use crate::image::{ColorType, Image};

pub struct Tga;

impl Loader for Tga {
    fn load(data: &[u8]) -> anyhow::Result<Image> {
        debug!("Use tga loader");

        let img = match RawTga::from_slice(data) {
            Ok(data) => data,
            Err(e) => return Err(anyhow!("{:?}", e)),
        };
        let header = img.header();
        let width = header.width as u32;
        let height = header.height as u32;

        // TODO: rewrite this
        let mut pixels = vec![0_u8; 3 * (width * height) as usize];
        for pixel in img.pixels() {
            let index = 3 * (pixel.position.y * width as i32 + pixel.position.x) as usize;
            pixels[index + 2] = (pixel.color & 0xFF) as u8;
            pixels[index + 1] = ((pixel.color >> 8) & 0xFF) as u8;
            pixels[index + 0] = ((pixel.color >> 16) & 0xFF) as u8;
        }
        let color_type = ColorType::RGB8;

        Ok(Image { data: pixels, width, height, color_type })
    }
}
