use zesty_engine2_derive::Matrix;



pub(super) trait Matrix: Sized {
    const SIZE: usize;
    const IDENTITY: Self;

    fn invert(&self) -> Result<Self, String>;
}

#[derive(Matrix, Clone)]
#[matrix(3)]
pub struct Matrix3([[f32; 3]; 3]);

#[derive(Matrix, Clone)]
#[matrix(4)]
pub struct Matrix4([[f32; 4]; 4]);
}
