

use cucumber::{given, then, when, World};
use raytracer::*;

#[derive(Debug, Default, World)]
pub struct TestWorld {
   ray: Ray,
   s: Sphere,
   i: Intersection,
   xs: Intersections,
   t: Matrix,
}

// Given
#[given(regex = r#"r ← ray\(point\((\d), (\d), (-*\d)\), vector\((\d), (\d), (\d)\)\)"#)]
async fn given_ray(world: &mut TestWorld,x:f64,y:f64,z:f64,i:f64,j:f64,k:f64) {
    world.ray = Ray::new(Point{x:x,y:y,z:z},Vector{x:i,y:j,z:k});
}

#[given(regex = r#"s ← sphere\(\)"#)]
async fn given_sphere(world: &mut TestWorld) {
  world.s = Sphere::default();
}


#[given(regex = r#"t ← translation\((\d), (\d), (-*\d)\)"#)]
async fn given_translation(world: &mut TestWorld,x:f64,y:f64,z:f64) {
  world.t = Matrix::translation(x, y, z);
}


//When
#[when(regex = r#"i ← intersect\(s, r\)"#)]
async fn s_intersect_r(world: &mut TestWorld) {
  world.i = intersection_sphere(&world.ray, &world.s)

}

#[when(regex = r#"xs ← intersect\(s, r\)"#)]
async fn s_intersects_r(world: &mut TestWorld) {
    let i = intersection_sphere(&world.ray, &world.s);
    world.xs.intersections = i.flatten();
}

#[when(regex = r#"set_transform\(s, t\)"#)]
async fn s_set_translation(world: &mut TestWorld) {
    let t: Matrix = world.t.clone();
    world.s.set_transform(t);

}

#[when(regex = r#"set_transform\(s, scaling\((\d), (\d), (-*\d)\)\)"#)]
async fn s_set_transform_scaling(world: &mut TestWorld,x:f64,y:f64,z:f64) {
    let t: Matrix = Matrix::scaling(x, y, z);
    world.s.set_transform(t);

}


#[when(regex = r#"set_transform\(s, translation\((\d), (\d), (-*\d)\)\)"#)]
async fn s_set_transform_trans(world: &mut TestWorld,x:f64,y:f64,z:f64) {
    let t: Matrix = Matrix::translation(x, y, z);
    world.s.set_transform(t);

}
//Then
#[then(regex = r#"i.count = (\d+)"#)]
async fn i_count(world: &mut TestWorld,expected:usize) {
  assert_eq!(world.i.intersections.len(),expected)
}

#[then(regex = r#"i\[(\d)\] = (-*\d+)"#)]
async fn i_at(world: &mut TestWorld,index:usize,expected:f64) {
  assert_eq!(world.i.intersections[index],expected)
}

#[then(regex = r#"xs.count = (\d+)"#)]
async fn xs_count(world: &mut TestWorld,expected:usize) {
  assert_eq!(world.xs.intersections.len(),expected)
}

#[then(regex = r#"xs\[(\d+)\].object = s"#)]
async fn xs_at(world: &mut TestWorld,index:usize) {
  assert_eq!(world.xs.intersections[index].object,Shape::Sphere(world.s.clone()));
}


#[then(regex = r#"xs\[(\d+)\].t = (\d+)"#)]
async fn xs_at_t(world: &mut TestWorld,index:usize,expected:f64) {
  assert_eq!(world.xs.intersections[index].t,expected);
} 
#[then(regex = r#"s.transform = identity_matrix"#)]
async fn transform_identity(world: &mut TestWorld) {
  assert_eq!(world.s.transform,Matrix::identity(4, 4));
}

#[then(regex = r#"s.transform = t"#)]
async fn s_transform_t(world: &mut TestWorld) {
  assert_eq!(world.s.transform,world.t);
}



fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(TestWorld::run(
        "tests/features/spheres.feature",
    ));
}
