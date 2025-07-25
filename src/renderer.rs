use thiserror::Error;

use crate::math_utils::vector::Vector2;
use crate::math_utils::FloatType;
use crate::object::Object;
use crate::camera::Camera;

const VERTEX_SIZE: usize = 13;
const VERTEX_COLOR: Color = Color::WHITE;

#[derive(Clone, Copy)]
pub struct Color(u32);

impl Color {
    // Common colors
    const WHITE: Self = Self(0x00_ff_ff_ff);
    const BLACK: Self = Self(0x00_00_00_00);
    const RED: Self = Self(0x00_ff_00_00);
    const GREEN: Self = Self(0x00_00_ff_00);
    const BLUE: Self = Self(0x00_00_00_ff);

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

#[derive(Debug, Error, PartialEq)]
pub enum RendererError {
    #[error("Attempted to draw out of buffer bounds")]
    OutOfBounds(Vector2<usize>)
}

impl Renderer {
    pub fn render(&self, obj: &Object, camera: &Camera, buffer: &mut [u32]) -> Result<(), RendererError> {
        for vert in &obj.mesh.vertices {
            let world_pos = obj.transform.local_to_world(*vert);
            let cam_pos = camera.transform.world_to_local(world_pos);
            let ncd_pos = camera.project_to_ncd_space(cam_pos);
            let screen_pos = Vector2::new([
                (((ncd_pos.x() + 1.0) * 0.5) * self.buffer_width as FloatType) as usize,
                (((ncd_pos.y() + 1.0) * 0.5) * self.buffer_height as FloatType) as usize
            ]);

            self.draw_vertex(buffer, screen_pos)?;
        }
        // Perform rasterization
        // Draw to buffer

        Ok(())
    }

    // Draw a square of `SIDE_LENGTH` centered at `center`
    fn draw_vertex(&self, buffer: &mut [u32], center: Vector2<usize>) -> Result<(), RendererError> {
        const HALF_SIDE: usize = VERTEX_SIZE / 2;

        let start = Vector2::new([
            center.x().saturating_sub(HALF_SIDE),
            center.y().saturating_sub(HALF_SIDE)
        ]);

        for x in 0..=2*HALF_SIDE {
            for y in 0..=2*HALF_SIDE {
                self.draw_pixel(
                    buffer,
                    start + Vector2::<usize>::new([x, y]),
                    VERTEX_COLOR
                )?;
            }
        }

        Ok(())
    }

    fn draw_pixel(&self, buffer: &mut [u32], position: Vector2<usize>, color: Color) -> Result<(), RendererError> {
        if let Some(pixel) = buffer.get_mut(position.x() + (position.y() * self.buffer_height)) {
            *pixel = color.u32_color();
        }
        else {
            return Err(RendererError::OutOfBounds(position));
        }

        Ok(())
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
