// TODO
// Learn what idiomatic Rust looks like!
// - Get rid of Copy/Clone traits
// - Use mutation confidentally!
// - Replace vomit inducing pairs gubbins near the bottom
// - Improve type safety of Vec2
// - Use constants rather than hard-coding

use std::env;
use text_colorizer::*;

use rand::prelude::*;

const IMAGE_SIZE: u32 = 1024;
const VEC_ZERO: Vec2 = Vec2(0.0, 0.0);
const SUN: Object = Object {
    position: Vec2(IMAGE_SIZE as f32 / 2.0, IMAGE_SIZE as f32 / 2.0),
    mass: 30.0,
    velocity: VEC_ZERO,
    force: VEC_ZERO,
};

fn main() {
    let args = parse_args();
    let mut objects: Vec<Object> = Vec::new();
    objects.push(SUN);

    for _i in 0..args.num_objects {
        let x: (f32, f32, f32, f32) = (
            rand::thread_rng().gen::<f32>(), // mass
            rand::thread_rng().gen::<f32>(), // velo
            rand::thread_rng().gen::<f32>(), // pos x
            rand::thread_rng().gen::<f32>(), // pos y
        );
        let obj = random_object(x);
        objects.push(obj);
    }

    println!("Objects{:?}", objects);

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(IMAGE_SIZE, IMAGE_SIZE);
    let mut ignored = 0;

    for i in 0..args.iterations {
        objects = update_all(&objects);

        for object in &objects {
            let x = object.position.0 as u32;
            let y = object.position.1 as u32;

            if x < IMAGE_SIZE as u32 && y < IMAGE_SIZE as u32 {
                let pixel = imgbuf.get_pixel_mut(x, y);
                let image::Rgb(_data) = *pixel;
                *pixel = image::Rgb([255u8, i as u8, i as u8]);
            } else {
                ignored = ignored + 1;
            }
        }
    }

    println!("ignored {}", ignored);

    // Save the image
    imgbuf.save(args.output).unwrap();
}

fn random_velocity(r: f32, pos: &Vec2) -> Vec2 {
    let sun_direction = unit(&sub(&pos, &SUN.position));
    let direction = rotate90(&sun_direction);
    scale(&direction, r * 0.3 + 0.3)
}

fn random_object((mass, vel, a, b): (f32, f32, f32, f32)) -> Object {
    let p = random_position(a, b);
    Object {
        position: Vec2(p.0, p.1),
        mass: mass * 0.2,
        velocity: random_velocity(vel, &p),
        force: VEC_ZERO,
    }
}

fn random_position(x: f32, y: f32) -> Vec2 {
    let r = x * 150.0 + 80.0;
    let theta = y * 2.0 * std::f32::consts::PI;
    add(&SUN.position, &Vec2(r * theta.cos(), r * theta.sin()))
}

#[derive(Debug)]
struct Arguments {
    num_objects: i32,
    iterations: i32,
    output: String,
}

fn print_usage() {
    eprintln!("{} - simulate some bodies under gravity", "orbit".green());
    eprintln!("Usage: orbit <num_objects> <delta>  <output>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 3 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments:@ expected 3, got {}.",
            "Error:".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }

    Arguments {
        num_objects: args[0].parse().unwrap(),
        iterations: args[1].parse().unwrap(),
        output: args[2].clone(),
    }
}

#[derive(Debug, PartialEq)]
struct Vec2(f32, f32);

type Position = Vec2;
type Velocity = Vec2;
type Force = Vec2;

fn add(a: &Vec2, b: &Vec2) -> Vec2 {
    Vec2(a.0 + b.0, a.1 + b.1)
}

fn sub(a: &Vec2, b: &Vec2) -> Vec2 {
    Vec2(b.0 - a.0, b.1 - a.1)
}

fn distance(a: &Vec2, b: &Vec2) -> f32 {
    let x = (a.0 - b.0).powf(2.0);
    let y = (a.1 - b.1).powf(2.0);
    (x + y).sqrt()
}

#[test]
fn test_distance() {
    let a = VEC_ZERO;
    let b = Vec2(3.0, 3.0);

    assert_eq!((18.0f32).sqrt(), distance(&a, &b));
}

fn scale(a: &Vec2, d: f32) -> Vec2 {
    Vec2(a.0 * d, a.1 * d)
}

fn magnitude(a: &Vec2) -> f32 {
    (a.0 * a.0 + a.1 * a.1).sqrt()
}

