use super::object::*;
use super::vec2::*;

use rand::prelude::*;

pub struct Scene {
    pub space_size: u32,
    pub num_objects: u32,
}

impl Scene {
    fn sun(&self) -> Object {
        Object {
            position: Vec2(self.space_size as f64 / 2.0, self.space_size as f64 / 2.0),
            mass: 30.0,
            velocity: VEC_ZERO,
            force: VEC_ZERO,
        }
    }

    fn random_velocity(&self, r: f64, pos: &Vec2) -> Vec2 {
        let sun_direction = unit(&sub(&pos, &self.sun().position));
        let direction = rotate90(&sun_direction);
        scale(&direction, r * 0.3 + 0.3)
    }

    fn random_object(&self, (mass, vel, a, b): (f64, f64, f64, f64)) -> Object {
        let p = self.random_position(a, b);
        Object {
            position: Vec2(p.0, p.1),
            mass: mass * 0.2,
            velocity: self.random_velocity(vel, &p),
            force: VEC_ZERO,
        }
    }

    fn random_position(&self, x: f64, y: f64) -> Vec2 {
        let r = x * 150.0 + 80.0;
        let theta = y * 2.0 * std::f64::consts::PI;
        add(
            &self.sun().position,
            &Vec2(r * theta.cos(), r * theta.sin()),
        )
    }

    pub fn create(&self) -> Vec<Object> {
        let mut objects: Vec<Object> = Vec::new();
        objects.push(self.sun());

        for _i in 0..self.num_objects {
            let x: (f64, f64, f64, f64) = (
                rand::thread_rng().gen::<f64>(), // mass
                rand::thread_rng().gen::<f64>(), // velo
                rand::thread_rng().gen::<f64>(), // pos x
                rand::thread_rng().gen::<f64>(), // pos y
            );
            let obj = self.random_object(x);
            objects.push(obj);
        }

        objects
    }
}
