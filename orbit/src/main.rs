fn main() {
    println!("Hello, world!");
}

struct Vec2 (f32,f32);
type Position = Vec2;
type Velocity = Vec2;
type Force = Vec2;

fn add(a: &Vec2, b: &Vec2) -> Vec2 {
    Vec2(a.0 + b.0, a.1 + b.1)
}

fn sub(a: &Vec2, b: &Vec2) -> Vec2 {
    Vec2(a.0 - b.0, a.1 - b.1)
}

fn average(a: &Vec2, b: &Vec2) -> Vec2 {
    Vec2((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0)
}

fn distance(a: &Vec2, b: &Vec2) -> f32 {
    let x = (a.0 - b.0).powf(2.0);
    let y = (a.1 - b.1).powf(2.0);
    (x+y).sqrt()
}

fn scale(a: &Vec2, d: f32) -> Vec2 {
    Vec2(a.0*d, a.1*d)
}

fn magnitude(a: &Vec2) -> f32 {
    (a.0*a.0 + a.1*a.1).sqrt()
}

fn unit(a: &Vec2) -> Vec2 {
    let m = magnitude(&a);
    if m == 0.0 {
        Vec2(a.0, a.1)
    }
    else {
        scale(&a, 1.0/m)
    }
}

fn rotate90(a: &Vec2) -> Vec2 {
    Vec2(-a.1,a.0)
}

struct Object {
    position: Position,
    mass: f32,
    velocity: Velocity,
    force: Force
}

fn gravity(m1: f32, m2: f32, r: f32) -> f32 {
    if r == 0.0 {
        0.0
    } else {
        (m1 * m2) / (r * r)
    }
}

fn force_between(a: &Object, b: &Object) -> Force {
    let uv = unit(&sub(&b.position, &a.position));
    let g = gravity(a.mass, b.mass, distance(&a.position, &b.position));
    scale(&uv,g)
}

fn accumulate_forces(a: &Object, b: &Vec<Object>) -> Object {
    let f = b.iter().fold(Vec2(0.0,0.0), | acc, x | {
        add(&acc,&force_between(x, a))
    });
    Object{
        position: Vec2(a.position.0, a.position.1),
        mass: a.mass,
        velocity: Vec2(a.velocity.0, a.velocity.1),
        force: f
    }
}