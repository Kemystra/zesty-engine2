use crate::math_utils;
use math_utils::{FloatType, transform_3d_point};
use math_utils::matrix::Matrix4;
use math_utils::quaternion::Quaternion;
use math_utils::vector::{Vector3, Vector, vector};


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
        transform_3d_point(&self.matrix, pos)
    }

    pub fn world_to_local(&self, pos: Vector3<FloatType>) -> Vector3<FloatType> {
        transform_3d_point(&self.inverse_matrix, pos)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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
}
