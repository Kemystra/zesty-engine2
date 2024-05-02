use std::rc::Rc;
use std::time::Instant;
use std::num::NonZeroU32;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use softbuffer::{Context, Surface};


fn main() {
    #[derive(Default)]
    struct App {
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
                    for index in 0..(width * height) {
                        let y = (index + self.redraw_count as u32) / width;
                        let x = (index + self.redraw_count as u32) % width;
                        let red = x % 255;
                        let green = y % 255;
                        let blue = (x * y) % 255;

                        buffer[index as usize] = blue | (green << 8) | (red << 16);
                    }

                    buffer.present().unwrap();

                    self.redraw_count += 1;
                    println!("Redraw count: {}, Time: {}",
                        self.redraw_count,
                        now.elapsed().as_millis()
                    );

                },

                _ => (),
            }
        }

        fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
            self.window.as_ref().unwrap().request_redraw();
        }
    }

    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
