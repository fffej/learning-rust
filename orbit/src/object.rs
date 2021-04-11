use super::vec2::{ * };

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Object {
    pub position: Vec2,
    pub mass: f64,
    pub velocity: Vec2,
    pub force: Vec2,
}

pub fn gravity(m1: f64, m2: f64, r: f64) -> f64 {
    if r == 0.0 {
        0.0
    } else {
        (m1 * m2) / (r * r)
    }
}

pub fn force_between(a: &Object, b: &Object) -> Vec2 {
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
