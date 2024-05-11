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
}
