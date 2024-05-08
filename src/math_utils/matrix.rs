use zesty_engine2_derive::Matrix;


pub(super) trait Matrix {}

#[derive(Matrix)]
pub struct Matrix3([[f32; 3]; 3]);

#[derive(Matrix)]
pub struct Matrix4([[f32; 4]; 4]);
}
