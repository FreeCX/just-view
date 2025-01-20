pub struct Image {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub depth: u8,
    pub color_type: ColorType,
}

pub enum ColorType {
    RGB,
    RGBA,
}

#[derive(Default)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}
