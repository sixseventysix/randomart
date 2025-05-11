pub fn x(x: f32, _y: f32) -> f32 {
    x
}

pub fn y(_x: f32, y: f32) -> f32 {
    y
}

pub fn const_(v: f32) -> f32 {
    v
}

pub fn sin(v: f32) -> f32 {
    v.sin()
}

pub fn cos(v: f32) -> f32 {
    v.cos()
}

pub fn exp(v: f32) -> f32 {
    v.exp()
}

pub fn sqrt(v: f32) -> f32 {
    v.sqrt().max(0.0)
}

pub fn neg(v: f32) -> f32 {
    -v
}

pub fn add(a: f32, b: f32) -> f32 {
    (a + b) / 2.0
}

pub fn mul(a: f32, b: f32) -> f32 {
    a * b
}

pub fn div(a: f32, b: f32) -> f32 {
    if b.abs() > 1e-6 {
        a / b
    } else {
        0.0
    }
}

pub fn modulo(a: f32, b: f32) -> f32 {
    if b.abs() > 1e-6 {
        a % b
    } else {
        0.0
    }
}

pub fn mix(a: f32, b: f32, c: f32, d: f32) -> f32 {
    let a = a + 1.0;
    let b = b + 1.0;
    let c = c + 1.0;
    let d = d + 1.0;
    let numerator = a * c + b * d;
    let denominator = (a + b).max(1e-6);
    (numerator / denominator) - 1.0
}

pub fn mixu(a: f32, b: f32, c: f32, d: f32) -> f32 {
    (a * c + b * d) / (a + b + 1e-6)
}