use crate::scene::Scene;

pub struct Renderer {
}

impl Renderer {
    pub fn render(scene: Scene, buffer: &mut [u32]) {
        // Read scene
        // Open and load 3D objects
        // Perform rasterization
        // Draw to buffer
    }

    pub fn new() -> Self {
        Self {}
    }
}
