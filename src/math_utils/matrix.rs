use std::fmt::Debug;
use std::ops::Index;
use std::ops::IndexMut;
use std::default::Default;

use float_cmp::ApproxEq;

use super::{FloatType, vector::{Vector, vector}};

#[derive(Clone, PartialEq)]
pub struct Matrix<const N: usize>([[FloatType; N]; N]);

pub type Matrix3 = Matrix<3>;
pub type Matrix4 = Matrix<4>;

impl<const N: usize> Matrix<N> {
    pub fn new(arr: [[FloatType; N]; N]) -> Self {
        Self(arr)
    }

    pub fn identity_matrix() -> Self {
        let mut matrix = [[0.0; N]; N];
        for i in 0..N {
            matrix[i][i] = 1.0;
        }

        Self(matrix)
    }

    pub const SIZE: usize = N;

    pub fn invert(&self, is_homogenous: bool) -> Result<Matrix<N>, String> {
        let total_row: usize = if is_homogenous { N-1 } else { N };
        let total_column: usize = N;

        if *self == Self::identity_matrix() {
            return Ok(self.clone());
        }

        let mut matrix = self.clone();
        let mut inv_matrix = Matrix::<N>::identity_matrix();

        ensure_pivot_non_zero(&mut matrix, &mut inv_matrix, total_row, total_column)?;
        forward_substitution(&mut matrix, &mut inv_matrix, total_row, total_column);
        scale_pivot_to_one(&mut matrix, &mut inv_matrix, total_row, total_column);
        backward_substitution(&mut matrix, &mut inv_matrix, total_row, total_column);

        Ok(inv_matrix)
    }

    pub fn multiply_vector(&self, vector: Vector<N,FloatType>) -> Vector<N,FloatType> {
        let mut result_array = [0.0; N];

        for i in 0..Vector::<N, FloatType>::SIZE {
            let mut sum_multiply = 0.0;
            for j in 0..Vector::<N, FloatType>::SIZE {
                sum_multiply += vector[j] * self[i][j];
            }

            result_array[i] = sum_multiply;
        }

        Vector::new(result_array)
    }
}

impl<const N: usize> Default for Matrix<N> {
    fn default() -> Self {
        Self::identity_matrix()
    }
}

impl<const N: usize> Index<usize> for Matrix<N> {
    type Output = [FloatType; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for Matrix<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const N: usize> ApproxEq for Matrix<N> {
    type Margin = <FloatType as ApproxEq>::Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        for i in 0..N {
            for j in 0..N {
                if !self[i][j].approx_eq(other[i][j], margin) { return false }
            }
        }

        true
    }
}

impl<const N: usize> Debug for Matrix<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pretty_arr = "".to_owned();
        for i in 0..N {
            pretty_arr.push_str(&format!("{:?}\n", self.0[i]));
        }

        write!(f, "Matrix[\n{}]", &pretty_arr)
    }
}

fn ensure_pivot_non_zero<const N: usize>(
    matrix: &mut Matrix<N>, inv_matrix: &mut Matrix<N>,
    total_row: usize, total_column: usize
) -> Result<(), String> {
    for col in 0..total_column {
        // Making sure pivot is a non-zero number
        // If zero, swap row with one that has the biggest absolute value
        let mut pivot = col;
        let mut pivot_val = matrix[col][col];

        // It should be noted that singular matrix is uncommon in the context of transformation.
        // The following implementation is subpar; there's a chance for singular matrix to escape
        // this function.
        if pivot_val != 0.0 {
            continue
        }

        for row in 0..total_row {
            if matrix[row][col].abs() > pivot_val.abs() {
                pivot = row;
                pivot_val = matrix[row][col];
            }
        }

        if pivot_val == 0.0 {
            return Err("Matrix has no inverse".to_string())
        }

        swap_row(matrix, pivot, col);
        swap_row(inv_matrix, pivot, col);
    }
    Ok(())
}

