use std::ops::Index;
use std::ops::IndexMut;


#[cfg_attr(test, derive(PartialEq))]
#[derive(Clone, Debug)]
pub(crate) struct Matrix<const N: usize>([[f32; N]; N]);

pub(crate) type Matrix3 = Matrix<3>;
pub(crate) type Matrix4 = Matrix<4>;

impl<const N: usize> Matrix<N> {
    pub fn identity_matrix() -> Self {
        let mut matrix = [[0.0; N]; N];
        for i in 0..N {
            matrix[i][i] = 1.0;
        }

        Self(matrix)
    }

    pub const SIZE: usize = N;
}

impl<const N: usize> Index<usize> for Matrix<N> {
    type Output = [f32; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for Matrix<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

pub fn invert_matrix<const N: usize>(matrix: &Matrix<N>, is_homogenous: bool) -> Result<Matrix<N>, String> {
    let total_row: usize = N;
    let total_column: usize = if is_homogenous { N-1 } else { N };

    let mut matrix = matrix.clone();
    let mut inv_matrix = Matrix::<N>::identity_matrix();

    ensure_pivot_non_zero(&mut matrix, &mut inv_matrix, total_row, total_column)?;
    matrix_forward_substitution(&mut matrix, &mut inv_matrix, total_row, total_column);
    matrix_scale_pivot_to_one(&mut matrix, &mut inv_matrix, total_row, total_column);
    matrix_backward_substitution(&mut matrix, &mut inv_matrix, total_row, total_column);

    Ok(inv_matrix)
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

        if pivot_val != 0.0 {
            return Ok(())
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

        let mut tmp = matrix[pivot];
        matrix[pivot] = matrix[col];
        matrix[col] = tmp;

        tmp = inv_matrix[pivot];
        inv_matrix[pivot] = inv_matrix[col];
        inv_matrix[col] = tmp;
    }
    Ok(())
}

fn matrix_forward_substitution<const N: usize>(
    matrix: &mut Matrix<N>, inv_matrix: &mut Matrix<N>,
    total_row: usize, total_column: usize
) -> () {
    // Make every value under pivot points zero
    for col in 0..total_column {
        for row in (col+1)..total_row {
            let multiplier = -(matrix[row][col] / matrix[col][col]);
            // Multiply pivot point's row by multiplier, and add to current row
            matrix_add_multiply_row(matrix, row, col, multiplier)
        }
    }
}

fn matrix_scale_pivot_to_one<const N: usize>(
    matrix: &mut Matrix<N>, inv_matrix: &mut Matrix<N>,
    total_row: usize, total_column: usize
) -> () {
    // Divide each row to turn the pivot into 1
    for col in 0..total_column {
        let divisor = matrix[col][col];
        for i in 0..total_column {
            matrix[col][i] /= divisor;
            inv_matrix[col][i] /= divisor;
        }
        matrix[col][col] = 1.0;
    }
}

fn matrix_backward_substitution<const N: usize>(
    matrix: &mut Matrix<N>, inv_matrix: &mut Matrix<N>,
    total_row: usize, total_column: usize
) -> () {
    // Backward substitution
    for row in 0..total_row {
        for col in (row+1)..total_column {
            let constant = matrix[row][col];
            for i in 0..total_column {
                matrix[row][i] -= matrix[col][i] * constant;
                inv_matrix[row][i] -= inv_matrix[col][i] * constant;
            }

            matrix[row][col] = 0.0;
        }
    }
}

// 3 functions that perform the elementary row operations needed for inverting a matrix
// 1. Swapping rows
fn matrix_swap_row<const N: usize>(
    matrix: &mut Matrix<N>,
    row_index_1: usize,
    row_index_2: usize
) -> () {
    let tmp = matrix[row_index_1].clone();
    matrix[row_index_1] = matrix[row_index_2];
    matrix[row_index_2] = tmp;
}

// 2. Add rows (possibly with a multiplier)
fn matrix_add_multiply_row<const N: usize>(
    matrix: &mut Matrix<N>,
    target_row_index: usize,
    add_row_index: usize,
    multiplier: f32,
) -> () {
    let columns = Matrix::<N>::SIZE;

    for col in 0..columns {
        matrix[target_row_index][col] += multiplier * matrix[add_row_index][col];
    }
}

// 3. Multiply the whole row with a scalar value
fn matrix_scale_row<const N: usize>(
    matrix: &mut Matrix<N>,
    target_row_index: usize,
    multiplier: f32
) -> () {
    let columns = Matrix::<N>::SIZE;

    for col in 0..columns {
        matrix[target_row_index][col] *= multiplier;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(matrix, Matrix([
            [1.00, 2.00, 9.00],
            [6.00, 3.00, 5.00],
            [3.00, 0.00, 8.00]
        ]))
    }

    #[test]
    fn test_ensure_pivot_non_zero_impossible() {
        let mut matrix = Matrix([
            [1.00, 2.00, 9.00],
            [0.00, 0.00, 0.00],
            [6.00, 3.00, 5.00]
        ]);
        let mut dummy_inv_matrix = Matrix::<3>::identity_matrix();
        let total_row = 3;
        let total_column = 3;
        let result = ensure_pivot_non_zero(&mut matrix, &mut dummy_inv_matrix, total_row, total_column);

        assert_eq!(result, Err("Matrix has no inverse".to_string()));
    }

    #[test]
    fn test_matrix_forward_substitution() {
        let mut matrix = Matrix([
            [1.00, 2.00, 9.00],
            [0.00, 8.00, 0.00],
            [6.00, 3.00, 5.00]
        ]);
        let mut dummy_inv_matrix = Matrix::<3>::identity_matrix();
        let total_row = 3;
        let total_column = 3;

        matrix_forward_substitution(&mut matrix, &mut dummy_inv_matrix, total_row, total_column);
        // Will add proper assert statement later
        // For now, will manually check algorithm result
        println!("{:?}", matrix);
        assert_eq!(true, true);
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

        matrix_scale_pivot_to_one(&mut matrix, &mut dummy_inv_matrix, total_row, total_column);
        assert_eq!(matrix, Matrix([
            [1.00, 2.00/4.00, 9.00/4.00],
            [0.00, 1.00, 7.00/8.00],
            [0.00, 0.00, 1.00]
        ]))
    }

    #[test]
    fn test_backward_substitution() {
        let mut matrix = Matrix([
            [1.00, 2.00, 9.00],
            [0.00, 1.00, 7.00],
            [0.00, 0.00, 1.00]
        ]);
        let mut dummy_inv_matrix = Matrix::<3>::identity_matrix();
        let total_row = 3;
        let total_column = 3;

        matrix_backward_substitution(&mut matrix, &mut dummy_inv_matrix, total_row, total_column);
        assert_eq!(matrix, Matrix::<3>::identity_matrix())
    }
}
