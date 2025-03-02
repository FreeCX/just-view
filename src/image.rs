#[derive(Clone)]
pub struct Image {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub color_type: ColorType,
}

#[derive(Clone)]
pub enum ColorType {
    RGB8,
    RGBA8,
}

#[derive(Default)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}
