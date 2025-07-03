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

        let tan_half_pov = (fov * 0.5).tan();
        let near_far_diff = f - n;
        Self (
            // width scaler
            1.0 / (ASPECT_RATIO * tan_half_pov),
            // height scaler
            1.0 / tan_half_pov,
            // z scaler
            1.0 / near_far_diff,
            // z repositioning
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
}
