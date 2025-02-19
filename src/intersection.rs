use crate::Sphere;

#[derive(Debug, PartialEq, Default)]
pub struct Intersection{
    pub t: f64,
}


impl Intersection {
    pub fn new(i:f64,o: &Sphere) -> Self{
        Self{
            t:i
        }
    }
}