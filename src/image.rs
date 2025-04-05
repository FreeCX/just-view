#[derive(Clone)]
pub struct Image {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub color_type: ColorType,
}

impl Image {
    pub fn empty() -> Image {
        Image { data: Vec::new(), width: 0, height: 0, color_type: ColorType::RGB8 }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ColorType {
    RGB8,
    RGBA8,
}

#[derive(Default)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}
