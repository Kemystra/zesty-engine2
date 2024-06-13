use crate::math_utils;
use math_utils::matrix::Matrix4;
use math_utils::quaternion::Quaternion;
use math_utils::vector::Vector3;


// Any operation that impacts the field `matrix` must set the dirty flag
pub struct Transform {
    matrix: Matrix4,
    inverse_matrix: Matrix4,

    rotation: Quaternion,
    scale: Vector3<f32>,
    position: Vector3<f32>,

    dirty_flag: bool
}

impl Transform {
    pub fn local_to_world(&self, pos: Vector3<f32>) -> Vector3<f32> {
    }

    pub fn world_to_local(&self, pos: Vector3<f32>) -> Vector3<f32> {
    }
}
