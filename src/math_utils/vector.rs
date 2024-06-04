use num_traits::Num;


#[derive(Clone, Copy)]
pub struct Vector<const N: usize, T>([T; N]);

pub type Vector3<T> = Vector<3,T>;
pub type Vector4<T> = Vector<4,T>;
pub type Vector2<T> = Vector<2,T>;

impl<const N: usize, T> Vector<N,T>
where T: Num + Copy {
    pub fn x(&self) -> T {
        self.0[0]
    }

    pub fn y(&self) -> T {
        self.0[1]
    }

    pub fn z(&self) -> T {
        self.0[2]
    }

    pub fn set_x(&mut self, value: T) -> () {
        self.0[0] = value;
    }

    pub fn set_y(&mut self, value: T) -> () {
        self.0[1] = value;
    }

    pub fn set_z(&mut self, value: T) -> () {
        self.0[2] = value;
    }

    pub fn array(&self) -> &[T; N] {
        &self.0
    }
}

macro_rules! vector {
    ($val:expr) => {
        Vector([$val])
    };

    ($($vals:expr),+) => {
        Vector([$($vals),+])
    }
}

pub(crate) use vector;
