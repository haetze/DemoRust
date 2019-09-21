use std::io;

#[derive(Copy,Clone,Debug)]
enum Object {
    Sphere(f64),
    Cube(f64),
    Cone(f64, f64),
}

impl Object {
    fn volume(&self) -> f64 {
        let pi = std::f64::consts::PI;
        match self {
            Object::Sphere(r) => (4.0/3.0) * pi * r.powf(3.0),
            Object::Cube(a) => a.powf(3.0),
            Object::Cone(r, h) => (1.0/3.0) * pi * r.powf(2.0) * h,
        }
    }

}


fn main() {
    let cube = Object::Cube(5.6);
    let sphere = Object::Sphere(3.47396);
    let cone = Object::Cone(4.31664,9.0);
    println!("Volume Cube: {}", cube.volume());
    println!("Volume Sphere: {}", sphere.volume());
    println!("Volume Cone: {}", cone.volume());
    
    
}
