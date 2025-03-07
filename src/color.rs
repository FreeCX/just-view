#[derive(Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn black() -> Color {
        Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }

    pub fn unpack(&self) -> (f32, f32, f32, f32) {
        (self.r, self.g, self.b, self.a)
    }

    pub fn inverse(&mut self) {
        self.r = (self.r - 1.0).abs();
        self.g = (self.g - 1.0).abs();
        self.b = (self.b - 1.0).abs();
    }
}
