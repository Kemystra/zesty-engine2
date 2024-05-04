use winit::event_loop::{ControlFlow, EventLoop};
use clap::Parser;


mod app;

use app::App;

#[derive(Parser, Debug)]
struct Args {
    filename: String
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
