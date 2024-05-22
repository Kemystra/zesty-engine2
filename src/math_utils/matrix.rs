use zesty_engine2_derive::Matrix;

use std::ops::Index;
use std::ops::IndexMut;


pub(super) trait Matrix: Sized + Clone + Index<usize> + IndexMut<usize> {
    const SIZE: usize;
    const IDENTITY: Self;

    fn invert(&self) -> Result<Self, String>;
}

macro_rules! impl_index_mut_matrices {
    ($Matrix:ident, $size:expr) => {
         impl Index<usize> for $Matrix {
            type Output = [f32; $size];

            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl IndexMut<usize> for $Matrix {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.0[index]
            }
        }
    };
}

#[derive(Matrix, Clone)]
#[matrix(3)]
pub struct Matrix3([[f32; 3]; 3]);

impl_index_mut_matrices!(Matrix3, 3);

#[derive(Matrix, Clone)]
#[matrix(4)]
pub struct Matrix4([[f32; 4]; 4]);

impl_index_mut_matrices!(Matrix4, 4);
// 3 functions that perform the elementary row operations needed for inverting a matrix
// 1. Swapping rows
fn matrix_swap_row<T, const N: usize>(
    matrix: &mut T,
    row_index_1: usize,
    row_index_2: usize
) -> ()
where T: Matrix + Index<usize, Output = [f32; N]> {
    let tmp = matrix[row_index_1].clone();
    matrix[row_index_1] = matrix[row_index_2];
    matrix[row_index_2] = tmp;
}

// 2. Add rows (possibly with a multiplier)
fn matrix_add_multiply_row<T, const N: usize>(
    matrix: &mut T,
    target_row_index: usize,
    add_row_index: usize,
    multiplier: f32,
) -> ()
where T: Matrix + Index<usize, Output = [f32; N]> {
    let columns = T::SIZE;

    for col in 0..columns {
        matrix[target_row_index][col] += multiplier * matrix[add_row_index][col];
    }
}

// 3. Multiply the whole row with a scalar value
fn matrix_scale_row<T, const N: usize>(
    matrix: &mut T,
    target_row_index: usize,
    multiplier: f32
) -> ()
where T: Matrix + Index<usize, Output = [f32; N]> {
    let columns = T::SIZE;

    for col in 0..columns {
        matrix[target_row_index][col] *= multiplier;
    }
}
