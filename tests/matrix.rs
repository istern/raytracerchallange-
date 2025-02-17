use cucumber::{given,then, gherkin::Step, World};
use raytracer::matrix::*;
use raytracer::tuple::*;

#[derive(Debug, Default, World)]
pub struct MatrixWorld {
    matrix: Matrix,
    matrix_b: Matrix,
    tuple_a: Tuple,
    tuple_b: Tuple,
}



#[given(regex = r#"the following (\d)x(\d) matrix M:"#)]
#[given(regex = r#"the following (\d)x(\d) matrix A:"#)]
async fn given_matrix(world: &mut MatrixWorld,row:usize, col:usize, step: &Step) {
    // Extract table from the Cucumber step context
    let table = step.table().expect("Expected a table but got none");
    world.matrix = Matrix::new(row,col);
    
    for (row_idx, row) in table.rows.iter().enumerate() {
        for (col_idx, value) in row.iter().enumerate() {
            world.matrix[row_idx][col_idx] = value.parse::<f64>().expect("Invalid number in matrix");
        }
    }
}

#[given(regex = r#"the following matrix A:"#)]
async fn given_matrix_fixed_4_x_4_a(world: &mut MatrixWorld, step: &Step) {
    // Extract table from the Cucumber step context
    let table = step.table().expect("Expected a table but got none");
    world.matrix = Matrix::new(4,4);
    
    for (row_idx, row) in table.rows.iter().enumerate() {
        for (col_idx, value) in row.iter().enumerate() {
            world.matrix[row_idx][col_idx] = value.parse::<f64>().expect("Invalid number in matrix");
        }
    }
}

#[given(regex = r#"the following matrix B:"#)]
async fn given_matrix_fixed_4_x_4_b(world: &mut MatrixWorld, step: &Step) {
    // Extract table from the Cucumber step context
    let table = step.table().expect("Expected a table but got none");
    world.matrix_b = Matrix::new(4,4);
    
    for (row_idx, row) in table.rows.iter().enumerate() {
        for (col_idx, value) in row.iter().enumerate() {
            world.matrix_b[row_idx][col_idx] = value.parse::<f64>().expect("Invalid number in matrix");
        }
    }
}

#[given(regex = r#"A ← transpose\(identity_matrix\)"#)]
async fn given_a_transpose_id(world: &mut MatrixWorld) {
    let identity = Matrix::identity(4, 4);
    world.matrix = transpose(&identity);
}


#[given(regex = r#"a ← tuple\((\d), (\d), (\d), (\d)\)"#)]
async fn given_a_and_tuple(world: &mut MatrixWorld, x: f64, y: f64, z: f64, w: f64) {
    world.tuple_a = Tuple{x:x,y:y,z:z,w:w};
}


#[given(regex = r#"b ← tuple\((\d), (\d), (\d), (\d)\)"#)]
async fn given_and_tuple(world: &mut MatrixWorld, x: f64, y: f64, z: f64, w: f64) {
    world.tuple_b = Tuple{x:x,y:y,z:z,w:w};
}

#[then(regex = r#"M\[(\d+),(\d+)\] = (-*\d+.*\d*)"#)]
async fn verify_index(world: &mut MatrixWorld, row: usize, col: usize, expected: f64) {
    let actual = world.matrix[row][col];
    assert_eq!(actual,expected)
}

#[then(regex = r#"A = B"#)]
async fn a_equals_b(world: &mut MatrixWorld,) {
   
    assert_eq!(world.matrix,world.matrix_b)
}

#[then(regex = r#"A != B"#)]
async fn a_differ_b(world: &mut MatrixWorld,) {
   
    assert_ne!(world.matrix,world.matrix_b)
}


#[then(regex = r#"A \* b = tuple\((\d+), (\d+), (\d+), (\d+)\)"#)]
async fn a_times_tuple(world: &mut MatrixWorld, x: f64, y: f64, z: f64, w: f64) {
    
    let expected = Tuple{x:x,y:y,z:z,w:w};
    let actual = &world.matrix * &world.tuple_b;
    assert_eq!(actual,expected);
}

#[then(regex = r#"A \* B is the following 4x4 matrix:"#)]
async fn a_mul_b(world: &mut MatrixWorld,step: &Step) {
   
    let table = step.table().expect("Expected a table but got none");
    let mut expected = Matrix::new(4,4);
    
    for (row_idx, row) in table.rows.iter().enumerate() {
        for (col_idx, value) in row.iter().enumerate() {
            expected[row_idx][col_idx] = value.parse::<f64>().expect("Invalid number in matrix");
        }
    }

    let result = &world.matrix * &world.matrix_b;
    assert_eq!(result,expected)
}


#[then(regex = r#"A \* identity_matrix = A"#)]
async fn a_indeity(world: &mut MatrixWorld) {
   
    let id_matrix = Matrix::identity(4,4);
    let result = &world.matrix * &id_matrix;
  
    assert_eq!(result,world.matrix)
}

#[then(regex = r#"identity_matrix \* a = a"#)]
async fn identity_tuple(world: &mut MatrixWorld) {
   
    let id_matrix = Matrix::identity(4,4);
    let result = &id_matrix * &world.tuple_a;
    
  
    assert_eq!(result,world.tuple_a)
}

#[then(regex = r#"transpose\(A\) is the following matrix:"#)]
async fn transpose_matrix(world: &mut MatrixWorld,step: &Step) {
   
    let table = step.table().expect("Expected a table but got none");
    let mut expected = Matrix::new(4,4);
    
    for (row_idx, row) in table.rows.iter().enumerate() {
        for (col_idx, value) in row.iter().enumerate() {
            expected[row_idx][col_idx] = value.parse::<f64>().expect("Invalid number in matrix");
        }
    }
    let result = transpose(&world.matrix);
    assert_eq!(result,expected)
}

#[then(regex = r#"A = identity_matrix"#)]
async fn a_eq_indeity(world: &mut MatrixWorld) {
   
    let id_matrix = Matrix::identity(4,4);
    
  
    assert_eq!(world.matrix,id_matrix)
}

#[then(regex = r#"determinant\(A\) = (\d+)"#)]
async fn a_determinant(world: &mut MatrixWorld,determinat:f64) {
   
    let result = world.matrix.determinant();
    assert_eq!(result,determinat)
} 

fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(MatrixWorld::run(
        "tests/features/matrices.feature",
    ));
}