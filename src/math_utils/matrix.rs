use std::ops::Index;
use std::ops::IndexMut;


pub(crate) struct Matrix<const N: usize>([[f32; N]; N]);

pub(crate) type Matrix3 = Matrix<3>;
pub(crate) type Matrix4 = Matrix<4>;

impl<const N: usize> Matrix<N> {
    const SIZE: usize = N;
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
