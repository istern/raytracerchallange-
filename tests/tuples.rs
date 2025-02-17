
use cucumber::{given,when,then, World};

use raytracer::tuple::*; // `self` includes reflect and cross
use raytracer::vector::*;
use raytracer::point::*;
use raytracer::color::*;
const EPSILON:f64=1e-5;


#[derive(Debug, Default, World)]
pub struct TestWorld {
    tuple: Tuple,
    tuple_2: Tuple,
    point: Point,
    point_2: Point,
    vector: Vector,
    vector_2: Vector,
    r: Vector,
    norm: Vector,
    color: Color,
    color_2: Color,
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
#[given(regex = r#"a ← vector\(([-\d.]+), ([-\d.]+), ([-\d.]+)\)"#)]
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
#[given(regex = r#"b ← vector\(([-\d.]+), ([-\d.]+), ([-\d.]+)\)"#)]
#[given(regex = r#"n ← vector\(([-\d.]+), ([-\d.]+), ([-\d.]+)\)"#)]
async fn set_vector_2(world: &mut TestWorld, x: f64, y: f64, z: f64) {
    let vector = Vector{
        x:x, 
        y:y, 
        z:z,
    };
    world.vector_2 = vector;
}

#[given(regex = r#"n ← vector\(√([-*\d]\/[2]), √([-*\d]\/[2]), √*([-*\d]\/*[2]*)"#)]
async fn set_vector_2_sqrt(world: &mut TestWorld) {
    let vector = Vector{
        x:(2.0_f64).sqrt() / 2.0, 
        y:(2.0_f64).sqrt() / 2.0, 
        z:0.0_f64,
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

#[then(regex = r#"a \* ([-\d.]+) = tuple\(([-\d.]+), ([-\d.]+), ([-\d.]+), ([-\d.]+)\)"#)]
async fn multiply_tuple(world: &mut TestWorld, i:f64 ,x: f64, y: f64, z: f64,w: f64) {
   let result = world.tuple * i;
   let expected = Tuple{ x, y, z, w};
   assert_eq!(result,expected)
}

#[then(regex = r#"a / ([-\d.]+) = tuple\(([-\d.]+), ([-\d.]+), ([-\d.]+), ([-\d.]+)\)"#)]
async fn divsion_tuple(world: &mut TestWorld, i:f64 ,x: f64, y: f64, z: f64,w: f64) {
   let result = world.tuple / i;
   let expected = Tuple{ x, y, z, w};
   assert_eq!(result,expected)
}

#[then(regex = r#"magnitude\(v\) = ([-\d.]+)"#)]
async fn magnitude_vector(world: &mut TestWorld,expected: f64) {
   let result = magnitude(world.vector);
   assert_eq!(result,expected)
}

#[then(regex = r#"magnitude\(v\) = √([-\d.]+)"#)]
async fn magnitude_vector_sqrt(world: &mut TestWorld,number: f64) {
   let result = magnitude(world.vector);
   let expected = number.sqrt();
   assert_eq!(result,expected)
}

#[then(regex = r#"normalize\(v\) = vector\(([-\d]+), ([-\d]+), ([-\d]+)\)"#)]
#[then(regex = r#"normalize\(v\) = approximately vector\(([\d].[\d]+), ([\d].[\d]+), ([\d].[\d]+)\)"#)]
async fn normalize_vector(world: &mut TestWorld,x: f64, y: f64, z: f64) {
   let result = normalize(world.vector);
   let expected = Vector{x:x,y:y,z:z};
   assert!((result.x - expected.x).abs() < EPSILON);
   assert!((result.y - expected.y).abs() < EPSILON);
   assert!((result.z - expected.z).abs() < EPSILON);
}

#[when(regex = r#"norm ← normalize\(v\)"#)]
async fn set_norm_normalize_vector(world: &mut TestWorld) {
   world.norm = normalize(world.vector);
}

#[then(regex = r#"magnitude\(norm\) = ([-\d.]+)"#)]
async fn verify_norm_vector(world: &mut TestWorld,expected: f64) {
   let result = magnitude(world.norm);
   assert_eq!(result,expected);
}

#[then(regex = r#"dot\(a, b\) = ([-\d.]+)"#)]
async fn dot_vector(world: &mut TestWorld,expected: f64) {
   let result = dot(world.vector,world.vector_2);
   assert_eq!(result,expected);
}

#[then(regex = r#"cross\(a, b\) = vector\((-*[\d+]), (-*[\d+]), (-*[\d])\)"#)]
async fn cross_vector(world: &mut TestWorld,x: f64, y: f64, z: f64) {
   let result = cross(world.vector,world.vector_2);
   let expected = Vector{x:x,y:y,z:z};
   assert_eq!(result,expected);
}

#[then(regex = r#"cross\(b, a\) = vector\((-*[\d+]), (-*[\d+]), (-*[\d])\)"#)]
async fn cross_vector_2_vector(world: &mut TestWorld,x: f64, y: f64, z: f64) {
   let result = cross(world.vector_2,world.vector);
   let expected = Vector{x:x,y:y,z:z};
   assert_eq!(result,expected);
}


#[given(regex = r#"c ← color\((-*[\d]+.[\d]+), (-*[\d]+.[\d]+), (-*[\d]+.[\d]+)\)"#)]
#[given(regex = r#"c1 ← color\((-*[\d]+.*[\d]*), (-*[\d]+.[\d]+), (-*[\d]+.[\d]+)\)"#)]
async fn set_color(world: &mut TestWorld, red: f64, green: f64, blue: f64) {
    let color = Color{
        red:red, 
        green:green, 
        blue:blue,
    };
    world.color = color;
}

#[given(regex = r#"c2 ← color\((-*[\d]+.*[\d]*), (-*[\d]+.*[\d]*), (-*[\d]+.*[\d]*)\)"#)]
async fn set_color_2(world: &mut TestWorld, red: f64, green: f64, blue: f64) {
    let color = Color{
        red:red, 
        green:green, 
        blue:blue,
    };
    world.color_2 = color;
}

#[then(regex = r#"c\.red = (-*[\d]+.[\d]+)"#)]
async fn color_red(world: &mut TestWorld,expected:f64) {
   assert_eq!(world.color.red,expected);
}

#[then(regex = r#"c\.green = (-*[\d]+.[\d]+)"#)]
async fn color_green(world: &mut TestWorld,expected:f64) {
   assert_eq!(world.color.green,expected);
}

#[then(regex = r#"c\.blue = (-*[\d]+.[\d]+)"#)]
async fn color_blue(world: &mut TestWorld,expected:f64) {
   assert_eq!(world.color.blue,expected);
}

#[then(regex = r#"c1 \+ c2 = color\((-*[\d].[\d]), (-*[\d].[\d]), (-*[\d].[\d])\)"#)]
async fn color_add(world: &mut TestWorld,red: f64, green: f64, blue: f64) {
   let result = world.color + world.color_2;
   let expected = Color{
    red:red,
    green:green,
    blue:blue,
   };
   assert_eq!(result,expected);
}

#[then(regex = r#"c1 - c2 = color\((-*[\d].[\d]), (-*[\d].[\d]), (-*[\d].[\d])\)"#)]
async fn color_sub(world: &mut TestWorld,red: f64, green: f64, blue: f64) {
   let result = world.color - world.color_2;
   let expected = Color{
    red:red,
    green:green,
    blue:blue,
   };
   assert!((result.red - expected.red).abs() < EPSILON);
   assert!((result.green - expected.green).abs() < EPSILON);
   assert!((result.blue - expected.blue).abs() < EPSILON);
}

#[then(regex = r#"c \* ([-\d.]+) = color\((-*[\d].[\d]), (-*[\d].[\d]), (-*[\d].[\d])\)"#)]
async fn multiply_color(world: &mut TestWorld, i:f64 ,red: f64, green: f64, blue: f64) {
   let result = world.color * i;
   let expected = Color{ red:red, green:green, blue:blue};
   assert_eq!(result,expected)
}

#[then(regex = r#"c1 \* c2 = color\((-*[\d].*[\d]*), (-*[\d].*[\d]*), (-*[\d].*[\d]*)\)"#)]
async fn multiply_color_with_color(world: &mut TestWorld, red: f64, green: f64, blue: f64) {
   let result = world.color * world.color_2;
   let expected = Color{ red:red, green:green, blue:blue};
   assert!((result.red - expected.red).abs() < EPSILON);
   assert!((result.green - expected.green).abs() < EPSILON);
   assert!((result.blue - expected.blue).abs() < EPSILON);
}

#[when(regex = r#"r ← reflect\(v, n\)"#)]
async fn when_reflect_vector(world: &mut TestWorld) {
   world.r = reflect(world.vector,world.vector_2);
}

#[then(regex = r#"r = vector\(([-\d]+), ([-\d]+), ([-\d]+)\)"#)]
async fn then_reflect_vector(world: &mut TestWorld,x: f64, y: f64, z: f64) {
   let expected = Vector{
    x:x,
    y:y,
    z:z,
   };
   assert!((world.r.x - expected.x).abs() < EPSILON);
   assert!((world.r.y - expected.y).abs() < EPSILON);
   assert!((world.r.z - expected.z).abs() < EPSILON);
}

fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(TestWorld::run(
        "tests/features/tuples.feature",
    ));
}