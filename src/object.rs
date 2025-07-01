use tobj::{load_obj, LoadError, Mesh};

use crate::transform::Transform;


pub struct Object {
    transform: Transform,
    mesh: Mesh
}

impl Object {
    pub fn new(filename: &str) -> Result<Self, LoadError> {
        let (mut models, _) = load_obj(filename, &tobj::GPU_LOAD_OPTIONS)?;

        Ok(Self {
            transform: Transform::default(),
            // Get the first model, since we are assuming there will only be 1 mesh
            mesh: models.swap_remove(0).mesh
        })
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }
}
