use std::{fmt::Debug, ops::{Index, IndexMut, Add}};
use std::default::Default;

use num_traits::{Float, Num};
use float_cmp::ApproxEq;


#[derive(Clone, Copy, PartialEq)]
pub struct Vector<const N: usize, T>([T; N]);

pub type Vector3<T> = Vector<3,T>;
pub type Vector4<T> = Vector<4,T>;
pub type Vector2<T> = Vector<2,T>;

impl<const N: usize, T: Num + Copy> Vector<N,T>
where T: Num + Copy {
    pub fn new(arr: [T; N]) -> Self {
        Self(arr)
    }

    pub const SIZE: usize = N;

    // Construct a vector that has value 1 in all of its field
    pub fn one() -> Self {
        Self([T::one(); N])
    }

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

impl<const N: usize, T: Num + Copy> Default for Vector<N,T> {
    fn default() -> Self {
        vector![T::zero(); N]
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

impl<const N: usize, T: Float + ApproxEq> ApproxEq for Vector<N,T> {
    type Margin = <T as ApproxEq>::Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        for i in 0..N {
            if !self[i].approx_eq(other[i], margin) { return false }
        }

        true
    }
}

/* Arithmetic Operations */
impl<const N: usize, T: Num + Copy> Add for Vector<N, T> {
    type Output = Vector<N, T>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut output = Vector::<N,T>::new([T::zero(); N]);
        for i in 0..N {
            output.0[i] = self.0[i] + rhs.0[i];
        }

        output
    }
}

impl<const N: usize, T> Debug for Vector<N,T>
where T: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector{}{:?}", N, self.0)
    }
}

#[macro_export]
macro_rules! vector [
    ($val:expr; $count:expr) => {
        Vector::new([$val; $count])
    };

    ($val:expr) => {
        Vector::new([$val])
    };

    ($($vals:expr),+) => {
        Vector::new([$($vals),+])
    }
];

pub use vector;

pub mod prelude {
    pub use super::{Vector, vector, Vector2, Vector3, Vector4};
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use float_cmp::approx_eq;
    use num_traits::Float;

    pub fn approx_cmp_vector<const N: usize, T: Float + ApproxEq>
    (vec1: Vector<N,T>, vec2: Vector<N,T>) {
        assert!(approx_eq!(Vector<N,T>, vec1, vec2));
    }
}
