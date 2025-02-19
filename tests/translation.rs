use cucumber::{given, then, when, World};
use raytracer::*;

#[derive(Debug, Default, World)]
pub struct TransfomrationWorld {
    translation: Matrix,
    inv: Matrix,
    point: Point,
    vector: Vector,
    from: Point,
    to: Point,
    up: Vector,
    half_quarter: Matrix,
    full_quarter: Matrix,
    a: Matrix,
    b: Matrix,
    c: Matrix,
    p2: Point,
    p3: Point,
    p4: Point,
    t: Matrix,
}



#[given(regex = r#"transform ← translation\((-*\d+), (-*\d+), (-*\d)\)"#)]
async fn given_transformation(world: &mut TransfomrationWorld, x: f64, y: f64, z: f64) {
    world.translation = Matrix::translation(x, y, z);
}

#[given(regex = r#"transform ← scaling\((-*\d+), (-*\d+), (-*\d)\)"#)]
async fn given_scaling(world: &mut TransfomrationWorld, x: f64, y: f64, z: f64) {
    world.translation = Matrix::scaling(x, y, z);
}

#[given(regex = r#"p ← point\((-*\d+), (-*\d+), (-*\d)\)"#)]
async fn given_point(world: &mut TransfomrationWorld, x: f64, y: f64, z: f64) {
    world.point = Point { x: x, y: y, z: z };
}

#[given(regex = r#"from ← point\((-*\d+), (-*\d+), (-*\d)\)"#)]
async fn given_from(world: &mut TransfomrationWorld, x: f64, y: f64, z: f64) {
    world.from = Point { x: x, y: y, z: z };
}

#[given(regex = r#"to ← point\((-*\d+), (-*\d+), (-*\d)\)"#)]
async fn given_to(world: &mut TransfomrationWorld, x: f64, y: f64, z: f64) {
    world.to = Point { x: x, y: y, z: z };
}

#[given(regex = r#"up ← vector\((-*\d+), (-*\d+), (-*\d)\)"#)]
async fn given_up(world: &mut TransfomrationWorld, x: f64, y: f64, z: f64) {
    world.up = Vector { x: x, y: y, z: z };
}

#[given(regex = r#"inv ← inverse\(transform\)"#)]
async fn given_inv_transformation(world: &mut TransfomrationWorld) {
    world.inv = inverse(&world.translation);
}

#[given(regex = r#"v ← vector\((-*\d+), (-*\d+), (-*\d+)\)"#)]
async fn given_vector(world: &mut TransfomrationWorld, x: f64, y: f64, z: f64) {
    world.vector = Vector { x: x, y: y, z: z };
}

#[given(regex = r#"half_quarter ← rotation_x\(π / 4\)"#)]
async fn given_rotate_x_4(world: &mut TransfomrationWorld) {
    world.half_quarter = Matrix::rotation_x(std::f64::consts::PI / 4.0);
}

#[given(regex = r#"full_quarter ← rotation_x\(π / 2\)"#)]
async fn given_rotate_x_2(world: &mut TransfomrationWorld) {
    world.full_quarter = Matrix::rotation_x(std::f64::consts::PI / 2.0);
}

#[given(regex = r#"half_quarter ← rotation_y\(π / 4\)"#)]
async fn given_rotate_y_4(world: &mut TransfomrationWorld) {
    world.half_quarter = Matrix::rotation_y(std::f64::consts::PI / 4.0);
}

#[given(regex = r#"full_quarter ← rotation_y\(π / 2\)"#)]
async fn given_rotate_y_2(world: &mut TransfomrationWorld) {
    world.full_quarter = Matrix::rotation_y(std::f64::consts::PI / 2.0);
}


#[given(regex = r#"half_quarter ← rotation_z\(π / 4\)"#)]
async fn given_rotate_z(world: &mut TransfomrationWorld) {
    world.half_quarter =  Matrix::rotation_z(std::f64::consts::PI / 4.0)
}

