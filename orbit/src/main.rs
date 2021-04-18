// TODO
// Learn what idiomatic Rust looks like!
// - Get rid of Copy/Clone traits
// - Use mutation confidentally!
// - Replace vomit inducing pairs gubbins near the bottom
// - Improve type safety of Vec2

mod object;
mod scene;
mod vec2;

use std::env;
use text_colorizer::*;
use webp_animation::Encoder;

use object::*;
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

struct RGBA(u8, u8, u8, u8);

struct Frame {
    values: Vec<u8>,
}

impl Frame {
    pub fn render(&mut self, obj: &Object) {
        let x = obj.position.0 as usize;
        let y = obj.position.1 as usize;

        let weight = obj.mass;
        self.draw_circle(x, y, (weight / 2.0) as i32, &RGBA(255, 255, 255, 255));
    }

    pub fn values(&self) -> &Vec<u8> {
        &self.values
    }

    fn draw_circle(&mut self, xc: usize, yc: usize, radius: i32, pixel: &RGBA) {
        let mut x: i32 = 0;
        let mut y: i32 = radius;
        let mut d: i32 = 3 - 2 * radius;
        self.draw_circle_int(xc, yc, x, y, pixel);

        while y >= x {
            x += 1;
            if d <= 0 {
                d = d + (4 * x) + 6;
            } else {
                y -= 1;
                d = d + 4 * (x - y) + 10;
            }
            self.draw_circle_int(xc, yc, x, y, pixel);
        }
    }

    // usize, but needing negatives results in daftness. Sorry everyone.
    fn draw_circle_int(&mut self, xc: usize, yc: usize, x: i32, y: i32, pixel: &RGBA) {
        let xpos = (xc as i32 + x) as usize;
        let ypos = (yc as i32 + y) as usize;
        let xneg = (xc as i32 - x) as usize;
        let yneg = (yc as i32 - y) as usize;

        self.set_pixel(xpos, ypos, pixel);
        self.set_pixel(xneg, ypos, pixel);
        self.set_pixel(xpos, yneg, pixel);
        self.set_pixel(xneg, yneg, pixel);

        let xpos_ = (xc as i32 + y) as usize;
        let ypos_ = (yc as i32 + x) as usize;
        let xneg_ = (xc as i32 - y) as usize;
        let yneg_ = (yc as i32 - x) as usize;

        self.set_pixel(xpos_, ypos_, pixel);
        self.set_pixel(xneg_, ypos_, pixel);
        self.set_pixel(xpos_, yneg_, pixel);
        self.set_pixel(xneg_, yneg_, pixel);
    }

    fn offset(&self, x: usize, y: usize) -> usize {
        (4usize) * x + (y * (IMAGE_SIZE as usize) * 4usize)
    }

    fn set_pixel(&mut self, x: usize, y: usize, pixel: &RGBA) {
        let array_pos = self.offset(x, y);

        if array_pos + 3 <= self.values.len() {
            self.values[array_pos] = pixel.0;
            self.values[array_pos + 1] = pixel.1;
            self.values[array_pos + 2] = pixel.2;
            self.values[array_pos + 3] = pixel.3;
        }
    }
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
