/* Ported from CORE-MATH's correctly-rounded sine of binary32 value
(core-math/src/binary32/sin/sinf.c).

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

use super::trig::{kernel, kernel_big, non_finite, reduce, reduce_big};

const MINUS_ONE_SIXTH: f32 = f32::from_bits(0xbe2aaaab);

const EXCEPTIONS: [(u32, f32, f32); 3] = [
    (0x46199998, f32::from_bits(0xbeb1fa5d), f32::from_bits(0xb2000000)),
    (0x3f3adc51, f32::from_bits(0x3f2ab445), f32::from_bits(0xb2800000)),
    (0x3fa7832a, f32::from_bits(0x3f7741b6), f32::from_bits(0xb2800000)),
];

#[cold]
#[inline(never)]
fn exception(x: f32) -> f32 {
    let arg = x.to_bits() & 0x7fff_ffff;
    let &(_, rh, rl) = EXCEPTIONS.iter().find(|e| e.0 == arg).unwrap();
    let sgn = 1.0f32.copysign(x);
    sgn * rh + sgn * rl
}

#[cold]
#[inline(never)]
fn sinf_big(x: f32) -> f32 {
    if let Some(r) = non_finite(x) {
        return r;
    }
    let (z, ia) = reduce_big(x.to_bits());
    kernel_big(z, ia) as f32
}

pub fn sinf(x: f32) -> f32 {
    let ax = x.to_bits() << 1;
    if ax > 0x9900_0000 || ax < 0x7300_0000 {
        if ax < 0x6600_0000 {
            return if ax == 0 { x } else { (-x).mul_add(x.abs(), x) };
        }
        if ax < 0x7300_0000 {
            return (MINUS_ONE_SIXTH * x).mul_add(x * x, x);
        }
        return sinf_big(x);
    }
    if ax == EXCEPTIONS[0].0 << 1 || ax == EXCEPTIONS[1].0 << 1 || ax == EXCEPTIONS[2].0 << 1 {
        return exception(x);
    }
    let (z, ia) = reduce(x, ax < 0x822d_97c8);
    kernel(z, ia) as f32
}