#[given(regex = r#"full_quarter ← rotation_z\(π / 2\)"#)]
async fn given_rotate_z_2(world: &mut TransfomrationWorld) {
    world.full_quarter = Matrix::rotation_z(std::f64::consts::PI / 2.0);
}

#[given(regex = r#"inv ← inverse\(half_quarter\)"#)]
async fn given_inverse_half(world: &mut TransfomrationWorld) {
    world.inv = inverse(&world.half_quarter);
}


#[given(regex = r#"transform ← shearing\((-*\d+), (-*\d+), (-*\d+), (-*\d+), (-*\d+), (-*\d+)\)"#)]
async fn given_shearing(world: &mut TransfomrationWorld,xy:f64,xz:f64,yx:f64,yz:f64,zx:f64,zy:f64) {
    world.translation = Matrix::shearing(xy,xz,yx,yz,zx,zy);
}


#[given(regex = r#"A ← rotation_x\(π / 2\)"#)]
async fn given_a_rotation(world: &mut TransfomrationWorld) {
    world.a = Matrix::rotation_x(std::f64::consts::PI / 2.0)
}

#[given(regex = r#"B ← scaling\((-*\d+), (-*\d+), (-*\d+)\)"#)]
async fn given_b_scaling(world: &mut TransfomrationWorld,x:f64,y:f64,z:f64) {
    world.b = Matrix::scaling(x,y,z);
}

#[given(regex = r#"C ← translation\((-*\d+), (-*\d+), (-*\d+)\)"#)]
async fn given_c_translation(world: &mut TransfomrationWorld,x:f64,y:f64,z:f64) {
    world.c = Matrix::translation(x,y,z);
}






#[when(regex = r#"p2 ← A \* p"#)]
async fn p2_a(world: &mut TransfomrationWorld) {
   world.p2 = &world.a * &world.point;
}

#[when(regex = r#"p3 ← B \* p2"#)]
async fn p3(world: &mut TransfomrationWorld) {
   world.p3 = &world.b * &world.p2;
}

#[when(regex = r#"p4 ← C \* p3"#)]
async fn p4(world: &mut TransfomrationWorld) {
   world.p4 = &world.c * &world.p3;
}

#[when(regex = r#"T ← C \* B \* A"#)]
async fn t_c_b_a(world: &mut TransfomrationWorld) {
   let c_b = &world.c * &world.b; 
   world.t = &c_b * &world.a;
}

#[then(regex = r#"transform \* p = point\((-*\d+), (-*\d+), (-*\d+)\)"#)]
async fn transform_point_matrix(world: &mut TransfomrationWorld, x: f64, y: f64, z: f64) {
    let expected = Point { x: x, y: y, z: z };
    let result = &world.translation * &world.point;

    assert_eq!(result, expected);
}

#[then(regex = r#"transform \* v = v$"#)]
async fn transform_vector(world: &mut TransfomrationWorld) {
    let result = &world.translation * &world.vector;
    assert_eq!(result, world.vector);
}

#[then(regex = r#"transform \* v = vector\((-*\d+), (-*\d+), (-*\d+)\)"#)]
async fn transform_vector_with_values(world: &mut TransfomrationWorld, x: f64, y: f64, z: f64) {
    let expected = Vector { x: x, y: y, z: z };
    let result = &world.translation * &world.vector;
    assert_eq!(result, expected);
}

#[then(regex = r#"inv \* v = vector\((-*\d+), (-*\d+), (-*\d+)\)"#)]
async fn inv_scaling_vector_matrix(world: &mut TransfomrationWorld, x: f64, y: f64, z: f64) {
    let expected = Vector { x: x, y: y, z: z };
    let result = &world.inv * &world.vector;

    assert_eq!(result, expected);
}

