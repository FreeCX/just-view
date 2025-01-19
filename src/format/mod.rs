use std::{fs, path::Path};

use mime_guess::mime;
use miniquad::TextureParams;

#[cfg(feature = "jpg")]
mod jpg;
#[cfg(feature = "png")]
mod png;

use crate::image::Image;

#[allow(unused)]
trait Loader {
    fn load(data: &[u8]) -> Image;
}

pub fn load_image<P: AsRef<Path>>(filename: P) -> Image {
    let mime_type = mime_guess::MimeGuess::from_path(&filename).first().unwrap();
    let data = fs::read(filename).unwrap();
    return match mime_type.subtype() {
        #[cfg(feature = "png")]
        mime::PNG => png::Png::load(&data),
        #[cfg(feature = "jpg")]
        mime::JPEG => jpg::Jpg::load(&data),
        mime => panic!("file format {mime} not supported"),
    };
}

impl From<&Image> for TextureParams {
    fn from(image: &Image) -> Self {
        TextureParams {
            kind: miniquad::TextureKind::Texture2D,
            // TODO
            format: miniquad::TextureFormat::RGB8,
            wrap: miniquad::TextureWrap::Clamp,
            min_filter: miniquad::FilterMode::Nearest,
            mag_filter: miniquad::FilterMode::Nearest,
            mipmap_filter: miniquad::MipmapFilterMode::None,
            width: image.width,
            height: image.height,
            allocate_mipmaps: false,
            sample_count: 0,
        }
    }
}
