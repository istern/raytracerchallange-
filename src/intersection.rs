use crate::{transform, vector::*, Ray, Sphere,matrix::*};


#[derive(Debug, PartialEq,Clone)]
pub enum Shape {
   
    Sphere(Sphere)
}

impl Default for Shape {
    fn default() -> Self {
        Shape::Sphere(Sphere::default()) // ✅ Default to a unit sphere
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Intersections{
    pub intersections: Vec<Inter>,
}

pub fn intersection(ray: &Ray,shape: &Shape) -> Intersection{
    match shape {
        Shape::Sphere(sphere) => intersection_sphere(ray, sphere),
    } 
}

#[derive(Debug, PartialEq, Default)]
pub struct Inter{
    pub t: f64,
    pub object: Shape
}


#[derive(Debug, PartialEq, Default)]
pub struct Intersection{
    pub intersections: Vec<f64>,
    pub object: Shape
}

impl Intersection{
   pub fn flatten(self) -> Vec<Inter> {
        self.intersections
            .into_iter()
            .map(|t| Inter { t, object: self.object.clone() }) // ✅ Convert each `t` into a new struct
            .collect()
    }
}

pub fn intersection_sphere(ray: &Ray,sphere: &Sphere) -> Intersection{
    let ray2 = transform(ray, &inverse(&sphere.transform));

    let sphere_to_ray = ray2.origin - sphere.center;
    let a = dot(ray2.direction,ray2.direction);
    let b = 2.0 * dot(ray2.direction,sphere_to_ray);
    let c = dot(sphere_to_ray,sphere_to_ray) - sphere.radius.powi(2);

    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
        return Intersection {
            intersections: Vec::new(), 
            object: Shape::Sphere(sphere.clone())
        };
    }

    let sqrt_discriminant = discriminant.sqrt();
    let t1 = (-b - sqrt_discriminant) / (2.0 * a);
    let t2 = (-b + sqrt_discriminant) / (2.0 * a);

   
        return Intersection{
            intersections:vec![t1,t2],
            object: Shape::Sphere(sphere.clone())
        } 
 
}