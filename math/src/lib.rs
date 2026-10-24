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
