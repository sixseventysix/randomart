pub mod pixel_buffer;

pub fn pixel_position(pixel: usize, size: u32) -> f32 {
    (pixel as f32 / (size - 1) as f32) * 2.0 - 1.0
}

pub fn to_byte(value: f32) -> u8 {
    ((value + 1.0) * 127.5).clamp(0.0, 255.0) as u8
}
