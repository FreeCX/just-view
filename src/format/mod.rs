use mime_guess::mime;

#[cfg(feature = "jpg")]
mod jpg;
#[cfg(feature = "png")]
mod png;

use crate::image::Image;

#[allow(unused)]
trait Loader {
    fn load(data: &[u8]) -> Image;
}

pub fn load_by_mime(mime: &mime::Name<'_>, data: &[u8]) -> Image {
    match *mime {
        #[cfg(feature = "png")]
        mime::PNG => png::Png::load(data),
        #[cfg(feature = "jpg")]
        mime::JPEG => jpg::Jpg::load(data),
        _ => panic!("file format {mime} not supported"),
    }
}
