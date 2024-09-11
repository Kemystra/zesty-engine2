use crate::object::Object;
use crate::math_utils::{vector, matrix};
use crate::camera::Camera;
use vector::Vector3;


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
    pub fn render(&self, obj: &Object, camera: &Camera, buffer: &mut [u32]) {
        obj.mesh().vertices.iter().for_each(|vert| {
            let pos = Vector3::new(vert.position);
            let world_pos = obj.transform().local_to_world(pos);
        })
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
