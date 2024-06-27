use std::f32::consts::FRAC_PI_4;

use zesty_engine2::math_utils;
use math_utils::{transform_3d_point, vector, matrix, quaternion, FloatType};
use quaternion::Quaternion;
use vector::prelude::*;
use matrix::Matrix4;


const DEG_45: f32 = FRAC_PI_4;

#[test]
fn test_translate_point() {
    let point = vector![1.0, 0.0, 1.0];
    let mut matrix = Matrix4::identity_matrix();

    matrix[3] = [2.4, 5.6, 90.1, 1.0];
    let new_point = transform_3d_point(&matrix, point);
    assert_eq!(new_point, vector![3.4, 5.6, 91.1]);
}

fn rotation_test_helper(point: Vector3<FloatType>, rot: Quaternion, expected_point: Vector3<FloatType>) {
    let mut matrix = Matrix4::identity_matrix();
    rot.edit_3d_matrix(&mut matrix, Vector3::one());

    let new_point = transform_3d_point(&matrix, point);
    assert_eq!(new_point, expected_point);
}

#[test]
fn test_rotate_point_around_x_axis() {
    rotation_test_helper(
        vector![0.0, 1.0, 0.0],
        Quaternion::from_euler_angles(DEG_45, 0.0, 0.0),
        vector![0.0, 0.7071067812, 0.7071067812]
    )
}

#[test]
fn test_rotate_point_around_y_axis() {
    rotation_test_helper(
        vector![1.0, 0.0, 0.0],
        Quaternion::from_euler_angles(0.0, DEG_45, 0.0),
        vector![0.7071067812, 0.0, -0.7071067812]
    )
}

#[test]
fn test_rotate_point_around_z_axis() {
    rotation_test_helper(
        vector![0.0, 1.0, 0.0],
        Quaternion::from_euler_angles(0.0, 0.0, DEG_45),
        vector![-0.7071067812, 0.7071067812, 0.0]
    )
}
