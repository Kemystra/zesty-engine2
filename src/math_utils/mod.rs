pub mod vector;
pub mod matrix;
pub mod quaternion;


pub type Float = f32;
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

        let result = transform_3d_vector(&matrix, vector);

        let expected = Vector3::new([3.0, 5.0, 7.0]);
        assert_eq!(result.array(), expected.array());
    }
}
