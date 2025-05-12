#[inline(always)]
pub fn x(x: f32, _y: f32) -> f32 {
    x
}

#[inline(always)]
pub fn y(_x: f32, y: f32) -> f32 {
    y
}

#[inline(always)]
pub fn const_(v: f32) -> f32 {
    v
}

#[inline(always)]
pub fn sin(v: f32) -> f32 {
    v.sin()
}

#[inline(always)]
pub fn cos(v: f32) -> f32 {
    v.cos()
}

#[inline(always)]
pub fn exp(v: f32) -> f32 {
    v.exp()
}

#[inline(always)]
pub fn sqrt(v: f32) -> f32 {
    v.sqrt().max(0.0)
}

#[inline(always)]
pub fn neg(v: f32) -> f32 {
    -v
}

#[inline(always)]
pub fn add(a: f32, b: f32) -> f32 {
    (a + b) / 2.0
}

#[inline(always)]
pub fn mul(a: f32, b: f32) -> f32 {
    a * b
}

#[inline(always)]
pub fn div(a: f32, b: f32) -> f32 {
    if b.abs() > 1e-6 {
        a / b
    } else {
        0.0
    }
}

#[inline(always)]
pub fn modulo(a: f32, b: f32) -> f32 {
    if b.abs() > 1e-6 {
        a % b
    } else {
        0.0
    }
}

#[inline(always)]
pub fn mix(a: f32, b: f32, c: f32, d: f32) -> f32 {
    let a = a + 1.0;
    let b = b + 1.0;
    let c = c + 1.0;
    let d = d + 1.0;
    let numerator = a * c + b * d;
    let denominator = (a + b).max(1e-6);
    (numerator / denominator) - 1.0
}

#[inline(always)]
pub fn mixu(a: f32, b: f32, c: f32, d: f32) -> f32 {
    (a * c + b * d) / (a + b + 1e-6)
}