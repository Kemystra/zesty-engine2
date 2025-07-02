use crate::transform::Transform;


pub struct Camera {
    pub transform: Transform
}

impl Camera {
    pub fn new() -> Self {
        Self {
            transform: Transform::default()
        }
    }
}
