use crate::math_utils::vector::Vector3;


pub struct Mesh {
    vertices: Vec<Vector3<f32>>,
    indices: Vec<Vector3<usize>>
}
