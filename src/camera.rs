use crate::transform::Transform;
use crate::math_utils::matrix::Matrix4;
use crate::math_utils::FloatType;

const ASPECT_RATIO : FloatType = 16.0 / 9.0;

pub struct Camera {
    pub transform: Transform,
    pub projection_matrix: Matrix4,
    near_plane_distance: FloatType,
    vertical_fov_angle: FloatType
}

impl Camera {
    pub fn new(n: FloatType, fov: FloatType) -> Self {
        Self {
            transform: Transform::default(),
            projection_matrix: Matrix4::default(),
            near_plane_distance: n,
            vertical_fov_angle: fov
        }
    }
}
