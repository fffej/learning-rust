// TODO
// Learn what idiomatic Rust looks like!
// - Get rid of Copy/Clone traits
// - Use mutation confidentally!
// - Replace vomit inducing pairs gubbins near the bottom
// - Improve type safety of Vec2

mod object;
mod render;
mod scene;
mod vec2;

use std::env;
use text_colorizer::*;
use webp_animation::Encoder;

use object::*;
use render::*;
use scene::*;

const IMAGE_SIZE: u32 = 1024;

fn main() {
    let args = parse_args();

    let scene = Scene {
        num_objects: args.num_objects as u32,
        space_size: IMAGE_SIZE,
    };
    let mut objects = scene.create();

    let dimensions = (IMAGE_SIZE, IMAGE_SIZE);
    const BUFFER_SIZE: usize = (IMAGE_SIZE as usize) * (IMAGE_SIZE as usize);
    let mut encoder = Encoder::new(dimensions).unwrap();

    for i in 0..args.iterations {
        let mut frame = Frame {
            values: [0, 0, 0, 255].repeat(BUFFER_SIZE),
            size: IMAGE_SIZE as usize,
        };

        objects = update_all(&objects);

        for &object in &objects {
            frame.render(&object);
        }

        encoder.add_frame(&frame.values(), i).unwrap();
    }

    let webp_data = encoder.finalize(args.iterations + 1).unwrap();
    std::fs::write(args.output, webp_data).unwrap();
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
