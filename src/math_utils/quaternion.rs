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
