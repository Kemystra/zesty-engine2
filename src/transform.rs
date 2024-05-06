use crate::math_utils;
use math_utils::matrix::Matrix4;
use math_utils::quaternion::Quaternion;
use math_utils::vector::Vector3;


pub struct Transform {
    matrix: Matrix4,
    inverse_matrix: Matrix4,

    rotation: Quaternion,
    scale: Vector3<f32>,
    position: Vector3<f32>,

    dirty_flag: bool
}
