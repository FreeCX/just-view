use super::Loader;
use crate::image::Image;

pub struct Jpg;

impl Loader for Jpg {
    fn load(_data: &[u8]) -> Image {
        todo!()
    }
}
