#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec2(pub f64, pub f64);

pub const VEC_ZERO: Vec2 = Vec2(0.0, 0.0);

pub fn add(a: &Vec2, b: &Vec2) -> Vec2 {
    Vec2(a.0 + b.0, a.1 + b.1)
}

pub fn sub(a: &Vec2, b: &Vec2) -> Vec2 {
    Vec2(b.0 - a.0, b.1 - a.1)
}

pub fn distance(a: &Vec2, b: &Vec2) -> f64 {
    let x = (a.0 - b.0).powf(2.0);
    let y = (a.1 - b.1).powf(2.0);
    (x + y).sqrt()
}

#[test]
fn test_distance() {
    let a = VEC_ZERO;
    let b = Vec2(3.0, 3.0);

    assert_eq!((18.0f64).sqrt(), distance(&a, &b));
}

pub fn scale(a: &Vec2, d: f64) -> Vec2 {
    Vec2(a.0 * d, a.1 * d)
}

pub fn magnitude(a: &Vec2) -> f64 {
    (a.0 * a.0 + a.1 * a.1).sqrt()
}

pub fn unit(a: &Vec2) -> Vec2 {
    let m = magnitude(&a);
    if m == 0.0 {
        Vec2(a.0, a.1)
    } else {
        scale(&a, 1.0 / m)
    }
}

pub fn rotate90(a: &Vec2) -> Vec2 {
    Vec2(-a.1, a.0)
}
