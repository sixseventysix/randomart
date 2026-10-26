mod cosf;
mod expf;
mod sinf;
mod trig;

pub use cosf::cosf;
pub use expf::expf;
pub use sinf::sinf;

const fn doubles<const N: usize>(bits: [u64; N]) -> [f64; N] {
    let mut out = [0.0; N];
    let mut i = 0;
    while i < N {
        out[i] = f64::from_bits(bits[i]);
        i += 1;
    }
    out
}

#[cfg(any(target_arch = "aarch64", target_feature = "fma"))]
fn mul_add(a: f64, b: f64, c: f64) -> f64 {
    a.mul_add(b, c)
}

#[cfg(not(any(target_arch = "aarch64", target_feature = "fma")))]
fn mul_add(a: f64, b: f64, c: f64) -> f64 {
    a * b + c
}

#[cfg(any(target_arch = "aarch64", target_feature = "fma"))]
fn mul_addf(a: f32, b: f32, c: f32) -> f32 {
    a.mul_add(b, c)
}

#[cfg(not(any(target_arch = "aarch64", target_feature = "fma")))]
fn mul_addf(a: f32, b: f32, c: f32) -> f32 {
    a * b + c
}
