use std::ops::{Index, IndexMut};

use super::matrix::Matrix4;
use super::vector::*;
use crate::math_utils::FloatType;

// This implementation of Quaternion does not care about the magnitude of itself. That is, it might
// not be a unit quaternion.
//
// Parameter arrangement in this implementation is Quaternion([w, x, y, z])
#[derive(Debug)]
pub struct Quaternion([FloatType; 4]);

#[cfg(test)]
impl PartialEq for Quaternion {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter()
            .zip(other.0.iter())
            .all(|(a,b)| (a-b).abs() < FloatType::EPSILON)
    }
}

impl Index<usize> for Quaternion {
    type Output = FloatType;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Quaternion {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Quaternion {
    pub fn sq_magnitude(&self) -> FloatType {
        self.0.into_iter().map(|e| e*e).reduce(|acc, e| acc+e).unwrap()
    }

    pub fn from_euler_angles<T: Into<FloatType>>(x: T, y: T, z: T) -> Self
    {
        // Got this abomination from Wikipedia lul
        // https://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles
        let a = x.into() * 0.5;
        let ca = a.cos();
        let sa = a.sin();

        let b = y.into() * 0.5;
        let cb = b.cos();
        let sb = b.sin();

        let c = z.into() * 0.5;
        let cc = c.cos();
        let sc = c.sin();

        Quaternion([
            cc*cb*ca + sc*sb*sa,
            cc*cb*sa - sc*sb*ca,
            cc*sb*ca + sc*cb*sa,
            sc*cb*ca - cc*sb*sa
        ])
    }

    fn scale(&mut self, num: FloatType) -> () {
        self.0.iter_mut().for_each(|x| *x *= num);
    }

    pub fn normalize(&mut self) -> () {
        let sq_magnitude = self.sq_magnitude();
        // The epsilon value is calculated by:
        // https://johannesugb.github.io/cpu-programming/tools/floating-point-epsilon-calculator/
        // Might not be the perfect value
        if (1.0 - sq_magnitude).abs() < 1.19209e-07 {
            // Based on Padé approximation. See:
            // https://stackoverflow.com/questions/11667783/quaternion-and-normalization
            self.scale(2.0 / (1.0 + sq_magnitude));
        }
        else {
            self.scale(1.0 / sq_magnitude.sqrt());
        }
    }

    pub fn edit_3d_matrix(&self, matrix: &mut Matrix4, original_scale: Vector3<FloatType>) -> () {
        let wx = self[0] * self[1] * 2.0;
        let wy = self[0] * self[2] * 2.0;
        let wz = self[0] * self[3] * 2.0;

        let xx = self[1] * self[1] * 2.0;
        let xy = self[1] * self[2] * 2.0;
        let xz = self[1] * self[3] * 2.0;

        let yy = self[2] * self[2] * 2.0;
        let yz = self[2] * self[3] * 2.0;

        let zz = self[3] * self[3] * 2.0;

        matrix[0][0] = (1.0 - yy - zz) * original_scale.x();
        matrix[0][1] = xy - wz;
        matrix[0][2] = xz + wy;
        matrix[1][0] = xy + wz;
        matrix[1][1] = (1.0 - xx - zz) * original_scale.y();
        matrix[1][2] = yz - wx;
        matrix[2][0] = xz - wy;
        matrix[2][1] = yz + wx;
        matrix[2][2] = (1.0 - xx - yy) * original_scale.z();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quaternion_from_euler_angle_x_only() {
        let q = Quaternion::from_euler_angles(1.0, 0.0, 0.0);
        assert_eq!(q, Quaternion([0.87758255, 0.47942555, 0.0, 0.0]));
    }

    #[test]
    fn quaternion_from_euler_angle_y_only() {
        let q = Quaternion::from_euler_angles(0.0, 1.0, 0.0);
        assert_eq!(q, Quaternion([0.87758255, 0.0, 0.47942555, 0.0]));
    }

    #[test]
    fn quaternion_from_euler_angle_z_only() {
        let q = Quaternion::from_euler_angles(0.0, 0.0, 1.0);
        assert_eq!(q, Quaternion([0.87758255, 0.0, 0.0, 0.47942555]));
    }

    #[test]
    fn quaternion_from_euler_angle_all() {
        let q = Quaternion::from_euler_angles(1.0, 1.0, 1.0);
        assert_eq!(q, Quaternion([0.7860666, 0.16751876, 0.5709415, 0.1675188]));
    }

    #[test]
    fn test_quaternion_squared_magnitude_getter() {
        let q = Quaternion([2.0, 8.0, 3.0, 1.0]);
        assert_eq!(q.sq_magnitude(), 78.0);
    }

    #[test]
    fn test_normalize_quaternion_1_component() {
        let mut q = Quaternion([69.0, 0.0, 0.0, 0.0]);
        q.normalize();

        assert_eq!(q, Quaternion([1.0, 0.0, 0.0, 0.0]));
    }

    #[test]
    fn test_normalize_quaternion_all_component() {
        let mut q = Quaternion([89.0, 89.0, 89.0, 89.0]);
        q.normalize();

        assert_eq!(q, Quaternion([0.5, 0.5, 0.5, 0.5]));
    }

    #[test]
    fn test_edit_matrix4_rotation() {
        // Value based on Euler angle (45, 45, 90). See
        // https://www.andre-gaschler.com/rotationconverter/
        let q = Quaternion([0.5719523, 0.3348807, 0.3348807, 0.6697614]);
        let mut mat = Matrix4::identity_matrix();

        q.edit_matrix(&mut mat, vector![1.0, 1.0, 1.0]);

        assert_eq!(mat, Matrix4::new([
            [-0.1214509, -0.5418530, 0.8316519, 0.0],
            [0.9904334, -0.1214509, 0.0655087, 0.0],
            [0.0655087, 0.8316519, 0.5514197, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]))
    }
}
