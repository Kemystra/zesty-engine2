use crate::math_utils;
use math_utils::{FloatType, transform_3d_point};
use math_utils::matrix::Matrix4;
use math_utils::quaternion::Quaternion;
use math_utils::vector::Vector3;


// Any operation that impacts the field `matrix` must set the dirty flag
pub struct Transform {
    matrix: Matrix4,
    inverse_matrix: Matrix4,

    rotation: Quaternion,
    scale: Vector3<FloatType>,
    position: Vector3<FloatType>,

    is_dirty: bool
}

impl Transform {
    pub fn new(position: Vector3<FloatType>, rotation: Quaternion, scale: Vector3<FloatType>) -> Self {
        let mut new_self = Self {
            matrix: Matrix4::identity_matrix(),
            inverse_matrix: Matrix4::identity_matrix(),

            rotation,
            scale,
            position,

            is_dirty: true,
        };

        new_self.update();
        new_self
    }

    pub fn update(&mut self) {
        if !self.is_dirty {
            return;
        }

        // Update position
        for i in 0..3 {
            self.matrix[3][i] = self.position[i];
        }

        // Update rotation and scale
        self.rotation.edit_3d_matrix(&mut self.matrix, self.scale);

        // Generate inverse matrix from original matrix
        // Should NEVER fail
        self.inverse_matrix = self.matrix.invert(true).unwrap();

        // Set dirty flag to false, regardless of initial value
        self.is_dirty = false;
    }

    // Interestingly, one of the way to classify rotations is passive vs. active.
    // Active rotations act directly on the point.
    // Passive rotations simply affect the frame of reference the object is in.
    // The reverse of an active rotation is its corresponding passive rotation that yield the same
    // object orientation.
    // Extending this to a transformation matrix:
    // The original matrix (local_to_world) directly move/orient the point.
    // Its inverse move/orient the frame of reference.
    pub fn local_to_world(&self, pos: Vector3<FloatType>) -> Vector3<FloatType> {
        transform_3d_point(&self.matrix, pos)
    }

    pub fn world_to_local(&self, pos: Vector3<FloatType>) -> Vector3<FloatType> {
        transform_3d_point(&self.inverse_matrix, pos)
    }

    pub fn rotate(&mut self, q: Quaternion) {
        self.rotation *= q;
        self.is_dirty = true;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::math_utils::{matrix::Matrix, vector::{vector, Vector}};

    fn init_test_transform() -> Transform {
        let deg45 = std::f32::consts::FRAC_PI_4;
        let deg20 = std::f32::consts::PI / 9.0;
        Transform::new(
            vector![1.0, 7.0, 2.5],
            Quaternion::from_euler_angles(deg45, 0.0, deg20),
            vector![1.0, 1.0, 1.0]
        )
    }

    #[test]
    fn test_get_point_position_in_world() {
        let transform = init_test_transform();
        let pos = vector![2.5, 1.89, 10.7];

        let pos_in_world = transform.local_to_world(pos);
        assert_eq!(pos_in_world, vector![3.9956496, 15.217265, 9.41482]);
    }

    #[test]
    fn test_get_point_position_in_local() {
        let transform = init_test_transform();
        let pos = vector![1.0, 2.0, -0.5];

        let pos_in_local = transform.world_to_local(pos);
        assert_eq!(pos_in_local, vector![0.4836896, -1.3289258, -5.6568546]);
    }

    #[test]
    fn test_rotate_transform() {
        let mut transform = init_test_transform();
        let initial_rotation = transform.rotation;

        let deg45 = std::f32::consts::FRAC_PI_4;
        let rot = Quaternion::from_euler_angles(deg45, 0.0, deg45);

        transform.rotate(rot);
        // Check if rotation is applied to rotation field first
        assert_eq!(transform.rotation, initial_rotation * rot);
        // Check for dirty flag
        assert!(transform.is_dirty);

        transform.update();
        println!("{:?}", transform.matrix);
        /*
        assert_eq!(transform.matrix, Matrix::new([
            [0.0, 0.0, 0.0, 1.0]
        ]))
        */
    }
}
