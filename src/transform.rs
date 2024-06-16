use crate::math_utils;
use math_utils::FloatType;
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

    dirty_flag: bool
}

impl Transform {
    pub fn new(position: Vector3<FloatType>, rotation: Quaternion, scale: Vector3<FloatType>) -> Self {
        let mut new_self = Self {
            matrix: Matrix4::identity_matrix(),
            inverse_matrix: Matrix4::identity_matrix(),

            rotation,
            scale,
            position,

            dirty_flag: true,
        };

        new_self.update();
        new_self
    }

    pub fn update(&mut self) {
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
        self.dirty_flag = false;
    }

    pub fn local_to_world(&self, pos: Vector3<FloatType>) -> Vector3<FloatType> {
    }

    pub fn world_to_local(&self, pos: Vector3<FloatType>) -> Vector3<FloatType> {
    }
}