fn forward_substitution<const N: usize>(
    matrix: &mut Matrix<N>, inv_matrix: &mut Matrix<N>,
    total_row: usize, total_column: usize
) -> () {
    // Make every value under pivot points zero
    for col in 0..total_column {
        for row in (col+1)..total_row {
            let multiplier = -(matrix[row][col] / matrix[col][col]);
            // Multiply pivot point's row by multiplier, and add to current row
            add_multiply_row(matrix, row, col, multiplier);
            add_multiply_row(inv_matrix, row, col, multiplier);
            matrix[row][col] = 0.0;
        }
    }
}

fn scale_pivot_to_one<const N: usize>(
    matrix: &mut Matrix<N>, inv_matrix: &mut Matrix<N>,
    _total_row: usize, total_column: usize
) -> () {
    // Divide each row to turn the pivot into 1
    for col in 0..total_column {
        // Fancy name for 'divisor'
        let inverse_multiplier = 1.0/matrix[col][col];
        scale_row(matrix, col, inverse_multiplier);
        scale_row(inv_matrix, col, inverse_multiplier);
        matrix[col][col] = 1.0;
    }
}

fn backward_substitution<const N: usize>(
    matrix: &mut Matrix<N>, inv_matrix: &mut Matrix<N>,
    total_row: usize, total_column: usize
) -> () {
    // Backward substitution
    for row in 0..total_row {
        for col in (row+1)..total_column {
            let multiplier = -matrix[row][col];
            add_multiply_row(matrix, row, col, multiplier);
            add_multiply_row(inv_matrix, row, col, multiplier);

            matrix[row][col] = 0.0;
        }
    }
}

// 3 functions that perform the elementary row operations needed for inverting a matrix
// 1. Swapping rows
fn swap_row<const N: usize>(
    matrix: &mut Matrix<N>,
    row_index_1: usize,
    row_index_2: usize
) -> () {
    let tmp = matrix[row_index_1].clone();
    matrix[row_index_1] = matrix[row_index_2];
    matrix[row_index_2] = tmp;
}

// 2. Add rows (possibly with a multiplier)
fn add_multiply_row<const N: usize>(
    matrix: &mut Matrix<N>,
    target_row_index: usize,
    add_row_index: usize,
    multiplier: FloatType,
) -> () {
    let columns = Matrix::<N>::SIZE;

    for col in 0..columns {
        matrix[target_row_index][col] += multiplier * matrix[add_row_index][col];
    }
}

// 3. Multiply the whole row with a scalar value
fn scale_row<const N: usize>(
    matrix: &mut Matrix<N>,
    target_row_index: usize,
    multiplier: FloatType
) -> () {
    let columns = Matrix::<N>::SIZE;

    for col in 0..columns {
        matrix[target_row_index][col] *= multiplier;
    }
}


#[cfg(test)]
pub mod tests {
    use float_cmp::approx_eq;
    use super::*;

    fn approx_cmp_matrix<const N: usize>(a: Matrix<N>, b: Matrix<N>) {
        assert!(approx_eq!(Matrix<N>, a, b))
    }

    #[test]
    fn test_ensure_pivot_non_zero() {
        let mut matrix = Matrix([
            [1.00, 2.00, 9.00],
            [3.00, 0.00, 8.00],
            [6.00, 3.00, 5.00]
        ]);
        let mut dummy_inv_matrix = Matrix::<3>::identity_matrix();
        let total_row = 3;
        let total_column = 3;
        let result = ensure_pivot_non_zero(&mut matrix, &mut dummy_inv_matrix, total_row, total_column);

        assert_eq!(result, Ok(()));
        approx_cmp_matrix(matrix, Matrix([
            [1.00, 2.00, 9.00],
            [6.00, 3.00, 5.00],
            [3.00, 0.00, 8.00]
        ]));
        approx_cmp_matrix(dummy_inv_matrix, Matrix([
            [1.00, 0.00, 0.00],
            [0.00, 0.00, 1.00],
            [0.00, 1.00, 0.00]
        ]));
    }

