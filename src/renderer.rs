use crate::scene::Scene;

pub struct Renderer {
    buffer_width: usize,
    buffer_height: usize
}

impl Renderer {
    pub fn render(&self, scene: Scene, buffer: &mut [u32]) {
        // Read scene
        // Open and load 3D objects
        // Perform rasterization
        // Draw to buffer
    }

    pub fn new() -> Self {
        Self {
            buffer_width: 0,
            buffer_height: 0
        }
    }

    pub fn update_buffer_size(&mut self, width: usize, height: usize) {
        self.buffer_width = width;
        self.buffer_height = height;
    }
}