fn unit(a: &Vec2) -> Vec2 {
    let m = magnitude(&a);
    if m == 0.0 {
        Vec2(a.0, a.1)
    } else {
        scale(&a, 1.0 / m)
    }
}

fn rotate90(a: &Vec2) -> Vec2 {
    Vec2(-a.1, a.0)
}

#[derive(Debug, PartialEq)]
struct Object {
    position: Position,
    mass: f32,
    velocity: Velocity,
    force: Force,
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

    scale(&uv, g)
}

fn accumulate_forces(a: &Object, b: &Vec<Object>) -> Object {
    let f = b
        .iter()
        .fold(VEC_ZERO, |acc, x| add(&acc, &force_between(x, a)));

    Object {
        position: Vec2(a.position.0, a.position.1),
        mass: a.mass,
        velocity: Vec2(a.velocity.0, a.velocity.1),
        force: f,
    }
}

fn calculate_forces_on_all(a: &Vec<Object>) -> Vec<Object> {
    a.iter().map(|o| accumulate_forces(o, a)).collect()
}

fn accelerate(o: &Object) -> Object {
    let av = add(&o.velocity, &scale(&o.force, 1.0 / o.mass));

    Object {
        position: Vec2(o.position.0, o.position.1),
        mass: o.mass,
        force: VEC_ZERO,
        velocity: av,
    }
}

fn accelerate_all(objs: &Vec<Object>) -> Vec<Object> {
    objs.iter().map(accelerate).collect()
}

fn reposition(a: &Object) -> Object {
    Object {
        position: add(&a.position, &a.velocity),
        mass: a.mass,
        velocity: Vec2(a.velocity.0, a.velocity.1),
        force: Vec2(a.force.0, a.force.1),
    }
}

fn reposition_all(a: &Vec<Object>) -> Vec<Object> {
    a.iter().map(reposition).collect()
}

fn collide(a: &Object, b: &Object) -> bool {
    distance(&a.position, &b.position) <= 3.0
}

fn merge(a: &Object, b: &Object) -> Object {
    let mx = a.mass;
    let my = b.mass;
    let merged_mass = mx + my;
    let s = mx / merged_mass;
    let p1 = &a.position;
    let p2 = &b.position;
    let uv = unit(&sub(&p2, &p1));
    let d = scale(&uv, s);
    let mv1 = scale(&a.velocity, mx);
    let mv2 = scale(&b.velocity, my);

    Object {
        position: add(&p1, &d),
        mass: merged_mass,
        velocity: scale(&add(&mv1, &mv2), 1.0 / merged_mass),
        force: add(&a.force, &b.force),
    }
}

fn collide_all(a: &Vec<Object>) -> Vec<Object> {
    let mut collided_pairs: Vec<(&Object, &Object)> = Vec::new();
    let mut inert: Vec<Object> = Vec::new();

    // Find all the pairs that have collided
    for src in a.iter() {
        for tgt in a.iter() {
            if src == tgt {
                continue;
            }

            // This makes me vomit
            if collide(src, tgt) && !collided_pairs.contains(&(tgt, src)) {
                collided_pairs.push((src, tgt));
            }
        }
    }

    // Find all the objects not involved in collisions
    for obj in a.iter() {
        let mut found = false;
        for (src, tgt) in collided_pairs.iter() {
            if obj == *src || obj == *tgt {
                found = true;
                break;
            }
        }

        if !found {
            inert.push(*obj);
        }
    }

    // Merge together the collided pairs
    let mut merged: Vec<Object> = collided_pairs.iter().map(|x| merge(x.0, x.1)).collect();
    inert.append(&mut merged);

    inert
}

// TODO pipeline/composition
fn update_all(a: &Vec<Object>) -> Vec<Object> {
    let x = collide_all(&a);
    let y = calculate_forces_on_all(&x);
    let z = accelerate_all(&y);

    reposition_all(&z)
}

#[test]
fn test_update_all() {
    let sun = Object {
        position: Vec2(512.0, 512.0),
        mass: 300000.0,
        velocity: VEC_ZERO,
        force: VEC_ZERO,
    };

    let obj = Object {
        position: VEC_ZERO,
        mass: 1.0,
        velocity: VEC_ZERO,
        force: VEC_ZERO,
    };

    let mut objects: Vec<Object> = Vec::new();
    objects.push(sun);
    objects.push(obj);

    let result = update_all(&objects);

    // Mass shouldn't change (and I should be able to enforce this with code right?)
    assert_eq!(sun.mass, result[0].mass);
    assert_eq!(obj.mass, result[1].mass);

    println!("Object: {:?}", objects);
    println!("Result: {:?}", result);
}
