

use cucumber::{given, then, when, World};
use raytracer::*;

#[derive(Debug, Default, World)]
pub struct TestWorld {
   origin: Point,
   direction: Vector,
   ray: Ray,
   m: Matrix,
   r2: Ray,
}

// Given
#[given(regex = r#"origin ← point\((\d), (\d), (\d)\)"#)]
async fn given_origin(world: &mut TestWorld,x:f64,y:f64,z:f64) {
  world.origin = Point{x:x,y:y,z:z};
}

#[given(regex = r#"direction ← vector\((\d), (\d), (\d)\)"#)]
async fn given_direction(world: &mut TestWorld,x:f64,y:f64,z:f64) {
  world.direction = Vector{x:x,y:y,z:z};
}

#[given(regex = r#"r ← ray\(point\((\d), (\d), (\d)\), vector\((\d), (\d), (\d)\)\)"#)]
async fn given_ray(world: &mut TestWorld,x:f64,y:f64,z:f64,i:f64,j:f64,k:f64) {
    world.ray = Ray::new(Point{x:x,y:y,z:z},Vector{x:i,y:j,z:k});
}

#[given(regex = r#"m ← translation\((\d), (\d), (\d)\)"#)]
async fn given_translation(world: &mut TestWorld,x:f64,y:f64,z:f64) {
  world.m = Matrix::translation(x, y, z);
}

#[given(regex = r#"m ← scaling\((\d), (\d), (\d)\)"#)]
async fn given_scaling(world: &mut TestWorld,x:f64,y:f64,z:f64) {
  world.m = Matrix::scaling(x, y, z);
}

//When

#[when(regex = r#"r ← ray\(origin, direction\)"#)]
async fn when_ray(world: &mut TestWorld) {
  world.ray = Ray::new(world.origin, world.direction);
}

#[when(regex = r#"r2 ← transform\(r, m\)"#)]
async fn when_transform(world: &mut TestWorld) {
    world.r2 = transform(&world.ray, &world.m);
}


//Then
#[then(regex = r#"r.origin = origin"#)]
async fn origin_origin(world: &mut TestWorld) {
    assert_eq!(world.ray.origin,world.origin);
}

#[then(regex = r#"r.direction = direction"#)]
async fn direction_direction(world: &mut TestWorld) {
    assert_eq!(world.ray.direction,world.direction);
}

#[then(regex = r#"position\(r, (-*\d\.*\d*)\) = point\((\d+\.*\d*), (\d+\.*\d*), (\d+\.*\d*)\)"#)]
async fn position_at(world: &mut TestWorld,t:f64, x:f64, y:f64, z:f64) {
    let expected = Point{x:x,y:y,z:z};
    let actual = position(&world.ray, t);
    assert_eq!(actual,expected);
}

#[then(regex = r#"r2\.origin = point\((\d+\.*\d*), (\d+\.*\d*), (\d+\.*\d*)\)"#)]
async fn r2_origin(world: &mut TestWorld, x:f64, y:f64, z:f64) {
    let expected = Point{x:x,y:y,z:z};
  
    assert_eq!(world.r2.origin,expected);
}

#[then(regex = r#"r2\.direction = vector\((\d+\.*\d*), (\d+\.*\d*), (\d+\.*\d*)\)"#)]
async fn r2_direction(world: &mut TestWorld, x:f64, y:f64, z:f64) {
    let expected = Vector{x:x,y:y,z:z};
  
    assert_eq!(world.r2.direction,expected);
}


fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(TestWorld::run(
        "tests/features/rays.feature",
    ));
}
