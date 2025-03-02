use std::fs;
use std::path::PathBuf;

use log::debug;
use magic::cookie::{Cookie, Flags, Load};

use crate::format;
use crate::image;

pub struct Filesystem {
    files: Vec<PathBuf>,
    cookie: Cookie<Load>,
    index: usize,
}

impl Filesystem {
    pub fn setup(filename: &str) -> Filesystem {
        debug!("Load magic");
        let cookie = Cookie::open(Flags::MIME_TYPE).unwrap().load(&Default::default()).unwrap();

        let current = PathBuf::from(filename).as_path().canonicalize().unwrap().to_path_buf();
        debug!("Startup file: {}", current.display());

        let directory = current.as_path().canonicalize().unwrap().parent().unwrap().to_path_buf();
        debug!("Working directory: {}", directory.display());

        let mut files = Vec::new();
        for entry in fs::read_dir(directory).unwrap() {
            if let Ok(item) = entry {
                if item.metadata().unwrap().is_file() {
                    let filename = item.path();
                    if format::is_image(&cookie, &filename) {
                        files.push(filename);
                    }
                }
            }
        }

        let index = files.iter().position(|i| i == &current).unwrap();
        debug!("Current index = {index}");

        Filesystem { files, index, cookie }
    }

    pub fn first(&mut self) -> bool {
        if self.index != 0 {
            self.index = 0;
            debug!("Current index = {}", self.index);
            true
        } else {
            false
        }
    }

    pub fn last(&mut self) -> bool {
        if self.index != self.files.len() - 1 {
            self.index = self.files.len() - 1;
            debug!("Current index = {}", self.index);
            true
        } else {
            false
        }
    }

    pub fn prev(&mut self) -> bool {
        if self.index != 0 {
            self.index -= 1;
            debug!("Current index = {}", self.index);
            true
        } else {
            false
        }
    }

    pub fn next(&mut self) -> bool {
        if self.index + 1 < self.files.len() {
            self.index += 1;
            debug!("Current index = {}", self.index);
            true
        } else {
            false
        }
    }

    pub fn data(&self) -> image::Image {
        let filename = &self.files[self.index];
        debug!("Load file: {}", filename.display());
        format::load_image(&self.cookie, filename)
    }
}
