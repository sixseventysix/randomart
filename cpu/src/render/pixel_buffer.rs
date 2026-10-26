/// A flat RGB image buffer. Each pixel is 3 consecutive bytes: R, G, B.
#[derive(PartialEq, Eq, Debug)]
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
}
