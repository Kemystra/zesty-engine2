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
            self.matrix[i][3] = self.position[i];
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

    use crate::math_utils::vector;
    use vector::{vector, Vector};
    use vector::tests::approx_cmp_vector;

    use crate::math_utils::quaternion::tests::approx_cmp_quaternion;

    fn init_test_transform() -> Transform {
        let deg45 = std::f32::consts::FRAC_PI_4;
        let deg20 = std::f32::consts::PI / 9.0;
        Transform::new(
            vector![1.0, 7.0, 2.5],
            Quaternion::from_euler_angles(deg45, 0.0, deg20),
            Vector3::one()
        )
    }

    #[test]
    fn test_get_point_position_in_world() {
        let transform = init_test_transform();
        let pos = vector![2.5, 1.89, 10.7];

        let pos_in_world = transform.local_to_world(pos);
        approx_cmp_vector(pos_in_world, vector![
            547988391453.0/100000000000.0,
            100056555803.0/50000000000.0,
            1140247437279.0/100000000000.0
       ]);
    }

    #[test]
    fn test_get_point_position_in_local() {
        let transform = init_test_transform();
        let pos = vector![1.0, 2.0, -0.5];

        let pos_in_local = transform.world_to_local(pos);
        approx_cmp_vector(pos_in_local, vector![
            -1209223815000000000.0/707106781005207013.0,
            -2721817732419718022000000000.0/499999999739863875201055153.0,
            600497389404096983000000000.0/499999999739863875201055153.0
        ]);
    }

    #[test]
    fn test_rotate_transform() {
        let mut transform = init_test_transform();
        let initial_rotation = transform.rotation;

        let deg45 = std::f32::consts::FRAC_PI_4;
        let rot = Quaternion::from_euler_angles(deg45, 0.0, deg45);

        transform.rotate(rot);
        // Check if rotation is applied to rotation field first
        approx_cmp_quaternion(transform.rotation, initial_rotation * rot);
        // Check for dirty flag
        assert!(transform.is_dirty);

        transform.update();
    }
}
