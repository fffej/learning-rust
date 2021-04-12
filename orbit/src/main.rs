// TODO
// Learn what idiomatic Rust looks like!
// - Get rid of Copy/Clone traits
// - Use mutation confidentally!
// - Replace vomit inducing pairs gubbins near the bottom
// - Improve type safety of Vec2

mod object;
mod vec2;

use rand::prelude::*;
use std::env;
use text_colorizer::*;
use webp_animation::Encoder;

use object::*;
use vec2::*;

const IMAGE_SIZE: u32 = 1024;
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
    const BUFFER_SIZE: usize = (IMAGE_SIZE as usize) * (IMAGE_SIZE as usize);
    let mut encoder = Encoder::new(dimensions).unwrap();

    let mut frame = [0, 0, 0, 255].repeat(BUFFER_SIZE);

    for i in 0..args.iterations {
        objects = update_all(&objects);

        for object in &objects {
            // work out x/y co-ordinates
            let x = object.position.0 as usize;
            let y = object.position.1 as usize;

            // RGBA
            let array_pos: usize = (4usize) * x + (y * (IMAGE_SIZE as usize) * 4usize);

            if array_pos <= frame.len() {
                frame[array_pos] = 255; // ignore the other bits
            } else {
                ignored = ignored + 1;
            }
        }

        encoder.add_frame(&frame, i).unwrap();
    }

    println!("ignored {}", ignored);

    let webp_data = encoder.finalize(args.iterations + 1).unwrap();
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
