use std::{fs, path::Path};

use log::debug;
use magic::cookie::{Cookie, Load};
use miniquad::TextureParams;

#[cfg(feature = "bmp")]
mod bmp;
#[cfg(feature = "jpg")]
mod jpg;
#[cfg(feature = "pcx")]
mod pcx;
#[cfg(feature = "png")]
mod png;
#[cfg(feature = "ppm")]
mod ppm;
#[cfg(feature = "webp")]
mod webp;

use crate::image::{ColorType, Image};

#[allow(unused)]
trait Loader {
    fn load(data: &[u8]) -> Image;
}

pub fn is_image<P>(cookie: &Cookie<Load>, filename: &P) -> bool
where
    P: AsRef<Path>,
{
    cookie.file(filename).unwrap().starts_with("image")
}

pub fn load_image<P>(cookie: &Cookie<Load>, filename: P) -> Image
where
    P: AsRef<Path>,
{
    let data = fs::read(filename).unwrap();
    debug!("Load file data");

    let type_info = cookie.buffer(&data).unwrap();
    debug!("Detect type: {}", type_info);

    return match type_info.as_str() {
        #[cfg(feature = "bmp")]
        "image/bmp" => bmp::Bmp::load(&data),
        #[cfg(feature = "jpg")]
        "image/jpeg" => jpg::Jpg::load(&data),
        #[cfg(feature = "pcx")]
        "image/vnd.zbrush.pcx" => pcx::Pcx::load(&data),
        #[cfg(feature = "png")]
        "image/png" => png::Png::load(&data),
        #[cfg(feature = "ppm")]
        "image/x-portable-pixmap" => ppm::Ppm::load(&data),
        #[cfg(feature = "webp")]
        "image/webp" => webp::Webp::load(&data),
        other => panic!("file format {other} not supported"),
    };
}

impl From<&Image> for TextureParams {
    fn from(image: &Image) -> Self {
        // TODO: пока без depth
        let format = match image.color_type {
            ColorType::RGB8 => miniquad::TextureFormat::RGB8,
            ColorType::RGBA8 => miniquad::TextureFormat::RGBA8,
        };

        TextureParams {
            kind: miniquad::TextureKind::Texture2D,
            format,
            wrap: miniquad::TextureWrap::Clamp,
            min_filter: miniquad::FilterMode::Linear,
            mag_filter: miniquad::FilterMode::Nearest,
            mipmap_filter: miniquad::MipmapFilterMode::None,
            width: image.width,
            height: image.height,
            allocate_mipmaps: false,
            sample_count: 1,
        }
    }
}
