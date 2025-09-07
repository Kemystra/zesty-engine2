use std::rc::Rc;
use std::time::Instant;
use std::num::NonZeroU32;
use std::f32::consts::PI;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use winit::dpi;

use softbuffer::{Context, Surface};
use clap::Parser;

pub mod renderer;
pub mod math_utils;
pub mod transform;
pub mod object;
pub mod camera;
pub mod scene;

use crate::math_utils::quaternion::Quaternion;
use crate::renderer::{RenderType, Renderer};
use crate::camera::Camera;
use crate::scene::Scene;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    config_filename: String,

    #[arg(short, long)]
    render_type: renderer::RenderType
}

pub struct App {
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    redraw_count: usize,

    scene: Scene,
    renderer: Renderer,
    render_type: RenderType,
    camera: Camera
}

impl App {
    pub fn new(args: Arguments) -> Self {
        let scene = Scene::new(&args.config_filename);
        Self {
            window: None,
            surface: None,
            redraw_count: 0,

            scene,
            renderer: Renderer::new(),
            render_type: args.render_type,
            camera: Camera::new(1.0, 100.0, 60.0)
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attr = Window::default_attributes()
            .with_resizable(false)
            .with_inner_size(dpi::LogicalSize::new(768, 432));

        let window = Rc::new(
            event_loop.create_window(
                window_attr
            ).unwrap()
        );

        self.window = Some(Rc::clone(&window));
        let context = Context::new(Rc::clone(&window)).unwrap();
        self.surface = Some(Surface::new(&context, Rc::clone(&window)).unwrap());
        self.redraw_count = 0;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let window_ref = self.window.as_ref().unwrap();
        if id != window_ref.id() {
            return
        }

        let now = Instant::now();

        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },

            WindowEvent::RedrawRequested => {
                if self.redraw_count < 10000 {
                    let angle = 1.0 * (PI / 180.0);
                    self.scene.object.transform.rotate(
                        Quaternion::from_euler_angles(angle, angle, 0.0)
                    );
                    self.scene.object.transform.update();
                }
                    
                let (width, height) = {
                    let size = window_ref.inner_size();
                    (size.width, size.height)
                };

                let surface_mut_ref = self.surface.as_mut().unwrap();
                surface_mut_ref.resize(
                    NonZeroU32::new(width).unwrap(),
                    NonZeroU32::new(height).unwrap(),
                ).unwrap();

                self.renderer.update_buffer_size(width as usize, height as usize);

                let mut buffer = surface_mut_ref.buffer_mut().unwrap();

                // Clear the buffer first
                buffer.fill(0);

                // Render here
                self.renderer.render(&self.scene.object, &self.camera, &mut buffer, self.render_type).unwrap();
                buffer.present().unwrap();

                self.redraw_count += 1;
                //println!("Redraw count: {}, FPS: {}",
                //    self.redraw_count,
                //    1_000_000 / now.elapsed().as_micros()
                //);
            },

            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.as_ref().unwrap().request_redraw();
    }
}
