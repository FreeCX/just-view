use std::fs;
use std::path::PathBuf;

use crate::format;
use crate::image;

#[derive(Default)]
pub struct Filesystem {
    files: Vec<PathBuf>,
    index: usize,
}

impl Filesystem {
    pub fn setup(filename: &str) -> Filesystem {
        let current: PathBuf = filename.into();
        println!("Startup file: {}", current.display());

        let directory = current.as_path().canonicalize().unwrap().parent().unwrap().to_path_buf();
        println!("Working directory: {}", directory.display());

        let mut files = Vec::new();
        for entry in fs::read_dir(directory).unwrap() {
            if let Ok(item) = entry {
                // TODO: надо добавлять в список только валидные изображения
                if item.metadata().unwrap().is_file() {
                    files.push(item.path());
                }
            }
        }

        let index = files.iter().position(|i| i == &current).unwrap();
        println!("Current index = {index}");

        Filesystem { files, index }
    }

    pub fn prev(&mut self) {
        if self.index != 0 {
            self.index -= 1;
            println!("Current index = {}", self.index);
        }
    }

    pub fn next(&mut self) {
        if self.index + 1 < self.files.len() {
            self.index += 1;
            println!("Current index = {}", self.index);
        }
    }

    pub fn data(&self) -> image::Image {
        let filename = &self.files[self.index];
        println!("Load file: {}", filename.display());
        format::load_image(filename)
    }
}
