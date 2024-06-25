use zesty_engine2::math_utils;
use math_utils::{transform_3d_point, vector, matrix, quaternion};
use quaternion::Quaternion;
use vector::prelude::*;
use matrix::Matrix4;



#[test]
fn test_translate_point() {
    let point = vector![1.0, 0.0, 1.0];
    let mut matrix = Matrix4::identity_matrix();

    matrix[3] = [2.4, 5.6, 90.1, 1.0];
    let new_point = transform_3d_point(&matrix, point);
    assert_eq!(new_point, vector![3.4, 5.6, 91.1]);
}
