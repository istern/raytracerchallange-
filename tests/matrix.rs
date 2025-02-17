use cucumber::{given,then, gherkin::Step, World};
use raytracer::matrix::*;
use raytracer::tuple::*;

#[derive(Debug, Default, World)]
pub struct MatrixWorld {
    matrix: Matrix,
    matrix_b: Matrix,
    matrix_c: Matrix,
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
#[given(regex = r#"the following 4x4 matrix B:"#)]
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


#[given(regex = r#"C ← A \* B"#)]
async fn c_from_a_b(world: &mut MatrixWorld) {
    world.matrix_c = &world.matrix * &world.matrix_b;
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

#[given(regex = r#"B ← submatrix\(A, (\d), (\d)\)"#)]
async fn given_and_b_sub_a(world: &mut MatrixWorld, row: usize, col: usize) {
    world.matrix_b =  submatrix(&world.matrix, row, col);
}

#[given(regex = r#"B ← inverse\(A\)"#)]
async fn b_eq_inverse_a(world: &mut MatrixWorld) {
    world.matrix_b = inverse(&world.matrix)
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

#[then(regex = r#"determinant\(A\) = (-*\d+)"#)]
async fn a_determinant(world: &mut MatrixWorld,determinat:f64) {
   
    let result = determinant(&world.matrix);
    assert_eq!(result,determinat)
} 

#[then(regex = r#"determinant\(B\) = (\d+)"#)]
async fn b_determinant(world: &mut MatrixWorld,determinat:f64) {
   
    let result = determinant(&world.matrix_b);
    assert_eq!(result,determinat)
} 


#[then(regex = r#"minor\(A, (\d), (\d)\) = (-*\d+)"#)]
async fn a_minor(world: &mut MatrixWorld,row:usize, col:usize, determinat:f64) {
   
    let result = minor(&world.matrix,row,col);
    assert_eq!(result,determinat)
} 

#[then(regex = r#"cofactor\(A, (\d), (\d)\) = (-*\d+)"#)]
async fn a_cofactor(world: &mut MatrixWorld,row:usize, col:usize, determinat:f64) {
   
    let result = cofactor(&world.matrix,row,col);
    assert_eq!(result,determinat)
} 



#[then(regex = r#"A is invertible"#)]
async fn a_is_inverttible(world: &mut MatrixWorld) {

    assert!(is_invertible(&world.matrix));
} 

#[then(regex = r#"B\[(\d),(\d)\] = (-*\d+)\/(\d+)"#)]
async fn b_is(world: &mut MatrixWorld,row:usize,col:usize,upper: f64,lower: f64) {

    let number = upper / lower;

    assert_eq!(world.matrix_b[row][col],number);
} 

#[then(regex = r#"A is not invertible"#)]
async fn a_is_not_inverttible(world: &mut MatrixWorld) {

    assert!(!is_invertible(&world.matrix));
} 


#[then(regex = r#"submatrix\(A, (\d), (\d)\) is the following (\d)x(\d) matrix:"#)]
async fn a_sub_2_2(world: &mut MatrixWorld,row: usize,col:usize,size_r: usize,size_c: usize,step: &Step) {
   
    let table = step.table().expect("Expected a table but got none");
    let result = submatrix(&world.matrix, row, col);
    let mut expected = Matrix::new(size_r,size_c);
    
    for (row_idx, row) in table.rows.iter().enumerate() {
        for (col_idx, value) in row.iter().enumerate() {
            expected[row_idx][col_idx] = value.parse::<f64>().expect("Invalid number in matrix");
        }
    }
    assert_eq!(result,expected)
} 

#[then(regex = r#"^^B is the following 4x4 matrix:"#)]
async fn b_matrix_is(world: &mut MatrixWorld,step: &Step) {
   
    let table = step.table().expect("Expected a table but got none");
    let mut expected = Matrix::new(4,4);
    
    for (row_idx, row) in table.rows.iter().enumerate() {
        for (col_idx, value) in row.iter().enumerate() {
            expected[row_idx][col_idx] = value.parse::<f64>().expect("Invalid number in matrix");
        }
    }
 
    assert!(approx_eq_matrix(&world.matrix_b,&expected,1e-4))
}

#[then(regex = r#"inverse\(A\) is the following 4x4 matrix:"#)]
async fn a_matrix_inverse_is(world: &mut MatrixWorld,step: &Step) {
    let table = step.table().expect("Expected a table but got none");
    let mut expected = Matrix::new(4,4);
    for (row_idx, row) in table.rows.iter().enumerate() {
        for (col_idx, value) in row.iter().enumerate() {
            expected[row_idx][col_idx] = value.parse::<f64>().expect("Invalid number in matrix");
        }
    }
    let result = inverse(&world.matrix);
    assert!(approx_eq_matrix(&result,&expected,1e-4))
}


#[then(regex = r#"C \* inverse\(B\) = A"#)]
async fn c_times_b_inverse_eq_a(world: &mut MatrixWorld) {
    let result = &world.matrix_c *  &inverse(&world.matrix_b);
    assert!(approx_eq_matrix(&result, &world.matrix, 1e-4))
}




pub fn approx_eq_matrix(matrix1: &Matrix, matrix2: &Matrix, epsilon: f64) -> bool {
    if matrix1.size() != matrix2.size() {
        return false; // ✅ Matrices must be the same size
    }

    for i in 0..matrix1.data.len() {
        for j in 0..matrix1.data[i].len() {
            if (matrix1.data[i][j] - matrix2.data[i][j]).abs() > epsilon {
                return false; 
            }
        }
    }

    true // ✅ Matrices are approximately equal
}


fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(MatrixWorld::run(
        "tests/features/matrices.feature",
    ));
}