use num_traits::Num;


pub struct Vector4<T: Num> {
    x: T,
    y: T,
    z: T,
    w: T
}

pub struct Vector3<T: Num> {
    x: T,
    y: T,
    z: T
}

pub struct Vector2<T: Num> {
    x: T,
    y: T,
}
