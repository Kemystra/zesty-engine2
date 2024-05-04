use std::rc::Rc;
use std::time::Instant;
use std::num::NonZeroU32;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use softbuffer::{Context, Surface};


#[derive(Default)]
pub struct App {
    window: Option<Rc<Window>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    redraw_count: usize
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Rc::new(event_loop.create_window(
            Window::default_attributes()).unwrap()
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
                let (width, height) = {
                    let size = window_ref.inner_size();
                    (size.width, size.height)
                };

                let surface_mut_ref = self.surface.as_mut().unwrap();
                surface_mut_ref.resize(
                    NonZeroU32::new(width).unwrap(),
                    NonZeroU32::new(height).unwrap(),
                ).unwrap();

                let mut buffer = surface_mut_ref.buffer_mut().unwrap();
                // Render here
                buffer.present().unwrap();

                self.redraw_count += 1;
                println!("Redraw count: {}, FPS: {}",
                    self.redraw_count,
                    1_000_000 / now.elapsed().as_micros()
                );

            },

            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.as_ref().unwrap().request_redraw();
    }
}
