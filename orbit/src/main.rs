// TODO
// Learn what idiomatic Rust looks like!
// - Get rid of Copy/Clone traits
// - Use mutation confidentally!
// - Replace vomit inducing pairs gubbins near the bottom
// - Improve type safety of Vec2

mod vec2;

use std::env;
use text_colorizer::*;
use rand::prelude::*;
use webp_animation::{Encoder};

use vec2::*;

const IMAGE_SIZE: u32 = 1024;
const VEC_ZERO: Vec2 = Vec2(0.0, 0.0);
const SUN: Object = Object {
    position: Vec2(IMAGE_SIZE as f64 / 2.0, IMAGE_SIZE as f64 / 2.0),
    mass: 30.0,
    velocity: VEC_ZERO,
    force: VEC_ZERO,
};

fn main() {
    let args = parse_args();
    let mut objects: Vec<Object> = Vec::new();
    objects.push(SUN);

    for _i in 0..args.num_objects {
        let x: (f64, f64, f64, f64) = (
            rand::thread_rng().gen::<f64>(), // mass
            rand::thread_rng().gen::<f64>(), // velo
            rand::thread_rng().gen::<f64>(), // pos x
            rand::thread_rng().gen::<f64>(), // pos y
        );
        let obj = random_object(x);
        objects.push(obj);
    }

    println!("Objects{:?}", objects);

    let mut ignored = 0;

    let dimensions = (IMAGE_SIZE, IMAGE_SIZE);
    const BUFFER_SIZE : usize = (IMAGE_SIZE as usize) * (IMAGE_SIZE as usize);
    let mut encoder = Encoder::new(dimensions).unwrap();
    

    let mut frame = [0,0,0,255].repeat(BUFFER_SIZE);

    for i in 0..args.iterations {     

        objects = update_all(&objects);

        for object in &objects {
            // work out x/y co-ordinates
            let x = object.position.0 as usize;
            let y = object.position.1 as usize;

            // RGBA
            let array_pos : usize = (4usize) * x + (y * (IMAGE_SIZE as usize) * 4usize);

            if array_pos <= frame.len() {
                frame[array_pos] = 255; // ignore the other bits
            } else {
                ignored = ignored + 1;
            }
        }

        encoder.add_frame(&frame, i).unwrap();
    }

    println!("ignored {}", ignored);

    let webp_data = encoder.finalize(args.iterations+1).unwrap();
    std::fs::write(args.output, webp_data).unwrap();
}

fn random_velocity(r: f64, pos: &Vec2) -> Vec2 {
    let sun_direction = unit(&sub(&pos, &SUN.position));
    let direction = rotate90(&sun_direction);
    scale(&direction, r * 0.3 + 0.3)
}

fn random_object((mass, vel, a, b): (f64, f64, f64, f64)) -> Object {
    let p = random_position(a, b);
    Object {
        position: Vec2(p.0, p.1),
        mass: mass * 0.2,
        velocity: random_velocity(vel, &p),
        force: VEC_ZERO,
    }
}

fn random_position(x: f64, y: f64) -> Vec2 {
    let r = x * 150.0 + 80.0;
    let theta = y * 2.0 * std::f64::consts::PI;
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

type Position = Vec2;
type Velocity = Vec2;
type Force = Vec2;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Object {
    pub position: Position,
    pub mass: f64,
    pub velocity: Velocity,
    pub force: Force,
}

pub fn gravity(m1: f64, m2: f64, r: f64) -> f64 {
    if r == 0.0 {
        0.0
    } else {
        (m1 * m2) / (r * r)
    }
}

pub fn force_between(a: &Object, b: &Object) -> Force {
    let uv = unit(&sub(&b.position, &a.position));
    let g = gravity(a.mass, b.mass, distance(&a.position, &b.position));

    scale(&uv, g)
}

pub fn accumulate_forces(a: &Object, b: &Vec<Object>) -> Object {
    let f = b
        .iter()
        .fold(VEC_ZERO, |acc, x| add(&acc, &force_between(x, a)));

    //println!("Force {:?}", f);        

    Object {
        position: Vec2(a.position.0, a.position.1),
        mass: a.mass,
        velocity: Vec2(a.velocity.0, a.velocity.1),
        force: f,
    }
}

pub fn calculate_forces_on_all(a: &Vec<Object>) -> Vec<Object> {
    a.iter().map(|o| accumulate_forces(o, a)).collect()
}

pub fn accelerate(o: &Object) -> Object {
    let av = add(&o.velocity, &scale(&o.force, 1.0 / o.mass));

    Object {
        position: Vec2(o.position.0, o.position.1),
        mass: o.mass,
        force: VEC_ZERO,
        velocity: av,
    }
}

pub fn accelerate_all(objs: &Vec<Object>) -> Vec<Object> {
    objs.iter().map(accelerate).collect()
}

pub fn reposition(a: &Object) -> Object {
    Object {
        position: add(&a.position, &a.velocity),
        mass: a.mass,
        velocity: Vec2(a.velocity.0, a.velocity.1),
        force: Vec2(a.force.0, a.force.1),
    }
}

pub fn reposition_all(a: &Vec<Object>) -> Vec<Object> {
    a.iter().map(reposition).collect()
}

pub fn collide(a: &Object, b: &Object) -> bool {
    distance(&a.position, &b.position) <= 3.0
}

pub fn merge(a: &Object, b: &Object) -> Object {
    let mx = a.mass;
    let my = b.mass;
    let merged_mass = mx + my;
    let s = mx / merged_mass;
    let p1 = &a.position;
    let p2 = &b.position;
    let uv = unit(&sub(&p1, &p2));
    let d = scale(&uv, s);
    let mv1 = scale(&a.velocity, mx);
    let mv2 = scale(&b.velocity, my);

    let new_position = add(&p1, &d);
    let new_velocity = scale(&add(&mv1, &mv2), 1.0 / merged_mass);
    let new_force = add(&a.force, &b.force);

    let result = Object {
        position: new_position,
        mass: merged_mass,
        velocity: new_velocity,
        force: new_force
    };

    result
}

pub fn collide_all(a: &Vec<Object>) -> Vec<Object> {
          
    let mut merged: Vec<Object> = Vec::new();
    let mut merged_indices: Vec<usize> = Vec::new();

    for i in 0..a.len() {
        
        for j in i+1..a.len() {            
            if collide(&a[i], &a[j]) {
                merged.push(merge(&a[i],&a[j]));
                merged_indices.push(i);
                merged_indices.push(j);
            }
        }        
    }  

    for i in 0..a.len() {
        if !merged_indices.contains(&i) { 
            merged.push(a[i]);
        }
    }
   

   merged
}

// TODO pipeline/composition
pub fn update_all(a: &Vec<Object>) -> Vec<Object> {
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
}
