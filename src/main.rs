use winit::event_loop::{ControlFlow, EventLoop};
use clap::Parser;

use zesty_engine2::{App, Arguments};


fn main() {
    let args = Arguments::parse();
    let mut app = App::new(args);

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();
}
