use super::vector::{Vector3, Vector4};


pub struct Matrix3 {
    x: Vector3<f32>,
    y: Vector3<f32>,
    z: Vector3<f32>
}

pub struct Matrix4 {
    x: Vector4<f32>,
    y: Vector4<f32>,
    z: Vector4<f32>,
    w: Vector4<f32>
}

pub struct TransformMatrix {
    matrix: Matrix3,
    translation: Vector3<f32>
}
