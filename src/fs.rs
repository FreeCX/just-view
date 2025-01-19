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
        println!("File to load: {}", current.display());
        let directory = current.as_path().canonicalize().unwrap().parent().unwrap().to_path_buf();
        println!("Working directory: {}", directory.display());
        Filesystem { current, directory }
    }

    pub fn prev(&mut self) {
        todo!()
    }

    pub fn next(&mut self) {
        todo!()
    }

    pub fn data(&self) -> image::Image {
        println!("Load file: {}", self.current.display());
        format::load_image(&self.current)
    }
}
