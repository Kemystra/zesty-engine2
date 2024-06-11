use std::ops::{Index, IndexMut};

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
        self.0.into_iter().reduce(|acc, e| acc + e*e).unwrap()
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quaternion_from_euler_angle_x_only() {
        let q = Quaternion::from_euler_angles(1.0, 0.0, 0.0);
        assert_eq!(q, Quaternion([0.87758, 0.47943, 0.0, 0.0]));
    }

    #[test]
    fn quaternion_from_euler_angle_y_only() {
        let q = Quaternion::from_euler_angles(0.0, 1.0, 0.0);
        assert_eq!(q, Quaternion([0.87758, 0.0, 0.47943, 0.0]));
    }

    #[test]
    fn quaternion_from_euler_angle_z_only() {
        let q = Quaternion::from_euler_angles(0.0, 0.0, 1.0);
        assert_eq!(q, Quaternion([0.87758, 0.0, 0.0, 0.47943]));
    }

    #[test]
    fn quaternion_from_euler_angle_all() {
        let q = Quaternion::from_euler_angles(1.0, 1.0, 1.0);
        assert_eq!(q, Quaternion([0.78607, 0.16752, 0.57094, 0.16752]));
    }
}
