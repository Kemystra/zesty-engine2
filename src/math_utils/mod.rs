pub mod vector;
pub mod matrix;
pub mod quaternion;

use matrix::Matrix4;
use vector::Vector3;

use std::ops::{Index, IndexMut};
use float_cmp::ApproxEq;


pub type FloatType = f32;

// Subtrait to ensure that main math structs have the necessary traits
trait MathStruct: ApproxEq + IndexMut<usize> + Index<usize> + PartialEq + Default {}

// IMPORTANT!
// Any part of code that deals with angles should be assumed to use radians
// All matrix are COLUMN-MAJOR and vectors are POST-MULTIPLIED with matrices.
// Coordinates are RIGHT-HANDED ffs, opposite to the original raster-3d-engine
// Any MENTION of Euler angles are assumed to be intrinsic Tait-Bryan angles with sequence
// Z-Y'-X"
// A rotation is POSITIVE if it moves clockwise when looking in the positive direction of its axis

// Applies 3D transformations represented as Matrix4 to a point represented as Vector3.
// This includes rotations, scaling, and translations
pub fn transform_3d_point(matrix: &Matrix4, vector: Vector3<FloatType>) -> Vector3<FloatType> {
    let mut result_array = [0.0, 0.0, 0.0];

    for i in 0..Vector3::<FloatType>::SIZE {
        let mut sum_multiply = 0.0;
        for j in 0..Vector3::<FloatType>::SIZE {
            sum_multiply += vector[j] * matrix[i][j];
        }

        // Add translation part
        result_array[i] = sum_multiply + matrix[i][3];
    }

    Vector3::new(result_array)
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::vector::tests::approx_cmp_vector;

    #[test]
    fn test_translate_point() {
        let matrix = Matrix4::new([
            [1.0, 0.0, 0.0, 2.0], // Translation by (2, 3, 4)
            [0.0, 1.0, 0.0, 3.0],
            [0.0, 0.0, 1.0, 4.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let vector = Vector3::new([1.0, 2.0, 3.0]);

        let result = transform_3d_point(&matrix, vector);

        let expected = Vector3::new([3.0, 5.0, 7.0]);
        approx_cmp_vector(result, expected);
    }

    #[test]
    fn test_rotate_point() {
        let matrix = Matrix4::new([
            [0.9397, 0.2418, 0.2418, 0.0],
            [0.0000, 0.7071,-0.7071, 0.0],
            [-0.3420, 0.6645, 0.6645, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]);

        let vector = Vector3::new([1.0, 2.0, 3.0]);
        let result = transform_3d_point(&matrix, vector);
        let expected = Vector3::new([2.1487, -0.7071, 2.9805]);

        approx_cmp_vector(result, expected);
    }

    #[test]
    fn test_full_transform_point() {
        let matrix = Matrix4::new([
            [0.9397, 0.2418, 0.2418, 2.0],
            [0.0000, 0.7071,-0.7071, 3.0],
            [-0.3420, 0.6645, 0.6645, 4.0],
            [0.0, 0.0, 0.0, 1.0]
        ]);

        let vector = Vector3::new([1.0, 2.0, 3.0]);
        let result = transform_3d_point(&matrix, vector);
        let expected = Vector3::new([4.1486998, 2.2929, 6.9805]);

        approx_cmp_vector(result, expected);
    }
}
