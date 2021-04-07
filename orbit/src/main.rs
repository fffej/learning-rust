// TODO
// Learn what idiomatic Rust looks like!
// - Get rid of Copy/Clone traits
// - Use mutation confidentally!
// - Replace vomit inducing pairs gubbins near the bottom
// - Improve type safety of Vec2

use text_colorizer::*;
use std::env;

fn main() {
    let args = parse_args();
}

#[derive(Debug)]
struct Arguments {
    num_objects: i32,
    delta: i32,
    output: String
}

fn print_usage() {
   eprintln!("{} - simulate some bodies under gravity", "orbit".green());
   eprintln!("Usage: orbit <num_objects> <delta>  <output>");
}

fn parse_args() -> Arguments {
    
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 3 {
        print_usage();
        eprintln!("{} wrong number of arguments:@ expected 3, got {}.", "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        num_objects: args[0].parse().unwrap(),
        delta: args[1].parse().unwrap(),
        output: args[2].clone()
    }
}


#[derive(Debug, PartialEq, Copy, Clone)]
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

#[derive(Debug, PartialEq, Copy, Clone)]
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

fn calculate_forces_on_all(a : &Vec<Object>) -> Vec<Object> {
  a.iter().map(|o| accumulate_forces(o,a)).collect()
}

fn reposition(a: &Object) -> Object {
    Object {
        position: add(&a.position, &a.velocity),
        mass: a.mass,
        velocity: Vec2(a.velocity.0, a.velocity.1),
        force: Vec2(a.force.0, a.force.1)
    }
}

fn reposition_all(a: &Vec<Object>) -> Vec<Object> {
    a.iter().map(|o| reposition(o)).collect()
}


fn collide(a: &Object, b: &Object) -> bool {
    distance (&a.position, &b.position) <= 3.0
}

fn merge(a: &Object, b: &Object) -> Object {
    let mx = a.mass;
    let my = b.mass;
    let merged_mass = mx + my;
    let s = mx / merged_mass;
    let p1 = &a.position;
    let p2 = &b.position;
    let uv = unit(&sub(&p2,&p1));
    let d = scale(&uv,s);
    let mv1 = scale(&a.velocity, mx);
    let mv2 = scale(&b.velocity, my);

    Object {
        position: add(&p1,&d),
        mass: merged_mass,
        velocity: scale(&add(&mv1,&mv2), 1.0/merged_mass),
        force: add(&a.force, &b.force)
    }

}

fn collide_all(a: &Vec<Object>) -> Vec<Object> {
    Vec::new()
}

fn update_all(a: &Vec<Object>) -> Vec<Object> {

    let r:Vec<Object> = Vec::new();
    let mut collided_pairs:Vec<(&Object,&Object)> = Vec::new();
    let mut inert:Vec<Object> = Vec::new();

    // Find all the pairs that have collided
    for src in a.iter() {
        for tgt in a.iter() {
            if src == tgt {
                continue;
            }

            let t = (src,tgt);

            // This makes me vomit  
            if collide(src,tgt) && !collided_pairs.contains( &(tgt,src) ){
                    collided_pairs.push( (src,tgt) );
            }
        }
    }

    // Find all the objects not involved in collisions
    for obj in a.iter() {
        let mut found = false;
        for (src,tgt) in collided_pairs.iter() {
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
    let mut merged : Vec<Object> = collided_pairs.iter().map(| x | merge(x.0,x.1)).collect();
    inert.append(&mut merged);

    inert
}