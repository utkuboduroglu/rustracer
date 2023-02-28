pub trait rasterized {
    fn dimensionsF32(&self) -> (f32, f32);
    fn dimensions(&self) -> (u32, u32);
}

pub struct canvas {
    _baseDimension: u32,
    scaleX: f32,
    scaleY: f32,
}

impl rasterized for canvas {
    fn dimensionsF32(&self) -> (f32, f32) {
        (
            (self._baseDimension as f32 * self.scaleX) as f32,
            (self._baseDimension as f32 * self.scaleY) as f32,
        )
    }

    fn dimensions(&self) -> (u32, u32) {
        let (x, y) = self.dimensionsF32();
        (x as u32, y as u32)
    }
}

impl canvas {
    pub fn new(baseDimension: u32, scaleX: f32, scaleY: f32) -> canvas {
        canvas {
            _baseDimension: baseDimension,
            scaleX,
            scaleY,
        }
    }
}
