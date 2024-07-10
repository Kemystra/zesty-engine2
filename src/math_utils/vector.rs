use std::{fmt::Debug, ops::{Index, IndexMut}};
use num_traits::Num;


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

impl<const N: usize, T> Debug for Vector<N,T>
where T: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector{}{:?}", N, self.0)
    }
}

#[macro_export]
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

pub use vector;

pub mod prelude {
    pub use super::{Vector, vector, Vector2, Vector3, Vector4};
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use float_cmp::{approx_eq, ApproxEq};
    use num_traits::Float;

    impl<const N: usize, T: Float + ApproxEq> ApproxEq for Vector<N,T> {
        type Margin = <T as ApproxEq>::Margin;

        fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
            let margin = margin.into();
            self.0.into_iter()
                .zip(other.0.into_iter())
                .all(|(x,y)| x.approx_eq(y, margin))
        }
    }

    pub fn approx_cmp_vector<const N: usize, T: Float + ApproxEq>
    (vec1: Vector<N,T>, vec2: Vector<N,T>) {
        assert!(approx_eq!(Vector<N,T>, vec1, vec2));
    }
}
