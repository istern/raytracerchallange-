

use cucumber::{given, then, when, World};
use raytracer::*;

#[derive(Debug, Default, World)]
pub struct TestWorld {
    sphere: Sphere,
    i: Intersection,
}

// Given
#[given(regex = r#"s ← sphere\(\)"#)]
async fn given_sphere(world: &mut TestWorld) {
    world.sphere = Sphere::new(0.0,0.0,0.0,0.0);
}


//When
#[when(regex = r#"xs ← intersect\(s, r\)"#)]
async fn intersection_point_sphere(world: &mut TestWorld,i: f64) {
    
    world.i = in(i,&world.sphere)
}
//Then
#[then(regex = r#"i\.t = \((\d+\.\d+)\)"#)]
async fn intersection_point_sphere_then(world: &mut TestWorld,expected: f64) { 
    assert_eq!(world.i.t,expected)
}

fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(TestWorld::run(
        "tests/features/intersections.feature",
    ));
}
