pub struct GenerateOutput {
    pub pixels: PixelBuffer,
    pub json: String,
}

pub struct ReadOutput {
    pub pixels: PixelBuffer,
}

/// A flat RGB image buffer. Each pixel is 3 consecutive bytes: R, G, B.
#[derive(PartialEq, Eq)]
pub struct PixelBuffer {
    pub width: u32,
    pub height: u32,
    /// Row-major RGB bytes, length == width * height * 3.
    pub data: Vec<u8>,
}

impl PixelBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0u8; width as usize * height as usize * 3],
        }
    }

    #[inline]
    pub fn put_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) {
        let idx = (y as usize * self.width as usize + x as usize) * 3;
        self.data[idx]     = r;
        self.data[idx + 1] = g;
        self.data[idx + 2] = b;
    }
}
