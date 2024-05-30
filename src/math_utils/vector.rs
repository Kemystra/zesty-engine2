use super::Float;


pub struct Vector<const N: usize>([Float; N]);

pub type Vector3 = Vector<3>;
pub type Vector4 = Vector<4>;
pub type Vector2 = Vector<2>;

impl<const N: usize> Vector<N> {
    pub fn x(&self) -> Float {
        self.0[0]
    }

    pub fn y(&self) -> Float {
        self.0[1]
    }

    pub fn z(&self) -> Float {
        self.0[2]
    }

    pub fn set_x(&mut self, value: Float) -> () {
        self.0[0] = value;
    }

    pub fn set_y(&mut self, value: Float) -> () {
        self.0[1] = value;
    }

    pub fn set_z(&mut self, value: Float) -> () {
        self.0[2] = value;
    }
}
