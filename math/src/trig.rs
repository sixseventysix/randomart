/* Ported from CORE-MATH's correctly-rounded sine and cosine of binary32 value
(core-math/src/binary32/sin/sinf.c, core-math/src/binary32/cos/cosf.c).

Copyright (c) 2022-2025 Alexei Sibidanov.

This file is part of the CORE-MATH project
(https://core-math.gitlabpages.inria.fr/).

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use super::doubles;

const A: [f64; 4] = doubles([0x3fc921fb54442d17, 0xbf54abbce6256a39, 0x3ec466bc5a518c16, 0xbe232bdc61074ff6]);
const B: [f64; 4] = doubles([0x3f93bd3cc9be45dc, 0xbf103c1f081b0833, 0x3e755d3c6fc9ac1f, 0xbdce1d3ff281b40d]);
const TB: [f64; 32] = doubles([
    0x0000000000000000, 0x3fc8f8b83c69a60b, 0x3fd87de2a6aea963, 0x3fe1c73b39ae68c8,
    0x3fe6a09e667f3bcd, 0x3fea9b66290ea1a3, 0x3fed906bcf328d46, 0x3fef6297cff75cb0,
    0x3ff0000000000000, 0x3fef6297cff75cb0, 0x3fed906bcf328d46, 0x3fea9b66290ea1a3,
    0x3fe6a09e667f3bcd, 0x3fe1c73b39ae68c8, 0x3fd87de2a6aea963, 0x3fc8f8b83c69a60b,
    0x0000000000000000, 0xbfc8f8b83c69a60b, 0xbfd87de2a6aea963, 0xbfe1c73b39ae68c8,
    0xbfe6a09e667f3bcd, 0xbfea9b66290ea1a3, 0xbfed906bcf328d46, 0xbfef6297cff75cb0,
    0xbff0000000000000, 0xbfef6297cff75cb0, 0xbfed906bcf328d46, 0xbfea9b66290ea1a3,
    0xbfe6a09e667f3bcd, 0xbfe1c73b39ae68c8, 0xbfd87de2a6aea963, 0xbfc8f8b83c69a60b,
]);
const IPI: [u64; 4] = [0xfe5163abdebbc562, 0xdb6295993c439041, 0xfc2757d1f534ddc0, 0xa2f9836e4e441529];

const THIRTY_TWO_OVER_PI: f64 = f64::from_bits(0x40145f306dc9c883);
const THIRTY_TWO_OVER_PI_HI: f64 = f64::from_bits(0x40145f306e000000);
const THIRTY_TWO_OVER_PI_LO: f64 = f64::from_bits(0xbe2b1bbead603d8b);
const ROUND: f64 = f64::from_bits(0x4338000000000000);
const TWO_TO_MINUS_64: f64 = f64::from_bits(0x3bf0000000000000);

fn polynomials(z2: f64) -> (f64, f64) {
    let z4 = z2 * z2;
    let aa = z4.mul_add(z2.mul_add(A[3], A[2]), z2.mul_add(A[1], A[0]));
    let bb = z4.mul_add(z2.mul_add(B[3], B[2]), z2.mul_add(B[1], B[0]));
    (aa, bb)
}

fn table(ia: i32) -> (f64, f64) {
    (TB[(ia & 31) as usize], TB[(ia.wrapping_add(8) & 31) as usize])
}

pub(super) fn reduce(x: f32, small: bool) -> (f64, i32) {
    let x = x as f64;
    let (z, id) = if small {
        let idh = THIRTY_TWO_OVER_PI * x;
        let id = idh.round_ties_even();
        (idh - id, id)
    } else {
        reduce_large(x)
    };
    (z, (ROUND + id).to_bits() as i32)
}

#[cold]
#[inline(never)]
fn reduce_large(x: f64) -> (f64, f64) {
    let idl = THIRTY_TWO_OVER_PI_LO * x;
    let idh = THIRTY_TWO_OVER_PI_HI * x;
    let id = idh.round_ties_even();
    ((idh - id) + idl, id)
}

pub(super) fn reduce_big(u: u32) -> (f64, i32) {
    let e = (u >> 23) & 0xff;
    let m = ((u & (!0 >> 9)) | 1 << 23) as u128;
    let p0 = m * IPI[0] as u128;
    let p1 = m * IPI[1] as u128 + (p0 >> 64);
    let p2 = m * IPI[2] as u128 + (p1 >> 64);
    let p3 = m * IPI[3] as u128 + (p2 >> 64);
    let (p3h, p3l, p2l, p1l) = ((p3 >> 64) as u64, p3 as u64, p2 as u64, p1 as u64);
    let s = e as i32 - 124 - 23;
    let (i, a) = if s < 64 {
        (p3h << s | p3l >> (64 - s), p3l << s | p2l >> (64 - s))
    } else if s == 64 {
        (p3l, p2l)
    } else {
        (p3l << (s - 64) | p2l >> (128 - s), p2l << (s - 64) | p1l >> (128 - s))
    };
    let (i, a) = (i as i32, a as i64);
    let sgn = (u as i32) >> 31;
    let i = i.wrapping_sub((a >> 63) as i32);
    let z = (a ^ sgn as i64) as f64 * TWO_TO_MINUS_64;
    (z, (i ^ sgn).wrapping_sub(sgn))
}

pub(super) fn non_finite(x: f32) -> Option<f32> {
    let ax = x.to_bits() << 1;
    if ax < 0xff << 24 {
        return None;
    }
    Some(if ax << 8 != 0 { x + x } else { f32::NAN })
}

pub(super) fn kernel(z: f64, ia: i32) -> f64 {
    let z2 = z * z;
    let (aa, bb) = polynomials(z2);
    let (s0, c0) = table(ia);
    (-bb).mul_add(z2 * s0, aa.mul_add(z * c0, s0))
}

pub(super) fn kernel_big(z: f64, ia: i32) -> f64 {
    let (aa, bb) = polynomials(z * z);
    let (s0, c0) = table(ia);
    z.mul_add(aa.mul_add(c0, -(bb * (z * s0))), s0)
}
