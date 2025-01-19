use std::path::PathBuf;

use crate::format;
use crate::image;

#[derive(Default)]
pub struct Filesystem {
    current: PathBuf,
    directory: PathBuf,
}

impl Filesystem {
    pub fn setup(filename: &str) -> Filesystem {
        let current: PathBuf = filename.into();
        let directory = current.as_path().parent().unwrap().to_path_buf();
        Filesystem { current, directory }
    }

    pub fn prev(&mut self) {
        todo!()
    }

    pub fn next(&mut self) {
        todo!()
    }

    pub fn data(&self) -> image::Image {
        format::load_image(&self.current)
    }
}
