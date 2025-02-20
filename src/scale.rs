pub struct Scale {
    pub aspect: f32,
    pub zoom: f32,
}

impl Scale {
    pub fn new() -> Scale {
        Scale { aspect: 1.0, zoom: 100.0 }
    }

    pub fn zoom_in(&mut self) {
        if self.zoom < 32_000.0 {
            self.zoom *= 1.25;
        }
    }

    pub fn zoom_out(&mut self) {
        if self.zoom > 5.0 {
            self.zoom *= 0.8;
        }
    }

    pub fn zoom_reset(&mut self) {
        self.zoom = 100.0;
    }

    pub fn calc_aspect(&mut self, w1: f32, h1: f32, w2: f32, h2: f32) -> (f32, f32) {
        let zoom = self.zoom as f32 / 100.0;
        self.aspect = if w1 > w2 || h1 > h2 { (w2 / w1).min(h2 / h1) } else { 1.0 };
        let ix = zoom * (w1 * self.aspect) / w2;
        let iy = zoom * (h1 * self.aspect) / h2;
        (ix, iy)
    }
}
