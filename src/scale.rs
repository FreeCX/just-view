const ZOOM: [u16; 18] = [1, 2, 5, 10, 20, 40, 60, 80, 100, 125, 150, 200, 300, 500, 1000, 3000, 5000, 10000];
const DEFAULT: usize = 8;

pub struct Scale {
    aspect: f32,
    zoom_index: usize,
}

impl Scale {
    pub fn new() -> Scale {
        Scale { aspect: 1.0, zoom_index: DEFAULT }
    }

    pub fn zoom(&self) -> u16 {
        ZOOM[self.zoom_index]
    }

    pub fn zoom_in(&mut self) {
        if self.zoom_index + 1 < ZOOM.len() {
            self.zoom_index += 1;
        }
    }

    pub fn zoom_out(&mut self) {
        if self.zoom_index > 0 {
            self.zoom_index -= 1;
        }
    }

    pub fn zoom_reset(&mut self) {
        self.zoom_index = DEFAULT;
    }

    pub fn calc_aspect(&mut self, w1: f32, h1: f32, w2: f32, h2: f32) -> (f32, f32) {
        let zoom = self.zoom() as f32 / 100.0;
        self.aspect = if w1 > w2 || h1 > h2 { (w2 / w1).min(h2 / h1) } else { 1.0 };
        let ix = zoom * (w1 * self.aspect) / w2;
        let iy = zoom * (h1 * self.aspect) / h2;
        (ix, iy)
    }
}
