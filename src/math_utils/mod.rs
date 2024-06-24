pub mod vector;
pub mod matrix;
pub mod quaternion;

use matrix::Matrix4;
use vector::Vector3;


pub type FloatType = f32;

// IMPORTANT!
// Any part of code that deals with angles should be assumed to use radians
// All matrix are row-major
// Coordinates are RIGHT-HANDED ffs, opposite to the original raster-3d-engine
// Any MENTION of Euler angles are assumed to be intrinsic Tait-Bryan angles with sequence XYZ
// A rotation is POSITIVE if it moves clockwise when looking in the positive direction of its axis

// Applies 3D transformations represented as Matrix4 to a point represented as Vector3.
// This includes rotations, scaling, and translations
pub fn transform_3d_point(matrix: &Matrix4, vector: Vector3<FloatType>) -> Vector3<FloatType> {
    let mut result_array = [0.0, 0.0, 0.0];

    for i in 0..Vector3::<FloatType>::SIZE {
        let mut sum_multiply = 0.0;
        for j in 0..Vector3::<FloatType>::SIZE {
            sum_multiply += vector[j] * matrix[j][i];
        }

        // Add translation part
        result_array[i] = sum_multiply + matrix[3][i];
    }

    Vector3::new(result_array)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_3d_vector() {
        let matrix = Matrix4::new([
            [1.0, 0.0, 0.0, 0.0], // Translation by (2, 3, 4)
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [2.0, 3.0, 4.0, 1.0],
        ]);

        let vector = Vector3::new([1.0, 2.0, 3.0]);

        let result = transform_3d_point(&matrix, vector);

        let expected = Vector3::new([3.0, 5.0, 7.0]);
        assert_eq!(result.array(), expected.array());
    }
}
