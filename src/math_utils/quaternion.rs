use std::cmp::PartialEq;

use crate::math_utils::FloatType;


pub struct Quaternion([FloatType; 4]);

#[cfg(test)]
impl PartialEq for Quaternion {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter()
            .zip(other.0.iter())
            .all(|(a,b)| (a-b).abs() < FloatType::EPSILON)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quaternion_from_euler_angle_x_only() {
        let q = Quaternion::from_euler_angles(1,0,0);
        assert_eq!(q, Quaternion([0.87758, 0.47943, 0.0, 0.0]));
    }

    #[test]
    fn quaternion_from_euler_angle_y_only() {
        let q = Quaternion::from_euler_angles(0,1,0);
        assert_eq!(q, Quaternion([0.87758, 0.0, 0.47943, 0.0]));
    }

    #[test]
    fn quaternion_from_euler_angle_z_only() {
        let q = Quaternion::from_euler_angles(0,0,1);
        assert_eq!(q, Quaternion([0.87758, 0.0, 0.0, 0.47943]));
    }

    #[test]
    fn quaternion_from_euler_angle_all() {
        let q = Quaternion::from_euler_angles(1,1,1);
        assert_eq!(q, Quaternion([0.78607, 0.16752, 0.57094, 0.16752]));
    }
}
