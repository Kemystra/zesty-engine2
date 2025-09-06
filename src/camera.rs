use std::f32::consts::PI;

use crate::math_utils::vector::Vector3;
use crate::transform::Transform;
use crate::math_utils::FloatType;

const ASPECT_RATIO : FloatType = 16.0 / 9.0;

pub struct Camera {
    pub transform: Transform,
    near_plane_distance: FloatType,
    far_plane_distance: FloatType,
    vertical_fov_angle: FloatType,
    projection_data: ProjectionData
}

#[derive(Debug)]
pub struct ProjectionData(FloatType, FloatType, FloatType, FloatType);

impl ProjectionData {
    pub fn generate<T: Into<FloatType> + Copy>(n: T, f: T, fov: T)
    -> Self {
        let n = n.into();
        let f = f.into();
        let fov = fov.into();

        let tan_half_pov = (fov * 0.5 * PI/180.0).tan();
        let near_far_diff = f - n;
        Self (
            // width scaler
            1.0 / (ASPECT_RATIO * tan_half_pov),
            // height scaler
            1.0 / tan_half_pov,
            // z scaler * m1
            1.0 / near_far_diff,
            // z repositioning * m2
            n / near_far_diff
        )
    }
}

impl Camera {
    pub fn new<T: Into<FloatType> + Copy>(n: T, f: T, fov: T) -> Self {
        let n = n.into();
        let f = f.into();
        let fov = fov.into();

        Self {
            transform: Transform::default(),
            near_plane_distance: n,
            far_plane_distance: f,
            vertical_fov_angle: fov,
            projection_data: ProjectionData::generate(
                n, f, fov
            )
        }
    }

    pub fn project_to_ncd_space(&self, point: Vector3<FloatType>) -> Vector3<FloatType> {
        let ProjectionData(w_scaler, h_scaler, z_1, z_2) = self.projection_data;
        Vector3::new([
            (point.x() * w_scaler) / point.z(),
            (point.y() * h_scaler) / point.z(),
            ((point.z() * z_1) + z_2) / point.z()
        ])
    }

    pub fn update_projection_data(&mut self) {
        self.projection_data = ProjectionData::generate(
            self.near_plane_distance,
            self.far_plane_distance,
            self.vertical_fov_angle
        );
    }
}
