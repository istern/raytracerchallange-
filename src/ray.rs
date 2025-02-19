use crate::{Matrix, Point, Vector};

#[derive(Debug, PartialEq, Default)]
pub struct Ray{
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
       pub fn new(origin: Point, direction:Vector)-> Self{
        Self{origin:origin,direction:direction}
    }
}

pub fn position(ray: &Ray, t: f64)->Point{
    ray.origin + ray.direction * t
}

pub fn transform(ray: &Ray, matrix: &Matrix) -> Ray {
    Ray {
        origin: matrix * &ray.origin, // ✅ Transform the origin
        direction: matrix * &ray.direction, // ✅ Transform the direction
    }
}