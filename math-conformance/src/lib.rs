use randomart::ftz::disable_ftz;
use rayon::prelude::*;

unsafe extern "C" {
    fn cr_sinf_row(values: *mut f32, n: usize);
    fn cr_cosf_row(values: *mut f32, n: usize);
    fn cr_expf_row(values: *mut f32, n: usize);
}

pub fn core_math_sinf_row(values: &mut [f32]) {
    unsafe { cr_sinf_row(values.as_mut_ptr(), values.len()) }
}

pub fn core_math_cosf_row(values: &mut [f32]) {
    unsafe { cr_cosf_row(values.as_mut_ptr(), values.len()) }
}

pub fn core_math_expf_row(values: &mut [f32]) {
    unsafe { cr_expf_row(values.as_mut_ptr(), values.len()) }
}

const BLOCK: usize = 4096;
const SCATTER: u32 = 0x9e37_79b1;

pub struct Report {
    pub count: u64,
    pub examples: Vec<[u32; 3]>,
}

fn fill(values: &mut [f32], block: usize) {
    for (i, v) in values.iter_mut().enumerate() {
        *v = f32::from_bits(((block * BLOCK + i) as u32).wrapping_mul(SCATTER));
    }
}

fn same(a: f32, b: f32) -> bool {
    a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan())
}

pub fn compare(candidate: impl Fn(f32) -> f32 + Sync, reference: fn(&mut [f32])) -> Report {
    rayon::broadcast(|_| disable_ftz());
    (0..(1usize << 32) / BLOCK)
        .into_par_iter()
        .map_init(
            || (vec![0.0; BLOCK], vec![0.0; BLOCK]),
            |(input, expected), block| {
                fill(input, block);
                expected.copy_from_slice(input);
                reference(expected);
                let mut report = Report { count: 0, examples: Vec::new() };
                for (&x, &expected) in input.iter().zip(expected.iter()) {
                    let got = candidate(x);
                    if !same(expected, got) {
                        report.count += 1;
                        if report.examples.len() < 5 {
                            report.examples.push([x.to_bits(), expected.to_bits(), got.to_bits()]);
                        }
                    }
                }
                report
            },
        )
        .reduce(
            || Report { count: 0, examples: Vec::new() },
            |mut a, b| {
                a.count += b.count;
                a.examples.extend(b.examples);
                a.examples.truncate(5);
                a
            },
        )
}

pub fn print(report: &Report) {
    println!("{} mismatches", report.count);
    for [x, expected, got] in &report.examples {
        println!("  x = {x:#010x} ({:e}): expected {expected:#010x}, got {got:#010x}", f32::from_bits(*x));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_matches(candidate: impl Fn(f32) -> f32 + Sync, reference: fn(&mut [f32])) {
        let report = compare(candidate, reference);
        print(&report);
        assert_eq!(report.count, 0);
    }

    #[test]
    #[ignore]
    fn sinf_matches_core_math() {
        assert_matches(math::sinf, core_math_sinf_row);
    }

    #[test]
    #[ignore]
    fn cosf_matches_core_math() {
        assert_matches(math::cosf, core_math_cosf_row);
    }

    #[test]
    #[ignore]
    fn expf_matches_core_math() {
        assert_matches(math::expf, core_math_expf_row);
    }

    #[test]
    #[ignore]
    fn std_sin_is_not_correctly_rounded() {
        let report = compare(f32::sin, core_math_sinf_row);
        print(&report);
        assert!(report.count > 0);
    }
}
