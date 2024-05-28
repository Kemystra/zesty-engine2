use num_traits::Num;


pub(crate) trait Vector {
}

pub struct Vector3<T> {
    x: T,
    y: T,
    z: T
}

pub struct Vector2<T> {
    x: T,
    y: T,
}
