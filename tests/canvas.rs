use cucumber::{ World};
use raytracer::*;

const EPSILON:f64=1e-5;


#[derive(Debug, Default, World)]
pub struct CanvasWorld {
}
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(CanvasWorld::run(
        "tests/features/canvas.feature",
    ));
}