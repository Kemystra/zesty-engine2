use std::vec;

use tobj::{load_obj, LoadError};

use crate::transform::Transform;
use crate::math_utils;
use math_utils::FloatType;
use math_utils::vector::Vector3;


pub struct Object {
    pub transform: Transform,
    pub mesh: Mesh
}

pub type Face = [u32; 3];

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vector3<FloatType>>,
    pub faces: Vec<Face>
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: vec![],
            faces: vec![]
        }
    }
}

impl Object {
    pub fn new(filename: &str) -> Result<Self, LoadError> {
        let (mut models, _) = load_obj(filename, &tobj::GPU_LOAD_OPTIONS)?;
        let tobj_mesh = models.swap_remove(0).mesh;

        println!("Indices: {:?}", tobj_mesh.indices);
        println!("Posiitons: {:?}", tobj_mesh.positions);

        // MVP is to implement only OBJ files
        // so we just directly convert it to our mesh format here
        // Best way should be to make different FileLoaders for each filetype

        let mut mesh = Mesh::new();
        for i in 0..(tobj_mesh.indices.len() / 3) {
            let face = [
                tobj_mesh.indices[i * 3],
                tobj_mesh.indices[i * 3 + 1],
                tobj_mesh.indices[i * 3 + 2]
            ];

            mesh.faces.push(face);
        }

        for i in 0..(tobj_mesh.positions.len() / 3) {
            let vertex = [
                tobj_mesh.positions[i * 3],
                tobj_mesh.positions[i * 3 + 1],
                tobj_mesh.positions[i * 3 + 2]
            ];

            mesh.vertices.push(Vector3::new(vertex));
        }

        println!("Processed mesh:\n{:?}", mesh);

        Ok(Self {
            transform: Transform::default(),
            // Get the first model, since we are assuming there will only be 1 mesh
            mesh
        })
    }
}
