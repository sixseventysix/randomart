unsafe extern "C" {
    fn cr_sinf(x: f32) -> f32;
    fn cr_cosf(x: f32) -> f32;
    fn cr_expf(x: f32) -> f32;
    fn cr_sinf_row(values: *mut f32, n: usize);
    fn cr_cosf_row(values: *mut f32, n: usize);
    fn cr_expf_row(values: *mut f32, n: usize);
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

pub fn sinf_row(values: &mut [f32]) {
    unsafe { cr_sinf_row(values.as_mut_ptr(), values.len()) }
}

pub fn cosf_row(values: &mut [f32]) {
    unsafe { cr_cosf_row(values.as_mut_ptr(), values.len()) }
}

pub fn expf_row(values: &mut [f32]) {
    unsafe { cr_expf_row(values.as_mut_ptr(), values.len()) }
}
