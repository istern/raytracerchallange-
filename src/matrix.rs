use std::ops::{Index, IndexMut,Mul};

use crate::Tuple;


#[derive(Debug, PartialEq, Default, Clone)]
pub struct Matrix{
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
     pub fn new(rows: usize, cols: usize) -> Self {
        let data = vec![vec![0.0; cols]; rows]; // Creates a `rows × cols` matrix filled with 0.0
        Self { data }
    }

    pub fn identity(rows: usize, cols: usize) -> Self{
        let mut data = vec![vec![0.0; cols]; rows];
        for i in 0..rows {
            data[i][i] = 1.0; // Set diagonal elements to 1.0
        }
        Self{data}
    }

  


    pub fn size(&self) -> (usize, usize) {
        (self.data.len(), self.data.first().map_or(0, |r| r.len()))
    }
}

// ✅ Enable `m[row]` access (returns a row reference)
impl Index<usize> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, row: usize) -> &Self::Output {
        &self.data[row]
    }
}

// ✅ Enable `m[row][col] = value` (mutable access)
impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.data[row]
    }
}

impl Mul<&Tuple> for &Matrix {
    type Output = Tuple;

    fn mul(self, rhs: &Tuple) -> Self::Output {
        Tuple {
            x: self.data[0][0] * rhs.x + self.data[0][1] * rhs.y + self.data[0][2] * rhs.z + self.data[0][3] * rhs.w,
            y: self.data[1][0] * rhs.x + self.data[1][1] * rhs.y + self.data[1][2] * rhs.z + self.data[1][3] * rhs.w,
            z: self.data[2][0] * rhs.x + self.data[2][1] * rhs.y + self.data[2][2] * rhs.z + self.data[2][3] * rhs.w,
            w: self.data[3][0] * rhs.x + self.data[3][1] * rhs.y + self.data[3][2] * rhs.z + self.data[3][3] * rhs.w,
        }
    }
}

impl Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let (rows_a, cols_a) = self.size();
        let (rows_b, cols_b) = rhs.size();
        
        if cols_a != rows_b {
            panic!("Matrix multiplication not possible: {}x{} * {}x{}", rows_a, cols_a, rows_b, cols_b);
        }
        
        let mut result = Matrix::new(rows_a, cols_b);
        for i in 0..rows_a {
            for j in 0..cols_b {
                result.data[i][j] = (0..cols_a)
                    .map(|k| self.data[i][k] * rhs.data[k][j])
                    .sum();
               
            }
        }   
        result
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {  // ✅ Uses borrowed references
        let (rows_a, cols_a) = self.size();
        let (rows_b, cols_b) = rhs.size();

        if cols_a != rows_b {
            panic!("Matrix multiplication not possible: {}x{} * {}x{}", rows_a, cols_a, rows_b, cols_b);
        }

        let mut result = Matrix::new(rows_a, cols_b);

        for i in 0..rows_a {
            for j in 0..cols_b {
                result.data[i][j] = (0..cols_a)
                    .map(|k| self.data[i][k] * rhs.data[k][j])
                    .sum();
            }
        }

        result
    }
}

pub fn minor(matrix: &Matrix, row: usize, col: usize) -> f64 {
    let sub = submatrix(matrix, row, col); // ✅ Get the submatrix
    determinant(&sub) // ✅ Compute its determinant
}

pub fn cofactor(matrix: &Matrix, row: usize, col: usize) -> f64 {
    let sign = if (row + col) % 2 == 0 { 1.0 } else { -1.0 }; // ✅ Compute (-1)^(row+col)
    sign * minor(matrix, row, col) // ✅ Multiply minor by sign factor
}


pub fn determinant(matrix: &Matrix) -> f64 {
    let (rows, cols) = matrix.size();
    if rows != cols {
        panic!("Determinant can only be computed for square matrices!");
    }

    if rows == 2 {
        return (matrix.data[0][0] * matrix.data[1][1]) - (matrix.data[0][1] * matrix.data[1][0]);
    }

    let mut det = 0.0;
    for (j, &val) in matrix.data[0].iter().enumerate() {
        let sub = submatrix(matrix, 0, j);
        det += val * determinant(&sub) * if j % 2 == 0 { 1.0 } else { -1.0 };
    }

    det
}

pub fn transpose(matrix: &Matrix) -> Matrix {
    let (rows, cols) = matrix.size();
    let mut transposed =  Matrix::new(rows, cols);

    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = matrix[i][j]; 
        }
    }
    transposed
}

pub fn submatrix(matrix: &Matrix, row: usize, col: usize) -> Matrix {
    let submatrix = matrix.data
        .iter()
        .enumerate()
        .filter(|&(r, _)| r != row) // ✅ Remove the selected row
        .map(|(_, row_data)| {
            row_data.iter()
                .enumerate()
                .filter(|&(c, _)| c != col)
                .map(|(_, &val)| val)
                .collect()
        })
        .collect();

    Matrix { data: submatrix }
}


pub fn cofactor_matrix(matrix: &Matrix) -> Matrix {
    let (rows, cols) = matrix.size();
    let mut cofactors = vec![vec![0.0; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            cofactors[i][j] = cofactor(matrix, i, j);
        }
    }

    Matrix { data: cofactors }
}
pub fn inverse(matrix: &Matrix) -> Matrix {
    let det = determinant(matrix);
    if det == 0.0 {
        panic!("Determinat is 0");
    }

    let adjugate = transpose(&cofactor_matrix(&matrix)); 
    let inverse_data = adjugate.data
        .iter()
        .map(|row| row.iter().map(|&val| val / det).collect())
        .collect();

    Matrix { data: inverse_data } // ✅ Return the inverse
}

pub fn is_invertible(matrix: &Matrix)-> bool{
    return determinant(matrix) != 0.0
}