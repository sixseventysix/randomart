pub fn pixel_position(pixel: usize, size: u32) -> f32 {
    (pixel as f32 / (size - 1) as f32) * 2.0 - 1.0
}
