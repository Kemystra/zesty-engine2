use thiserror::Error;

use crate::math_utils::vector::{Vector2, Vector3};
use crate::math_utils::FloatType;
use crate::object::Object;
use crate::camera::Camera;
use crate::transform::Transform;

const VERTEX_SIZE: usize = 13;
const VERTEX_COLOR: Color = Color::WHITE;
const EDGE_COLOR: Color = Color::WHITE;

#[derive(Clone, Copy)]
pub struct Color(u32);

impl Color {
    // Common colors
    pub const WHITE: Self = Self(0x00_ff_ff_ff);
    pub const BLACK: Self = Self(0x00_00_00_00);
    pub const RED: Self = Self(0x00_ff_00_00);
    pub const GREEN: Self = Self(0x00_00_ff_00);
    pub const BLUE: Self = Self(0x00_00_00_ff);

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self (
            ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
        )
    }

    pub fn u32_color(&self) -> u32 {
        self.0
    }
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum RenderType {
    Vertex,
    Edge,
    Face
}

#[derive(Debug)]
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
    pub fn render(&self, obj: &Object, camera: &Camera, buffer: &mut [u32], render_type: RenderType) -> Result<(), RendererError> {
        match render_type {
            RenderType::Vertex => self.vertex_render(obj, camera, buffer),
            RenderType::Edge => self.edge_render(obj, camera, buffer),
            RenderType::Face => unimplemented!("Not implemented yet!")
        }
    }

    pub fn vertex_render(&self, obj: &Object, camera: &Camera, buffer: &mut [u32]) -> Result<(), RendererError> {
        for vert in &obj.mesh.vertices {
            let screen_pos = self.obj_space_to_screen_space(*vert, &obj.transform, camera);
            self.draw_vertex(buffer, screen_pos)?;
        }
        // Perform rasterization
        // Draw to buffer

        Ok(())
    }

    pub fn edge_render(&self, obj: &Object, camera: &Camera, buffer: &mut [u32]) -> Result<(), RendererError> {
        for face in &obj.mesh.faces {
            for i in 0..3 {
                let edge = [face[i], face[(i+1) % 3]];
                let p1 = self.obj_space_to_screen_space(
                    obj.mesh.vertices[edge[0] as usize],
                    &obj.transform,
                    camera
                );
                let p2 = self.obj_space_to_screen_space(
                    obj.mesh.vertices[edge[1] as usize],
                    &obj.transform,
                    camera
                );
                self.bresenham_line(
                    EDGE_COLOR,
                    buffer,
                    p1.x() as isize, p1.y() as isize,
                    p2.x() as isize, p2.y() as isize
                );
            }
        }

        Ok(())
    }

    fn obj_space_to_screen_space(&self, position: Vector3<FloatType>, obj_transform: &Transform, camera: &Camera) -> Vector2<usize> {
        let world_pos = obj_transform.local_to_world(position);
        let cam_pos = camera.transform.world_to_local(world_pos);
        let ncd_pos = camera.project_to_ncd_space(cam_pos);
        let screen_pos = Vector2::new([
            (((ncd_pos.x() + 1.0) * 0.5) * self.buffer_width as FloatType) as usize,
            (((ncd_pos.y() + 1.0) * 0.5) * self.buffer_height as FloatType) as usize
        ]);

        screen_pos
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

    fn bresenham_line(
        &self, color: Color,
        buffer: &mut [u32],
        x0: isize, y0: isize,
        end_x: isize, end_y: isize) {

        let mut curr_x = x0;
        let mut curr_y = y0;

        let dx = (end_x - curr_x).abs();
        let dy = -(end_y - curr_y).abs();
        let mut error = dx + dy;

        let sx = if curr_x < end_x {1} else {-1};
        let sy = if curr_y < end_y {1} else {-1};

        loop {
            self.draw_pixel(buffer, Vector2::new([curr_x as usize, curr_y as usize]), color);
            if curr_x == end_x && curr_y == end_y {break}
            let e2 = error * 2;

            if e2 >= dy {
                if curr_x == end_x {break}
                error += dy;
                curr_x += sx;
            }

            if e2 <= dx {
                if curr_y == end_y {break}
                error += dx;
                curr_y += sy
            }
        }
    }

    pub fn draw_pixel(&self, buffer: &mut [u32], position: Vector2<usize>, color: Color) -> Result<(), RendererError> {
        if let Some(pixel) = buffer.get_mut(position.x() + (position.y() * self.buffer_width)) {
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


#[cfg(test)]
pub mod tests {

    use super::*;
    const TEST_BUFFER_WIDTH: usize = 100;
    const TEST_BUFFER_HEIGHT: usize = 50;
    const TEST_BUFFER_SIZE: usize = TEST_BUFFER_WIDTH * TEST_BUFFER_HEIGHT;

    fn init_renderer_and_buffer() -> (Renderer, [u32; TEST_BUFFER_SIZE]) { 
        let mut renderer = Renderer::new();
        let buffer = [0_u32; TEST_BUFFER_SIZE];
        renderer.update_buffer_size(TEST_BUFFER_WIDTH, TEST_BUFFER_HEIGHT);

        (renderer, buffer)
    }

    #[test]
    fn test_draw_pixel() {
        let (renderer, mut buffer) = init_renderer_and_buffer();
        renderer.draw_pixel(
            &mut buffer,
            Vector2::new([10, 90]),
            Color::WHITE
        ).unwrap();

        let mut correct_buffer = [0_u32; TEST_BUFFER_SIZE];
        correct_buffer[9010] = Color::WHITE.u32_color();

        assert_eq!(buffer, correct_buffer);
    }

    #[test]
    fn test_draw_pixel_out_of_bounds() {
        let (renderer, mut buffer) = init_renderer_and_buffer();
        let result = renderer.draw_pixel(
            &mut buffer,
            Vector2::new([100, 100]),
            Color::WHITE
        );

        assert_eq!(
            result,
            Err(RendererError::OutOfBounds(Vector2::new([100, 100])))
        )
    }

    #[test]
    fn test_draw_vertex() {
        let (renderer, mut buffer) = init_renderer_and_buffer();
        renderer.draw_vertex(&mut buffer, Vector2::new([20,20])).unwrap();

        // manually draw the vertex
        let mut correct_buffer = [0_u32; TEST_BUFFER_SIZE];
        for x in 14..=26 {
            for y in 14..=26 {
                renderer.draw_pixel(&mut correct_buffer, Vector2::new([x, y]), VERTEX_COLOR).unwrap();
            }
        }

        assert_eq!(buffer, correct_buffer);
    }
}
