use crate::{Matrix, Point};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Sphere{
    pub center: Point,
    pub radius: f64,
    pub transform: Matrix
}

impl Sphere {
    pub fn new(x:f64,y:f64,z:f64,radius:f64) -> Self{
        Self{
        center : Point{x:x, y:y, z:z},
           radius: radius,
           transform: Matrix::identity(4, 4)
        }
    }

    pub fn default() -> Self{
       Self { 
        center: Point{ x:0.0, y:0.0, z:0.0},
        radius: 1.0 ,
        transform: Matrix::identity(4, 4)
        }
    }

    pub fn set_transform(&mut self,transform: Matrix) {
            self.transform = transform;
    }
}