#[then(regex = r#"inv \* p = point\((-*\d+), (-*\d+), (-*\d+)\)"#)]
async fn inv_point_matrix(world: &mut TransfomrationWorld, x: f64, y: f64, z: f64) {
    let expected = Point { x: x, y: y, z: z };
    let result = &world.inv * &world.point;

    assert_eq!(result, expected);
}

#[then(regex = r#"half_quarter \* p = point\(0, √2/2, √2/2\)"#)]
async fn half_x_point(world: &mut TransfomrationWorld) {
    let expected = Point {
        x: 0.0,
        y: (2.0_f64).sqrt() / 2.0,
        z: (2.0_f64).sqrt() / 2.0,
    };
    let result = &world.half_quarter * &world.point;

    assert_eq!(result, expected);
}

#[then(regex = r#"full_quarter \* p = point\((-*\d+), (-*\d+), (-*\d+)\)"#)]
async fn full_x_point(world: &mut TransfomrationWorld, x: f64, y: f64, z: f64) {
    let expected = Point { x: x, y: y, z: z };
    let result = &world.full_quarter * &world.point;
    let epsilon = 1e-10;
    assert!(expected.approx_eq(&result, epsilon), "Points are not approximately equal!");
}

#[then(regex = r#"half_quarter \* p = point\(√2/2, 0, √2/2\)"#)]
async fn half_y_point(world: &mut TransfomrationWorld) {
    let expected = Point {
        x: (2.0_f64).sqrt() / 2.0,
        y: 0.0,
        z: (2.0_f64).sqrt() / 2.0,
    };
    let result = &world.half_quarter * &world.point;

    assert_eq!(result, expected);
}

#[then(regex = r#"inv \* p = point\(0, √2/2, -√2/2\)"#)]
async fn inv_half_y_point(world: &mut TransfomrationWorld) {
    let expected = Point {
        x: 0.0,
        y: (2.0_f64).sqrt() / 2.0,
        z: -(2.0_f64).sqrt() / 2.0,
    };
    let result = &world.inv * &world.point;

    let epsilon = 1e-1;
    assert!(result.approx_eq(&expected, epsilon));
}


#[then(regex = r#"half_quarter \* p = point\(-√2/2, √2/2, 0\)"#)]
async fn hald_half_z_point(world: &mut TransfomrationWorld) {
    let expected = Point {
        x:  -(2.0_f64).sqrt() / 2.0,
        y: (2.0_f64).sqrt() / 2.0,
        z: 0.0,
    };
    let result = &world.half_quarter * &world.point;

   
    assert_eq!(result, expected);
}


#[then(regex = r#"p2 = point\((-*\d+), (-*\d+), (-*\d+)\)"#)]
async fn p2_point(world: &mut TransfomrationWorld, x: f64, y: f64, z: f64) {
    let expected = Point { x: x, y: y, z: z };

    assert!(world.p2.approx_eq(&expected, 1e-1));
}


#[then(regex = r#"p3 = point\((-*\d+), (-*\d+), (-*\d+)\)"#)]
async fn p3_point(world: &mut TransfomrationWorld, x: f64, y: f64, z: f64) {
    let expected = Point { x: x, y: y, z: z };

    assert!(world.p3.approx_eq(&expected, 1e-1));
}

#[then(regex = r#"p4 = point\((-*\d+), (-*\d+), (-*\d+)\)"#)]
async fn p4_point(world: &mut TransfomrationWorld, x: f64, y: f64, z: f64) {
    let expected = Point { x: x, y: y, z: z };

    assert!(world.p4.approx_eq(&expected, 1e-1));
}

#[then(regex = r#"T \* p = point\((-*\d+), (-*\d+), (-*\d+)\)"#)]
async fn t_point(world: &mut TransfomrationWorld, x: f64, y: f64, z: f64) {
    let expected = Point { x: x, y: y, z: z };
    let result = &world.t * &world.point;
    assert!(result.approx_eq(&expected, 1e-1));
}
   

fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(TransfomrationWorld::run(
        "tests/features/transformations.feature",
    ));
}
