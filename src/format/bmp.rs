use log::debug;
use zune_bmp::BmpDecoder;

use super::Loader;
use crate::image::{ColorType, Image};
use anyhow::{Context, anyhow};

pub struct Bmp;

impl Loader for Bmp {
    fn load(data: &[u8]) -> anyhow::Result<Image> {
        debug!("Use bmp loader");

        let mut decoder = BmpDecoder::new(data);
        if let Err(e) = decoder.decode_headers() {
            return Err(anyhow!("{:?}", e));
        }

        let colorspace = decoder.get_colorspace().with_context(|| format!("Missing colorspace info"))?;
        let (width, height) = decoder.get_dimensions().with_context(|| format!("Missing dimension info"))?;
        let mut pixels = vec![0; decoder.output_buf_size().unwrap()];
        if let Err(e) = decoder.decode_into(&mut pixels) {
            return Err(anyhow!("{:?}", e));
        }

        // TODO
        let color_type = if colorspace.has_alpha() { ColorType::RGBA8 } else { ColorType::RGB8 };

        Ok(Image { data: pixels, width: width as u32, height: height as u32, color_type })
    }
}
