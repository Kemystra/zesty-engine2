use super::Float;


pub struct Vector<const N: usize>([Float; N]);

pub type Vector3 = Vector<3>;
pub type Vector4 = Vector<4>;
pub type Vector2 = Vector<2>;

impl<const N: usize> Vector<N> {
    pub fn x(&self) -> Float {
        
    }
}
