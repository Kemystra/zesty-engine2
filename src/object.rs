use crate::transform::Transform;
use crate::mesh::Mesh;


pub struct Object {
    transform: Transform,
    mesh: Option<Mesh>
}
