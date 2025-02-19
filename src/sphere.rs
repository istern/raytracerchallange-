use crate::Point;
use rand::Rng; 

#[derive(Debug, PartialEq, Default)]
pub struct Sphere{
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(x:f64,y:f64,z:f64,radius:f64) -> Self{
        Self{
           center : Point{x:x, y:y, z:z},
           radius: radius
        }
    }

    pub fn random() -> f64{
        let mut rng = rand::thread_rng();
        let num: f64 = rng.gen();
        num
    }
}