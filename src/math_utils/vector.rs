use std::{fmt::Debug, ops::{Index, IndexMut}};
use num_traits::{Num, Float};


#[derive(Clone, Copy)]
pub struct Vector<const N: usize, T>([T; N]);

pub type Vector3<T> = Vector<3,T>;
pub type Vector4<T> = Vector<4,T>;
pub type Vector2<T> = Vector<2,T>;

impl<const N: usize, T> Vector<N,T>
where T: Num + Copy {
    pub fn new(arr: [T; N]) -> Self {
        Self(arr)
    }

    pub const SIZE: usize = N;

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

impl<const N: usize, T> Index<usize> for Vector<N,T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize, T> IndexMut<usize> for Vector<N,T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const N: usize, T> PartialEq for Vector<N,T>
where T: Float {
    fn eq(&self, other: &Self) -> bool {
        self.0.into_iter()
            .zip(other.0.into_iter())
            .all(|(a,b)| (a-b).abs() < T::epsilon())
    }
}

impl<const N: usize, T> Debug for Vector<N,T>
where T: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector{}{:?}", N, self.0)
    }
}

macro_rules! vector [
    ($val:expr; $count:literal) => {
        Vector::new([$val; $count])
    };

    ($val:expr) => {
        Vector::new([$val])
    };

    ($($vals:expr),+) => {
        Vector::new([$($vals),+])
    }
];

pub(crate) use vector;