    #[test]
    fn test_forward_substitution() {
        let mut matrix = Matrix([
            [1.00, 2.00, 9.00],
            [0.00, 8.00, 0.00],
            [6.00, 3.00, 5.00]
        ]);
        let mut dummy_inv_matrix = Matrix::<3>::identity_matrix();
        let total_row = 3;
        let total_column = 3;

        forward_substitution(&mut matrix, &mut dummy_inv_matrix, total_row, total_column);
        approx_cmp_matrix(matrix, Matrix([
            [1.00, 2.00, 9.00],
            [0.00, 8.00, 0.00],
            [0.00, 0.00, -49.00]
        ]));
        approx_cmp_matrix(dummy_inv_matrix, Matrix([
            [1.00, 0.00, 0.00],
            [0.00, 1.00, 0.00],
            [-6.00, 1.125, 1.00]
        ]))
    }

    #[test]
    fn test_scale_pivot_to_one() {
        let mut matrix = Matrix([
            [4.00, 2.00, 9.00],
            [0.00, 8.00, 7.00],
            [0.00, 0.00, 5.00]
        ]);
        let mut dummy_inv_matrix = Matrix::<3>::identity_matrix();
        let total_row = 3;
        let total_column = 3;

        scale_pivot_to_one(&mut matrix, &mut dummy_inv_matrix, total_row, total_column);
        approx_cmp_matrix(matrix, Matrix([
            [1.00, 2.00/4.00, 9.00/4.00],
            [0.00, 1.00, 7.00/8.00],
            [0.00, 0.00, 1.00]
        ]));
        approx_cmp_matrix(dummy_inv_matrix, Matrix([
            [1.00/4.00, 0.00, 0.00],
            [0.00, 1.00/8.00, 0.00],
            [0.00, 0.00, 1.00/5.00],
        ]));
    }

    #[test]
    fn test_backward_substitution() {
        let mut matrix = Matrix([
            [1.0, 0.5, -0.5],
            [0.0, 1.0, 1.0],
            [0.0, 0.0, 1.0]
        ]);
        let mut dummy_inv_matrix = Matrix([
            [1.0, 0.0, 0.0],
            [1.5, 1.0, 0.0],
            [1.0, 0.0, 1.0]
        ]);
        let total_row = 3;
        let total_column = 3;

        backward_substitution(&mut matrix, &mut dummy_inv_matrix, total_row, total_column);
        approx_cmp_matrix(matrix, Matrix::<3>::identity_matrix());
        approx_cmp_matrix(dummy_inv_matrix, Matrix([
            [1.25, -0.5, 1.0],
            [0.5, 1.0, -1.0],
            [1.0, 0.0, 1.0]
        ]));
    }

    #[test]
    fn test_inverting_matrix() {
        let matrix = Matrix([
            [1.00, -3.00, 3.01],
            [4.50, 5.00, 7.00],
            [3.00, 97.00, 8.00]
        ]);

        let inv_matrix = matrix.invert(false).unwrap();
        approx_cmp_matrix(inv_matrix, Matrix([
            [-42600.00/44981.00, 63194.00/134943.00, -7210.00/134943.00],
            [-1000.00/44981.00, -206.00/134943.00, 1309.00/134943.00],
            [28100.00/44981.00, -21200.00/134943.00, 3700.00/134943.00]
        ]));
    }

    #[test]
    fn test_multiply_vector() {
        let matrix = Matrix([
            [6.22, -12.64, -12.80, 4.54],
            [8.34, 1.38, -6.47, 2.80],
            [-2.20, 0.86, 7.82, -8.86],
            [11.81, 10.94, -8.89, -14.73]
        ]);

        let vector = Vector::new([1.0, 2.0, 4.0, 1.0]);
        let result = matrix.multiply_vector(vector);

        assert!(
        result.approx_eq(
            vector![-1643.0/25.0, -599.0/50.0, 1097.0/50.0, -83.0/5.0],
            (1.0, 2)
        ));
    }
}
