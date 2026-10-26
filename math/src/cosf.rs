/* Ported from CORE-MATH's correctly-rounded cosine of binary32 value
(core-math/src/binary32/cos/cosf.c).

Copyright (c) 2022-2023 Alexei Sibidanov.

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

use super::trig::{kernel, kernel_big, non_finite, reduce, reduce_big};

const TWO_TO_MINUS_25: f32 = f32::from_bits(0x33000000);

const EXCEPTIONS: [(u32, f32, f32); 5] = [
    (0x4096cbe4, f32::from_bits(0x324cde2e), f32::from_bits(0xa5800000)),
    (0x5922aa80, f32::from_bits(0x3f08aebf), f32::from_bits(0xb2800000)),
    (0x5aa4542c, f32::from_bits(0x3efa40a4), f32::from_bits(0x32000000)),
    (0x5f18b878, f32::from_bits(0x3f7f14bb), f32::from_bits(0x32800000)),
    (0x6115cb11, f32::from_bits(0x3f78142f), f32::from_bits(0xb2800000)),
];

#[cold]
#[inline(never)]
fn exception(x: f32, r: f64) -> f32 {
    let arg = x.to_bits() & 0x7fff_ffff;
    match EXCEPTIONS.iter().find(|e| e.0 == arg) {
        Some(&(_, rh, rl)) => rh + rl,
        None => r as f32,
    }
}

#[cold]
#[inline(never)]
fn cosf_big(x: f32) -> f32 {
    if let Some(r) = non_finite(x) {
        return r;
    }
    let (z, ia) = reduce_big(x.to_bits());
    let r = kernel_big(z, ia.wrapping_add(8));
    let tail = r.to_bits().wrapping_add(6) & (!0 >> 36);
    if tail <= 12 {
        return exception(x, r);
    }
    r as f32
}

pub fn cosf(x: f32) -> f32 {
    let ax = x.to_bits() << 1;
    if ax > 0x9900_0000 || ax < 0x7300_0000 {
        if ax < 0x6600_0000 {
            return if ax == 0 { 1.0 } else { 1.0 - TWO_TO_MINUS_25 };
        }
        if ax < 0x7300_0000 {
            return (-0.5 * x).mul_add(x, 1.0);
        }
        return cosf_big(x);
    }
    if ax == EXCEPTIONS[0].0 << 1 {
        return exception(x, 0.0);
    }
    let (z, ia) = reduce(x, ax < 0x82a4_1896);
    kernel(z, ia.wrapping_add(8)) as f32
}
