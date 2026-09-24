extern "C" {
    fn cr_sinf(x: f32) -> f32;
    fn cr_cosf(x: f32) -> f32;
    fn cr_expf(x: f32) -> f32;
}

#[inline]
pub fn sinf(x: f32) -> f32 {
    unsafe { cr_sinf(x) }
}

#[inline]
pub fn cosf(x: f32) -> f32 {
    unsafe { cr_cosf(x) }
}

#[inline]
pub fn expf(x: f32) -> f32 {
    unsafe { cr_expf(x) }
}

#[inline]
pub fn sqrtf(x: f32) -> f32 {
    x.sqrt()
}
