use crate::scene::Scene;


pub struct Color(u32);

impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self (
            ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
        )
    }

    pub fn u32_color(&self) -> u32 {
        self.0
    }
}

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
