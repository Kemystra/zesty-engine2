use super::vector::{Vector3, Vector4};


pub struct Matrix3 {
    x: Vector3<f64>,
    y: Vector3<f64>,
    z: Vector3<f64>
}

pub struct Matrix4 {
    x: Vector4<f64>,
    y: Vector4<f64>,
    z: Vector4<f64>,
    w: Vector4<f64>
}
