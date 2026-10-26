/* Ported from CORE-MATH's correctly-rounded natural exponential function for
binary32 value (core-math/src/binary32/exp/expf.c).

Copyright (c) 2023-2025 Alexei Sibidanov.

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

use super::{doubles, mul_add};

const C: [f64; 6] = doubles([
    0x3fe62e42fefa39ef, 0x3fcebfbdff82c58f, 0x3fac6b08d702e0ed,
    0x3f83b2ab6fb92e5e, 0x3f55d886e6d54203, 0x3f2430976b8ce6ef,
]);
const B: [f64; 4] = doubles([0x3ff0000000000000, 0x3fe62e42fef4c4e7, 0x3fcebfd1b232f475, 0x3fac6b19384ecd93]);
const TB: [u64; 64] = [
    0x3ff0000000000000, 0x3ff02c9a3e778061, 0x3ff059b0d3158574, 0x3ff0874518759bc8,
    0x3ff0b5586cf9890f, 0x3ff0e3ec32d3d1a2, 0x3ff11301d0125b51, 0x3ff1429aaea92de0,
    0x3ff172b83c7d517b, 0x3ff1a35beb6fcb75, 0x3ff1d4873168b9aa, 0x3ff2063b88628cd6,
    0x3ff2387a6e756238, 0x3ff26b4565e27cdd, 0x3ff29e9df51fdee1, 0x3ff2d285a6e4030b,
    0x3ff306fe0a31b715, 0x3ff33c08b26416ff, 0x3ff371a7373aa9cb, 0x3ff3a7db34e59ff7,
    0x3ff3dea64c123422, 0x3ff4160a21f72e2a, 0x3ff44e086061892d, 0x3ff486a2b5c13cd0,
    0x3ff4bfdad5362a27, 0x3ff4f9b2769d2ca7, 0x3ff5342b569d4f82, 0x3ff56f4736b527da,
    0x3ff5ab07dd485429, 0x3ff5e76f15ad2148, 0x3ff6247eb03a5585, 0x3ff6623882552225,
    0x3ff6a09e667f3bcd, 0x3ff6dfb23c651a2f, 0x3ff71f75e8ec5f74, 0x3ff75feb564267c9,
    0x3ff7a11473eb0187, 0x3ff7e2f336cf4e62, 0x3ff82589994cce13, 0x3ff868d99b4492ed,
    0x3ff8ace5422aa0db, 0x3ff8f1ae99157736, 0x3ff93737b0cdc5e5, 0x3ff97d829fde4e50,
    0x3ff9c49182a3f090, 0x3ffa0c667b5de565, 0x3ffa5503b23e255d, 0x3ffa9e6b5579fdbf,
    0x3ffae89f995ad3ad, 0x3ffb33a2b84f15fb, 0x3ffb7f76f2fb5e47, 0x3ffbcc1e904bc1d2,
    0x3ffc199bdd85529c, 0x3ffc67f12e57d14b, 0x3ffcb720dcef9069, 0x3ffd072d4a07897c,
    0x3ffd5818dcfba487, 0x3ffda9e603db3285, 0x3ffdfc97337b9b5f, 0x3ffe502ee78b3ff6,
    0x3ffea4afa2a490da, 0x3ffefa1bee615a27, 0x3fff50765b6e4540, 0x3fffa7c1819e90d8,
];

const ILN2: f64 = f64::from_bits(0x3ff71547652b82fe);
const ILN2_HI: f64 = f64::from_bits(0x3ff7154765000000);
const ILN2_LO: f64 = f64::from_bits(0x3e05c17f0bbbe880);
const BIG: f64 = f64::from_bits(0x42d8000000000000);
const SUBNORMAL_SHIFT: f64 = f64::from_bits(0x4059d1d9fccf4770);
const SUBNORMAL_SCALE: f64 = f64::from_bits(0x36971547652b82ed);
const TWO_TO_MINUS_149: f64 = f64::from_bits(0x36a0000000000000);
const TWO_TO_MINUS_151: f64 = f64::from_bits(0x3680000000000000);
const ROUNDING_TEST: f64 = f64::from_bits(0x3de3edbbe4560327);

#[cold]
#[inline(never)]
fn expf_special(x: f32) -> Option<f32> {
    let u = x.to_bits();
    if u << 1 >= 0xff << 24 {
        return Some(if u << 1 > 0xff << 24 {
            x + x
        } else if u >> 31 == 0 {
            f32::INFINITY
        } else {
            0.0
        });
    }
    if u > 0xc2ce_8ec0 {
        let y = mul_add(x as f64 + SUBNORMAL_SHIFT, SUBNORMAL_SCALE, TWO_TO_MINUS_149);
        return Some(y.max(TWO_TO_MINUS_151) as f32);
    }
    if u >> 31 == 0 && u > 0x42b1_7217 {
        return Some(f32::INFINITY);
    }
    None
}

#[cold]
#[inline(never)]
fn expf_accurate(z: f64, ia: f64, s: f64) -> f32 {
    let h = mul_add(ILN2_LO, z, mul_add(ILN2_HI, z, ia));
    let h2 = h * h;
    let w = s * h;
    let p = mul_add(h2, mul_add(h2, mul_add(h, C[5], C[4]), mul_add(h, C[3], C[2])), mul_add(h, C[1], C[0]));
    mul_add(w, p, s) as f32
}

pub fn expf(x: f32) -> f32 {
    let z = x as f64;
    let a = ILN2 * z;
    let ux = x.to_bits() << 1;
    if ux > 0x8562_e42e || ux < 0x6f93_813e {
        if ux < 0x6f93_813e {
            return mul_add(z, mul_add(z, 0.5, 1.0), 1.0) as f32;
        }
        if let Some(r) = expf_special(x) {
            return r;
        }
    }
    let shifted = a + BIG;
    let u = shifted.to_bits();
    let ia = BIG - shifted;
    let h = a + ia;
    let s = f64::from_bits(TB[(u & 0x3f) as usize].wrapping_add((u >> 6) << 52));
    let h2 = h * h;
    let r = mul_add(h2, mul_add(h, B[3], B[2]), mul_add(h, B[1], B[0])) * s;
    let ub = r as f32;
    let lb = mul_add(-r, ROUNDING_TEST, r) as f32;
    if ub != lb {
        return expf_accurate(z, ia, s);
    }
    ub
}
