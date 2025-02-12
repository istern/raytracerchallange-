
use cucumber::{given,then, World};
use tuples::*;


#[derive(Debug, Default, World)]
pub struct TestWorld {
    tuple: Tuple,
    tuple_2: Tuple,
    point: Point,
    point_2: Point,
    vector: Vector,
    vector_2: Vector,
}

#[given(regex = r#"a ← tuple\(([-\d.]+), ([-\d.]+), ([-\d.]+), ([-\d.]+)\)"#)]
#[given(regex = r#"a1 ← tuple\(([-\d]+), ([-\d]+), ([-\d]+), ([-\d]+)\)"#)]
async fn set_tuple(world: &mut TestWorld, x: f64, y: f64, z: f64, w: f64) {
    let tuple = Tuple{
        x:x, 
        y:y, 
        z:z, 
        w:w
    };
    world.tuple = tuple;
}

#[then(regex = r#"a\.x = ([-\d.]+)"#)]
async fn check_x(world: &mut TestWorld, expected_x: f64) {
    assert_eq!(world.tuple.x, expected_x);
}

#[then(regex = r#"a\.y = ([-\d.]+)"#)]
async fn check_y(world: &mut TestWorld, expected_y: f64) {
    assert_eq!(world.tuple.y, expected_y);
}

#[then(regex = r#"a\.z = ([-\d.]+)"#)]
async fn check_z(world: &mut TestWorld, expected_z: f64) {
    assert_eq!(world.tuple.z, expected_z);
}

#[then(regex = r#"a\.w = ([-\d.]+)"#)]
async fn check_w(world: &mut TestWorld, expected_w: f64) {
    assert_eq!(world.tuple.w, expected_w);
}


#[then(regex = r#"a is a point"#)]
async fn check_is_point(world: &mut TestWorld) {
    assert!(world.tuple.is_point());
}

#[then(regex = r#"a is not a vector"#)]
async fn check_is_not_vector(world: &mut TestWorld) {
    assert!(!world.tuple.is_vector());
}

#[then(regex = r#"a is not a point"#)]
async fn check_is_not_point(world: &mut TestWorld) {
    assert!(!world.tuple.is_point());
}

#[then(regex = r#"a is a vector"#)]
async fn check_is_vector(world: &mut TestWorld) {
    assert!(world.tuple.is_vector());
}

#[given(regex = r#"p ← point\(([-\d.]+), ([-\d.]+), ([-\d.]+)\)"#)]
#[given(regex = r#"p1 ← point\(([-\d.]+), ([-\d.]+), ([-\d.]+)\)"#)]
async fn set_point(world: &mut TestWorld, x: f64, y: f64, z: f64) {
    let point = Point{
        x:x, 
        y:y, 
        z:z,
    };
    world.point = point;
}

#[given(regex = r#"p2 ← point\(([-\d.]+), ([-\d.]+), ([-\d.]+)\)"#)]
async fn set_point_2(world: &mut TestWorld, x: f64, y: f64, z: f64) {
    let point = Point{
        x:x, 
        y:y, 
        z:z,
    };
    world.point_2 = point;
}

#[then(regex = r#"p = tuple\(([-\d]+), ([-\d]+), ([-\d]+), ([-\d]+)\)"#)]
async fn check_p_equals_tuple(world: &mut TestWorld, x: f64, y: f64, z: f64, w: f64) {
    
    let expected = Tuple { x, y, z, w };


    assert_eq!(world.point, expected, "Expected tuple ({:?}), but got ({:?})", expected, world.point);
}


#[given(regex = r#"v ← vector\(([-\d.]+), ([-\d.]+), ([-\d.]+)\)"#)]
#[given(regex = r#"v1 ← vector\(([-\d.]+), ([-\d.]+), ([-\d.]+)\)"#)]
async fn set_vector(world: &mut TestWorld, x: f64, y: f64, z: f64) {
    let vector = Vector{
        x:x, 
        y:y, 
        z:z,
    };
    world.vector = vector;
}

#[given(regex = r#"v2 ← vector\(([-\d.]+), ([-\d.]+), ([-\d.]+)\)"#)]
#[given(regex = r#"zero ← vector\(([-\d.]+), ([-\d.]+), ([-\d.]+)\)"#)]
async fn set_vector_2(world: &mut TestWorld, x: f64, y: f64, z: f64) {
    let vector = Vector{
        x:x, 
        y:y, 
        z:z,
    };
    world.vector_2 = vector;
}

#[then(regex = r#"v = tuple\(([-\d]+), ([-\d]+), ([-\d]+), ([-\d]+)\)"#)]
async fn check_v_equals_tuple(world: &mut TestWorld, x: f64, y: f64, z: f64, w: f64) {
    
    let expected = Tuple { x, y, z, w };
    assert_eq!(world.vector, expected, "Expected tuple ({:?}), but got ({:?})", expected, world.vector);
}

#[given(regex = r#"a2 ← tuple\(([-\d]+), ([-\d]+), ([-\d]+), ([-\d]+)\)"#)]
async fn set_a2(world: &mut TestWorld, x: f64, y: f64, z: f64, w: f64) {
    world.tuple_2 = Tuple { x, y, z, w };
}

#[then(regex =r#"a1 \+ a2 = tuple\(([-\d]+), ([-\d]+), ([-\d]+), ([-\d]+)\)"#)]
async fn a1_add_a2(world: &mut TestWorld, x: f64, y: f64, z: f64, w: f64) {
   let result = world.tuple + world.tuple_2;
   let expected = Tuple{ x, y, z, w };
   assert_eq!(result,expected)
}

#[then(regex =r#"p1 - p2 = vector\(([-\d]+), ([-\d]+), ([-\d]+)\)"#)]
async fn p1_sub_p2(world: &mut TestWorld, x: f64, y: f64, z: f64) {
   let result = world.point - world.point_2;
   let expected = Vector{ x, y, z};
   assert_eq!(result,expected)
}

#[then(regex =r#"p - v = point\(([-\d]+), ([-\d]+), ([-\d]+)\)"#)]
async fn vector_sub_point(world: &mut TestWorld, x: f64, y: f64, z: f64) {
   let result = world.point - world.vector;
   let expected = Point{ x, y, z};
   assert_eq!(result,expected)
}

#[then(regex =r#"v1 - v2 = vector\(([-\d]+), ([-\d]+), ([-\d]+)\)"#)]
async fn vector_sub_vector(world: &mut TestWorld, x: f64, y: f64, z: f64) {
   let result = world.vector - world.vector_2;
   let expected = Vector{ x, y, z};
   assert_eq!(result,expected)
}

#[then(regex =r#"zero - v = vector\(([-\d]+), ([-\d]+), ([-\d]+)\)"#)]
async fn vector_sub_zero_vector(world: &mut TestWorld, x: f64, y: f64, z: f64) {
   let result = world.vector_2 - world.vector;
   let expected = Vector{ x, y, z};
   assert_eq!(result,expected)
}

#[then(regex =r#"-a = tuple\(([-\d]+), ([-\d]+), ([-\d]+), ([-\d]+)\)"#)]
async fn negate_tuple(world: &mut TestWorld, x: f64, y: f64, z: f64,w: f64) {
   let result = -world.tuple;
   let expected = Tuple{ x, y, z, w};
   assert_eq!(result,expected)
}


fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(TestWorld::run(
        "tests/features/tuples.feature",
    ));
}